use std::{
    fs::{remove_dir_all, remove_file},
    path::PathBuf,
};

use bonsize_core::scanner::{file_model::FileModel, get_directory_size, ScanOptions};

#[tauri::command]
async fn scan_directory(path: String) -> FileModel {
    let path_buf = PathBuf::from(path);
    let options = ScanOptions { quiet: true };

    get_directory_size(&path_buf, &options)
}

#[tauri::command]
async fn delete_file(path: String) -> bool {
    let path_buf = PathBuf::from(path);

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_directory, delete_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
