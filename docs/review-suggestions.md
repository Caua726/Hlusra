# Hlusra Code Review — SUGGESTIONS

> 233 suggestion sections from 44 code reviewers

---

### Suggestions (Nice to Have)

**10. Header back-button is duplicated three times**

The header with the back button SVG is rendered identically in three branches (loading, error, main). Extract it to a small component or variable.

**11. `backendOptions` derived from `encoders` may not contain the current `settings.video.backend`**

Line 153: `const backendOptions = Object.keys(encoders);`. If the probed encoder list does not include the user's currently saved backend (e.g., they previously configured "cuda" but are now on a machine without CUDA), the `<select>` will have a `value` that does not match any `<option>`, resulting in a visually blank select with no way to see the current value. Add the current value as a fallback option.

**12. Tab switching does not reset `error` or `saved`**

Related to items 2 and 5: the `setActiveTab` call (line 175) should also call `setError(null)` and `setSaved(false)` so that stale feedback from another tab does not leak.

**13. `recordings_dir` should use a folder picker instead of a raw text input**

Tauri provides `dialog.open` for folder selection. A plain text input for a filesystem path is error-prone and non-standard for desktop apps.

**14. Accessibility: toggle switches have no visible focus ring and no ARIA labels**

The checkbox-based toggle for "Iniciar minimizado" (lines 228-241) is visually styled with `sr-only` on the actual checkbox but has no `aria-label` or `role="switch"`. The focus state is also invisible because the peer classes only style the decorative div.

**15. No unsaved-changes warning when navigating away**

If the user modifies settings and clicks the back button without saving, all changes are silently discarded. Consider a confirmation dialog.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 10 | Suggestion | Header back-button duplicated 3 times |

---

| 11 | Suggestion | Current backend value may not appear in probed encoder options |

---

| 12 | Suggestion | Tab switch should clear `error` and `saved` |

---

| 13 | Suggestion | Use Tauri folder picker for `recordings_dir` instead of raw text input |

---

| 14 | Suggestion | Toggle lacks ARIA labels and visible focus state |

---

| 15 | Suggestion | No unsaved-changes warning on back navigation |

### Files Referenced

- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx` -- the reviewed file
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/api.ts` -- TypeScript types and API functions (spec source)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/settings/config.rs` -- Rust `AppSettings` struct (canonical spec)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/settings/defaults.rs` -- Rust defaults
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/models.rs` -- active model persistence logic (relevant to issue #6)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/commands.rs` -- `create_provider` which shows local provider ignores `settings.transcription.model` (relevant to issue #6)

---

### Suggestions (nice to have)

| # | Module | Issue | Description |
|---|--------|-------|-------------|
| 7 | All | FFmpeg CLI vs. ffmpeg-next | All modules use FFmpeg CLI (`std::process::Command`) instead of the `ffmpeg-next` crate. Consistent and pragmatic, but diverges from spec. Consider updating specs to reflect reality. |
| 8 | Library | MeetingUpdate unused | `MeetingUpdate` struct exists in `types.rs` but is never used. `update_title` is a specific method instead. Remove the dead type or implement the generic update. |
| 9 | Recorder | Stopped state unused | `RecordingState::Stopped` exists in the enum but is never assigned. `get_recording_status` only returns `Idle` or `Recording`. |

### What Was Done Well

- The overall modular architecture faithfully follows the spec's 5-module design with Library as central gateway.
- All Tauri IPC commands from every spec are registered and functional.
- Error handling is thorough throughout, with proper `thiserror` derives and `Serialize` implementations for Tauri IPC.
- Test coverage is solid: unit tests exist for types, DB operations, filesystem operations, chunker, prompt builder, embeddings serialization, and more.
- The transcription module is particularly well implemented: both providers work, word-level timestamps are extracted, model management with atomic downloads is production-ready.
- The RAG pipeline (chunk -> embed -> store -> search -> prompt -> stream) is fully wired end-to-end with proper status tracking.
- The export module handles all format combinations from the spec with proper multi-track mixing and SRT/PDF generation.
- Settings TOML structure and defaults match the spec exactly.
- Bug fixes in `api.rs` (transcript.json vs transcript.txt) and `transcript.rs` (TranscriptFile wrapper struct) show good attention to runtime correctness.

---

#### SUGGESTIONS

**7. Missing `encoding_format` field in `EmbeddingRequest` (line 8-11)**

The OpenAI embeddings API supports an `encoding_format` parameter (`"float"` or `"base64"`). The default is `"float"`, which matches the `Vec<f32>` deserialization. This works correctly as-is, but explicitly setting `encoding_format: "float"` would make the code self-documenting and protect against future API default changes.

```rust
#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
    encoding_format: String, // Always "float"
}
```

---

**8. `EmbeddingResponse.model` should not be `Option<String>` (line 27)**

The OpenAI API always returns `model` as a required field in the embeddings response. Making it `Option<String>` is overly permissive. The same applies to `usage` (line 28-29), which is always returned by OpenAI (though not necessarily by all compatible providers). Since these fields are `#[allow(dead_code)]`, this is cosmetic, but `model: String` would be more accurate to the API contract.

**Recommendation**: If targeting only OpenAI-compatible APIs that guarantee these fields, use `String` and `serde_json::Value`. If targeting a broader set of providers that might omit them, `Option` is fine but should be documented as such.

---

**9. The `index` field in `EmbeddingObject` is `usize` but OpenAI returns it as an integer (line 17)**

This works correctly for deserialization (serde will parse a JSON integer into `usize`), but `usize` is platform-dependent in size. The API returns a u32-range integer. Using `u32` or `usize` are both fine in practice; just noting this for awareness. No action needed.

---

**10. Test coverage is minimal (lines 161-190)**

The tests only verify serialization/deserialization of request and response types. There are no tests for:
- Error path handling (API error response parsing)
- Empty input handling in `embed_batch`
- Dimension mismatch detection
- Index-based sorting logic
- The `embed_one` -> `embed_batch` delegation

These could be tested with mock HTTP servers (e.g., `wiremock` or `mockito` crates) or by extracting the response-processing logic into a testable pure function.

---

**11. No rate-limiting or retry logic for transient failures**

Embedding APIs commonly return 429 (rate limit) or 5xx (transient server errors). There is no retry mechanism. The caller in `commands.rs` processes batches of 50 sequentially (lines 266-269), so a single transient failure will abort the entire indexing operation.

**Recommendation**: Consider adding at least a simple retry with exponential backoff for 429 and 5xx responses, or document this as a known limitation.

---

### Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 7 | Suggestion | Missing explicit `encoding_format: "float"` | 8-11 |

---

| 8 | Suggestion | `model` and `usage` as `Option` is overly permissive | 27-29 |

---

| 9 | Suggestion | `index: usize` vs `u32` -- cosmetic | 17 |

---

| 10 | Suggestion | Minimal test coverage | 161-190 |

---

| 11 | Suggestion | No retry/backoff for transient API failures | -- |

Issues #2 (lost error body), #3 (no timeout), and #4 (count mismatch) are the highest-impact items I would recommend addressing first. Issue #1 is functionally harmless in reqwest 0.12 but should be cleaned up. Issues #5 and #6 are design improvements worth considering in a follow-up pass.

---

#### SUGGESTION -- FFmpeg stderr is captured but stdout is not used; consider suppressing FFmpeg banner output

The FFmpeg invocation uses `.output()` which captures both stdout and stderr. This is fine, but FFmpeg prints its version banner and input file info to stderr even on success. Consider adding `-loglevel error` to reduce noise in the captured stderr, making actual error messages easier to parse:

```rust
.arg("-loglevel")
.arg("error")
```

This would go right after `-y` (line 65), before `-i`.


---

#### SUGGESTION -- The `extract_mic_to_wav` function does not add `-vn` (disable video)

While `-map 0:a:0` already selects only the audio stream, it is FFmpeg best practice to also pass `-vn` (no video output) to make the intent explicit and avoid edge cases where FFmpeg might try to include video metadata. This is a minor robustness improvement:

```rust
.arg("-vn")
```

This would go after the `-map` argument pair.


---

#### SUGGESTION -- Error type is `String` throughout

All errors are `String`-typed. This is adequate for the current size of the project and consistent with how the rest of the codebase operates (the `TranscriptionProvider` trait also returns `Result<_, String>`). However, if the codebase grows, a dedicated error enum would provide better programmatic error handling. No action needed now, but worth noting for the future.


---

#### SUGGESTION -- `TranscriptionInput` has a redundant field

`TranscriptionInput` contains both `meeting_dir` and `mkv_path`. Looking at the caller in `commands.rs` (lines 70-73), `meeting_dir` comes from `meeting.dir_path` and `mkv_path` comes from `library.get_artifact_path()`. In the orchestrator, `meeting_dir` is only used to derive the temp file path (line 38). The `mkv_path`'s parent directory is conceptually the same as `meeting_dir`. If that invariant always holds, you could derive the temp path from `mkv_path.parent()` and eliminate `meeting_dir` from the struct. However, if the recording file could ever live outside the meeting directory, keeping both is the safer choice. This is a minor design observation, not a defect.

---

### FFmpeg Command Correctness Summary

The final FFmpeg command assembled is:

```
ffmpeg -y -i <input.mkv> -map 0:a:0 -ac 1 -ar 16000 -codec:a pcm_s16le <output.wav>
```

Verification:
- **`-y`**: Overwrite output without prompting. Correct (needed for reruns).
- **`-i <input>`**: Input file. Correctly placed after global options.
- **`-map 0:a:0`**: Select first audio stream from first input. Correct for selecting mic track.
- **`-ac 1`**: Downmix to mono. Correct for Whisper.
- **`-ar 16000`**: Resample to 16 kHz. Correct for Whisper (which expects 16 kHz).
- **`-codec:a pcm_s16le`**: Encode as signed 16-bit little-endian PCM. This is the correct FFmpeg encoder name. Correct for Whisper.
- **Output file last**: Correct FFmpeg convention.
- **Argument order**: Global options, then input, then output options, then output file. This is the correct canonical order.

**The FFmpeg invocation is correct.** No argument ordering issues, no wrong codec names, no missing required flags.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 5 | Suggestion | Add `-loglevel error` to reduce FFmpeg stderr noise |

---

| 6 | Suggestion | Add `-vn` for explicit video disable alongside `-map 0:a:0` |

---

| 7 | Suggestion | `String` error type is fine now but consider an enum as codebase grows |

---

| 8 | Suggestion | `meeting_dir` field may be derivable from `mkv_path.parent()` |


---

| **tokio** | `"1"` | 1.x (1.44+ by May 2025) | CURRENT -- Features `["full"]` are correct for a desktop app, though `["full"]` is heavy. See suggestion below. |
| **uuid** | `"1"` | 1.x (1.16+ by May 2025) | CURRENT |
| **chrono** | `"0.4"` | 0.4.x (0.4.40+ by May 2025) | CURRENT -- Features `["serde"]` correct. |
| **thiserror** | `"2"` | 2.x (2.0.12+ by May 2025) | CURRENT `[VERIFY]` |
| **dirs** | `"6"` | 6.x (6.0.0 by May 2025) | CURRENT `[VERIFY]` |
| **toml** | `"0.8"` | 0.8.x (0.8.22+ by May 2025) | CURRENT `[VERIFY]` |

### Recorder Module

| Crate | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **gstreamer** | `"0.25"` | 0.25.x was the latest by May 2025 | CURRENT `[VERIFY]` -- gstreamer-rs follows GStreamer releases; 0.25 maps to GStreamer 1.26. A 0.26 may exist by now. |
| **gstreamer-video** | `"0.25"` | 0.25.x | CURRENT `[VERIFY]` -- must stay in lockstep with `gstreamer`. |
| **gstreamer-audio** | `"0.25"` | 0.25.x | CURRENT `[VERIFY]` -- must stay in lockstep with `gstreamer`. |
| **ashpd** | `"0.13"` | 0.13.x was latest by May 2025 | CURRENT `[VERIFY]` -- Features `["tokio", "screencast"]` are correct for Wayland screen capture via XDG Desktop Portal. |

### Transcription Module

| Crate | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **whisper-rs** | `"0.14"` | 0.14.x was latest by May 2025 | CURRENT `[VERIFY]` -- this crate has had frequent releases; a 0.15 or higher could exist by now. |
| **reqwest** | `"0.12"` | 0.12.x (0.12.15+ by May 2025) | CURRENT `[VERIFY]` -- Features `["json", "multipart", "stream", "blocking"]` are correct for the API provider use case. Note: `blocking` and async in the same binary is fine but ensure you are not mixing them accidentally. |
| **hound** | `"3.5"` | 3.5.x (3.5.1 by May 2025) | CURRENT -- this crate is very stable and rarely updated. |

### Export Module

| Crate | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **genpdf** | `"0.2"` | **POTENTIALLY OUTDATED** -- 0.2.0 was the latest as of May 2025, but this crate has very low maintenance activity. 0.3 did not exist as of May 2025. `[VERIFY]` | See recommendation below. |

### RAG Module

| Crate | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **futures-util** | `"0.3"` | 0.3.x (0.3.31+ by May 2025) | CURRENT -- no 0.4 existed. |

### Dev Dependencies

| Crate | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **tempfile** | `"3"` | 3.x (3.19+ by May 2025) | CURRENT |

---

## NPM DEPENDENCIES (package.json)

### Dependencies

| Package | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **@tauri-apps/api** | `^2.0.0` | 2.x (2.3+ by May 2025) | CURRENT -- `^2.0.0` resolves to latest 2.x. |
| **@tauri-apps/plugin-dialog** | `^2.0.0` | 2.x | CURRENT |
| **@tauri-apps/plugin-fs** | `^2.0.0` | 2.x | CURRENT |
| **motion** | `^12.36.0` | 12.x (motion.dev, successor to framer-motion) | CURRENT `[VERIFY]` -- motion has frequent releases; a newer 12.x or even 13.x could exist. |
| **react** | `^19.0.0` | 19.x (19.1.0 by May 2025) | CURRENT `[VERIFY]` |
| **react-dom** | `^19.0.0` | 19.x | CURRENT |

### Dev Dependencies

| Package | Specified | Latest Known (May 2025) | Status |
|---|---|---|---|
| **@tailwindcss/vite** | `^4.0.0` | 4.x (4.1+ by May 2025) | CURRENT |
| **@tauri-apps/cli** | `^2.0.0` | 2.x | CURRENT |
| **@types/react** | `^19.0.0` | 19.x | CURRENT |
| **@types/react-dom** | `^19.0.0` | 19.x | CURRENT |
| **@vitejs/plugin-react** | `^4.0.0` | 4.x (4.4+ by May 2025) | CURRENT `[VERIFY]` -- a 5.x could exist by March 2026. |
| **tailwindcss** | `^4.0.0` | 4.x | CURRENT |
| **typescript** | `~5.7.0` | **LIKELY OUTDATED** -- TypeScript 5.8 was released March 2025. By March 2026, TypeScript 5.9 or even 6.0 may be out. The `~` prefix locks to 5.7.x patches only. | OUTDATED |
| **vite** | `^6.0.0` | 6.x (6.3+ by May 2025) | CURRENT `[VERIFY]` -- a 7.x could exist by March 2026. |

---


---

## SUGGESTIONS (Nice to Have)

### 1. Consider narrowing `tokio` features
- **Current:** `features = ["full"]`

---

- **Suggestion:** `features = ["rt-multi-thread", "macros", "sync", "time", "io-util"]` -- only include what you actually use. `"full"` pulls in `net`, `process`, `signal`, `fs` which a Tauri desktop app likely does not need for its core async runtime. This reduces compile time slightly.

### 2. Missing `sqlite-vec` dependency
- **Observation:** The project memory mentions "sqlite-vec (vector search)" for the RAG module, but there is no `sqlite-vec` crate in `Cargo.toml`. The `rusqlite` crate has `"load_extension"` feature enabled, which suggests you plan to load sqlite-vec as a runtime extension. If that is the case, this is fine -- but make sure the sqlite-vec `.so` is bundled or available at runtime. Alternatively, consider adding it as a build dependency if there is a Rust wrapper crate available.

### 3. Missing `gstreamer-app` or `gstreamer-pbutils`
- **Observation:** For a recording pipeline that writes to MKV, you may need `gstreamer-app` (for appsrc/appsink) or `gstreamer-pbutils` (for encoding profiles and discoverer). If your pipeline uses only standard elements via string descriptions (e.g., `gst::parse::launch`), this is not strictly needed. But if you later need programmatic pipeline construction with custom buffers, you will need `gstreamer-app = "0.25"`.

### 4. `reqwest` `"blocking"` feature
- **Current:** `features = ["json", "multipart", "stream", "blocking"]`
- **Note:** You have both async and blocking features. Since you are already using `tokio` with `"full"` features, you likely do not need `"blocking"`. Using `reqwest::Client` (async) inside a `tokio::spawn` is generally preferable. If `"blocking"` is only used in tests, consider moving it to `[dev-dependencies]` or removing it.

---

## HOW TO VERIFY (Action Items)

Since I could not access live package registries, please run these commands to get definitive answers:

**For Rust:**
```bash
# Install cargo-outdated if not already installed
cargo install cargo-outdated

# Run from src-tauri/
cd /home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri
cargo outdated -R
```

**For npm:**
```bash
cd /home/caua/Documentos/Projetos-Pessoais/Hlusra
npx npm-check-updates
```

These commands will give you a definitive, real-time report of every dependency that has a newer version available, covering the 10-month gap since my knowledge cutoff.

---

## SUMMARY

The dependency set is **generally well-maintained and up-to-date** as of my last knowledge (May 2025). The version specifiers are sensible -- using major-version pinning (`"2"`, `"1"`) for stable crates and minor-version pinning (`"0.25"`, `"0.13"`) for pre-1.0 crates, which is correct Cargo semver practice. The npm side uses `^` ranges appropriately, except for TypeScript which uses `~` (also appropriate for TS, but the version is stale).

The most likely items to be outdated by March 2026 are: **TypeScript** (definitely), **gstreamer-rs** (possibly 0.26), **whisper-rs** (possibly 0.15+), and **rusqlite** (possibly 0.33+). Running `cargo outdated -R` and `npx npm-check-updates` will confirm.

---

### SUGGESTIONS (nice to have)

**S1. No keyboard shortcut to start/stop recording**
The primary action (recording a meeting) requires clicking a button. A global hotkey (e.g., Ctrl+R) would be natural for a desktop app that may be minimized or in the background.

**S2. No visual feedback differentiating "Gravar tela" toggle when already recording**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx`, lines 107

During recording, the info line says "Video" or "Audio", but it is extremely subtle (11px, 15% white opacity). A more prominent indicator of what is being captured would help.

**S3. Search in Gallery only filters by title**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx`, lines 43-45

The search input only filters by `m.title`. Users cannot search by date, transcription content, or status. For a growing library of meetings, this limits discoverability.

**S4. Settings page has no "Reset to defaults" option**
If the user enters bad values (e.g., 0 for FPS, invalid bitrate), there is no way to reset without knowing the original defaults.

**S5. No tooltip for the settings gear icon in Gallery footer**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx`, lines 116-124

The settings button is an icon-only button with no title/tooltip attribute. A user unfamiliar with the gear icon convention has no way to discover what it does.

**S6. Bitrate field in Settings accepts raw numbers with no unit context**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx`, line 353

The hint says "Em bps (2000000 = 2 Mbps)" for video bitrate, but the audio bitrate field (line 410-420) has no hint at all. Users may not know what unit to use.

**S7. Recordings directory is a raw text input instead of a folder picker**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx`, lines 195-205

Tauri provides a directory dialog, but the recordings_dir setting is just a freeform text input. Users must type the exact path, which is error-prone.

**S8. Chat input is a single-line `<input>` instead of a `<textarea>`**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx`, lines 240-247

For longer questions, a resizable textarea would be more comfortable. The current single-line input feels cramped for conversational use.

**S9. No visible count of meeting cards matching search**
When searching in the Gallery, the footer still shows the total count ("X reunioes") rather than the filtered count. This can be confusing when the search yields fewer results than expected.

**S10. Export success message is easy to miss**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ExportDialog.tsx`, line 204

The "Salvo em: /path/to/file" message is small green text (11px) that scrolls into view below the format buttons. A toast notification or a more prominent success state would be clearer.

**S11. The MeetingPage play button loads the entire file into memory as a blob**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 85-108

For large video recordings, `readFile()` loads the entire file into a JS blob. A 2 GB recording will consume 2 GB of browser memory. There is no loading indicator while this happens, and no error shown to the user if it fails (line 100 only logs to console).

**S12. MeetingCard has no aria-label explaining what clicking it does**
While the card has `role="button"` and `tabIndex={0}`, screen readers would benefit from an explicit label like "Abrir reuniao: [title]".

---

### Summary of Findings

| Severity | Count | Key Theme |
|----------|-------|-----------|

---

| SUGGESTION | 12 | Polish: keyboard shortcuts, folder picker, textarea, search improvements |

The highest priority fixes are **C1** (recording dead end) and **C3/C4** (silent data loss). C1 can make the app completely unusable without a restart. C3 and C4 will cause real frustration once users start relying on the chat and settings features.

---

## SUGGESTIONS

### 9. Duplicated config validation logic (lines 78-88 and 149-154)

The check `if config.embeddings_url.is_empty() || config.chat_url.is_empty()` is duplicated between `index_meeting` and `chat_message`. The error message is identical.

**Recommendation:** Extract into a method on `RagConfig`:

```rust
impl RagConfig {
    pub fn validate(&self) -> Result<(), RagCommandError> {
        if self.embeddings_url.is_empty() || self.chat_url.is_empty() {
            return Err(RagCommandError::Other(
                "RAG not configured -- set API keys in Settings".to_string(),
            ));
        }
        Ok(())
    }
}
```

### 10. `index_meeting` validates `chat_url` but does not use the chat endpoint (line 83)

`index_meeting` only does embedding and storage -- it never calls the chat API. Yet it requires `config.chat_url` to be non-empty and fails if it is not. A user who has configured embeddings but not chat would be blocked from indexing.

**Recommendation:** In `index_meeting`, only validate `embeddings_url`, `embeddings_api_key`, and `embeddings_model`. Validate the chat fields only in `chat_message`.

### 11. `refresh_rag_config` reads from disk on every command invocation (lines 53-63)

Every call to `index_meeting`, `reindex_meeting`, or `chat_message` calls `load_settings()`, which performs a filesystem read + TOML parse. During chat streaming, this means reading from disk before every single message.

**Recommendation:** Consider a lighter approach -- refresh only when settings are actually changed (e.g., have `update_settings` push the new config into `RagState`), or cache with a last-modified timestamp check.

### 12. Unused `rag: State<'_, RagState>` parameter in `get_chat_status` when status is not `Ready` (line 199)

The `rag` state is only used when `meeting.chat_status == ChatStatus::Ready` for the cross-check. When the status is anything else, the parameter is acquired but unused. This is harmless but slightly misleading in the API signature.

**Recommendation:** No change needed, but worth a comment explaining why the parameter exists.

### 13. `chat_message` does not validate `embeddings_model` is non-empty (line 150)

The config validation only checks `embeddings_url` and `chat_url`. If `embeddings_model` is empty, the API call to the embeddings endpoint will send an empty model string, which may produce a confusing 400 error from the API rather than a clear local validation error.

**Recommendation:** Also validate `embeddings_model`, `chat_model`, and the API keys are non-empty.

### 14. `do_index_meeting` allocates a `Vec<String>` clone of all chunk texts (line 263)

```rust
let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
```

This clones every chunk's text into a separate `Vec<String>` just to pass to `embed_batch`. For large transcripts, this doubles memory usage for the text data.

**Recommendation:** Consider making `embed_batch` accept `&[&str]` instead of `&[String]`, or pass chunk references directly. This is a minor optimization but aligns with the project's stated priority of efficiency.

---

## Summary

| Severity | Count | Key themes |
|----------|-------|------------|

---

| Suggestion | 6 | Duplicated validation, unnecessary chat_url check in index, disk reads per call, memory |


---

### Suggestions (nice to have)

**S1. `commands.rs` hardcodes `tracks` metadata even for audio-only recordings**

In `commands.rs` lines 104-107, the `stop_recording` command always reports two tracks (mic + system), regardless of whether `build_audio_only` was used (which only has a mic track). This gives incorrect metadata.

**S2. No `videorate` element in the video chain**

The video chain applies `videoscale` and `capsfilter` with a target framerate, but without a `videorate` element, the framerate cap in the caps filter may reject buffers rather than re-timing them. PipeWire screen sources can produce variable-rate frames. Adding `videorate` before the capsfilter would ensure smooth frame timing.

**S3. Error type should be a proper enum, not `String`**

All methods return `Result<_, String>`. This makes programmatic error handling impossible (callers cannot pattern-match on error types). Consider defining a `PipelineError` enum with variants like `ElementCreation`, `LinkFailure`, `StateChange`, `Timeout`, etc.

**S4. The `_actual_backend` return value from `create_video_encoder_with_fallback` is discarded**

On line 97, the actual encoder backend used after fallback is captured as `_actual_backend` and ignored. If the user requested VAAPI but fell back to Software, this information is never logged or surfaced. This should at minimum be logged with `eprintln!` for diagnostics.

**S5. No `streamable-buffers` property set on `matroskamux`**

For a recording use case where the file might be partially read (e.g., crash recovery), setting `matroskamux` property `streamable=true` would write cues inline, making the file partially recoverable. Without this, a crash before the mux writes its index results in a completely unplayable file.

**S6. `file_size()` reads filesystem metadata on every call**

`file_size()` (line 218) calls `std::fs::metadata()` every time. During active recording, GStreamer's `filesink` may buffer writes, so the filesystem metadata may not reflect the actual amount of data recorded. This is a minor inaccuracy but could be confusing in the UI.

---

### Summary Table

| ID | Severity | Area | Description |
|----|----------|------|-------------|

---

| S1 | Suggestion | Metadata | Track info hardcoded regardless of pipeline type |

---

| S2 | Suggestion | Pipeline Design | Missing `videorate` for variable framerate sources |

---

| S3 | Suggestion | Error Handling | String errors instead of typed enum |

---

| S4 | Suggestion | Diagnostics | Encoder fallback result is discarded silently |

---

| S5 | Suggestion | Robustness | No streamable/cues-inline setting on matroskamux |

---

| S6 | Suggestion | Accuracy | `file_size()` may be inaccurate during recording |


---

## SUGGESTIONS (Nice to Have)

### 9. `handleIndex` does not poll for "indexing" status completion

The `indexMeeting` API call (line 62) returns `void`. The code then immediately calls `getChatStatus` (line 63). If `indexMeeting` is a fire-and-forget operation that starts an async process on the backend, the status will likely be `"indexing"` when polled immediately after. The component will then show the indexing spinner (line 169), but there is no polling mechanism to detect when indexing completes. The user would need to navigate away and back.

Consider adding a polling interval (similar to how `TranscriptView` polls transcription status) to automatically transition from "indexing" to "ready".

### 10. Input is a single-line `<input>` but `handleKeyDown` checks for Shift+Enter

**File:** Lines 128-133, 240

```jsx
<input type="text" ... onKeyDown={handleKeyDown} />
```

```js
if (e.key === "Enter" && !e.shiftKey) {
```

The Shift+Enter check implies multi-line input support, but an `<input type="text">` cannot contain newlines. If multi-line questions were intended, this should be a `<textarea>`. If not, the `!e.shiftKey` check is unnecessary (though harmless).

### 11. Accessibility: send button has no accessible label

**File:** Lines 249-257

The send button contains only an SVG icon with no `aria-label`, `title`, or visible text. Screen readers will announce it as an unlabelled button.

**Fix:** Add `aria-label="Enviar mensagem"` to the button.

### 12. The `bg-brand-500/8` opacity notation on line 223

```jsx
"bg-brand-500/8 border border-brand-500/10 rounded-br-md"
```

The `/8` opacity shorthand means 8% opacity in Tailwind v4 (which uses `0-100` scale). This is valid in Tailwind v4 since it interprets bare numbers as percentages. Just noting this is correct but could be confusing to readers expecting the v3 `0-100` where `/8` would have been invalid (v3 used `/5`, `/10`, `/20`, etc.).

---

## Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 9 | Suggestion | No polling for indexing completion | 58-73 |

---

| 10 | Suggestion | Single-line `<input>` with Shift+Enter guard | 128-133, 240 |

---

| 11 | Suggestion | Send button has no accessible label | 249-257 |


---

#### SUGGESTION: System prompt could be stronger for RAG use case

The current system prompt is functional but could be improved in several ways:

1. **No explicit instruction to avoid hallucination**. The prompt says "If the answer is not in the provided context, say so honestly" but does not strongly prohibit fabricating information. For a meeting transcript RAG, hallucinated facts could be particularly harmful (wrong action items, wrong decisions attributed to wrong people).

2. **No instruction about speaker attribution**. Meeting transcripts often have speakers. The prompt does not guide the model on how to handle speaker identification or lack thereof.

3. **No instruction about timestamp format**. The prompt says "mention the timestamps" but does not tell the model what format the timestamps are in (MM:SS or HH:MM:SS), which could lead to the model reformatting them inconsistently.

4. **"Respond in the same language the user asks their question in"** -- This is good for the user (who is a BR Portuguese speaker per the project memory), but it could conflict with transcript content that is in a different language. Consider clarifying: respond in the user's language, but preserve proper nouns and technical terms from the transcript as-is.


---

#### SUGGESTION: Trailing newlines in context output

Line 74-80: The `build_context` function ends each excerpt with `\n\n`, meaning the final context string has two trailing newlines. This is harmless but wastes a couple of tokens. A minor cleanup would be to `trim_end()` the final string.


---

#### SUGGESTION: `ChatMessage` role field is a raw `String`

This is in `chat.rs` (line 9-12), not strictly in `prompt.rs`, but `prompt.rs` constructs `ChatMessage` values with hardcoded string literals (`"system"`, `"user"`). If a typo is introduced (e.g., `"systm"`), it will compile and run without error but produce incorrect API behavior. An enum for the role would provide compile-time safety:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}
```

This is outside the strict scope of `prompt.rs` but directly affects its correctness guarantees.


---

#### SUGGESTION: Test coverage gaps

The tests are solid for the happy path but miss a few edge cases:

- `format_timestamp` with a very large value (e.g., `86400.0` -- 24 hours) to verify HH:MM:SS handles >24h correctly (it would show `24:00:00`, which is fine, but untested).
- `format_timestamp` with `f64::NAN`, `f64::INFINITY`, or negative values.
- `build_context` with a chunk whose `text` is empty or whitespace-only (currently would produce `[Excerpt 1 | 00:00 - 00:05]\n\n\n` due to `trim()` on empty string).
- `build_messages` with a very long `user_question` (not that the function should reject it, but documenting the expectation).

---

### Summary Table

| Severity | Issue | Line(s) |
|---|---|---|

---

| Suggestion | System prompt could be stronger against hallucination | 18-27 |

---

| Suggestion | System prompt lacks speaker attribution guidance | 18-27 |

---

| Suggestion | Trailing newlines waste tokens in context output | 74-80 |

---

| Suggestion | `ChatMessage.role` as String has no compile-time safety | (chat.rs) |

---

| Suggestion | Test coverage gaps for edge cases | 86-144 |

The file is well-structured and does its job correctly for the common case. The most actionable improvement is merging the two system messages into one to ensure broad compatibility across OpenRouter-routed models.

---

# Suggestions (Nice to Have)

1. **Round record button loading indicator** (`RecordButton.tsx`, line 131). Consider adding a spinner or pulsing animation on the mic icon while `starting` is true. Currently only the secondary text button shows "Iniciando...".

2. **Model download progress** (`SettingsPage.tsx`, line 504). If the Tauri backend supports progress events, consider adding a progress bar or percentage for model downloads. The current "Baixando..." text gives no indication of progress for multi-gigabyte downloads.

3. **"Usar" model button loading state** (`SettingsPage.tsx`, line 492). Wrap `handleSetActiveModel` with a brief busy state so the button shows feedback while the API call resolves.

4. **Title save confirmation** (`MeetingPage.tsx`, line 243). Consider briefly flashing a checkmark or "Salvo" indicator after a successful title rename so the user knows it persisted.

5. **Settings: unsaved changes warning**. There is no guard preventing navigation away (via the back button) when settings have been modified but not saved. Consider either auto-saving or showing a confirmation dialog.

---

### SUGGESTIONS (Nice to have)

#### 8. `set_property` after `build()` vs. during builder chain

For the video encoder (lines 42-55), properties are set via `set_property` **after** `build()`. For the audio encoder (line 67), they are set in the builder chain via `.property(...)` **before** `build()`. This inconsistency is not a bug, but mixing styles makes the code harder to follow. The builder-chain style (`.property(...)`) is preferred in `gstreamer-rs` because it consolidates element configuration in one place and the error from `.build()` covers property-setting failures.

**Recommendation**: Move the video encoder bitrate setting into the builder chain when feasible. The current approach has the additional problem that `set_property` panics on failure, while `.property()` in the builder chain returns an error from `.build()`.

#### 9. The `Vulkan` arm in `create_video_encoder` is a no-op with a misleading comment

Lines 57-59:

```rust
EncoderBackend::Vulkan => {
    // Vulkan encoder properties vary, set if available
}
```

This arm does absolutely nothing. The comment says "set if available" but no conditional logic exists. Either implement the property setting or add a `log::warn!` so it is obvious during debugging that the Vulkan encoder is running with default settings.

#### 10. The three VAAPI/CUDA/Software arms are identical -- collapse them

Lines 48-56 repeat `encoder.set_property("bitrate", config.bitrate / 1000)` three times with only a comment difference. This can be collapsed with a `_` or `|` pattern:

```rust
match backend {
    EncoderBackend::Vaapi | EncoderBackend::Cuda | EncoderBackend::Software => {
        // ...set bitrate...
    }
    EncoderBackend::Vulkan => { /* ... */ }
}
```


---

But note: this suggestion conflicts with issue #2 above, because `svtav1enc` needs a different property name. The real fix is to dispatch on `(backend, codec)` or on the element name.

#### 11. No logging anywhere in the module

The module has zero `log::info!`, `log::warn!`, or `tracing` calls. For a module that does hardware probing and fallback, this makes debugging significantly harder. At minimum, log:
- Which encoders were found during `probe_available()`
- Which backend was selected after fallback
- Which bitrate/properties were applied

---

### Summary

| # | Severity | Issue |
|---|----------|-------|

---

| 8 | Suggestion | Inconsistent property-setting style (builder chain vs. post-build) |

---

| 9 | Suggestion | Vulkan arm is a no-op with misleading comment |

---

| 10 | Suggestion | Three identical match arms should be collapsed |

---

| 11 | Suggestion | No logging makes hardware probe debugging very difficult |

**Files reviewed**:
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/encode.rs` (primary target)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/types.rs` (data structures, element name mapping)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs` (usage context)


