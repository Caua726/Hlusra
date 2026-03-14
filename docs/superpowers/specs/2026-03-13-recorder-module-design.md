# Recorder Module — Design Spec

## Overview

The Recorder module handles all audio and video capture for Hlusra, a minimal desktop meeting recorder for Linux (Wayland/Hyprland). It is the first of 5 modules: Recorder, Library, Transcription, RAG/Chat, Settings/Export.

The module uses a three-layer architecture (Capture → Encode → Pipeline) with GStreamer for the recording pipeline and FFmpeg reserved for export/transcode (handled by the Export module).

## Architecture

### Layer 1 — Capture

Responsible for obtaining PipeWire sources for screen and audio.

**Screen Capture:**
- Uses `ashpd` crate to call `org.freedesktop.portal.ScreenCast`
- Flow: `CreateSession` → `SelectSources` (opens compositor picker) → `Start` → returns PipeWire fd + node ID
- Portal picker opens every recording session (no remembered state)
- Initial focus: Hyprland. X11 support deferred.

**Audio Capture:**
- Uses `pipewire-rs` to enumerate and connect to audio devices
- Two separate nodes:
  - **Mic**: input device (default in MVP, configurable later)
  - **System audio**: monitor node of the output device
- Each node becomes a separate `pipewiresrc` in the GStreamer pipeline

**Interface:**
```rust
Capture::screen() -> Result<PipeWireSource>       // fd + node_id
Capture::mic() -> Result<PipeWireSource>           // node_id
Capture::system_audio() -> Result<PipeWireSource>  // node_id
Capture::list_audio_devices() -> Vec<AudioDevice>  // exposed for future settings
```

### Layer 2 — Encode

Responsible for resolving which encoder to use and returning configured GStreamer elements.

**Backend × Codec matrix:**

| Backend  | H.264            | H.265            | AV1              |
|----------|------------------|------------------|------------------|
| VAAPI    | `vaapih264enc`   | `vaapih265enc`   | `vaapiav1enc`    |
| CUDA     | `nvh264enc`      | `nvh265enc`      | `nvav1enc`       |
| Vulkan   | `vulkanh264enc`  | `vulkanh265enc`  | `vulkanav1enc`   |
| Software | `x264enc`        | `x265enc`        | `svtav1enc`      |

**Probe:**
- On app startup, queries GStreamer registry for available encoders
- Returns a map of backend → supported codecs for the current machine
- Settings UI shows only available combinations

**Fallback chain:**
- User selects preferred (backend, codec) pair
- If unavailable at runtime: try same codec with next backend in order
- Last resort: software encoder (always available)
- Default: VAAPI + H.265

**Audio encoder:**
- `opusenc` — single implementation, software-only (lightweight enough)

**Configurable parameters (from Settings):**
- Video bitrate
- Audio bitrate
- FPS
- Resolution

**Interface:**
```rust
Encode::probe_available() -> HashMap<Backend, Vec<Codec>>
Encode::create_video_encoder(backend: Backend, codec: Codec, config: VideoConfig) -> Result<GstElement>
Encode::create_audio_encoder(config: AudioConfig) -> GstElement
```

### Layer 3 — Pipeline

Responsible for assembling the complete GStreamer pipeline, managing recording lifecycle.

**Pipeline with video (3 sources → 1 mux):**
```
pipewiresrc (screen)  → videoconvert → video_encoder → matroskamux → filesink
pipewiresrc (mic)     → audioconvert → opusenc       ↗ (track 1)
pipewiresrc (system)  → audioconvert → opusenc       ↗ (track 2)
```

**Pipeline audio-only (2 sources → 1 mux):**
```
pipewiresrc (mic)     → audioconvert → opusenc → matroskamux → filesink
pipewiresrc (system)  → audioconvert → opusenc ↗ (track 2)
```

**Container:** MKV for everything (both video and audio-only). MKV supports multiple Opus tracks and is resilient to crashes (recoverable without proper finalization).

