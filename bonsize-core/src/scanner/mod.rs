pub mod cache;
pub mod file_model;
pub mod file_size;

use crate::scanner::file_model::FileModel;
use crate::scanner::file_size::{
    ScanOptions, SizeType, get_file_size_logical, get_file_size_physical,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use std::fs::{self, Metadata};
use std::path::PathBuf;

pub fn get_directory_size(path: &PathBuf, options: &ScanOptions) -> FileModel {
    let mut root_file = FileModel::new(path.to_path_buf(), true, 0);

    let get_file_size = match options.size_type {
        SizeType::Logical => get_file_size_logical,
        SizeType::Physical => get_file_size_physical,
    };

    if let Ok(metadata) = fs::metadata(path) {
        root_file.size = get_file_size(&metadata);
    }

    scan_directory(&mut root_file, options, get_file_size);
    root_file
}

fn scan_directory(
    parent_dir: &mut FileModel,
    options: &ScanOptions,
    get_file_size: fn(&Metadata) -> u64,
) {
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

            file_model.size = file
                .metadata()
                .inspect_err(|_| {
                    if !options.quiet {
                        eprintln!(
                            "Error: Unable to determine the size of '{:?}'",
                            file_model.path
                        );
                    }
                })
                .map(|m| get_file_size(&m))
                .unwrap_or(0);

            if !file_type.is_file() {
                scan_directory(&mut file_model, options, get_file_size);
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