---

## Suggestions (Nice to Have)

### 9. `base_dir` Should Be Canonicalized at Construction Time

In `LibraryFs::new`, `base_dir` is stored as-is. If it contains relative components or `..`, all subsequent `join` operations will inherit those. Canonicalizing at construction time makes path containment checks (issue 3) simpler and prevents subtle bugs:

```rust
pub fn new(base_dir: PathBuf) -> std::io::Result<Self> {
    fs::create_dir_all(&base_dir)?;
    let base_dir = base_dir.canonicalize()?;
    Ok(LibraryFs { base_dir })
}
```

### 10. Missing Test: `delete_meeting_dir` on Nonexistent Path

The test `test_delete_meeting_dir` only tests deleting an existing directory. There is no test for calling `delete_meeting_dir` on a path that does not exist (which should succeed silently per current logic). Add:

```rust
#[test]
fn test_delete_nonexistent_meeting_dir() {
    let tmp = TempDir::new().unwrap();
    let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
    let result = lib_fs.delete_meeting_dir(&tmp.path().join("nonexistent"));
    assert!(result.is_ok());
}
```

### 11. Missing Test: `read_artifact` on Nonexistent Artifact


---

| 9 | Suggestion | Canonicalize `base_dir` at construction time | 10-13 |

---

| 10 | Suggestion | Missing test for deleting nonexistent directory | tests |

---

| 11 | Suggestion | Missing test for reading nonexistent artifact | tests |

---

| 12 | Suggestion | `delete_media_files` hardcodes artifact kinds | 42-52 |

---

| 13 | Suggestion | No logging with paths in error messages | all methods |


---

## SUGGESTIONS (Nice to Have)

### 8. No test for SRT output format correctness

The test module tests `format_srt_timestamp` and `format_readable_timestamp` in isolation, and tests `resolve_output_path`. But there is no test that `export_srt` produces a valid SRT file (even from synthetic data). A test that creates a temp dir with a valid `transcript.json`, calls `export_srt`, and asserts the output string would catch regressions like wrong line ordering, missing blank lines, or the empty-text issue.

### 9. SRT spec compliance: the final subtitle block should not have a trailing blank line

The SRT specification says blocks are *separated* by blank lines, not *terminated*. The current code unconditionally appends `\n\n` after every block, including the last one. Most players tolerate this, but strictly compliant parsers may not.

**Fix:** Either `trim_end` the final string, or conditionally omit the trailing newlines for the last block.

### 10. `fs::copy` return value is discarded

**File:** lines 49 and 59.

`fs::copy` returns `Result<u64, io::Error>` where the `u64` is the number of bytes copied. The code uses `?` to propagate the error, which is correct, but the bytes-copied value is silently discarded. This is fine as-is, but logging the byte count could help debugging.

### 11. PDF: no handling for extremely long meetings

For a 3-hour meeting with dense segments, the PDF could be hundreds of pages. `genpdf` builds the entire document in memory before rendering. There is no progress reporting or memory guard. Not urgent, but worth a TODO comment.

### 12. Consider adding a `#[cfg(test)]` integration test for `load_segments`

Since `load_segments` is the hinge point between raw JSON and all downstream exports (SRT and PDF), a focused test that round-trips a `TranscriptResult` through `serde_json::to_string` and then parses it back via `load_segments` (using a temp dir) would be high-value.

---

## Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 8 | Suggestion | No integration test for SRT output correctness | -- |

---

| 9 | Suggestion | Trailing blank line after last SRT block (strict spec) | 85 |

---

| 10 | Suggestion | `fs::copy` byte count discarded | 49, 59 |

---

| 11 | Suggestion | No memory/progress guard for very large PDFs | 93-165 |

---

| 12 | Suggestion | No round-trip test for `load_segments` | -- |

**Relevant files reviewed:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/transcript.rs` (primary)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/mod.rs` (ExportError definition)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/types.rs` (TranscriptFormat, SaveMode, resolve_output_path)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/types.rs` (canonical TranscriptResult and Segment types)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/orchestrator.rs` (how transcript.json is serialized)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/commands.rs` (Tauri command layer)

---

## SUGGESTIONS

### S1. Consider adding a `videorate` element before the capsfilter

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 84-95

The capsfilter enforces a framerate, but `videoscale` only handles spatial scaling, not framerate conversion. If the PipeWire source produces frames at a different rate than the configured `fps`, the capsfilter negotiation may fail. Adding a `videorate` element before the capsfilter would handle framerate conversion:

```
pipewiresrc -> queue -> videoconvert -> videoscale -> videorate -> capsfilter -> encoder
```

### S2. The `probe_encoders` command calls `gst::init()` redundantly

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs`, line 143

`gstreamer::init()` is already called in `lib.rs:53` at startup. Calling it again is safe (it is idempotent) but unnecessary and suggests the author was unsure whether init had been called. Remove the redundant call for clarity.

### S3. Consider using `gst::Pipeline::with_name()` for debuggability

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 24, 71

Named pipelines are easier to debug with `GST_DEBUG` and tools like `gst-dot`. Using `gst::Pipeline::with_name("hlusra-audio")` or `gst::Pipeline::with_name("hlusra-video")` would help.

### S4. The `start_time` is set in the constructor and then again in `start()`

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 11, 57, 170

`start_time` is initialized to `Instant::now()` during `build_audio_only` / `build_with_video` (construction time), then overwritten in `start()`. The construction-time value is never used. Consider using `Option<Instant>` or initializing with a sentinel to make the intent clearer.

### S5. Bus message handling could watch for errors during playback, not just at stop

Currently, errors from the pipeline (e.g., encoder failure, disk full) are only detected when `stop()` is called. During active recording, there is no bus watch. If the pipeline enters an error state mid-recording, the user will not know until they try to stop. Consider spawning a bus watch thread or using `bus.add_watch()` to detect errors and emit them to the frontend in real time.

### S6. `output_path.to_string_lossy()` may corrupt non-UTF8 paths

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 41, 133

If the output path contains non-UTF8 characters, `to_string_lossy()` will replace them with the Unicode replacement character, resulting in a wrong file path for `filesink`. On Linux, paths can contain arbitrary bytes. Consider using `to_str().ok_or("Invalid path")?` to fail cleanly instead of silently corrupting the path.

---

## Summary

| Severity | Count | Key Items |
|----------|-------|-----------|

---

| SUGGESTION | 6 | Add videorate; remove redundant init; name pipelines; clean up start_time; add bus watch; handle non-UTF8 paths |

The most urgent fix is **C1** (VAAPI element names). On any modern GStreamer 1.22+ system, hardware-accelerated encoding via VA-API will silently fail and fall back to software encoding (x264enc/x265enc), which will use significantly more CPU. The fix is to update `gst_element_name()` in `types.rs` to use the `va` plugin names (`vah264enc`, `vah265enc`, etc.).

The second most urgent fix is **C2** (system audio capture). The current approach will not capture desktop audio. This requires either a PipeWire monitor source approach or deferring system audio capture to a later milestone.

---

### SUGGESTIONS (Nice to Have)

**7. Redundant `Content-Type` header (lines 113, 220)**

When using `.json(&body)`, reqwest already sets `Content-Type: application/json` automatically. The explicit `.header("Content-Type", "application/json")` is harmless but redundant. Removing it reduces noise.

**8. Buffer reallocation on every line extraction (line 152)**

```rust
buffer = buffer[newline_pos + 1..].to_string();
```

This allocates a new `String` and copies the remaining bytes on every single line. For a streaming LLM response with hundreds of chunks, this is a lot of small allocations. Consider using `buffer.drain(..newline_pos + 1)` to modify in place, or use a `VecDeque<u8>`/`bytes::BytesMut` for more efficient prefix removal.

**9. Error handling duplication between `chat_stream` and `chat_once` (lines 118-132 vs 227-240)**

The error-response parsing logic is duplicated verbatim. Extract a helper like:

```rust
async fn parse_api_error(response: reqwest::Response) -> ChatError { ... }
```

**10. Channel buffer size of 64 is arbitrary (line 134)**

The channel capacity of 64 is fine for most cases but has no documentation explaining why it was chosen. A comment would help future maintainers understand whether this was tuned or arbitrary. For an LLM token stream where the consumer is just forwarding to a Tauri event emitter, a smaller buffer (8-16) would provide equivalent throughput with less memory overhead, since the consumer (in `commands.rs`) processes tokens as fast as they arrive.

**11. Test coverage gaps**

The existing tests cover serialization and the `[DONE]` non-JSON check, which is good. Missing test scenarios:
- SSE line parsing with multi-line buffers, partial chunks, and `data:` prefix extraction
- Error response parsing (`ApiErrorBody` deserialization)
- The streaming task's behavior when the receiver is dropped (backpressure / early cancellation)

These could be tested by extracting the line-parsing logic into a standalone function that takes a `&str` line and returns an `Option<Result<String, ChatError>>`, making it testable without needing an HTTP server.

**12. `#[allow(dead_code)]` on `finish_reason` (line 32)**

If `finish_reason` is never read, consider removing the field entirely and using `#[serde(deny_unknown_fields)]` or just relying on serde's default behavior of ignoring unknown fields. Alternatively, if you intend to use `finish_reason` in the future (e.g., to detect `"content_filter"` stops), add a `// TODO` comment.

---

### Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 7 | Suggestion | Redundant Content-Type header with `.json()` | 113, 220 |

---

| 8 | Suggestion | Buffer reallocation per line; use `drain()` instead | 152 |

---

| 9 | Suggestion | Duplicated error-response parsing logic | 118-132, 227-240 |

---

| 10 | Suggestion | Channel capacity undocumented and possibly oversized | 134 |

---

| 11 | Suggestion | No tests for SSE parsing edge cases | 261-299 |

---

| 12 | Suggestion | `#[allow(dead_code)]` on unused field; remove or document intent | 32 |


---

## Suggestions (Nice to Have)

**8. Memory-efficient media loading:** Instead of `readFile` (loads entire file into JS heap), consider using Tauri's `convertFileSrc()` which creates a `asset://localhost/` URL that the WebView can stream without loading the full file into memory. This works for supported formats.

