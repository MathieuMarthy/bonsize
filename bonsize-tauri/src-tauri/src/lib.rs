use std::{
    fs::{remove_dir_all, remove_file},
    path::PathBuf,
};

use bonsize_core::scanner::{
    file_model::FileModel,
    file_size::{ScanOptions, SizeType},
    get_directory_size,
};

#[tauri::command]
async fn scan_directory(path: String, use_physical_size: bool) -> FileModel {
    let path_buf = PathBuf::from(path);
    let options = ScanOptions {
        quiet: true,
        size_type: if use_physical_size {
            SizeType::Physical
        } else {
            SizeType::Logical
        },
    };

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
    #[cfg(target_os = "linux")]
    {
        // Workaround for WebKitGTK / Wayland / NVIDIA protocol errors (Error 71 dispatching to Wayland display)
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }

        // Force GTK dark theme variant because Bonsize UI is dark by default
        if std::env::var("GTK_THEME").is_err() {
            std::env::set_var("GTK_THEME", "Adwaita:dark");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_directory, delete_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
