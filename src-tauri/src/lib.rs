mod library;

use library::Library;
use std::path::PathBuf;

fn get_recordings_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Hlusra")
        .join("recordings")
}

fn get_db_path() -> PathBuf {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("hlusra");
    std::fs::create_dir_all(&data_dir).ok();
    data_dir.join("library.db")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let library = Library::new(
        &get_db_path(),
        get_recordings_dir(),
    ).expect("Failed to initialize library");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(library)
        .invoke_handler(tauri::generate_handler![
            library::commands::list_meetings,
            library::commands::get_meeting,
            library::commands::update_meeting_title,
            library::commands::delete_meeting,
            library::commands::get_thumbnail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