**9. Audio-only system audio capture:** The audio-only pipeline (`build_audio_only`) only captures mic. The video pipeline attempts both mic and system audio. For consistency, consider adding system audio capture to the audio-only pipeline as well, or at least reflecting the single-track reality in the UI.

**10. Chat event listening race condition:** In `ChatPanel.tsx`, the `listen()` calls at lines 90-107 are async and set up after the `chatMessage` invoke at line 118. There is a theoretical window where early stream events could be missed if the backend responds extremely fast. In practice, the event setup is `await`ed before `chatMessage` is invoked, so this is actually fine -- the ordering is correct.

**11. Error handling in `formatError`:** The pattern matching in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/format.ts` is well done and provides user-friendly Portuguese messages. However, the check `raw.includes("ffmpeg")` is case-sensitive and might miss errors like "FFmpeg not found".

---

## What Was Done Well

- **Architecture:** Clean 5-module separation (Recorder, Library, Transcription, RAG, Export) with well-defined boundaries. The Library acts as a central coordinator without leaking implementation details.
- **Type safety:** Strong serde-based serialization with `rename_all = "snake_case"` consistently applied. TypeScript interfaces accurately mirror Rust structs.
- **Error handling:** Custom error types with `thiserror` and `serde::Serialize` implementations throughout. Errors propagate cleanly from backend to frontend.
- **Testing:** Comprehensive unit tests in every module -- db roundtrips, artifact operations, chunker behavior, serialization, type conversions. Good coverage for a project of this scope.
- **Concurrency safety:** Mutex usage with proper `map_err` (not `unwrap`), `spawn_blocking` for CPU-intensive work (transcription, model download), and correct lock scoping to minimize contention.
- **UX:** Portuguese localization throughout the UI, clean state management with proper loading/error/success states, and thoughtful touches like the gallery key-based remount pattern for fresh data loading.

---

## SUGGESTIONS

### 7. `get_thumbnail` returns `Vec<u8>` -- consider returning base64

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/commands.rs` (line 28)

If the thumbnail feature is eventually used, returning `Option<Vec<u8>>` will serialize as a JSON array of numbers (e.g., `[255, 216, 255, ...]`), which is extremely inefficient for binary data. Consider returning a base64-encoded string instead, or serving thumbnails via Tauri's asset protocol.

### 8. Empty `try {} catch {}` for window resize

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx` (line 43)

```typescript
try {
  const win = getCurrentWindow();
  const [w, h] = HOME_VIEWS.includes(view) ? [800, 400] : [800, 600];
  await win.setSize(new LogicalSize(w, h));
} catch {}
```

Silent swallowing of errors makes debugging impossible. At minimum, add `console.warn` in the catch block.

### 9. Consider adding `core:window:allow-set-focus` to capabilities

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/capabilities/default.json`

Currently the window permissions include `core:window:default` and `core:window:allow-set-size`. If you ever need to bring the window to focus (e.g., after a recording finishes from a minimized state), you will need `allow-set-focus`. Not needed now, but worth noting.

### 10. Consistent error handling style -- `String` vs custom error types

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` -- uses `Result<_, String>`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/commands.rs` -- uses `Result<_, String>`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/commands.rs` -- uses `Result<_, LibraryError>`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/commands.rs` -- uses `Result<_, RagCommandError>`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/commands.rs` -- uses `Result<_, ExportError>`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/settings/commands.rs` -- uses `Result<_, SettingsError>`

The recorder and transcription modules use raw `String` errors while others use proper error types. This is a consistency issue. The `String` approach works but loses structured error information. Consider creating `RecorderError` and `TranscriptionError` types with `Serialize` impls to match the pattern used elsewhere.

### 11. The `probe_encoders` command re-initializes GStreamer

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` (line 143)

```rust
gstreamer::init().map_err(|e| format!("GStreamer init failed: {}", e))?;
```

GStreamer is already initialized in `lib.rs` at startup. Calling `gstreamer::init()` again is harmless (it is idempotent) but unnecessary. The `map_err` path will never be reached since init already succeeded. Consider removing it for clarity.

---

## Summary Table

| # | Severity | Area | Issue |
|---|----------|------|-------|

---

| 6 | SUGGESTION | Types | `get_thumbnail` returns `Vec<u8>` (inefficient JSON serialization) |

---

| 7 | SUGGESTION | Error handling | Empty catch block in window resize |

---

| 8 | SUGGESTION | Capabilities | Consider `allow-set-focus` for future use |

---

| 9 | SUGGESTION | Consistency | Mixed error styles (`String` vs typed errors) across modules |

---

| 10 | SUGGESTION | Cleanup | `probe_encoders` redundantly calls `gstreamer::init()` |


---

## SUGGESTIONS (Nice to have)

### 8. Redundant `.clone()` on `tracks` in `finalize_meeting`

**Location:** Lines 92 and 101

```rust
let tracks = info.tracks.clone();  // line 92
// ...
tracks: tracks.clone(),           // line 101
```

`tracks` is cloned from `info.tracks`, then cloned again into the `Meeting` struct. Since `info` is consumed by value, you can move `info.tracks` instead of cloning, and use a single reference for the `db.insert_meeting` call. The double clone is wasteful.

**Fix:** Take `info` by value (already the case), destructure or move `info.tracks` into the struct directly.

### 9. `get_meeting` swallows the actual database error

**Location:** Lines 113-116

```rust
db.get_meeting(id).map_err(|_| LibraryError::NotFound(id.to_string()))
```

Every `rusqlite::Error` is mapped to `NotFound`, including actual database corruption or connection errors. A real DB failure will be reported to the user as "Meeting not found" which is misleading and will make debugging difficult.

**Recommendation:** Distinguish between `QueryReturnedNoRows` (map to `NotFound`) and all other errors (map to `Db`):

```rust
db.get_meeting(id).map_err(|e| match e {
    rusqlite::Error::QueryReturnedNoRows => LibraryError::NotFound(id.to_string()),
    other => LibraryError::Db(other),
})
```

### 10. `db.delete_meeting` silently succeeds for non-existent IDs

**Location:** `db.rs` line 170-173

`LibraryDb::delete_meeting` does not check `self.conn.changes()`. If the ID does not exist, the DELETE is a no-op and it returns `Ok(())`. This is inconsistent with `update_title`, `update_transcription_status`, etc., which all check `changes() < 1` and return an error. In `api.rs` this is masked because `delete_meeting` looks up the meeting first, but the inconsistency in the DB layer could cause silent failures if `delete_meeting` is ever called directly.

### 11. `LibraryError` does not have a variant for "already exists" / duplicate operations

There is no protection against calling `finalize_meeting` with the same `id` after it has already been finalized. The `prepared` HashMap would return `None` (since the entry was removed on first finalize), so it would return `NotFound`. This is technically correct but semantically misleading -- a better error would be "already finalized."

### 12. `eprintln!` for logging

**Location:** Multiple lines (62, 73, 85, 129, 134)

The codebase uses raw `eprintln!` for logging. For a Tauri application, consider using the `log` crate (or `tracing`) which integrates with Tauri's logging plugin and supports log levels, filtering, and structured output. The `[library]` prefix is a manual workaround for what log targets provide natively.

### 13. No test for concurrent access or error paths

The test suite covers the happy path well but does not test:
- Calling `finalize_meeting` with an unknown ID (should return `NotFound`)
- Calling `delete_meeting` on a non-existent meeting
- Calling `prepare_meeting` followed by another `prepare_meeting` without finalizing the first
- The `get_meeting_detail` transcript fallback logic (JSON -> TXT -> None)

---

## Summary Table

| # | Severity | Issue | Location (lines) |
|---|----------|-------|-------------------|

---

| 8 | Suggestion | Redundant double `.clone()` on tracks | 92, 101 |

---

| 9 | Suggestion | All DB errors swallowed as `NotFound` in `get_meeting` | 113-116 |

---

| 10 | Suggestion | `db.delete_meeting` does not check affected rows (in db.rs) | db.rs:170-173 |

---

| 11 | Suggestion | No "already finalized" error variant | 78-90 |

---

| 12 | Suggestion | Raw `eprintln!` instead of structured logging | Multiple |

---

| 13 | Suggestion | Missing tests for error paths and edge cases | 216-321 |


---

### SUGGESTIONS (nice to have)

**7. `load_settings()` is called on every `transcribe_meeting` invocation (line 19 via `create_provider`)**

This reads and parses the TOML file from disk on every call. For a desktop app with infrequent transcription this is fine, but if settings were managed as Tauri state (like `Library` is), it would be more consistent with the rest of the architecture.


---

Severity: Suggestion.

**8. `get_transcription_status` (line 131) loads the entire `Meeting` struct just to return one field**

This fetches all meeting data including `dir_path`, `tracks`, etc. from the database just to read `transcription_status`. A dedicated `get_transcription_status` query on `LibraryDb` would be more efficient, though for a desktop app this is unlikely to be a bottleneck.


---

Severity: Suggestion.

**9. `download_model` (line 159) provides no progress feedback**

Large models (e.g., `large` at ~3 GB) will block with no progress indication to the frontend. Consider emitting Tauri events during the download to show progress.


---

Severity: Suggestion -- user experience improvement.

**10. The `get_artifact_path` call on line 57 acquires the Library's DB mutex lock (it calls `get_meeting` internally), then on line 66 `update_transcription_status` acquires it again**

These are two separate lock acquisitions in the same logical block (lines 52-74). While not a deadlock risk (both are on the same mutex, acquired sequentially), it means another thread could interleave between them. Combining these into a single lock acquisition would be slightly more efficient and atomic.


---

Severity: Suggestion.

**11. Missing `#[must_use]` or log on the silenced error in the failure branch (line 110)**

```rust
let _ = library.update_transcription_status(&id, TranscriptionStatus::Failed);
```

If this DB update fails (e.g., the database is locked or corrupted), the meeting's status will remain `Processing` forever, with no log entry. Consider at least logging the error:

```rust
if let Err(e) = library.update_transcription_status(&id, TranscriptionStatus::Failed) {
    eprintln!("[transcription] Failed to set status to Failed for {id}: {e}");
}
```


---

Severity: Suggestion -- aids debugging stuck meetings.

---

### Summary Table

| # | Issue | Severity | Line(s) |
|---|-------|----------|---------|

---

| 7 | Settings read from disk on every call | Suggestion | 19 |

---

| 8 | Full Meeting loaded just for status field | Suggestion | 131-138 |

---

| 9 | No download progress events | Suggestion | 159-165 |

---

| 10 | Two separate mutex acquisitions in one logical block | Suggestion | 52-74 |

---

| 11 | Silenced DB error on failure status update | Suggestion | 110 |

The most actionable fix is issue #2 (concurrent transcription guard). Issues #4 and #5 are low-effort, high-value improvements that prevent confusing user-facing errors.

---

**SUGGESTION 1: The error for "ffmpeg not found" is indistinguishable from other IO errors.**

The `map_err(|e| format!("Failed to run ffmpeg: {e}"))` path handles both "binary not found" (`io::ErrorKind::NotFound`) and other IO failures identically. Consider checking `e.kind() == io::ErrorKind::NotFound` to give users a clearer message like "FFmpeg is not installed or not in PATH".


---

**SUGGESTION 2: No `-f wav` flag, but this is fine.**

FFmpeg infers the output format from the `.wav` extension, and `pcm_s16le` is the default WAV codec. No issue here, but adding `-f wav` would make intent explicit.


---

**SUGGESTION 3: Edge case -- MKV has no audio streams.**

If the recording has no audio (e.g., a video-only recording), FFmpeg will fail with an error about stream `0:a:0` not being found. The error is captured and propagated, so this does not crash, but the error message from FFmpeg will be opaque to the user. Consider probing for audio streams first (like `audio.rs` does with `count_audio_streams`) or wrapping the error with a user-friendly message.

---

### FILE 2: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/audio.rs`

**What it does:** Exports audio from MKV to MP3/WAV/Opus/OGG, mixing multiple audio tracks when the format requires it.

**Reconstructed command (2-track mixdown, MP3):**
```
ffmpeg -y -i recording.mkv -filter_complex "[0:a]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]" -map "[aout]" -vn -codec:a libmp3lame -q:a 2 audio.mp3
```

**Reconstructed command (1-track, MP3):**
```
ffmpeg -y -i recording.mkv -vn -codec:a libmp3lame -q:a 2 audio.mp3
```


---

**SUGGESTION 4: The `pan` filter arithmetic.**


---

**SUGGESTION 5: Consider using `amix` instead of `amerge+pan`.**

The `amix=inputs=2` filter would produce a simpler command and automatically normalizes volumes. The `amerge` approach gives more control but is more complex. For a meeting recorder, `amix` may be more appropriate since it handles volume normalization:
```
[0:a:0][0:a:1]amix=inputs=2:duration=longest[aout]
```

---

### FILE 3: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/video.rs`

**What it does:** Exports video from MKV to MP4 or MKV with H.264 or H.265 codec.

**Reconstructed command (H.265 to H.264 transcode to MP4):**
```
ffmpeg -y -i recording.mkv -codec:v libx264 -preset medium -codec:a copy -f mp4 video.mp4
```

**Reconstructed command (H.265 stream-copy to MP4):**
```
ffmpeg -y -i recording.mkv -codec:v copy -codec:a copy -f mp4 video.mp4
```


---

**SUGGESTION 6: No `-map` flags in video export.**

Without `-map`, FFmpeg selects the "best" video stream and the "best" audio stream. For an MKV with one video and two audio tracks, this means only one audio track is carried over. If the intent is to preserve both audio tracks (mic + system), add `-map 0` to copy all streams. If the intent is to keep only one mixed audio track, the user should be aware that the second audio track is dropped.


---

**SUGGESTION 7: No CRF/quality parameter for libx264.**

When transcoding to H.264 at line 45:
```rust
cmd.arg("-codec:v").arg("libx264").arg("-preset").arg("medium");
```

There is no `-crf` value specified. FFmpeg's default CRF for libx264 is 23, which is reasonable for general use. However, for meeting recordings (typically low motion), a higher CRF like 28 could significantly reduce file size with no perceptible quality loss. Consider making this configurable or setting an explicit value.

---

### CROSS-CUTTING CONCERNS

**SECURITY: No command injection risk.**

All three files use `Command::new("ffmpeg")` with `.arg()` for each argument. File paths are passed as `OsStr` via `.arg(path)`. This is safe against command injection. Verified across all files.

**ERROR HANDLING: Adequate but could be improved.**


---

All three files check `output.status.success()` and capture stderr. The `ExportError::Io` variant (via `From<std::io::Error>`) covers the "ffmpeg not found" case. However, as noted in Suggestion 1, the error message does not distinguish "not installed" from "permission denied" or other IO errors.

**EDGE CASE: Very long recordings.**

The code uses `.output()` which buffers all of stdout and stderr in memory. For a very long recording, FFmpeg's stderr can be verbose (it logs progress every few seconds). This is unlikely to be a problem in practice (stderr is typically a few KB even for hour-long recordings), but for extreme cases, consider using `.spawn()` + `.wait_with_output()` or piping stderr to `/dev/null` when not needed.

**EDGE CASE: Empty MKV files / corrupt recordings.**

