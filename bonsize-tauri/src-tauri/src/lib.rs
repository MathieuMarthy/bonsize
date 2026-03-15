use std::path::PathBuf;

use bonsize_core::scanner::{file_model::FileModel, get_directory_size, ScanOptions};

#[tauri::command]
async fn scan_directory(path: String) -> FileModel {
    let path_buf = PathBuf::from(path);
    let options = ScanOptions { quiet: true };

    get_directory_size(&path_buf, &options)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
