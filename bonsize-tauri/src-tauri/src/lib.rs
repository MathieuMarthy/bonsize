mod sandbox;

use std::{
    fs::{remove_dir_all, remove_file},
    path::{Path, PathBuf},
};

use bonsize_core::scanner::{
    file_model::FileModel,
    file_size::{ScanOptions, SizeType},
    get_directory_size,
};
use sandbox::Mode;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
async fn scan_directory(path: String, use_physical_size: bool) -> FileModel {
    let host_path = PathBuf::from(path);
    let options = ScanOptions {
        quiet: true,
        size_type: if use_physical_size {
            SizeType::Physical
        } else {
            SizeType::Logical
        },
    };

    let mut model = if sandbox::mode() == Mode::Partial && host_path == Path::new("/") {
        scan_partial_root(&options)
    } else {
        get_directory_size(&sandbox::to_sandbox(&host_path), &options)
    };

    sandbox::tree_to_host(&mut model);
    model
}

/// Flatpak without `host-root`: `/` is the sandbox, so build a fake `/`
/// from the host dirs we can reach.
fn scan_partial_root(options: &ScanOptions) -> FileModel {
    let mut root = FileModel::new(PathBuf::from("/"), true, 0);

    for entry in sandbox::partial_root_entries() {
        let mut child = get_directory_size(&entry, options);
        shift_depth(&mut child, 1);
        root.size += child.size;
        root.children.push(child);
    }

    root.children.sort_by_key(|c| std::cmp::Reverse(c.size));
    root
}

fn shift_depth(model: &mut FileModel, offset: usize) {
    model.depth += offset;
    for child in &mut model.children {
        shift_depth(child, offset);
    }
}

#[tauri::command]
async fn delete_file(path: String) -> bool {
    let path_buf = sandbox::to_sandbox(Path::new(&path));

    let function_to_delete = if path_buf.is_dir() {
        remove_dir_all
    } else {
        remove_file
    };

    match function_to_delete(&path_buf) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error deleting {}: {}", path_buf.display(), e);
            false
        }
    }
}

#[tauri::command]
async fn open_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let path_buf = sandbox::to_sandbox(Path::new(&path));

    app.opener()
        .open_path(path_buf.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    {
        // Workaround for WebKitGTK / Wayland / NVIDIA protocol errors (Error 71 dispatching to Wayland display)
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }

        // Force GTK dark theme variant because Bonsize UI is dark by default
        if std::env::var("GTK_THEME").is_err() {
            std::env::set_var("GTK_THEME", "Adwaita:dark");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_directory, delete_file, open_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
