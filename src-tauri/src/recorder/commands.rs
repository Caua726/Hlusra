use std::sync::Mutex;
use tauri::{Manager, State};
use crate::library::api::Library;
use crate::library::types::{RecordingInfo, TrackInfo};
use crate::recorder::capture::ScreenCapture;
use crate::recorder::pipeline::RecordingPipeline;
use crate::recorder::types::*;
use crate::settings::config::load_settings;

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
    app: tauri::AppHandle,
    with_video: bool,
    library: State<'_, Library>,
    recorder: State<'_, RecorderState>,
) -> Result<String, String> {
    tracing::info!("start_recording called, with_video={}", with_video);

    let prepared = library.prepare_meeting().map_err(|e| {
        tracing::error!("prepare_meeting failed: {}", e);
        format!("Falha ao preparar reunião: {}", e)
    })?;
    tracing::info!("meeting prepared: id={}, dir={:?}", prepared.id, prepared.dir_path);

    let output_path = prepared.dir_path.join("recording.mkv");

    let settings = load_settings().unwrap_or_default();

    let video_codec = match settings.video.codec.as_str() {
        "h264" => VideoCodec::H264,
        "av1" => VideoCodec::Av1,
        _ => VideoCodec::H265,
    };
    let video_backend = match settings.video.backend.as_str() {
        "cuda" => EncoderBackend::Cuda,
        "software" => EncoderBackend::Software,
        _ => EncoderBackend::Vaapi,
    };
    let (width, height) = match settings.video.resolution.as_str() {
        "1080p" => (1920, 1080),
        "480p" => (854, 480),
        "1440p" => (2560, 1440),
        "4k" | "2160p" => (3840, 2160),
        _ => (1280, 720),
    };

    let video_config = VideoConfig {
        codec: video_codec,
        backend: video_backend,
        bitrate: settings.video.bitrate,
        fps: settings.video.fps,
        width,
        height,
    };
    let audio_config = AudioConfig {
        bitrate: settings.audio.bitrate,
    };

    let cancel_prepared = |lib: &Library, id: &str| {
        if let Err(e) = lib.cancel_prepared(id) {
            tracing::error!("cancel_prepared failed (orphan directory may remain): {}", e);
        }
    };

    let mut pipeline = if with_video {
        tracing::info!("building video pipeline, requesting screen...");
        let mut capture = ScreenCapture::new();
        let source = capture.request_screen().await.map_err(|e| {
            tracing::error!("screen capture failed: {}", e);
            cancel_prepared(&library, &prepared.id);
            format!("Falha na captura de tela: {}", e)
        })?;
        tracing::info!("screen captured: node_id={}", source.node_id);
        let p = RecordingPipeline::build_with_video(output_path, &source, &video_config, &audio_config).map_err(|e| {
            tracing::error!("build_with_video failed: {}", e);
            cancel_prepared(&library, &prepared.id);
            format!("Falha ao montar pipeline de vídeo: {}", e)
        })?;
        *recorder.capture.lock().map_err(|_| "Recorder lock poisoned".to_string())? = Some(capture);
        p
    } else {
        tracing::info!("building audio-only pipeline...");
        RecordingPipeline::build_audio_only(output_path, &audio_config).map_err(|e| {
            tracing::error!("build_audio_only failed: {}", e);
            cancel_prepared(&library, &prepared.id);
            format!("Falha ao montar pipeline de áudio: {}", e)
        })?
    };

    tracing::info!("pipeline built, starting...");
    pipeline.start().map_err(|e| {
        tracing::error!("pipeline.start() failed: {}", e);
        cancel_prepared(&library, &prepared.id);
        format!("Falha ao iniciar gravação: {}", e)
    })?;
    tracing::info!("pipeline started successfully");

    // TODO: Floating widget disabled for now — user reported it as unwanted popup.
    // Re-enable when widget UX is polished (configurable in settings).
    // let _ = tauri::WebviewWindowBuilder::new(&app, "widget", ...);

    // I23: Lock pipeline first, then meeting_id. If meeting_id lock fails,
    // take the pipeline back and stop it to avoid inconsistent state.
    *recorder.pipeline.lock().map_err(|_| "Recorder lock poisoned".to_string())? = Some(pipeline);
    if let Err(e) = recorder.current_meeting_id.lock().map(|mut guard| {
        *guard = Some(prepared.id.clone());
    }) {
        tracing::error!("meeting_id lock failed, rolling back pipeline: {}", e);
        if let Ok(mut pl) = recorder.pipeline.lock() {
            if let Some(mut stale) = pl.take() {
                let _ = stale.stop();
            }
        }
        cancel_prepared(&library, &prepared.id);
        return Err("Recorder lock poisoned".to_string());
    }

    Ok(prepared.id)
}

#[tauri::command]
pub async fn stop_recording(
    app: tauri::AppHandle,
    library: State<'_, Library>,
    recorder: State<'_, RecorderState>,
) -> Result<crate::library::types::Meeting, String> {
    tracing::info!("stop_recording called");

    // Close the floating recording widget window
    if let Some(w) = app.get_webview_window("widget") {
        let _ = w.close();
    }
    let pipeline = {
        let mut pipeline_lock = recorder.pipeline.lock().map_err(|_| "Recorder lock poisoned".to_string())?;
        pipeline_lock.take().ok_or("Nenhuma gravação ativa")?
    };

    // Capture duration BEFORE stop so EOS wait isn't included
    let duration = pipeline.duration_secs();
    let has_video = pipeline.has_video();

    tracing::info!("stopping pipeline (sending EOS)...");
    let pipeline = tokio::task::spawn_blocking(move || -> Result<RecordingPipeline, String> {
        let mut pipeline = pipeline;
        pipeline.stop().map_err(|e| {
            tracing::error!("pipeline.stop() failed: {}", e);
            format!("Falha ao parar gravação: {}", e)
        })?;
        Ok(pipeline)
    })
    .await
    .map_err(|e| format!("spawn_blocking join error: {}", e))?
    ?;

    let file_size = pipeline.file_size();
    tracing::info!("pipeline stopped, duration={}s, size={}bytes", duration, file_size);

    // Release screen capture fd
    *recorder.capture.lock().map_err(|_| "Recorder lock poisoned".to_string())? = None;

    let meeting_id = recorder.current_meeting_id.lock().map_err(|_| "Recorder lock poisoned".to_string())?.take()
        .ok_or("No meeting ID")?;

    // Build track metadata dynamically based on whether video was recorded
    let tracks = if has_video {
        vec![
            TrackInfo { index: 0, label: "video".into(), codec: "h265".into() },
            TrackInfo { index: 1, label: "mic".into(), codec: "opus".into() },
            TrackInfo { index: 2, label: "system".into(), codec: "opus".into() },
        ]
    } else {
        vec![
            TrackInfo { index: 0, label: "mic".into(), codec: "opus".into() },
            TrackInfo { index: 1, label: "system".into(), codec: "opus".into() },
        ]
    };

    let info = RecordingInfo {
        duration_secs: duration,
        has_video,
        file_size,
        tracks,
    };

    // Library tracks dir_path internally from prepare_meeting
    tracing::info!("finalizing meeting {}...", meeting_id);
    let meeting = library.finalize_meeting(&meeting_id, info)
        .map_err(|e| {
            tracing::error!("finalize_meeting failed: {}", e);
            format!("Falha ao salvar reunião: {}", e)
        })?;
    tracing::info!("meeting finalized: {:?}", meeting.id);

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
