use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use dirs::cache_dir;
use crate::config::CACHE_TTL_MINS;
use crate::scanner::file_model::FileModel;


fn get_cache_file(scanned_path: &PathBuf) -> Option<PathBuf> {
    let mut cache_path = cache_dir()?;

    cache_path.push(env!("CARGO_PKG_NAME"));

    let mut hasher = DefaultHasher::new();
    scanned_path.hash(&mut hasher);
    let path_hash = format!("{:x}.bin", hasher.finish());
    cache_path.push(path_hash);

    Some(cache_path)
}

fn is_cache_valid(cache_file_path: &PathBuf) -> bool {
    let date_of_cache = fs::metadata(&cache_file_path)
        .and_then(|metadata| metadata.modified()).ok();

    match date_of_cache {
        Some(date) => {
            let now = SystemTime::now();
            date.add(Duration::from_mins(CACHE_TTL_MINS)) >= now
        },
        None => false
    }
}

pub fn load_cache(scanned_path: &PathBuf) -> Option<FileModel> {
    let cache_file_path = get_cache_file(scanned_path)?;

    if !is_cache_valid(&cache_file_path) {
        return None;
    }

    let bytes = fs::read(&cache_file_path).ok()?;
    bincode::deserialize(&bytes).ok()
}

pub fn save_cache(scanned_path: &PathBuf, model: &FileModel) -> Option<()> {
    let cache_path = get_cache_file(scanned_path)?;
    let bytes = bincode::serialize(model).ok()?;

    // check if cache dir exists, if not create it
    if let Some(cache_dir) = cache_path.parent() && !cache_dir.exists() {
        if let Err(e) = fs::create_dir_all(cache_dir) {
            eprintln!("failed to create cache directory: {}", e);
            return None;
        }
    }

    fs::write(&cache_path, bytes)
        .inspect_err(|_| {
            eprintln!("failed to write cache file: {}", cache_path.display());
        }).ok()
}
