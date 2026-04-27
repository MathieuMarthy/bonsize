use crate::config::BLOCK_SIZE;
use std::fs::Metadata;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SizeType {
    Logical,
    Physical,
}

pub struct ScanOptions {
    pub quiet: bool, // suppress error messages (e.g., permission denied)
    pub size_type: SizeType,
}

#[cfg(unix)]
pub fn get_file_size_logical(metadata: &Metadata) -> u64 {
    metadata.len()
}

#[cfg(unix)]
pub fn get_file_size_physical(metadata: &Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.blocks() * BLOCK_SIZE
}

#[cfg(not(unix))]
pub fn get_file_size_logical(metadata: &Metadata) -> u64 {
    metadata.len()
}

#[cfg(not(unix))]
pub fn get_file_size_physical(metadata: &Metadata) -> u64 {
    // TODO: implement physical size for windows

    // temp
    get_file_size_logical(metadata)
}