If the recording pipeline crashes or is killed, the MKV file may be truncated or empty. FFmpeg will fail with an error, which is captured and returned. The error message from FFmpeg will be something like "Invalid data found when processing input." This is handled correctly (the error propagates), but the user-facing message could be friendlier.

---

### Issue Summary Table

| # | Severity | File | Issue |
|---|----------|------|-------|

---

| 6 | SUGGESTION | `orchestrator.rs:78` | "FFmpeg not found" error is indistinguishable from other IO errors |

---

| 7 | SUGGESTION | `orchestrator.rs` | No pre-check for "file has no audio streams" before running ffmpeg |

---

| 8 | SUGGESTION | `video.rs:45` | No explicit CRF value for libx264; relies on FFmpeg default of 23 |

---

| 9 | SUGGESTION | `video.rs` | No `-map` flags means only one audio track is preserved in video export |

---

| 10 | SUGGESTION | `audio.rs:70` | Consider `amix` over `amerge+pan` for simpler mixdown with volume normalization |


---

## Suggestions (Nice to Have)

### 7. `LocalProvider` does not implement `Send + Sync` explicitly

The trait bound `TranscriptionProvider: Send + Sync` requires the implementor to be `Send + Sync`. `LocalProvider` only holds a `WhisperModel` (which is `String + u64 + bool`), so it auto-derives `Send + Sync`. This is fine but worth noting -- if `LocalProvider` ever holds a `WhisperContext` for reuse, it would break.

### 8. No reuse of `WhisperContext` across calls

Each call to `transcribe()` creates a new `WhisperContext` (loading the model from disk). For a meeting recorder that may transcribe multiple chunks, this is very expensive -- model loading can take hundreds of milliseconds to seconds. Consider caching the `WhisperContext` in the `LocalProvider` struct (wrapped in `Arc<Mutex<>>` or similar).

### 9. The `detected_language` default is `"auto"` (line 97)

```rust
let mut detected_language = String::from("auto");
```

If language detection fails, the caller receives `"auto"` as the language, which is not an actual language code. Consider using `"unknown"` or returning an `Option<String>` to make the failure explicit.

### 10. Missing `#[allow(unused)]` or tests

There are no unit tests for `load_wav_samples` or integration tests for the transcription pipeline. Given the complexity of the WAV loading and timestamp conversion, at least a unit test for `load_wav_samples` with a known WAV buffer would be valuable.

### 11. Integer normalization for non-16-bit audio (line 36)

```rust
let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
```

This correctly computes the max value for signed integer normalization for any bit depth (8-bit, 16-bit, 24-bit, 32-bit). However, `bits_per_sample` is a `u16`, and if it were ever 0 (malformed file), `bits_per_sample - 1` would underflow. The `hound` crate likely validates this, so this is very low risk.

---

## Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 7 | SUGGESTION | `Send + Sync` derived implicitly -- fragile if struct changes | 10-12 |

---

| 8 | SUGGESTION | `WhisperContext` recreated on every call -- expensive | 70-74 |

---

| 9 | SUGGESTION | `"auto"` as fallback language is ambiguous | 97 |

---

| 10 | SUGGESTION | No tests for WAV loading or timestamp conversion | -- |

---

| 11 | SUGGESTION | Theoretical underflow on `bits_per_sample - 1` | 36 |

---

## Action Required


---

## SUGGESTIONS

### S1. Shadow-naming `from_str` methods on enums

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/types.rs` (lines 134, 156, 180)

The custom `from_str` methods on `MediaStatus`, `TranscriptionStatus`, and `ChatStatus` shadow the standard `std::str::FromStr` trait. While they work, implementing the trait instead would be more idiomatic and allow using `.parse()`:

```rust
impl std::str::FromStr for MediaStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> { ... }
}
```

### S2. `get_recording_status` holds the pipeline mutex for the entire status read

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` (lines 126-138)

The `file_size()` call inside the mutex lock hits the filesystem (`std::fs::metadata`). For a polled status endpoint this is fine at low frequency, but if the frontend polls aggressively, the filesystem I/O while holding the lock could cause contention. Not urgent but worth noting if polling increases.

### S3. Consider `unchecked_transaction` safety

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/vector_store.rs` (lines 275, 314)

`unchecked_transaction()` is used because `Connection` is not `&mut self`. This is safe here because the `VectorStore` is behind a `Mutex`, so only one thread accesses the connection at a time. A brief comment documenting this invariant would help future maintainers.

### S4. Duplicate test helpers across `audio.rs` and `video.rs`

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/audio.rs` (lines 111-127)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/video.rs` (lines 67-87)

Both files have identical tests for `resolve_output_path` (Save and SaveAs modes). These are already covered by the comprehensive tests in `types.rs` (lines 150-165). The duplicates in `audio.rs` and `video.rs` can be removed.

### S5. `get_thumbnail` returns `Option<Vec<u8>>` -- consider base64 or asset protocol

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/commands.rs` (lines 28-35)

Returning raw bytes for a JPEG thumbnail through Tauri's IPC serializes the entire binary as a JSON array of numbers, which is very inefficient. Consider either returning a file path (and using Tauri's asset protocol) or base64-encoding the data.

### S6. `eprintln!` used extensively for logging

Multiple files use `eprintln!` for logging throughout the codebase. Consider adopting a structured logging crate like `tracing` or `log` with log levels, which would allow filtering by severity and integrating with Tauri's log infrastructure.

---

## What Was Done Well

- **Parameterized SQL everywhere.** All database queries use `params![]` placeholders. No SQL injection vectors exist.
- **Consistent mutex-poison handling.** Every `Mutex::lock()` call uses `.map_err(|_| ...)` to return errors rather than panicking, except for the one `assert_eq!` noted above.
- **Clean FFmpeg CLI invocation.** The FFmpeg/ffprobe calls pass arguments positionally through `.arg()` rather than shell interpolation, preventing command injection.
- **Good separation of concerns.** The orchestrator pattern in transcription keeps the heavy work on blocking threads while holding locks only briefly.
- **Transaction usage in vector store.** Batch inserts use transactions for atomicity and performance.
- **Thorough test coverage.** The 53 tests cover the data model, DB operations, filesystem operations, serialization, and type round-trips well.

---

### SUGGESTIONS (nice to have)

**S1. The `h1` title element is used as a button (line 252-261)**

An `<h1>` with `onClick` is not semantically correct for an interactive element. Screen readers will not announce it as clickable. Consider wrapping it in a `<button>` or using `role="button"` with `tabIndex={0}` and a keyboard handler for Enter/Space.

**S2. Back buttons lack `aria-label` (lines 213, 237)**

The back button contains only an SVG icon and no text. Screen readers will announce it as an empty button. Add `aria-label="Voltar"`.

**S3. Play button lacks `aria-label` (line 291)**

Same issue -- SVG-only button with no accessible label. Add `aria-label="Reproduzir"` (or toggle to `"Pausar"` when playing).

**S4. Badge lookup could fall back gracefully (lines 228-229)**

`TRANSCRIPTION_BADGE[meeting.transcription_status]` and `CHAT_BADGE[meeting.chat_status]` will return `undefined` if the backend ever adds a new status value. The rendering at lines 317 and 322 does handle `undefined` with `{transBadge && ...}`, so it won't crash, but it will silently hide the badge. Consider a fallback: `const transBadge = TRANSCRIPTION_BADGE[meeting.transcription_status] ?? { label: meeting.transcription_status, cls: "bg-white/5 text-white/30" };`

**S5. `formatDate` duplicates locale knowledge (line 21-30)**

`formatDate` hardcodes `"pt-BR"`. If internationalization is ever needed, this should use a shared locale constant or context. Low priority given the project scope, but worth noting.

**S6. No `key` on the conditional media element (lines 334-338)**

When switching between `<video>` and `<audio>` (if `has_video` somehow changes on reload), React may try to morph one into the other rather than remounting. Adding a `key` would force a clean remount:

```tsx
{meeting.has_video ? (
  <video key="video" ref={setMediaRef} src={mediaBlobUrl} />
) : (
  <audio key="audio" ref={setMediaRef} src={mediaBlobUrl} />
)}
```

**S7. `actionError` is never cleared by subsequent successful actions**

After `handleRetranscribe` or `handleReindex` succeeds, the `actionError` from a previous failure remains visible until the next action attempt clears it (line 165, 178). Consider clearing it on success as well, or on a timer.

**S8. Export and Chat buttons do not indicate when chat/transcript is unavailable**

`onChat(getMeetingContext())` always fires regardless of `chat_status`. The parent decides what to do, which is fine architecturally, but the button gives no visual hint that chat is not ready. A subtle disabled state or tooltip would improve UX.

---

### Summary table

| # | Severity | Area | One-line summary |
|---|----------|------|------------------|

---

| S1 | Suggestion | A11y | `h1` used as interactive element for title editing |

---

| S2 | Suggestion | A11y | Back buttons missing `aria-label` |

---

| S3 | Suggestion | A11y | Play button missing `aria-label` |

---

| S4 | Suggestion | Defensive | Badge lookup has no fallback for unknown statuses |

---

| S5 | Suggestion | i18n | Hardcoded `"pt-BR"` locale in `formatDate` |

---

| S6 | Suggestion | React | Missing `key` on conditional video/audio elements |

---

| S7 | Suggestion | UX | `actionError` not cleared after successful action |

---

| S8 | Suggestion | UX | Chat/Export buttons give no hint when feature unavailable |


---

**Suggestions (nice to have):**

13. **Icon container sizes** are `w-10 h-10` in most places but `w-12 h-12` for the play button. Standardize to one size or make the difference more deliberate.

14. **Gallery footer height** (`h-10`) differs from header height (`h-12`). Minor but notable.


---

### SUGGESTIONS (nice to have)

**9. `TranscriptResult` is defined in TS but never returned by any command**

The `TranscriptResult`, `Segment`, and `Word` interfaces (lines 77-95) exist in `api.ts` but no command returns them. They are only used indirectly: `MeetingDetail.transcript` is `string | null` (the raw JSON string), and the frontend would need to `JSON.parse()` it into a `TranscriptResult`. Consider either:
- Adding a dedicated `get_transcript(id: string): Promise<TranscriptResult | null>` command, or
- Documenting that the frontend should `JSON.parse(meetingDetail.transcript)` and cast to `TranscriptResult`.

**10. `formatError` coverage gaps**

Reviewing all error patterns from the Rust backend:

| Rust Error Pattern | Covered in `formatError`? |
|---|---|
| `"Falha ao preparar reuniao"` | Yes (starts with "Falha") |
| `"Falha na captura de tela"` | Yes (starts with "Falha") |
| `"Falha ao montar pipeline"` | Yes (starts with "Falha") |
| `"Falha ao iniciar gravacao"` | Yes (starts with "Falha") |
| `"Falha ao parar gravacao"` | Yes (starts with "Falha") |
| `"Falha ao salvar reuniao"` | Yes (starts with "Falha") |
| `"Nenhuma gravacao ativa"` | Yes (starts with "Nenhuma") |
| `"Recorder lock poisoned"` | Yes (matches "lock poisoned") |
| `"No meeting ID"` | **NO** -- falls to generic fallback. Shows English text. |
| `"StateChangeError"` | Yes |
| `"Meeting not found: ..."` | Yes (matches "not found") |
| `"Database error: ..."` | **NO** -- falls to generic fallback |
| `"IO error: ..."` | **NO** -- falls to generic fallback |
| `"RAG not configured"` | Partially ("not configured" matches) |
| `"TOML deserialization error"` | **NO** -- falls to generic fallback |
| `"TOML serialization error"` | **NO** -- falls to generic fallback |
| `"GStreamer init failed"` | **NO** -- falls to generic fallback |
| `"Transcription task panicked"` | **NO** -- falls to generic fallback |
| `"Download task panicked"` | **NO** -- falls to generic fallback |
| `"Failed to save transcript.json"` | **NO** -- falls to generic fallback |
| `"Source file not found"` | Yes (matches "not found") |
| `"FFmpeg failed"` | Yes (matches "ffmpeg", case-insensitive? No -- `includes` is case-sensitive, but the Rust string is "FFmpeg" not "ffmpeg") |

**ISSUE: `"FFmpeg failed"` will NOT match** `raw.includes("ffmpeg")` because JavaScript `String.includes` is case-sensitive. The Rust error says `"FFmpeg failed"` (capital F), but the check looks for lowercase `"ffmpeg"`. This means FFmpeg export errors will show the raw English error to the user instead of the Portuguese translation.

Similarly, the `"FFmpeg not found"` pattern on line 39 checks for lowercase `"ffmpeg"`, but the actual FFmpeg-related errors from the Rust backend use `"FFmpeg"` (mixed case). You should use case-insensitive matching:

```typescript
if (raw.toLowerCase().includes("ffmpeg")) return "FFmpeg nao encontrado...";
```

Or add the capital variant: `if (raw.includes("ffmpeg") || raw.includes("FFmpeg"))`.

**11. `formatError` does not handle `"DB error"` patterns**

Several Rust commands emit errors like `"DB error (set processing): ..."` or `"DB error: ..."`. These fall through to the generic fallback, showing raw English text. Consider adding:

```typescript
if (raw.includes("DB error") || raw.includes("Database error")) return "Erro no banco de dados. Reinicie o Hlusra.";
```

**12. `formatError` does not handle panic-related errors**

Rust commands use `tokio::task::spawn_blocking` and surface panics as `"Transcription task panicked"` or `"Download task panicked"`. Consider:

```typescript
if (raw.includes("panicked")) return "Erro interno inesperado. Reinicie o Hlusra.";
```

---

### Summary of Required Fixes

| # | Severity | File | Issue |
|---|---|---|---|

---

| 10 | Suggestion | `format.ts` | Add patterns for `"DB error"`, `"panicked"`, `"No meeting ID"` |

Files reviewed:
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/api.ts`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/format.ts`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/types.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/commands.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/types.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/commands.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/types.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/commands.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/types.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/settings/commands.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/settings/config.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/commands.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/types.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/mod.rs`
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/lib.rs`

---

### SUGGESTIONS (Nice to Have)

#### 8. `-loglevel` flag for ffmpeg

The ffmpeg call does not specify `-loglevel`. By default ffmpeg writes verbose info to stderr. Consider adding `-loglevel error` or `-loglevel warning` to reduce noise, similar to how ffprobe is called with `-v error`.

#### 9. Quality parameter for Opus

The Opus export on line 88 uses `libopus` with default settings. Consider adding `-b:a 128k` (or another reasonable bitrate) to control output quality explicitly, as the other formats (MP3 with `-q:a 2`, OGG with `-q:a 4`) all have explicit quality settings.

#### 10. Tests only cover `resolve_output_path`, not the audio export logic

The tests in this file (lines 111-127) are duplicates of tests already in `types.rs` (lines 150-164) and test only `resolve_output_path`. There are no tests for the actual `export_audio` function or `count_audio_streams`. Even unit tests that verify command argument construction (by wrapping `Command` in a trait and mocking it) would add value.

#### 11. The `pan` filter uses `<` (soft assignment) instead of `=` (hard assignment)

