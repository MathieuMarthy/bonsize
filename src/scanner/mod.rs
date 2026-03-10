pub mod file_model;

use crate::scanner::file_model::FileModel;
use std::fs;
use std::path::PathBuf;

pub fn get_directory_size(path: &PathBuf) -> FileModel {
    let mut root_file = FileModel::new(path.to_path_buf(), true, 0);
    scan_directory(&mut root_file);
    root_file
}

fn scan_directory(parent_dir: &mut FileModel) {
    let paths = match fs::read_dir(&parent_dir.path) {
        Ok(paths) => paths,
        Err(_) => {
            eprintln!("Error: Unable to read directory '{:?}'", parent_dir.path);
            return;
        }
    };

    for path in paths {
        let file = match path {
            Ok(file) => file,
            Err(_) => {
                eprintln!(
                    "Error: Unable to read a file in directory '{:?}'",
                    parent_dir.path
                );
                continue;
            }
        };
        let file_type = match file.file_type() {
            Ok(file_type) => file_type,
            Err(_) => {
                eprintln!(
                    "Error: Unable to determine the type of a file in directory '{:?}'",
                    parent_dir.path
                );
                continue;
            }
        };

        if file_type.is_symlink() {
            continue;
        }

        let mut file_model = FileModel::new(file.path(), file_type.is_dir(), parent_dir.depth + 1);

        if file_type.is_file() {
            file_model.size = file
                .metadata()
                .inspect_err(|_| {
                    eprintln!(
                        "Error: Unable to determine the size of file '{:?}'",
                        file_model.path
                    )
                })
                .map(|m| m.len())
                .unwrap_or(0);
        } else {
            scan_directory(&mut file_model);
        }

        parent_dir.size += file_model.size;
        parent_dir.children.push(file_model);
    }
}
