pub mod file_model;
pub mod cache;

use crate::scanner::file_model::FileModel;
use std::fs;
use std::fs::Metadata;
use std::path::PathBuf;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use crate::config::BLOCK_SIZE;

#[cfg(unix)]
fn get_file_size(metadata: &Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.blocks() * BLOCK_SIZE
}

#[cfg(not(unix))]
fn get_file_size(metadata: &Metadata) -> u64 {
    metadata.len()
}

pub struct ScanOptions {
    pub quiet: bool, // suppress error messages (e.g., permission denied)
}

pub fn get_directory_size(path: &PathBuf, options: &ScanOptions) -> FileModel {
    let mut root_file = FileModel::new(path.to_path_buf(), true, 0);
    scan_directory(&mut root_file, options);
    root_file
}

fn scan_directory(parent_dir: &mut FileModel, options: &ScanOptions) {
    let entries: Vec<_> = match fs::read_dir(&parent_dir.path) {
        Ok(paths) => paths
            .filter_map(|entry| match entry {
                Ok(e) => Some(e),
                Err(_) => {
                    if !options.quiet {
                        eprintln!(
                            "Error: Unable to read a file in directory '{:?}'",
                            parent_dir.path
                        );
                    }
                    None
                }
            })
            .collect(),
        Err(_) => {
            if !options.quiet {
                eprintln!("Error: Unable to read directory '{:?}'", parent_dir.path);
            }
            return;
        }
    };

    let children: Vec<FileModel> = entries
        .into_par_iter()
        .filter_map(|file| {
            let file_type = match file.file_type() {
                Ok(ft) => ft,
                Err(_) => {
                    if !options.quiet {
                        eprintln!(
                            "Error: Unable to determine the type of a file in directory '{:?}'",
                            file.path()
                        );
                    }
                    return None;
                }
            };

            if file_type.is_symlink() {
                return None;
            }

            let depth = parent_dir.depth + 1;
            let mut file_model = FileModel::new(file.path(), file_type.is_dir(), depth);

            if file_type.is_file() {
                file_model.size = file
                    .metadata()
                    .inspect_err(|_| {
                        if !options.quiet {
                            eprintln!(
                                "Error: Unable to determine the size of file '{:?}'",
                                file_model.path
                            );
                        }
                    })
                    .map(|m| get_file_size(&m))
                    .unwrap_or(0);
            } else {
                scan_directory(&mut file_model, options);
            }

            Some(file_model)
        })
        .collect();

    for child in children {
        parent_dir.size += child.size;
        parent_dir.children.push(child);
    }

    parent_dir.children.par_sort_by(|a, b| {
        b.is_directory
            .cmp(&a.is_directory)
            .then_with(|| b.size.cmp(&a.size))
    });
}
