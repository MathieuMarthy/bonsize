use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileModel {
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: u64,
    pub children: Vec<FileModel>,
    pub depth: usize,
}

impl FileModel {
    pub fn new(path: PathBuf, is_directory: bool, depth: usize) -> FileModel {
        FileModel {
            path,
            is_directory,
            size: 0,
            children: Vec::new(),
            depth
        }
    }

    pub fn get_flattened_files<'a>(&'a self, files: &mut Vec<&'a FileModel>) {
        files.push(&self);

        for child in &self.children {
            child.get_flattened_files(files);
        }
    }
}