```
pan=stereo|c0<c0+c1|c1<c0+c1
```

The `<` operator in the pan filter means "map with auto-gain normalization" while `=` means "map literally." For a mixdown of equal-loudness voice streams, `=` with explicit gain control (e.g., dividing by stream count) would give more predictable volume levels. With `<`, ffmpeg applies its own normalization which may or may not match expectations. This is subjective, but worth being intentional about.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 8 | Suggestion | Add `-loglevel error` to ffmpeg calls |

---

| 9 | Suggestion | Add explicit bitrate for Opus codec |

---

| 10 | Suggestion | Tests are duplicated from types.rs; no coverage for export logic |

---

| 11 | Suggestion | Consider `=` vs `<` in pan filter for predictable gain |

Issues 1, 2, and 3 are all in the same filter string on line 70 and should be fixed together. The corrected filter construction would look something like:

```rust
if stream_count >= 2 {
    let inputs: String = (0..stream_count)
        .map(|i| format!("[0:a:{}]", i))
        .collect();
    let all_channels: String = (0..stream_count)
        .map(|i| format!("c{}", i))
        .collect::<Vec<_>>()
        .join("+");
    let filter = format!(
        "{}amerge=inputs={},pan=stereo|c0<{}|c1<{}[aout]",
        inputs, stream_count, all_channels, all_channels
    );
    cmd.arg("-filter_complex")
        .arg(&filter)
        .arg("-map")
        .arg("[aout]");
}
```

**Note on web search:** I was unable to run web searches or access external documentation due to permission restrictions. My analysis of the ffmpeg amerge filter syntax is based on my knowledge of ffmpeg's filter system. The key facts -- that amerge requires one labeled input pad per stream, that `inputs=` must match the number of fed streams, and that `pan` channel references correspond to amerge's output channel indices -- are well-established ffmpeg behaviors. If you want me to verify any specific point against live documentation, you would need to grant WebSearch or Bash permissions.

---

### SUGGESTION: conn.changes() returns usize, not comparable with < 1

In db.rs line 131:
```rust
if self.conn.changes() < 1 {
```

`Connection::changes()` returns `usize` in rusqlite 0.32. Comparing `usize < 1` is valid (equivalent to `== 0`), but `self.conn.changes() == 0` would be more idiomatic.

**Verdict: CORRECT** -- rusqlite API is used properly throughout both files.

---

## 5. reqwest (0.12.28 with json, multipart, stream, blocking)

**Files:** `.../transcription/api.rs`, `.../transcription/models.rs`, `.../rag/embeddings.rs`, `.../rag/chat.rs`

### CORRECT: blocking::Client and blocking::multipart usage (api.rs)

```rust
use reqwest::blocking::multipart;
let client = reqwest::blocking::Client::new();
let form = multipart::Form::new().part("file", file_part).text("model", ...);
request.multipart(form).send()?;
```

`reqwest::blocking::multipart::Part::file(path)` and `Form::new().part().text()` are correct 0.12 API.

### CORRECT: blocking::get (models.rs)

```rust
reqwest::blocking::get(&url)?;
```

This is a convenience function in reqwest 0.12, returning a `Response` that implements `Read` for streaming the body with `io::copy`.

### CORRECT: Async Client usage (embeddings.rs, chat.rs)

```rust
let client = Client::new();
client.post(&url).header(...).json(&body).send().await?;
response.json::<T>().await?;
```

All correct 0.12 async API.

### CORRECT: bytes_stream() for SSE streaming (chat.rs)

Line 137:
```rust
let mut byte_stream = response.bytes_stream();
```

`Response::bytes_stream()` returns a `Stream<Item = Result<Bytes, Error>>` in reqwest 0.12 when the `stream` feature is enabled. This is used correctly with `futures_util::StreamExt::next()`.

### CORRECT: bearer_auth (api.rs)
`request.bearer_auth(&self.api_key)` is correct 0.12 API.


---

### SUGGESTION: Manual Authorization header vs bearer_auth (embeddings.rs, chat.rs)

In embeddings.rs line 115 and chat.rs line 112:
```rust
.header("Authorization", format!("Bearer {}", self.api_key))
```

This works, but reqwest provides `.bearer_auth()` as a convenience method (which is used in api.rs). For consistency, all three files should use the same approach. Using `.bearer_auth(&self.api_key)` is cleaner and avoids the manual `format!` string.


---

### SUGGESTION: Content-Type header is unnecessary with .json()

In embeddings.rs line 116 and chat.rs line 113:
```rust
.header("Content-Type", "application/json")
.json(&body)
```

When using `.json(&body)`, reqwest automatically sets the `Content-Type: application/json` header. The explicit `.header("Content-Type", "application/json")` is redundant. It will not cause bugs (it just overwrites the same value), but it is unnecessary code.


---

**Verdict: CORRECT** -- All reqwest API usage is valid. Minor consistency suggestions only.

---

## 6. hound (3.5)

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/local.rs`

### CORRECT: WavReader::open
`hound::WavReader::open(audio_path)` is the correct API.

### CORRECT: reader.spec() and sample format handling
```rust
let spec = reader.spec();
spec.channels, spec.sample_rate, spec.sample_format, spec.bits_per_sample
```
All correct hound 3.5 API.

### CORRECT: into_samples and SampleFormat matching
```rust
reader.into_samples::<i32>().filter_map(|s| s.ok()).map(...)
reader.into_samples::<f32>().filter_map(|s| s.ok())
```
`into_samples::<T>()` consumes the reader and returns an iterator of `Result<T, Error>`. Correct API.


---

### SUGGESTION: Silently dropping errors with filter_map

Lines 39 and 46 use `filter_map(|s| s.ok())` which silently discards any sample read errors. For a meeting recorder application, it might be preferable to propagate the first error rather than silently producing a shorter audio buffer. A corrupted WAV file would result in a truncated transcription with no warning.

**Verdict: CORRECT** -- hound API is used properly.

---

## 7. genpdf (0.2.0)

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/transcript.rs`

### CORRECT: Font loading
```rust
genpdf::fonts::from_files("path", "FontName", None)
```
This is the correct genpdf 0.2 API. The third argument is an optional `Option<&str>` for specifying font variants.

### CORRECT: Document creation
```rust
let mut doc = genpdf::Document::new(font_family);
doc.set_title("Meeting Transcript");
```
Correct 0.2 API.

### CORRECT: SimplePageDecorator
```rust
let mut decorator = genpdf::SimplePageDecorator::new();
decorator.set_margins(10);
doc.set_page_decorator(decorator);
```
Correct 0.2 API.

### CORRECT: Pushing elements
```rust
doc.push(genpdf::elements::Paragraph::new("text").styled(style));
doc.push(genpdf::elements::Break::new(1.0));
```
`Paragraph::new()`, `.styled()`, and `Break::new()` are all correct 0.2 API. The `Element` trait import (`use genpdf::Element`) on line 1 is needed for the `.styled()` method.

### CORRECT: Style construction
```rust
genpdf::style::Style::new().bold().with_font_size(18)
```
Correct 0.2 builder pattern.

### CORRECT: render_to_file
```rust
doc.render_to_file(output_path)?;
```
Correct 0.2 API.

**Verdict: CORRECT** -- genpdf 0.2 API is used properly.

---

## 8. chrono (0.4 with serde)

**Files:** `.../library/types.rs`, `.../library/db.rs`

### CORRECT: DateTime<Utc> with serde derive
In types.rs, `DateTime<Utc>` is used as a struct field with `#[derive(Serialize, Deserialize)]`. With the `serde` feature enabled in chrono 0.4, this automatically uses RFC 3339 serialization.

### CORRECT: to_rfc3339 and parse_from_rfc3339
In db.rs:
```rust
meeting.created_at.to_rfc3339()                              // serialization
chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)  // deserialization
    .unwrap_or_default()
    .with_timezone(&chrono::Utc)
```

`to_rfc3339()` and `parse_from_rfc3339()` are stable chrono 0.4 API. The `parse_from_rfc3339` returns `Result<DateTime<FixedOffset>, ParseError>`, and `.with_timezone(&Utc)` correctly converts to `DateTime<Utc>`.


---

### SUGGESTION: unwrap_or_default() on parse failure

Lines 111-113 (db.rs) and 182-183:
```rust
.unwrap_or_default()
```

If the stored RFC 3339 string is somehow corrupt, `unwrap_or_default()` produces `DateTime::default()` which is the Unix epoch (1970-01-01). This is a reasonable defensive fallback but could silently mask database corruption. Consider logging a warning.

**Verdict: CORRECT** -- chrono API is used properly.

---

## 9. motion (motion.dev) -- Frontend

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`

### CORRECT: Import path
```typescript
import { motion, AnimatePresence } from "motion/react";
```
This is the correct import path for motion.dev (formerly Framer Motion). After the rebrand, the package name is `motion` and the React-specific exports come from `motion/react`.

### CORRECT: AnimatePresence with mode="wait"
```tsx
<AnimatePresence mode="wait">
```
`mode="wait"` is the correct prop (replaced the old `exitBeforeEnter` from Framer Motion v5). Correct usage.

### CORRECT: motion.div with animation props
```tsx
<motion.div
  key={view}
  initial={{ opacity: 0, scale: 0.97, y: 8 }}
  animate={{ opacity: 1, scale: 1, y: 0 }}
  exit={{ opacity: 0, scale: 0.97, y: -8 }}
  transition={{ duration: 0.2, ease: [0.16, 1, 0.3, 1] }}
>
```
All props are correct motion.dev API. The cubic bezier `ease` array format is supported.

**Verdict: CORRECT** -- motion.dev API is used properly.

---

## Summary Table

| Library | Version | Verdict | Details |
|---|---|---|---|
| gstreamer | 0.25.1 | **CORRECT** | All builder patterns, Pipeline, Element, Bus, Caps, State APIs correct |

---

## Suggestions (Nice to Have)

**4.** In `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/local.rs` lines 39/46: `filter_map(|s| s.ok())` silently drops WAV sample read errors. Consider propagating the first error to surface corrupt audio files.

**5.** In `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/db.rs` lines 111/182: `unwrap_or_default()` on `parse_from_rfc3339` silently falls back to Unix epoch on corrupt timestamps. Consider logging a warning.

**6.** In `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/db.rs` line 131: `self.conn.changes() < 1` is equivalent to `== 0` but less idiomatic for `usize`.

---

#### SUGGESTION -- Issue 6: No overlap between chunks degrades RAG retrieval quality


---

#### SUGGESTION -- Issue 7: `current_texts.join(" ")` double-spaces when segment text has leading/trailing whitespace

If `segment.text` is `" hello "`, then joining with `" "` produces `"... hello  next..."` with a double space. The final `.trim()` only trims the outer edges, not internal double spaces. This is cosmetic but can affect embedding quality.

**Recommendation:** Either trim each segment's text before pushing, or normalize the joined text:

```rust
current_texts.push(segment.text.trim().to_string());
```

---


---

#### SUGGESTION -- Issue 8: Test `test_multiple_segments_grouped_by_chunk_size` asserts 4 chunks but doesn't verify chunk content

The test on line 131 verifies the count (4 chunks) and the chunk indices, but does not verify the actual text content of any chunk. This means a bug that shuffles or drops segment text would not be caught.

**Recommendation:** Add content assertions:

```rust
assert_eq!(chunks[0].text, "one two three");
assert_eq!(chunks[1].text, "four five six");
```

---


---

#### SUGGESTION -- Issue 9: No test for `chunk_size = 1` with multi-word segments

When `chunk_size = 1` and a segment has 3 words, the segment still gets included as a whole chunk (per Issue 4). The existing test `test_chunk_ids_are_unique` uses `chunk_size = 1` with single-word segments, which does not exercise the oversized-segment path.

**Recommendation:** Add a test that verifies behavior when a segment exceeds `chunk_size`:

```rust
#[test]
fn test_segment_exceeding_chunk_size_is_not_split() {
    let tr = TranscriptResult {
        language: "en".into(),
        segments: vec![make_segment(0.0, 5.0, "one two three four five")],
        full_text: String::new(),
    };
    let chunks = chunk_transcript("m1", &tr, 2);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].text, "one two three four five");
}
```

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 6 | Suggestion | No chunk overlap hurts retrieval quality at segment boundaries |

---

| 7 | Suggestion | Double spaces possible in chunk text from untrimmed segment text |

---

| 8 | Suggestion | Test assertions missing for chunk text content |

---

| 9 | Suggestion | No test exercising oversized-segment behavior |

---

### Suggestions (nice to have)

**S1. Tests do not exercise `load_settings` / `save_settings` with the real config path**

The test `test_save_and_load_to_custom_path` manually writes/reads TOML to a temp dir, bypassing `load_settings` and `save_settings` entirely. This means the actual I/O functions (including `create_dir_all`, the `exists()` check, and the default-creation path) are never tested. The `config_path()` function is hardcoded, making it impossible to inject a test path. Consider accepting an optional path parameter or making `config_path` configurable for testing.

**S2. `PartialEq` is not derived on settings structs**

The roundtrip test (line 137) compares individual fields. Deriving `PartialEq` on all settings structs would allow `assert_eq!(parsed, settings)` which is both more thorough and more readable.

**S3. Missing `#[serde(rename_all = "snake_case")]` or explicit `#[serde(rename)]`**

The TOML keys currently rely on serde's default behavior (which is to use the Rust field names as-is). This works since all fields already use snake_case, but making it explicit with `#[serde(rename_all = "snake_case")]` would serve as documentation and protection against accidental camelCase fields in the future.

**S4. `save_settings` does not write atomically**

`fs::write` truncates and writes in place. If the process is killed mid-write (e.g., power loss, `kill -9`), the config file can be left empty or partially written. The standard approach is to write to a temporary file in the same directory and then atomically rename it:

```rust
use std::io::Write;
let tmp_path = path.with_extension("toml.tmp");
let mut f = fs::File::create(&tmp_path)?;
f.write_all(content.as_bytes())?;
f.sync_all()?;
fs::rename(&tmp_path, &path)?;
```

**S5. Consider `#[serde(deny_unknown_fields)]` for forward compatibility detection**

If you want to warn users who have stale/typo'd fields in their config, `deny_unknown_fields` would cause deserialization to fail with a helpful error message pointing to the unknown key. This is a trade-off against forward compatibility (it would break if a newer version of the app writes fields that an older version does not know about), but for a single-user desktop app it can be useful for catching typos.

---

### Summary Table

| ID | Severity | Issue |
|----|----------|-------|

---

| S1 | Suggestion | Tests bypass `load_settings`/`save_settings`, missing coverage on I/O paths |

---

| S2 | Suggestion | Derive `PartialEq` for easier and more thorough assertions |

---

| S3 | Suggestion | Explicit `#[serde(rename_all)]` for self-documenting TOML key strategy |

---

| S4 | Suggestion | Non-atomic writes risk config corruption on crash |

---

| S5 | Suggestion | Consider `deny_unknown_fields` for typo detection |

The most impactful fix is **C1** (`#[serde(default)]`) -- it is a one-line annotation per struct, uses the `Default` impls you already wrote, and prevents the app from breaking every time the schema evolves.

