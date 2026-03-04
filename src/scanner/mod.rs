pub mod file_model;

use crate::scanner::file_model::FileModel;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub fn get_directory_size(path: &PathBuf) -> Result<FileModel, Error> {
    let mut root_file = FileModel::new(path.to_path_buf(), true);
    scan_directory(&mut root_file)?;
    Ok(root_file)
}

fn scan_directory(parent_dir: &mut FileModel) -> Result<(), Error> {
    let paths = fs::read_dir(&parent_dir.path)?;

    for path in paths {
        let file = path?;
        let file_type = file.file_type()?;

        if file_type.is_symlink() {
            continue;
        }

        let mut file_model = FileModel::new(file.path(), file_type.is_dir());

        if file_type.is_file() {
            file_model.size = file.metadata()?.len();
        } else {
            scan_directory(&mut file_model)?;
        }

        parent_dir.size += file_model.size;
        parent_dir.children.push(file_model);
    }

    Ok(())
}
