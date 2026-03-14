# Hlusra — Consolidated Code Review Report

> 44 code reviewers executed across 30+ source files.
> Organized by priority. Each issue includes location, explanation, and recommended fix.

---

## CRITICAL (must fix — broken functionality, crashes, data loss, security)

### C1. `motion.div` missing `flex flex-col` — root cause of ALL layout bugs
**File:** `src/App.tsx:144`
**Why:** Gallery, MeetingPage, ChatPanel, ExportDialog, SettingsPage all return fragments with `shrink-0` headers, `flex-1` content, and `shrink-0` footers. These CSS properties only work inside a flex column container. The `motion.div` wrapper has `className="w-screen h-screen"` with no `flex flex-col`, so all child layouts fall back to block layout. Headers don't stick, content doesn't scroll, footers don't pin.
**Fix:** Change to `className="w-full h-full flex flex-col"`.

### C2. FFmpeg amerge filter syntax wrong — audio export will fail
**File:** `src-tauri/src/export/audio.rs:70`
**Why:** `[0:a]amerge=inputs=2,...` is invalid. `amerge` needs separate labeled input pads per stream: `[0:a:0][0:a:1]amerge=inputs=2,...`. Current syntax either errors out or merges only one stream.
**Fix:** Build input pads dynamically: `let inputs = (0..count).map(|i| format!("[0:a:{}]", i)).collect()`.

### C3. RecordingPipeline has no `Drop` — GStreamer resource leak
**File:** `src-tauri/src/recorder/pipeline.rs`
**Why:** If the pipeline is dropped without calling `stop()` (error path, panic, app close), GStreamer resources (PipeWire connections, file handles, encoders) are never released. The output MKV file may be corrupt.
**Fix:** `impl Drop for RecordingPipeline { fn drop(&mut self) { let _ = self.pipeline.set_state(gst::State::Null); } }`

### C4. GStreamer VAAPI element names deprecated
**File:** `src-tauri/src/recorder/types.rs:72-74`
**Why:** `vaapih264enc`, `vaapih265enc`, `vaapiav1enc` belong to the deprecated `gstreamer-vaapi` plugin removed in GStreamer 1.24+. On GStreamer 1.26 (which gstreamer-rs 0.25 targets), these elements don't exist. VAAPI hardware encoding silently falls back to software.
**Fix:** Update to `va` plugin names: `vah264enc`, `vah265enc`, `vaav1enc`.

### C5. Vulkan encoder element names are fictional
**File:** `src-tauri/src/recorder/types.rs:78-80`
**Why:** `vulkanh264enc`, `vulkanh265enc`, `vulkanav1enc` do not exist in any GStreamer release. The Vulkan plugin provides no video encoders. Dead code that gives users a false option.
**Fix:** Either remove the `Vulkan` backend variant entirely or gate it behind a clear "experimental/future" flag. Return `None`/error for unsupported combos.

### C6. `nvav1enc` does not exist
**File:** `src-tauri/src/recorder/types.rs:77`
**Why:** GStreamer has no NVIDIA AV1 encoder element. This name was fabricated. Probe will never find it, but users see it as an option in settings.
**Fix:** Remove `(Cuda, Av1)` combo or mark as unsupported.

### C7. `svtav1enc` bitrate property is `target-bitrate`, not `bitrate` — runtime panic
**File:** `src-tauri/src/recorder/encode.rs:55`
**Why:** `set_property("bitrate", ...)` will panic because `svtav1enc` has no property named `bitrate`. The correct property is `target-bitrate`.
**Fix:** Dispatch on element name: if `svtav1enc`, use `target-bitrate`.

### C8. Settings completely ignored by recorder
**File:** `src-tauri/src/recorder/commands.rs:40-41`
**Why:** `start_recording` hardcodes `VideoConfig::default()` and `AudioConfig::default()`. The user's Settings (codec, backend, bitrate, fps, resolution) are never read. The entire Settings > Video and Settings > Audio panels are non-functional.
**Fix:** Load settings via `load_settings()` at the start of `start_recording` and construct configs from user values.

### C9. Track metadata hardcoded — wrong for audio-only and video recordings
**File:** `src-tauri/src/recorder/commands.rs:104-107`
**Why:** Always reports 2 tracks (mic + system, both opus) regardless of actual pipeline. Audio-only has 1 track. Video has 3 (video + mic + system with wrong indices). This corrupts metadata in the DB and breaks export.
**Fix:** Build tracks dynamically based on `pipeline.has_video()` and actual track count.