---

### SUGGESTIONS (Nice to Have)

**9. Race condition on concurrent `download_model` calls for the same model**

If two Tauri commands fire simultaneously for the same model (e.g., user double-clicks), both will pass the `dest.exists()` check on line 55, both will download to the same `.part` file, and the results are undefined (interleaved writes or one truncating the other). A file lock (`flock` / `fs2::FileExt::lock_exclusive`) on the `.part` file would prevent this.

**10. `set_active_model` does not use atomic write for the `.active_model` file**

`fs::write` on line 115 is not atomic. A crash mid-write could leave a corrupted `.active_model` file. Since this file is tiny, the risk is low, but for consistency with the download approach, writing to `.active_model.tmp` then renaming would be more robust.

**11. Missing `#[cfg(test)]` module**

There are zero tests for any function in this file. At minimum, unit tests for `list_available_models`, `get_active_model` fallback logic, and `set_active_model` validation would catch regressions quickly and could use a `tempdir` to avoid touching real state.

**12. `size_bytes` values in the catalogue are approximate and unchecked**

The `size_bytes` field is presumably shown to users in the UI, but it is never validated against the actual download size (`Content-Length` header). If the upstream model files change size (which they have historically), the displayed sizes become stale. Consider either updating them from the HTTP response headers or dropping the field in favor of a runtime check.

**13. `models_dir()` creates directories on every call**

Every public function in this module calls `models_dir()`, which calls `fs::create_dir_all` every time. After the first successful call, the directory exists and subsequent calls are wasted syscalls. This is harmless but wasteful. An `OnceLock<PathBuf>` would be a clean fix.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 9 | Suggestion | Race condition on concurrent downloads of the same model |

---

| 10 | Suggestion | `set_active_model` does not atomically write `.active_model` |

---

| 11 | Suggestion | No unit tests |

---

| 12 | Suggestion | `size_bytes` values are hardcoded approximations, never validated |

---

| 13 | Suggestion | `models_dir()` calls `create_dir_all` on every invocation |


---

## SUGGESTIONS (Nice to Have)

### 10. `embedding_to_blob` could use `bytemuck` for zero-copy

The current implementation allocates and copies byte-by-byte. The `bytemuck` crate allows direct reinterpretation of `&[f32]` as `&[u8]`, which is zero-copy and eliminates allocation:

```rust
fn embedding_to_blob(embedding: &[f32]) -> &[u8] {
    bytemuck::cast_slice(embedding)
}
```

This is a minor performance improvement relevant when inserting many chunks.

### 11. `search` returns `Vec<Chunk>` without distance scores

The search query orders by `v.distance` but does not return the distance value. For RAG quality, distance/similarity scores are useful for setting a relevance threshold (e.g., discard chunks with distance > 0.8). Consider returning a `Vec<(Chunk, f32)>` or a `ScoredChunk` struct.

### 12. No index on `chunks_vec.chunk_id` join column

The `INNER JOIN chunks c ON c.id = v.chunk_id` relies on `chunks.id` being the PRIMARY KEY (which it is), but large-scale queries could benefit from ensuring the join is efficient. This is fine for the current scale.

### 13. `default_db_path` uses `unwrap_or(PathBuf::from("."))` 

Line 125-128. If `dirs::data_local_dir()` returns `None` (unlikely but possible on exotic systems), the database is created in the current working directory (`.`). For a Tauri app, the cwd is unpredictable. Consider failing explicitly instead.

### 14. Missing `Send` / `Sync` consideration comment

`VectorStore` wraps `rusqlite::Connection` which is `Send` but not `Sync`. The `Mutex<VectorStore>` in `RagState` handles this correctly, but a doc comment on `VectorStore` noting this constraint would help future maintainers.

### 15. Tests only cover blob serialization, not database operations

The test module has a single test for `embedding_to_blob`. There are no tests for `open`, `insert_chunk`, `search`, `init_vector_table`, `delete_meeting_chunks`, or model change detection. Using `tempfile` (already in dev-dependencies) with an in-memory or temp-dir database would allow testing the full lifecycle. These tests would have caught issue #3 (search query correctness).

---

## Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 10 | Suggestion | Use `bytemuck` for zero-copy blob serialization |

---

| 11 | Suggestion | Return distance scores from search |

---

| 12 | Suggestion | Join efficiency note (fine at current scale) |

---

| 13 | Suggestion | `default_db_path` fallback to `.` is fragile |

---

| 14 | Suggestion | Add `Send`/`!Sync` doc comment |

---

| 15 | Suggestion | No integration tests for database operations |

**Priority recommendation**: Fix issues 1, 2, and 3 first. Issue 3 (search correctness) directly affects the RAG pipeline's ability to find relevant context for chat responses. Issues 1 and 2 risk silent data corruption on unexpected shutdowns.

---

### SUGGESTIONS (Nice to Have)

**7. Missing `Default` derive on status enums**

`MediaStatus`, `TranscriptionStatus`, and `ChatStatus` all have logical defaults (`Present`, `Pending`, `NotIndexed` respectively), as evidenced by the fallback values in their `from_str` methods and the DB schema defaults. Adding `#[derive(Default)]` with `#[default]` on the appropriate variant would make this explicit and enable `..Default::default()` patterns.

**8. `MeetingUpdate` is extremely thin (lines 121-124)**

```rust
pub struct MeetingUpdate {
    pub title: Option<String>,
}
```

This struct wraps a single `Option<String>`. It exists but is never used anywhere in the codebase (the command at line 18 of `commands.rs` takes `title: String` directly, not `MeetingUpdate`). This is dead code. Either remove it or integrate it into the update command.

**9. `TrackInfo::index` is `usize` (line 51)**

`usize` is platform-dependent (32 or 64 bits). When serialized via serde on a 64-bit system and deserialized on a 32-bit system (or vice versa), large values could overflow. While track indices will never be large enough for this to matter in practice, `u32` would be more portable and explicit about the size.

**10. Missing `PartialEq`/`Eq` on struct types**

`Meeting`, `MeetingSummary`, `MeetingDetail`, `TrackInfo`, `RecordingInfo`, and `PreparedMeeting` all lack `PartialEq`. This makes them harder to test (assertions require field-by-field comparison). The test at line 231 already works around this by asserting individual fields.

**11. `chrono::DateTime<Utc>` serde format**

With the `serde` feature enabled in `Cargo.toml` (line 17: `chrono = { version = "0.4", features = ["serde"] }`), `DateTime<Utc>` serializes to RFC 3339 format by default (e.g., `"2026-03-14T10:30:00Z"`). This is correct and consistent with how `db.rs` stores it via `to_rfc3339()` (line 78) and parses it via `parse_from_rfc3339` (lines 111, 181). No issue here -- the formats are aligned. However, note that chrono's default serde uses a slightly more permissive parser than `parse_from_rfc3339` -- it also accepts formats like `2026-03-14T10:30:00.123456789+00:00` with nanosecond precision and explicit timezone offsets. This means a value serialized by serde and then parsed by `parse_from_rfc3339` in `db.rs` will round-trip correctly, but the reverse is not guaranteed if the DB contains non-standard timestamps. This is low risk but worth being aware of.

**12. No `#[serde(deny_unknown_fields)]` on any struct**

All structs will silently accept and ignore extra JSON fields during deserialization. For Tauri commands receiving data from the frontend, this means typos in field names go undetected. Adding `#[serde(deny_unknown_fields)]` on command input types like `MeetingUpdate` and `DeleteMode` would catch these.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 7 | Suggestion | Missing `Default` derive on status enums |

---

| 8 | Suggestion | `MeetingUpdate` is dead code |

---

| 9 | Suggestion | `TrackInfo::index` should be `u32` not `usize` |

---

| 10 | Suggestion | Missing `PartialEq`/`Eq` on structs hinders testing |

---

| 11 | Suggestion | chrono serde format is correct but note subtle parser differences |

---

| 12 | Suggestion | No `deny_unknown_fields` on input types |

Relevant files examined:
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/types.rs` (the reviewed file)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/db.rs` (primary consumer of `from_str`/`as_str` and `PathBuf` serialization)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/api.rs` (struct construction and conversion)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/commands.rs` (Tauri command boundary where serde matters)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/Cargo.toml` (chrono serde feature confirmation)

---

## SUGGESTIONS (nice to have)

### S1. No `Drop` implementation for `ScreenCapture`

If `request_screen` is called a second time on the same `ScreenCapture`, the old `OwnedFd` is silently replaced and dropped. This would invalidate any `PipeWireSource` from a previous call while a pipeline might still be using it. Consider either:
- Making `request_screen` consume `self` (take ownership), returning `(ScreenCapture, PipeWireSource)`
- Or asserting that `self.fd.is_none()` at the top of `request_screen`

### S2. The `use std::os::fd::AsRawFd` on line 62 should be at the top of the file

Placing imports inside function bodies is valid Rust but unconventional. Move it to the file-level imports.

### S3. Error type -- consider a proper error enum

All errors are `String`-typed via `.map_err(|e| format!(...))`. This is fine for an MVP but loses error structure. A dedicated `CaptureError` enum would allow callers to match on specific failure modes (e.g., user cancelled the portal dialog vs. D-Bus failure).

### S4. `ScreenCapture` could implement `Default`

Since `ScreenCapture::new()` just returns `Self { node_id: None, fd: None }`, it is equivalent to a `Default` implementation. Consider `#[derive(Default)]` and removing the manual `new()`, or keeping `new()` and adding `impl Default`.

---

## Summary of Required Fixes

| # | Severity | Line(s) | Issue |
|---|----------|---------|-------|

---

| S1 | Suggestion | 24 | Double-call to `request_screen` silently invalidates prior fd |

---

| S2 | Suggestion | 62 | Inner `use` statement should be at file top |

---

| S3 | Suggestion | all | String errors lose structure |

---

| S4 | Suggestion | 19-21 | `ScreenCapture` could derive `Default` |


---

## SUGGESTION Issues


---

### 9. SUGGESTION: `emit_to("main", ...)` uses the window label, not `listen()`'s global scope

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/commands.rs`, lines 181, 184, 190

The Rust side uses `app.emit_to("main", "chat-stream-chunk", &text)`. In Tauri v2, `emit_to` with a window label sends an event specifically to that window's webview. On the frontend, `listen()` from `@tauri-apps/api/event` listens globally. This actually works correctly because `listen()` receives both global and window-targeted events. No action needed, but worth noting for maintainability -- if the window label ever changes from "main", the events would stop arriving.


---

### 10. SUGGESTION: `withVideo` parameter name -- Tauri v2 auto-converts camelCase

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/api.ts`, line 184

The frontend sends `{ withVideo }` (camelCase). The Rust command expects `with_video: bool` (snake_case). Tauri v2 automatically converts camelCase arguments to snake_case, so this works. Same applies to `meetingId` -> `meeting_id` and `saveMode` -> `save_mode`. No bug here, but this implicit conversion is fragile -- if someone uses an underscore in the JS name (e.g., `with_video`), it would also work. Consider using `#[tauri::command(rename_all = "camelCase")]` explicitly on the Rust side to make the contract explicit and documented.


---

### 11. SUGGESTION: `lib.rs` startup uses `expect()` -- process-terminating panics

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/lib.rs`, lines 53, 55-58, 60-61, 109

Four `expect()` calls during startup. While these are appropriate for truly fatal initialization errors (GStreamer not available, database cannot open), they produce ugly panic messages with no user-facing context. Consider using a proper error dialog or logging before exit.


---

### 12. SUGGESTION: `SettingsPage` loads all data without cancellation

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx`, lines 44-65

The `loadAll` function in `useEffect` has no cleanup or cancellation. If the user navigates away before all 4 `Promise.all` calls complete, state updates will fire on an unmounted component. This is a minor React warning issue.


---

### 13. SUGGESTION: `TranscriptView` polling continues if the component re-mounts

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/TranscriptView.tsx`, lines 42-63

After calling `handleTranscribe`, a polling interval is set up. If the user navigates away and back, the old interval is cleaned up (line 38-40), but `transcribing` state resets to `false`, so the UI shows "Transcrever agora" even if a transcription is in progress on the backend. The `transcriptionStatus` prop would still say "processing" from the parent, so this is mitigated -- but only if the parent re-fetches the meeting data.


---

### 14. SUGGESTION: Duplicate `formatSize` function

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx`, lines 85-89
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 32-37
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingCard.tsx`, lines 9-14
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx`, lines 11-17

The `formatSize` function is duplicated 4 times with slight variations. Consider extracting it to `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/format.ts` alongside the existing `formatDuration`.

---

## Summary

| Severity | Count | Key Items |
|----------|-------|-----------|

---

| SUGGESTION | 6 | emit_to label fragility, explicit rename_all, startup panics, cancellation, code duplication |


---

## SUGGESTIONS (nice to have)

### 8. The `scale` animation creates a layout shift illusion

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, lines 146-148

```typescript
initial={{ opacity: 0, scale: 0.97, y: 8 }}
animate={{ opacity: 1, scale: 1, y: 0 }}
exit={{ opacity: 0, scale: 0.97, y: -8 }}
```

Using `scale` on a full-screen element means the element briefly appears 3% smaller (revealing the black background at the edges), then snaps to full size. This is a common aesthetic choice, but because the element is `w-screen h-screen`, the 3% gap is visible as a black border around all four edges during animation. Consider whether `scale` is adding visual value here or just flickering.

### 9. Custom cubic-bezier could be a named constant

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, line 149

```typescript
transition={{ duration: 0.2, ease: [0.16, 1, 0.3, 1] }}
```

This is an "ease-out-expo"-style curve. For readability and reuse, extract it:

```typescript
const EASE_OUT_EXPO = [0.16, 1, 0.3, 1] as const;
```

### 10. `MeetingCtx` type is duplicated

The `MeetingCtx` interface in App.tsx duplicates the structure defined inline in `MeetingPage.tsx`'s `onChat` and `onExport` prop types. If one changes, they can drift apart. Consider exporting a shared type from a `types.ts` file.

### 11. `galleryKey` ref pattern works but is unconventional

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, line 25

```typescript
const galleryKey = useRef(0);
```

Incrementing a ref and using it as a React `key` forces Gallery to remount. This works because `renderView()` is called on every render (so it reads the current ref value), but it is fragile -- if `renderView()` were memoized, the key would be stale. A `useState<number>` would be more idiomatic and resistant to future refactors.

---

## Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 8 | Suggestion | `scale: 0.97` shows black edges during transition | 146-148 |

---

| 9 | Suggestion | Extract cubic-bezier constant | 149 |

---

| 10 | Suggestion | Duplicated `MeetingCtx` type | 14-19 |

---

| 11 | Suggestion | `galleryKey` ref pattern is fragile vs `useState` | 25 |

---

**Issue 7 is very likely the root cause of any scrolling/layout problems** in Gallery, MeetingPage, ChatPanel, ExportDialog, and SettingsPage. All of those components return fragments and depend on their parent being a flex column container. The `motion.div` at line 143 does not provide this. Fixing line 144's className from `"w-screen h-screen"` to `"w-full h-full flex flex-col"` would resolve layout for all child views simultaneously.

---

