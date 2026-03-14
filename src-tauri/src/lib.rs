mod library;
mod recorder;
mod transcription;
mod rag;
mod settings;
mod export;

use library::Library;
use rag::commands::RagState;
use rag::types::RagConfig;
use rag::vector_store::VectorStore;
use recorder::commands::RecorderState;
use std::path::PathBuf;
use std::sync::Mutex;

fn default_recordings_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Hlusra")
        .join("recordings")
}

fn get_recordings_dir() -> PathBuf {
    match settings::config::load_settings() {
        Ok(s) => {
            let dir = PathBuf::from(&s.general.recordings_dir);
            if !dir.as_os_str().is_empty() {
                dir
            } else {
                default_recordings_dir()
            }
        }
        Err(e) => {
            tracing::warn!(
                "failed to load settings, using default recordings dir: {}",
                e
            );
            default_recordings_dir()
        }
    }
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
    tracing_subscriber::fmt::init();

    gstreamer::init().expect("GStreamer init failed");

    let library = Library::new(
        &get_db_path(),
        get_recordings_dir(),
    ).expect("Failed to initialize library");

    let vector_store = VectorStore::open_default()
        .expect("Failed to open RAG vector store");

    let rag_state = RagState {
        store: Mutex::new(vector_store),
        config: Mutex::new(RagConfig::default()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(library)
        .manage(RecorderState::new())
        .manage(rag_state)
        .invoke_handler(tauri::generate_handler![
            // Library
            library::commands::list_meetings,
            library::commands::get_meeting,
            library::commands::update_meeting_title,
            library::commands::delete_meeting,
            library::commands::get_thumbnail,
            // Recorder
            recorder::commands::start_recording,
            recorder::commands::stop_recording,
            recorder::commands::get_recording_status,
            recorder::commands::probe_encoders,
            // Settings
            settings::commands::get_settings,
            settings::commands::update_settings,
            // RAG / Chat
            rag::commands::index_meeting,
            rag::commands::reindex_meeting,
            rag::commands::chat_message,
            rag::commands::get_chat_status,
            // Export
            export::commands::export_audio,
            export::commands::export_video,
            export::commands::export_transcript,
            // Transcription
            transcription::commands::transcribe_meeting,
            transcription::commands::retranscribe_meeting,
            transcription::commands::get_transcription_status,
            transcription::commands::list_available_models,
            transcription::commands::get_downloaded_models,
            transcription::commands::download_model,
            transcription::commands::cancel_download,
            transcription::commands::get_active_model,
            transcription::commands::set_active_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