### C10. MKV/H.265 won't play in Chromium WebView
**File:** `src/components/MeetingPage.tsx:85-108`
**Why:** WebKitGTK (Tauri's webview on Linux) does not support MKV container or H.265 codec in HTML5 `<video>`/`<audio>`. The `readFile` + Blob approach also loads the entire file into JS memory (OOM risk for large recordings).
**Fix:** Use `convertFileSrc()` for streaming, or transcode to web-compatible format for preview, or at minimum extract audio to Opus/OGG for audio-only playback.

### C11. Chat bubble `max-w-[...]` template literal — Tailwind won't compile
**File:** `src/components/ChatPanel.tsx:221`
**Why:** `` max-w-[${msg.role === "user" ? "75" : "80"}%] `` assembles class names via interpolation. Tailwind v4 does static analysis and cannot detect dynamically constructed classes. Neither `max-w-[75%]` nor `max-w-[80%]` will be in the CSS output. Bubbles stretch to 100% width.
**Fix:** Use complete literal strings: `msg.role === "user" ? "max-w-[75%]" : "max-w-[80%]"`.

### C12. Recording view dead end — no back button if start fails
**File:** `src/components/RecordButton.tsx:33-37, 92`
**Why:** When `isRecordingView` is true and `handleStart()` fails, the recording UI renders (showing "Gravando" + "Parar") but no recording is active. There's no back button. The user is trapped.
**Fix:** Add a back/cancel button in the recording view error state, or navigate back on start failure.

### C13. Double `startRecording` call — home + recording view both start
**File:** `src/components/RecordButton.tsx:33-37` + `src/App.tsx:59-62`
**Why:** User clicks record in home view → `handleStart()` fires → `onRecordingStart` navigates to recording view → new RecordButton mounts with `isRecordingView=true` → auto-start useEffect fires `handleStart()` again. Two `start_recording` IPC calls race.
**Fix:** Either remove auto-start from recording view (it already started in home) or don't start in home view (only navigate, let recording view handle start).

### C14. `stop_recording` is synchronous — freezes UI up to 5 seconds
**File:** `src-tauri/src/recorder/commands.rs:78`
**Why:** `stop_recording` is `pub fn` (not async). `pipeline.stop()` blocks waiting for EOS for up to 5 seconds. Tauri runs sync commands on the main thread, freezing the entire UI.
**Fix:** Make it `async` with `spawn_blocking`.

### C15. Export commands are synchronous — freeze UI during FFmpeg
**File:** `src-tauri/src/export/commands.rs:13,31,51`
**Why:** `export_audio`, `export_video`, `export_transcript` are all `pub fn`. FFmpeg transcode can take minutes. UI completely frozen.
**Fix:** Make async with `spawn_blocking`.

### C16. Opus in MP4 will fail or produce unplayable file
**File:** `src-tauri/src/export/video.rs:49-50`
**Why:** Recording uses Opus audio. Copying Opus into MP4 without `-strict experimental` fails on most FFmpeg builds. Even when it works, most players can't play Opus-in-MP4.
**Fix:** For MP4 output, transcode audio to AAC: `-codec:a aac -b:a 128k`.

### C17. Video export assumes source is always H.265
**File:** `src-tauri/src/export/video.rs:36-47`
**Why:** Stream-copy decision hardcodes "if target is H.265, copy". But the user can record in H.264 or AV1. Exporting H.264-as-H.265 produces a broken file.
**Fix:** Probe source codec with ffprobe before deciding stream-copy vs transcode.

### C18. Settings `#[serde(default)]` missing — adding any field breaks existing configs
**File:** `src-tauri/src/settings/config.rs`
**Why:** No struct has `#[serde(default)]`. When a new field is added to `AppSettings` in a future update, existing `config.toml` files will fail to deserialize, crashing the app on startup.
**Fix:** Add `#[serde(default)]` to all 6 settings structs.

### C19. `SettingsPage` `Number("")` produces NaN — crashes backend
**File:** `src/components/SettingsPage.tsx` (multiple lines)
**Why:** Numeric inputs use `Number(e.target.value)`. Empty input produces `NaN`, serialized to null/undefined, crashing the Rust `u32` deserializer.
**Fix:** Guard with `Number(e.target.value) || existingValue`.

### C20. SSE chat stream: buffer residual lost on stream end
**File:** `src-tauri/src/rag/chat.rs:141-198`
**Why:** When the HTTP body stream ends, any data remaining in the string buffer that doesn't end with `\n` is silently discarded. The last token of an LLM response can be swallowed.
**Fix:** Process remaining buffer after the while loop exits.

### C21. SSE chat stream: UTF-8 corruption on multi-byte char split
**File:** `src-tauri/src/rag/chat.rs:146`
**Why:** `String::from_utf8_lossy` on raw TCP chunks will replace partial multi-byte characters (split across chunks) with replacement characters, corrupting text.
**Fix:** Use a `Vec<u8>` buffer instead of `String`, only decode complete lines.

### C22. Whisper "large" model URL returns 404
**File:** `src-tauri/src/transcription/types.rs` (all_models) + `models.rs:59`
**Why:** `ggml-large.bin` doesn't exist on Hugging Face. Only `ggml-large-v1.bin`, `ggml-large-v2.bin`, `ggml-large-v3.bin` exist.
**Fix:** Change to `large-v3` producing `ggml-large-v3.bin`.

### C23. `set_active_model` path traversal vulnerability
**File:** `src-tauri/src/transcription/models.rs:107`
**Why:** Uses inline `format!("ggml-{model_name}.bin")` without validating against catalogue. Passing `../../etc/passwd` as model_name checks for arbitrary file existence.
**Fix:** Look up model in catalogue first, use `model.filename()`.

### C24. Delete order FS-before-DB creates inconsistent state
**File:** `src-tauri/src/library/api.rs:172-178`
**Why:** If filesystem delete succeeds but DB delete fails, the meeting row persists pointing to a deleted directory. All subsequent operations on that meeting fail with confusing IO errors.
**Fix:** Reverse order: delete from DB first, then filesystem.

### C25. `init_vector_table` not transactional — data loss on crash
**File:** `src-tauri/src/rag/vector_store.rs:181-207`
**Why:** DROP + DELETE + CREATE + SET_META as independent statements. Crash mid-sequence leaves DB in corrupted state with no chunks AND no virtual table.
**Fix:** Wrap in `unchecked_transaction`.

### C26. Vector search can return 0 results for valid meetings
**File:** `src-tauri/src/rag/vector_store.rs:345-354`
**Why:** sqlite-vec `MATCH` returns global top-k, then JOIN filters by meeting_id. If other meetings have many chunks, all top-k results come from other meetings.
**Fix:** Pre-filter by meeting_id or use total chunk count as k.

---

## IMPORTANT (should fix — significant UX/quality issues)

### I1. No concurrent transcription guard — same meeting can be transcribed twice simultaneously
**File:** `src-tauri/src/transcription/commands.rs:45-74`

### I2. Chat history lost on navigation — no persistence
**File:** `src/components/ChatPanel.tsx:20`

### I3. Settings lost without warning on back navigation
**File:** `src/components/SettingsPage.tsx`

### I4. Play/pause icon never updates to reflect state
**File:** `src/components/MeetingPage.tsx:291-298`

### I5. `stagger` CSS class has no corresponding rule — animations never fire
**File:** `src/styles/app.css` (missing), used in ~20 elements

### I6. Floating widget missing (spec requirement)
**File:** Spec says `widget.rs` — not implemented

### I7. Thumbnail generation missing (spec requirement)
**File:** No `thumbnail.rs`, `finalize_meeting` doesn't generate thumbnails

### I8. sqlite-vec not bundled — fails silently on systems without it
**File:** `src-tauri/src/rag/vector_store.rs:104-121`

### I9. RAG API URLs require full endpoint path but placeholder shows base URL
**File:** `src-tauri/src/rag/embeddings.rs:114`, `chat.rs:111`

### I10. Migrations not wrapped in transactions
**File:** `src-tauri/src/library/db.rs:64`

### I11. `open_in_memory()` skips PRAGMAs (foreign_keys = ON)
**File:** `src-tauri/src/library/db.rs:45-49`

### I12. TOCTOU races in filesystem delete operations
**File:** `src-tauri/src/library/fs.rs:36-52`

### I13. No path containment validation — directory traversal possible
**File:** `src-tauri/src/library/fs.rs:15-18`

### I14. `save_artifact` not atomic — partial writes on crash
**File:** `src-tauri/src/library/fs.rs:25-29`

### I15. `delete_meeting_dir` follows symlinks
**File:** `src-tauri/src/library/fs.rs:35-39`

### I16. `assert_eq!` in `insert_chunks` panics in production
**File:** `src-tauri/src/rag/vector_store.rs:274`

### I17. API keys stored in plain text with default permissions
**File:** `src-tauri/src/settings/config.rs`

### I18. No request timeout on reqwest clients — can hang forever
**File:** `src-tauri/src/rag/embeddings.rs:75`, `chat.rs:88`, `transcription/api.rs:74`

### I19. `duration_secs()` called after `stop()` includes EOS wait time
**File:** `src-tauri/src/recorder/commands.rs:101`

### I20. Orphaned directories if finalize_meeting never called
**File:** `src-tauri/src/library/api.rs:58-75`

### I21. `.part` file never cleaned up on download failure
**File:** `src-tauri/src/transcription/models.rs`

### I22. `from_str` shadows `FromStr` trait, silently swallows bad data
**File:** `src-tauri/src/library/types.rs:134,156,180`

### I23. 3 Mutex locks in start_recording — inconsistent state on partial failure
**File:** `src-tauri/src/recorder/commands.rs:72-73`

### I24. `get_meeting` swallows all DB errors as NotFound
**File:** `src-tauri/src/library/api.rs:113-116`

### I25. Portuguese without accents throughout the app
**File:** All frontend components

### I26. Error text uses 4 different sizes and 2 colors
**File:** Multiple components

### I27. Primary buttons have 3 paddings and 2 background styles
**File:** Multiple components

### I28. `formatError` case-sensitive — "FFmpeg" not caught by "ffmpeg" check
**File:** `src/lib/format.ts:39`

### I29. `update()` in SettingsPage reads stale closure instead of functional updater
**File:** `src/components/SettingsPage.tsx:67-71`

### I30. Multipart upload missing MIME type
**File:** `src-tauri/src/transcription/api.rs:65-66`

### I31. Dual system messages in RAG prompt — may break on OpenRouter models
**File:** `src-tauri/src/rag/prompt.rs:39-50`

### I32. `emit_to` errors silently swallowed — chat stream could be invisible
**File:** `src-tauri/src/rag/commands.rs:181,184,190`

### I33. `chunk_size = 0` not guarded in chunker
**File:** `src-tauri/src/rag/chunker.rs:43`

### I34. Token estimation breaks for CJK languages
**File:** `src-tauri/src/rag/chunker.rs` (whitespace split)

### I35. Missing `-movflags +faststart` for MP4 export
**File:** `src-tauri/src/export/video.rs`

### I36. No AV1 export format despite recorder supporting AV1
**File:** `src-tauri/src/export/types.rs`

### I37. `get_thumbnail` wrapper missing in api.ts
**File:** `src/lib/api.ts`

### I38. Delete modal no Escape key handler
**File:** `src/components/MeetingPage.tsx:399`

### I39. `retranscribe_meeting` sets Pending then immediately overwrites to Processing
**File:** `src-tauri/src/transcription/commands.rs:118-127`

### I40. `reindex_meeting` deletes chunks before validating config
**File:** `src-tauri/src/rag/commands.rs:113-126`

---

## SUGGESTIONS (nice to have — polish, consistency, best practices)

### S1. Add `videorate` element before capsfilter in video pipeline
### S2. Name GStreamer pipelines for debug (`Pipeline::with_name`)
### S3. Add bus watch during recording to detect errors in real-time
### S4. Use `bearer_auth()` consistently instead of manual header
### S5. Remove redundant `Content-Type` header with `.json()`
### S6. `conn.changes() == 0` more idiomatic than `< 1`
### S7. Extract `formatSize` to shared format.ts (duplicated 4x)
### S8. Narrow tokio features from `["full"]`
### S9. Add keyboard shortcuts for record/stop
### S10. recordings_dir should use Tauri folder picker
### S11. Chat input should be textarea, not input
### S12. Gallery footer count should reflect filtered search
### S13. Add `#[serde(deny_unknown_fields)]` on input types
### S14. Add `PartialEq`/`Eq` derives on struct types for testing
### S15. Remove unused `MeetingUpdate` struct
### S16. Remove duplicate tests from audio.rs/video.rs (already in types.rs)
### S17. Add `Drop` cleanup for ScreenCapture (prevent double request_screen)
### S18. Extract error-response parsing helper in chat.rs/embeddings.rs
### S19. Add download progress reporting via Tauri events
### S20. Add download cancellation mechanism
### S21. Cache WhisperContext across transcription calls
### S22. Use `log`/`tracing` crate instead of `eprintln!`
### S23. Add `aria-label` to icon-only buttons throughout
### S24. Add `role="alert"` to error messages
### S25. Auto-dismiss "Salvo!" indicator after 3 seconds
### S26. Extract save button from SettingsPage (duplicated 5x)
### S27. Add unsaved-changes warning on back navigation
### S28. TypeScript pinned to ~5.7, should bump to latest
### S29. No chunk overlap in RAG chunker (hurts retrieval quality)
### S30. `ArtifactKind` missing `#[serde(rename_all)]` (inconsistent with other enums)
### S31. Consider `bytemuck` for zero-copy embedding blob serialization
### S32. Return distance scores from vector search
### S33. Validate bitrate ranges before passing to GStreamer
### S34. Add explicit CRF for libx264 transcode
### S35. Probe source codec before stream-copy decision in video export
### S36. `settings.transcription.model` vs `activeModelName` disconnected in SettingsPage
### S37. PDF margins too tight (10pt) for printing
### S38. SRT empty segments should be skipped
### S39. No tests for FFmpeg argument construction
### S40. No tests for database operations in vector_store.rs