### SUGGESTIONS

**7. `ApiProvider` fields are never read outside the struct -- consider `#[allow(dead_code)]` or adding getters**

The struct fields `base_url`, `api_key`, and `model` are all private with no accessor methods. This is fine for encapsulation, but if Rust emits dead_code warnings during compilation (it won't because they are read within `transcribe`), it could be misleading. This is a non-issue currently, just a note.

**8. Consider adding a `language` parameter to the form**

The local provider uses `params.set_language(None)` for auto-detect. The API provider does not send a `language` field, which means the API will also auto-detect. This is consistent behavior, which is good. However, if the application ever wants to support user-specified language, the `TranscriptionProvider` trait would need to be extended. Consider adding an optional `language` field to `ApiProvider` or to the trait method signature for future extensibility.

**9. The `ApiResponse` struct could benefit from an `#[allow(dead_code)]` or using `duration`**

The OpenAI verbose_json response also includes a `duration` field (the audio length in seconds). This is not captured in `ApiResponse`, which means it is silently discarded. If this information is useful for the UI (e.g., showing recording duration), it could be captured.

**10. No unit tests for `ApiProvider`**

There are no tests anywhere in the project for this file (confirmed by the empty `tests/` directory glob and no `#[cfg(test)]` module in `api.rs`). At minimum, the response deserialization logic should be tested with sample JSON payloads to catch regressions. You could add tests for the `ApiResponse` -> `TranscriptResult` conversion without hitting an actual API.

**11. Consider using `reqwest::blocking::Client::builder()` for User-Agent**

The OpenAI API and many compatible servers log User-Agent. `Client::new()` sends the default reqwest User-Agent. Setting a custom one like `"hlusra/0.1.0"` would help with API-side debugging and rate-limit attribution.

---

## Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 7 | SUGGESTION | Fields never accessed outside struct (non-issue currently) |

---

| 8 | SUGGESTION | No `language` parameter support for future extensibility |

---

| 9 | SUGGESTION | `duration` field from API response is discarded |

---

| 10 | SUGGESTION | No unit tests for deserialization or conversion logic |

---

| 11 | SUGGESTION | No custom User-Agent set |

---

## Files Reviewed

- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/api.rs` -- primary review target
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/types.rs` -- internal types consumed by api.rs
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/provider.rs` -- trait definition
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/orchestrator.rs` -- caller context (temp WAV creation)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/commands.rs` -- ApiProvider instantiation context
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/Cargo.toml` -- reqwest feature flags verified (`blocking`, `multipart`, `json` all present)

---

## Suggestions (Nice to Have)

### 9. No index on `created_at` column

`list_meetings()` does `ORDER BY created_at DESC`. Without an index, this is a full table scan + sort. For a personal meeting recorder this is unlikely to matter, but adding `CREATE INDEX IF NOT EXISTS idx_meetings_created_at ON meetings(created_at DESC)` in migration v1 would be trivially cheap and future-proof.

### 10. Column indices are positional -- fragile and hard to maintain (lines 109-119, 176-192)

Both `row_to_meeting` and the `list_meetings` closure use hardcoded column indices (`row.get(0)?`, `row.get(1)?`, etc.). If the SELECT column order is ever changed or a column is added in the middle, these will silently read the wrong data with no compile-time safety. Consider using `row.get::<_, String>("column_name")?` (the named-column variant of `Row::get`) for better maintainability, or at minimum add a comment documenting the expected column order at each call site.

### 11. `list_meetings` and `row_to_meeting` have duplicated parsing logic

The date parsing, boolean conversion, and status enum parsing logic is repeated between `list_meetings` (lines 111-119) and `row_to_meeting` (lines 181-191). If any parsing logic changes, both must be updated in lockstep. Consider extracting shared parsing helpers.

### 12. Custom `from_str` methods shadow the standard library trait

The `MediaStatus::from_str`, `TranscriptionStatus::from_str`, and `ChatStatus::from_str` methods in types.rs are inherent methods that shadow `std::str::FromStr`. This prevents implementing the standard trait, which would allow using `.parse::<MediaStatus>()` idiomatically. Not a db.rs issue per se, but it affects how this module interacts with the types.

### 13. `open_in_memory` is `pub` but only used in `#[cfg(test)]` contexts

In `api.rs` line 49, `open_in_memory` is called inside a `#[cfg(test)]` block. If it is never called in production code, consider gating it with `#[cfg(test)]` to prevent accidental misuse.

### 14. No test for migration idempotency


---

| 9 | Suggestion | No index on `created_at` | schema |

---

| 10 | Suggestion | Positional column indices are fragile | 109-119, 176-192 |

---

| 11 | Suggestion | Duplicated parsing logic between `list_meetings` and `row_to_meeting` | 107-123, 175-193 |

---

| 12 | Suggestion | `from_str` shadows `std::str::FromStr` trait | types.rs |

---

| 13 | Suggestion | `open_in_memory` should be `#[cfg(test)]` | 45-49 |

---

| 14 | Suggestion | Missing test for migration idempotency | tests |

---

| 15 | Suggestion | `test_list_meetings_ordered` does not verify ordering | 235-241 |

---

## Files Reviewed

- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/db.rs` (primary review target)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/types.rs` (for type definitions and `as_str`/`from_str` implementations)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/Cargo.toml` (for rusqlite version and features)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/api.rs` (for usage context of `LibraryDb`)

---

## Suggestions (Nice to Have)

### 11. Tests assert wrong values

Line 94: `assert_eq!(EncoderBackend::Vaapi.gst_element_name(&VideoCodec::H265), "vaapih265enc");`

The tests encode the same wrong element names. Once the names are fixed, all four test assertions must be updated.

### 12. Missing `Hash` on `RecordingState`

For consistency with `EncoderBackend`, consider adding `Hash` to `RecordingState` as well.

### 13. Consider a `Codec` + `Backend` availability check

Rather than blindly returning element name strings for unsupported combinations, consider adding:

```rust
impl EncoderBackend {
    pub fn supports_codec(&self, codec: VideoCodec) -> bool { ... }
}
```

### 14. `AudioConfig` is very thin

`AudioConfig` only has `bitrate`. Consider whether the codec (Opus, AAC, etc.) and channel count should also live here, or if this will be expanded later.

---

## Summary Table

| # | Severity | Issue | Lines |
|---|----------|-------|-------|

---

| 10 | Suggestion | Tests assert wrong (stale) values | 94-97 |

---

| 11 | Suggestion | Add `supports_codec()` method | -- |

---

| 12 | Suggestion | `AudioConfig` is very thin | 46-48 |

---

## What Was Done Well

- The type structure is clean: separate enums for codec and backend, separate config structs for video and audio.
- `serde(rename_all = "snake_case")` is correctly applied for consistent serialization.
- The `Copy` derive is correctly applied to small enums (`EncoderBackend`, `VideoCodec`, `RecordingState`).
- Having a test module is good practice, even though the asserted values are currently wrong.
- The exhaustive match in `gst_element_name` (no wildcard arm) ensures the compiler will flag missing variants if new codecs/backends are added.

---

## Recommended Action


---

### Suggestions (nice to have)

#### 8. No progress reporting for long transcodes

`Command::new("ffmpeg")...output()` blocks the calling thread until ffmpeg finishes. For large recordings, H.265-to-H.264 transcoding can take many minutes. The Tauri command (`commands.rs` line 31) runs synchronously, which will block the Tauri async runtime thread. Consider:
- Running the ffmpeg process asynchronously with `tokio::process::Command`.
- Parsing ffmpeg's progress output (`-progress pipe:1`) to report progress to the frontend.
- At minimum, marking the Tauri command as `async` and spawning the process on a blocking thread.

#### 9. No validation that ffmpeg is installed

If `ffmpeg` is not on `$PATH`, `Command::new("ffmpeg").output()` returns an `io::Error` with kind `NotFound`. This is captured by the `From<std::io::Error>` impl on `ExportError`, but the resulting error message ("No such file or directory") is cryptic for users. A pre-flight check or a more descriptive error variant would improve the user experience.

#### 10. Tests only cover path resolution, not ffmpeg argument construction

The two tests in `video.rs` (lines 72-86) test `resolve_output_path`, which is actually a function from `types.rs` and is already tested there. There are no tests for the ffmpeg argument construction logic itself. Consider a unit test that builds the `Command` and inspects its arguments (using `Command::get_args()`) without actually running ffmpeg.

#### 11. Duplicate tests

The tests `test_resolve_output_save` and `test_resolve_output_save_as` in `video.rs` lines 72-86 are exact duplicates of the tests in `types.rs` lines 150-164 (with only the filename string differing). These add no additional coverage and should be removed or replaced with video-specific tests.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 8 | Suggestion | No progress reporting; synchronous blocking on long transcodes |

---

| 9 | Suggestion | No user-friendly error when ffmpeg is not installed |

---

| 10 | Suggestion | No tests for ffmpeg argument construction logic |

---

| 11 | Suggestion | Duplicate tests from types.rs add no coverage |

---

### Files referenced

- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/video.rs` -- the file under review
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/types.rs` -- `VideoFormat` enum and unused `codec_name()` method
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/mod.rs` -- `ExportError` definition
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/commands.rs` -- Tauri command wrappers
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/types.rs` -- `VideoCodec` enum showing H264/H265/Av1 support
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/settings/defaults.rs` -- default audio codec is Opus, default video codec is H.265

---

### SUGGESTIONS (Nice to Have)

**8. `formatSize` duplicates utility logic (line 85-89)**

The `formatSize` function is defined inline in the component. This is a general utility that could live in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/format.ts` alongside `formatDuration` and `formatTimer`. It also lacks a GB tier, which would matter for long video recordings.

**9. Accessibility gaps**

- The record button (line 131-140) has no `aria-label`. Screen readers would read nothing meaningful for an SVG-only button. Add `aria-label="Iniciar gravacao"`.
- The stop button (line 110-116) has text content, which is fine, but lacks `aria-live` or `role="status"` on the timer/status region so screen readers are not informed of elapsed time updates.
- The error messages (lines 118, 167) should have `role="alert"` so assistive technology announces them.
- The "Gravando" indicator (line 100) and elapsed time (line 103-105) should be wrapped in an `aria-live="polite"` region.
- The checkbox toggle (line 152-165) has `sr-only` on the input, which is correct, but the visible label "Gravar tela" is not programmatically associated via `htmlFor`/`id`. The wrapping `<label>` element handles this implicitly, so this is acceptable but could be more explicit.

**10. No `key` prop concern, but conditional rendering logic is fragile (line 92)**

```tsx
if (isRecordingView || recording) {
```

This condition conflates two different states. `isRecordingView` is a prop; `recording` is internal state. If `isRecordingView` is `false` but `recording` is `true` (home-view instance after clicking record but before `onRecordingStart` navigates away), the home-view instance briefly renders the recording UI instead of the home UI. This is a visual flash that happens during the transition. It is minor because the parent immediately switches views, but it could cause a layout jump within the `AnimatePresence` exit animation.

**11. `disabled` state does not cover the home view's record button fully**

The circular record button and the text button both check `disabled={starting}`, but neither checks `recording`. If state somehow gets desynced (e.g., due to the double-start in issue #4), a user could click the button while already recording. The backend may reject it, but the UI should prevent it.

**12. No loading spinner or visual feedback during `starting` state on the circular button**

When `starting` is true, the circular button gets `disabled:opacity-40`, which is subtle. The text button below it shows "Iniciando...", but the primary interaction target (the large circular button) gives no textual or animated feedback. Users may not notice it is loading.

---

### Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 8 | Suggestion | formatSize should live in shared format.ts |

---

| 9 | Suggestion | Multiple accessibility gaps (aria-label, role="alert", aria-live) |

---

| 10 | Suggestion | Fragile conditional render logic conflating prop and state |

---

| 11 | Suggestion | disabled state does not guard against recording===true |

---

| 12 | Suggestion | No visual loading indicator on the circular button |

Issues #1 and #4 together form the most urgent problem: the current architecture causes `startRecording` to be invoked twice per user action (once from the home-view click, once from the recording-view auto-start). This needs an architectural decision about which instance owns the recording lifecycle.

---

## SUGGESTIONS (Nice to Have)

### 8. `VideoConfig` and `AudioConfig` are hardcoded to defaults, ignoring user settings

**File:** `commands.rs` lines 40-41

```rust
let video_config = VideoConfig::default();
let audio_config = AudioConfig::default();
```

The project has a settings module (`settings::commands`), but the recording commands ignore user-configured encoding preferences entirely. The user cannot control codec, backend, bitrate, fps, or resolution.

**Recommendation:** Load the user's settings and derive `VideoConfig`/`AudioConfig` from them, falling back to defaults if settings are unavailable.

### 9. Error messages mix Portuguese and English

**File:** `commands.rs` throughout

Some errors are in Portuguese ("Falha ao preparar reuniao", "Nenhuma gravacao ativa") while others are in English ("Recorder lock poisoned", "No meeting ID"). This inconsistency may confuse users or developers.

**Recommendation:** Pick one language for user-facing errors (Portuguese makes sense given the user locale) and another for developer/log-level errors (English), and apply the convention consistently.

### 10. `probe_encoders` calls `gstreamer::init()` redundantly

**File:** `commands.rs` line 143

GStreamer is already initialized in `lib.rs` line 53 (`gstreamer::init().expect(...)`) before the Tauri app runs. The `probe_encoders` command calls `gstreamer::init()` again. While `gst::init()` is idempotent and safe to call multiple times, it is unnecessary work and slightly misleading -- it suggests the function might be called before app initialization.

**Recommendation:** Remove the redundant `gstreamer::init()` call, or add a comment explaining it is intentionally defensive.

### 11. `RecorderState` does not implement `Default`

**File:** `commands.rs` lines 9-23

`RecorderState::new()` is a straightforward default constructor. Implementing `Default` would be more idiomatic and allow use with `..Default::default()` patterns.

### 12. `pipeline` is consumed by `take()` but `duration_secs()` and `file_size()` are called after

**File:** `commands.rs` lines 85, 92, 101-103

After `pipeline_lock.take()` on line 85, the pipeline is moved out of the `Option`. This is correct -- the pipeline is consumed. But it means the pipeline is dropped at the end of `stop_recording`. Since `RecordingPipeline` does not implement `Drop` to call `set_state(Null)`, this is actually fine because `stop()` already sets it to `Null`. However, if `stop()` returns an error (line 88-91), the pipeline is dropped without being set to Null, which could leak GStreamer resources. The error path does `set_state(Null)` inside `pipeline.stop()` for the timeout case and the error-message case, so this is currently handled, but fragile.

---

## Summary Table

| # | Severity | Issue |
|---|----------|-------|

---

| 8 | Suggestion | `VideoConfig`/`AudioConfig` hardcoded to defaults, ignoring user settings |

---

| 9 | Suggestion | Error messages inconsistently mix Portuguese and English |

---

| 10 | Suggestion | Redundant `gstreamer::init()` in `probe_encoders` |

---

| 11 | Suggestion | `RecorderState` does not implement `Default` |

---

| 12 | Suggestion | Pipeline drop safety on error paths is correct but fragile |


---

