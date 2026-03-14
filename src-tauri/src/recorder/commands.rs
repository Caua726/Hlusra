use std::sync::Mutex;
use tauri::State;
use crate::library::api::Library;
use crate::library::types::{RecordingInfo, TrackInfo};
use crate::recorder::capture::ScreenCapture;
use crate::recorder::pipeline::RecordingPipeline;
use crate::recorder::types::*;

pub struct RecorderState {
    pipeline: Mutex<Option<RecordingPipeline>>,
    capture: Mutex<Option<ScreenCapture>>,  // must outlive pipeline to keep PipeWire fd alive
    current_meeting_id: Mutex<Option<String>>,
}

impl RecorderState {
    pub fn new() -> Self {
        Self {
            pipeline: Mutex::new(None),
            capture: Mutex::new(None),
            current_meeting_id: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn start_recording(
    with_video: bool,
    library: State<'_, Library>,
    recorder: State<'_, RecorderState>,
) -> Result<String, String> {
    eprintln!("[recorder] start_recording called, with_video={}", with_video);

    let prepared = library.prepare_meeting().map_err(|e| {
        eprintln!("[recorder] prepare_meeting failed: {}", e);
        format!("Falha ao preparar reunião: {}", e)
    })?;
    eprintln!("[recorder] meeting prepared: id={}, dir={:?}", prepared.id, prepared.dir_path);

    let output_path = prepared.dir_path.join("recording.mkv");
    let video_config = VideoConfig::default();
    let audio_config = AudioConfig::default();

    let mut pipeline = if with_video {
        eprintln!("[recorder] building video pipeline, requesting screen...");
        let mut capture = ScreenCapture::new();
        let source = capture.request_screen().await.map_err(|e| {
            eprintln!("[recorder] screen capture failed: {}", e);
            format!("Falha na captura de tela: {}", e)
        })?;
        eprintln!("[recorder] screen captured: node_id={}", source.node_id);
        let p = RecordingPipeline::build_with_video(output_path, &source, &video_config, &audio_config).map_err(|e| {
            eprintln!("[recorder] build_with_video failed: {}", e);
            format!("Falha ao montar pipeline de vídeo: {}", e)
        })?;
        *recorder.capture.lock().map_err(|_| "Recorder lock poisoned".to_string())? = Some(capture);
        p
    } else {
        eprintln!("[recorder] building audio-only pipeline...");
        RecordingPipeline::build_audio_only(output_path, &audio_config).map_err(|e| {
            eprintln!("[recorder] build_audio_only failed: {}", e);
            format!("Falha ao montar pipeline de áudio: {}", e)
        })?
    };

    eprintln!("[recorder] pipeline built, starting...");
    pipeline.start().map_err(|e| {
        eprintln!("[recorder] pipeline.start() failed: {}", e);
        format!("Falha ao iniciar gravação: {}", e)
    })?;
    eprintln!("[recorder] pipeline started successfully");

    *recorder.pipeline.lock().map_err(|_| "Recorder lock poisoned".to_string())? = Some(pipeline);
    *recorder.current_meeting_id.lock().map_err(|_| "Recorder lock poisoned".to_string())? = Some(prepared.id.clone());

    Ok(prepared.id)
}

#[tauri::command]
pub fn stop_recording(
    library: State<'_, Library>,
    recorder: State<'_, RecorderState>,
) -> Result<crate::library::types::Meeting, String> {
    eprintln!("[recorder] stop_recording called");
    let mut pipeline_lock = recorder.pipeline.lock().map_err(|_| "Recorder lock poisoned".to_string())?;
    let pipeline = pipeline_lock.take().ok_or("Nenhuma gravação ativa")?;

    eprintln!("[recorder] stopping pipeline (sending EOS)...");
    pipeline.stop().map_err(|e| {
        eprintln!("[recorder] pipeline.stop() failed: {}", e);
        format!("Falha ao parar gravação: {}", e)
    })?;
    eprintln!("[recorder] pipeline stopped, duration={}s, size={}bytes", pipeline.duration_secs(), pipeline.file_size());

    // Release screen capture fd
    *recorder.capture.lock().map_err(|_| "Recorder lock poisoned".to_string())? = None;

    let meeting_id = recorder.current_meeting_id.lock().map_err(|_| "Recorder lock poisoned".to_string())?.take()
        .ok_or("No meeting ID")?;

    let info = RecordingInfo {
        duration_secs: pipeline.duration_secs(),
        has_video: pipeline.has_video(),
        file_size: pipeline.file_size(),
        tracks: vec![
            TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
            TrackInfo { index: 1, label: "system".to_string(), codec: "opus".to_string() },
        ],
    };

    // Library tracks dir_path internally from prepare_meeting
    let meeting = library.finalize_meeting(&meeting_id, info)
        .map_err(|e| e.to_string())?;

    Ok(meeting)
}

#[tauri::command]
pub fn get_recording_status(
    recorder: State<'_, RecorderState>,
) -> Result<RecordingStatus, String> {
    let pipeline_lock = recorder.pipeline.lock().map_err(|_| "Recorder lock poisoned".to_string())?;
    Ok(match pipeline_lock.as_ref() {
        Some(p) => RecordingStatus {
            state: RecordingState::Recording,
            duration_secs: p.duration_secs(),
            file_size: p.file_size(),
        },
        None => RecordingStatus {
            state: RecordingState::Idle,
            duration_secs: 0.0,
            file_size: 0,
        },
    })
}

#[tauri::command]
pub fn probe_encoders() -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    gstreamer::init().map_err(|e| format!("GStreamer init failed: {}", e))?;
    let available = crate::recorder::encode::probe_available();
    Ok(available.into_iter()
        .map(|(backend, codecs)| {
            (format!("{:?}", backend).to_lowercase(),
             codecs.iter().map(|c| format!("{:?}", c).to_lowercase()).collect())
        })
        .collect())
}
