# Settings/Export Module — Design Spec

## Overview

The Settings module manages all app configuration via a TOML file. The Export module handles converting meeting media and transcripts into various output formats using FFmpeg.

## Settings

### Storage

- File: `~/.config/hlusra/config.toml`
- Loaded on app startup
- Saved on every change
- Human-readable, manually editable

### Configuration Sections

```toml
[general]
recordings_dir = "~/Hlusra/recordings"
auto_meeting_name = "Reunião {date} {time}"
start_minimized = false

[audio]
codec = "opus"
bitrate = 64000
# input_device and output_device default to system default
# configurable in future

[video]
codec = "h265"
backend = "vaapi"           # vaapi | cuda | vulkan | software
container = "mkv"
bitrate = 2000000
fps = 15
resolution = "720p"
# pixel_format = auto

[transcription]
provider = "local"          # local | api
api_url = ""
api_key = ""
model = "base"              # for local: tiny | base | small | medium | large

[rag]
embeddings_url = ""
embeddings_api_key = ""
embeddings_model = ""
chat_url = ""
chat_api_key = ""
chat_model = ""
chunk_size = 500
top_k = 5
```

### Defaults

All settings have sensible defaults. The app works out of the box for local-only use (recording + local transcription). API keys are only needed for remote transcription and RAG/Chat features.

## Export

### Supported Formats

**Audio:**

| Format | Extension | Notes |
|--------|-----------|-------|
| MP3 | `.mp3` | Multi-track mixed to single |
| WAV | `.wav` | Multi-track mixed to single |
| Opus | `.opus` | Can preserve tracks in supported containers |
| OGG | `.ogg` | Single track (mixed) |

**Video:**

| Container | Codec | Extension |
|-----------|-------|-----------|
| MP4 | H.264 | `.mp4` |
| MP4 | H.265 | `.mp4` |
| MKV | H.264 | `.mkv` |
| MKV | H.265 | `.mkv` |

**Transcript:**

| Format | Extension | Notes |
|--------|-----------|-------|
| TXT | `.txt` | Plain text |
| JSON | `.json` | Full structure with timestamps |
| SRT | `.srt` | Subtitle format, generated from segments |
| PDF | `.pdf` | Formatted document |

### Processing

- All media export uses FFmpeg (`ffmpeg-next` crate)
- Audio export from multi-track MKV: extracts and mixes tracks when target format requires single track
- Video export: transcodes from H.265/MKV source to target codec/container
- Transcript export: TXT and JSON are already generated; SRT is generated from segments; PDF is formatted from transcript
- Export is synchronous in MVP (blocks UI during export). This is a known limitation — async export with progress indicator is planned for a future iteration

### Save Modes

Two buttons in the UI:

```rust
enum SaveMode {
    Save,                     // saves to meeting directory
    SaveAs { path: PathBuf }, // saves to user-chosen location via file picker
}
```

- **Save**: exported file is saved directly in the meeting's directory
- **Save As**: opens native file picker, user chooses destination

## Tauri Commands

### Settings

```rust
get_settings() -> AppSettings
update_settings(settings: AppSettings) -> Result<()>
```

### Export

```rust
export_audio(id: String, format: AudioFormat, save_mode: SaveMode) -> Result<PathBuf>
export_video(id: String, format: VideoFormat, save_mode: SaveMode) -> Result<PathBuf>
export_transcript(id: String, format: TranscriptFormat, save_mode: SaveMode) -> Result<PathBuf>
```

## Dependencies

### Rust crates

| Crate | Purpose |
|-------|---------|
| `toml` | Config file parsing/serialization |
| `serde` | Deserialization of config into structs |
| `ffmpeg-next` | Media transcode/export |
| `dirs` | Platform-standard config/data directories |
| `genpdf` | PDF generation for transcript export |

## File Structure

```
src-tauri/src/settings/
    mod.rs          — public exports
    config.rs       — load/save TOML, AppSettings struct
    defaults.rs     — default values for all settings

src-tauri/src/export/
    mod.rs          — public exports
    audio.rs        — audio export (FFmpeg, track mixing)
    video.rs        — video export (FFmpeg, transcode)
    transcript.rs   — transcript export (TXT/JSON/SRT/PDF)
    types.rs        — AudioFormat, VideoFormat, TranscriptFormat, SaveMode, enums
```

## Decisions Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Config format | TOML | Readable, editable, Rust ecosystem standard |
| Config location | ~/.config/hlusra/ | XDG standard on Linux |
| Export processing | Synchronous (MVP limitation) | Async with progress planned for future iteration |
| PDF crate | genpdf | Lightweight, pure Rust, sufficient for formatted transcript |
| Save UX | Save + Save As | Save is quick (meeting dir), Save As gives control |
| Transcript formats | TXT, JSON, SRT, PDF | Covers reading, data, subtitles, sharing |
