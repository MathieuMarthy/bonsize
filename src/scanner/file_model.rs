use std::path::PathBuf;

pub struct FileModel {
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: u64,
    pub children: Vec<FileModel>,
}

impl FileModel {
    pub fn new(path: PathBuf, is_directory: bool) -> FileModel {
        FileModel {
            path,
            is_directory,
            size: 0,
            children: Vec::new(),
        }
    }
}