**Audio tracks:** Always recorded as separate tracks in the container.
- Track 1: microphone
- Track 2: system audio
- Mixed to single track only during export when target format requires it (MP3, WAV, single-track OGG)
- Transcription uses mic track (track 1) for better results

**States:**
```
Idle → Recording → Stopped
```
- `Idle`: no pipeline exists
- `Recording`: pipeline running, floating widget visible
- `Stopped`: pipeline finalized, file closed, ready for Library ingestion

**Integration with Library:**

The Recorder does not generate meeting IDs or create directories. The flow is:
1. `start_recording` calls `Library::prepare_meeting()` → gets `PreparedMeeting { id, dir_path }`
2. Recorder uses `dir_path` for GStreamer `filesink` (writes `recording.mkv` directly)
3. `stop_recording` calls `Library::finalize_meeting(id, recording_info)` to populate metadata

**Tauri Commands (IPC):**
```rust
start_recording(with_video: bool) -> Result<RecordingId>
stop_recording() -> Result<RecordingInfo>
get_status() -> RecordingState  // Idle | Recording { duration, file_size }
```

**RecordingInfo (passed to Library on finalize):**
```rust
RecordingInfo {
    duration: Duration,
    has_video: bool,
    file_size: u64,
    tracks: Vec<TrackInfo>,  // mic, system — TrackInfo defined in Library types
}
```

**File naming:** `recording.mkv` inside the Library-managed meeting directory (e.g., `~/Hlusra/recordings/2026-03-13_a3f2/recording.mkv`)

## Floating Widget

A minimal always-on-top Tauri window shown during recording.

- Shows: recording timer (HH:MM:SS), red pulsing indicator, stop button
- Position: screen corner, draggable
- Appears on recording start, closes on stop
- Communicates with main window via Tauri event system
- Separate Tauri window: `always_on_top`, no decoration, minimal size

## Crash Recovery

MKV is inherently resilient to incomplete writes. If the app or system crashes mid-recording, the MKV file can be recovered using tools like `mkvmerge` or GStreamer's own demuxer. No additional segmentation strategy is needed for the MVP.

## Dependencies

### Rust crates

| Crate | Purpose |
|-------|---------|
| `gstreamer` + `gstreamer-video` + `gstreamer-audio` | Recording pipeline |
| `ashpd` | XDG Desktop Portal (screen capture) |
| `pipewire-rs` | Audio device enumeration |
| `tauri` | IPC with frontend, widget window |

### System packages (runtime)

| Package | Purpose |
|---------|---------|
| `gstreamer` + plugins (good, bad, ugly) | Pipeline elements |
| `pipewire` | Audio/video capture |
| `xdg-desktop-portal` + compositor implementation | Screen picker |

## File Structure

```
src-tauri/src/recorder/
  mod.rs          — public exports
  capture.rs      — Capture layer (portal + PipeWire sources)
  encode.rs       — Encode layer (probe + encoder factory)
  pipeline.rs     — Pipeline layer (assembly + state management)
  widget.rs       — floating widget window control
  types.rs        — RecordingInfo, TrackInfo, Backend, Codec, enums
```

## Decisions Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Media framework for recording | GStreamer | Native PipeWire integration, pipeline-based fits real-time capture |
| Media framework for export | FFmpeg (via ffmpeg-next) | Superior codec support, better for transcode tasks |
| Container format | MKV for everything | Multi-track support, crash resilience, universal |
| Audio tracks | Separate (mic + system) | Better transcription quality, flexible export |
| Default video codec | H.265 | Best size/quality ratio for screen content |
| Default encoding backend | VAAPI | Most stable GPU acceleration on Linux |
| Available backends | VAAPI, CUDA, Vulkan, Software | Configurable, covers all GPU vendors |
| Screen picker | Portal every time | No state to manage, user always confirms |
| Recording UI | Floating widget | Always visible, minimal, draggable |
| Crash recovery | MKV inherent resilience | No extra segmentation needed |
