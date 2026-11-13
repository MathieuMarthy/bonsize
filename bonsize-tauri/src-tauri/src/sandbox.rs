//! Path translation between the host file system and the Flatpak sandbox.
//!
//! Inside a Flatpak, `/` is the sandbox root: `/usr` is the runtime, `/app` is
//! the app, and the host's `/usr`, `/etc`, `/var`... are hidden or moved.
//! The frontend always works with host paths. Every path coming from the
//! frontend goes through `to_sandbox`, every path sent back goes through
//! `to_host`.
//!
//! Two Flatpak modes:
//! - `HostRoot`: `--filesystem=host-root` (Flatpak >= 1.17) mounts the whole
//!   host at `/run/host/root`. Full view.
//! - `Partial`: only `--filesystem=host`. `/home`, `/media`, `/mnt`, `/opt`,
//!   `/srv` keep their path, `/usr`, `/etc`, `/bin`... live in `/run/host`.
//!   `/var`, `/boot`, `/root`, `/tmp` are not reachable.

use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use bonsize_core::scanner::file_model::FileModel;

const RUN_HOST: &str = "/run/host";
const HOST_ROOT: &str = "/run/host/root";

/// Top-level host dirs exposed under `/run/host` by `host` / `host-os` / `host-etc`.
const HOST_OS_DIRS: &[&str] = &["usr", "etc", "bin", "sbin", "lib", "lib32", "lib64"];

/// Top-level host dirs shared at the same path by `--filesystem=host`.
const SHARED_DIRS: &[&str] = &["home", "media", "mnt", "opt", "srv"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Native,
    HostRoot,
    Partial,
}

pub fn mode() -> Mode {
    static MODE: OnceLock<Mode> = OnceLock::new();
    *MODE.get_or_init(detect_mode)
}

fn detect_mode() -> Mode {
    if !Path::new("/.flatpak-info").exists() {
        return Mode::Native;
    }

    let host_root_ok = std::fs::read_dir(HOST_ROOT)
        .map(|mut entries| entries.next().is_some())
        .unwrap_or(false);

    if host_root_ok {
        Mode::HostRoot
    } else {
        Mode::Partial
    }
}

/// First normal component of an absolute path (`/usr/lib` -> `usr`).
fn first_component(path: &Path) -> Option<&str> {
    path.components().find_map(|c| match c {
        Component::Normal(s) => s.to_str(),
        _ => None,
    })
}

fn strip_root(path: &Path) -> &Path {
    path.strip_prefix("/").unwrap_or(path)
}

/// Host path -> path usable inside this process.
pub fn to_sandbox(host: &Path) -> PathBuf {
    to_sandbox_in(mode(), host)
}

fn to_sandbox_in(mode: Mode, host: &Path) -> PathBuf {
    // Already a sandbox path (e.g. picked in the GTK dialog): keep it.
    if mode == Mode::Native || host.starts_with(RUN_HOST) || !host.is_absolute() {
        return host.to_path_buf();
    }

    match mode {
        Mode::HostRoot => Path::new(HOST_ROOT).join(strip_root(host)),
        Mode::Partial => match first_component(host) {
            Some(dir) if HOST_OS_DIRS.contains(&dir) => {
                Path::new(RUN_HOST).join(strip_root(host))
            }
            _ => host.to_path_buf(),
        },
        Mode::Native => unreachable!(),
    }
}

/// Path inside this process -> host path shown to the user.
pub fn to_host(sandbox: &Path) -> PathBuf {
    to_host_in(mode(), sandbox)
}

fn to_host_in(mode: Mode, sandbox: &Path) -> PathBuf {
    let prefix = match mode {
        Mode::Native => return sandbox.to_path_buf(),
        Mode::HostRoot => Path::new(HOST_ROOT),
        Mode::Partial => Path::new(RUN_HOST),
    };

    match sandbox.strip_prefix(prefix) {
        Ok(rest) if mode == Mode::HostRoot => Path::new("/").join(rest),
        Ok(rest) if first_component(rest).is_some_and(|d| HOST_OS_DIRS.contains(&d)) => {
            Path::new("/").join(rest)
        }
        _ => sandbox.to_path_buf(),
    }
}

/// Rewrite every path of a scanned tree to host paths.
pub fn tree_to_host(model: &mut FileModel) {
    if mode() == Mode::Native {
        return;
    }
    model.path = to_host(&model.path);
    for child in &mut model.children {
        tree_to_host(child);
    }
}

/// In `Partial` mode, `/` cannot be scanned as is: build a fake root from
/// the host dirs we can reach. Returns the sandbox paths to scan.
pub fn partial_root_entries() -> Vec<PathBuf> {
    HOST_OS_DIRS
        .iter()
        .map(|d| Path::new(RUN_HOST).join(d))
        .chain(SHARED_DIRS.iter().map(|d| Path::new("/").join(d)))
        // Skip missing dirs and merged-usr symlinks (/bin -> usr/bin).
        .filter(|p| p.symlink_metadata().is_ok_and(|m| m.is_dir()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn native_is_identity() {
        assert_eq!(to_sandbox_in(Mode::Native, &p("/usr/lib")), p("/usr/lib"));
        assert_eq!(to_host_in(Mode::Native, &p("/run/host/usr")), p("/run/host/usr"));
    }

    #[test]
    fn host_root_round_trip() {
        let m = Mode::HostRoot;
        assert_eq!(to_sandbox_in(m, &p("/")), p("/run/host/root/"));
        assert_eq!(to_sandbox_in(m, &p("/var/log")), p("/run/host/root/var/log"));
        assert_eq!(to_host_in(m, &p("/run/host/root")), p("/"));
        assert_eq!(to_host_in(m, &p("/run/host/root/var/log")), p("/var/log"));
    }

    #[test]
    fn partial_maps_os_dirs_only() {
        let m = Mode::Partial;
        assert_eq!(to_sandbox_in(m, &p("/usr/share")), p("/run/host/usr/share"));
        assert_eq!(to_sandbox_in(m, &p("/home/me")), p("/home/me"));
        assert_eq!(to_host_in(m, &p("/run/host/etc/fstab")), p("/etc/fstab"));
        assert_eq!(to_host_in(m, &p("/home/me")), p("/home/me"));
        // Other /run/host entries are not host dirs.
        assert_eq!(to_host_in(m, &p("/run/host/fonts")), p("/run/host/fonts"));
    }

    #[test]
    fn sandbox_paths_pass_through() {
        let m = Mode::HostRoot;
        assert_eq!(to_sandbox_in(m, &p("/run/host/root/var")), p("/run/host/root/var"));
    }
}
