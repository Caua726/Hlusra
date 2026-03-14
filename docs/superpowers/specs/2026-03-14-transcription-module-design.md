# Transcription Module — Design Spec

## Overview

The Transcription module converts meeting audio into text with word-level timestamps. It supports two providers behind a common trait: local transcription via `whisper-rs` and remote transcription via any OpenAI-compatible API endpoint.

## Architecture

### Provider Trait

```rust
trait TranscriptionProvider {
    fn transcribe(&self, audio: &Path) -> Result<TranscriptResult>;
}
```

Two implementations:

- **LocalProvider** — uses `whisper-rs` (Rust bindings for whisper.cpp), runs in-process
- **ApiProvider** — HTTP client targeting any OpenAI Whisper-compatible endpoint

### Orchestrator Flow

1. Recording stops → app prompts user: "Transcribe now?"
2. If yes, Orchestrator gets meeting media path from Library
3. Extracts mic track (track 1) from MKV via FFmpeg → temporary WAV
4. Sends audio to the active provider
5. Receives result with word-level timestamps
6. Groups words into segments
7. Saves `transcript.json` + `transcript.txt` via Library
8. Updates `transcription_status` via Library

### Why Mic Track Only

The mic track (track 1) is used instead of the mixed audio because:
- Cleaner signal for speech recognition
- System audio often contains music, notifications, other noise
- Better transcription accuracy

## Data Model

### TranscriptResult

```rust
TranscriptResult {
    language: String,            // auto-detected
    segments: Vec<Segment>,
    full_text: String,
}
```

### Segment

```rust
Segment {
    start: f64,                  // seconds
    end: f64,
    text: String,
    words: Vec<Word>,
}
```

### Word

```rust
Word {
    start: f64,                  // seconds
    end: f64,
    text: String,
    confidence: f32,
}
```

### Output Files

- `transcript.json` — full structure with segments, words, and timestamps
- `transcript.txt` — plain text for quick reading

## Providers

### LocalProvider (whisper-rs)

- Uses `whisper-rs` crate (bindings to whisper.cpp)
- Runs in the app process, no external dependencies
- **Bundled model**: tiny or base (~75-150MB), included with the app
- **Downloadable models**: small, medium, large — user downloads via settings
- Language detection: automatic (Whisper built-in)
- Timestamps: word-level via whisper.cpp's `full_get_segment_*` API

### ApiProvider (HTTP)

- HTTP client compatible with OpenAI Whisper API format
- Configurable: URL + API key
- Covers: OpenAI directly, local servers (faster-whisper-server, whisper.cpp --server), any compatible endpoint
- User configures in settings

## Model Management

```rust
list_available_models() -> Vec<WhisperModel>      // tiny, base, small, medium, large
get_downloaded_models() -> Vec<WhisperModel>
download_model(model: WhisperModel) -> Result<()> // with progress reporting
get_active_model() -> WhisperModel
set_active_model(model: WhisperModel) -> Result<()>
```

Models stored in `~/.local/share/hlusra/models/`.

## Tauri Commands

```rust
transcribe_meeting(id: String) -> Result<()>
get_transcription_status(id: String) -> TranscriptionStatus
retranscribe_meeting(id: String) -> Result<()>
list_available_models() -> Vec<WhisperModel>
get_downloaded_models() -> Vec<WhisperModel>
download_model(model: String) -> Result<()>
get_active_model() -> WhisperModel
set_active_model(model: String) -> Result<()>
```

## Dependencies

### Rust crates

| Crate | Purpose |
|-------|---------|
| `whisper-rs` | Local transcription (whisper.cpp bindings) |
| `reqwest` | HTTP client for API provider |
| `ffmpeg-next` | Audio track extraction from MKV |

## File Structure

```
src-tauri/src/transcription/
    mod.rs            — public exports
    orchestrator.rs   — main flow (extract → transcribe → save)
    provider.rs       — trait TranscriptionProvider
    local.rs          — LocalProvider (whisper-rs)
    api.rs            — ApiProvider (HTTP client)
    models.rs         — Whisper model management (download, list, select)
    types.rs          — TranscriptResult, Segment, Word, enums
```

## Decisions Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Provider architecture | Trait with two implementations | Clean swap, same contract |
| Local engine | whisper-rs (whisper.cpp bindings) | In-process, no external binary needed |
| Remote protocol | OpenAI Whisper API compatible | Covers OpenAI + local servers + others |
| Bundled model | tiny/base | Small enough to ship, functional for basic use |
| Trigger | User-prompted after recording | User controls when/if transcription runs |
| Language | Auto-detection | Whisper handles this natively |
| Timestamp granularity | Word-level, grouped into segments | Maximum flexibility, clean display |
| Audio source | Mic track only | Cleaner signal, better accuracy |
