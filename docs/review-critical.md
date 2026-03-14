# Hlusra Code Review — CRITICAL Issues

> 161 critical issue sections from 44 code reviewers

---

- **CRITICAL** -- This template literal approach will NOT work with Tailwind. Tailwind requires complete class names at build time for its JIT compiler. The class `max-w-[75%]` must appear as a literal string, not constructed via interpolation. The generated class `max-w-[75%]` or `max-w-[80%]` will NOT be in the CSS output and the max-width constraint will not apply. Both user and assistant messages will have no max-width limit.

#### 8.4 User message text
- **Mockup:** `<p class="text-[13px] text-white/70">` 
- **Implementation:** `<p className="text-[13px] leading-relaxed break-words whitespace-pre-wrap text-white/70">`
- **MINOR** -- Adds `leading-relaxed break-words whitespace-pre-wrap` for better text rendering. Justified.

#### 8.5 Assistant message bubble
- **Mockup:** `<div class="glass-heavy rounded-2xl rounded-bl-md px-4 py-3 max-w-[80%]">`

---

- **Implementation:** Same `max-w-` issue as user bubble (see 8.3 CRITICAL above).

#### 8.6 Assistant message text
- **Mockup:** Uses `<strong>` and `<span>` with brand colors for formatted responses.
- **Implementation:** Plain `text-[13px] text-white/50 leading-relaxed` -- the mockup's rich formatting with `<strong class="text-white/70">` and `<span class="text-brand-400/70">` is not reproduced because the implementation renders plain text from the chat stream.
- **MINOR** -- Expected difference; real chat output is plain text, not pre-formatted HTML.

#### 8.7 Input area
- `class="p-4 border-t border-white/5 shrink-0"` -- **MATCH**
- Input classes: **MATCH**
- Send button:
  - **Mockup:** `class="px-5 py-3 bg-gradient-to-r from-brand-500 to-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-95 glow-sm hover:shadow-lg hover:shadow-brand-500/20"`
  - **Implementation:** adds `border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed`
  - **MINOR** -- Interactive state additions.

#### 8.8 Not-indexed / indexing states
- These states are **not in the mockup** (the mockup only shows the "ready" chat state).
- The implementation adds indexing prompt and spinner views using the same design language.
- **N/A** -- Justified additions for real functionality.

---

### 9. EXPORT VIEW

#### 9.1 Header
- **MATCH** -- identical to mockup.

#### 9.2 Audio/Video/Transcript sections
- Section container: `class="glass-card rounded-2xl p-5 stagger"` -- **MATCH**
- Section icon + title: **MATCH** (same SVG paths, same classes)
- Format buttons:
  - **Mockup:** `class="export-opt glass-input rounded-xl py-2.5 text-[11px] text-white/50 hover:text-white/80 hover:border-brand-500/30 hover:bg-brand-500/5 transition-all active:scale-95 font-medium"`
  - **Implementation:** `className="glass-input rounded-xl py-2.5 text-[11px] font-medium transition-all active:scale-95 cursor-pointer bg-transparent ..."`
  - The implementation conditionally applies selected state classes (`border-brand-500/40 bg-brand-500/10 text-brand-400`) vs unselected (`text-white/50 hover:text-white/80 hover:border-brand-500/30 hover:bg-brand-500/5`).
  - **MINOR** -- `export-opt` class removed (it was only used for JS toggling in the mockup). Adds `cursor-pointer bg-transparent`.

#### 9.3 Save buttons footer
- `class="p-5 border-t border-white/5 shrink-0 flex gap-3"` -- **MATCH**
- Primary "Salvar" button: **MATCH** (implementation adds `border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed`)
- Secondary "Salvar como..." button:
  - **Mockup:** `class="flex-1 py-3 glass-heavy rounded-xl text-[12px] text-white/50 font-medium hover:text-white/80 transition-all active:scale-[0.98]"`
  - **Implementation:** adds `border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed`
  - **MINOR** -- Interactive state additions.

---

### 10. SETTINGS VIEW

#### 10.1 Header
- **MATCH** -- identical.

#### 10.2 Sidebar navigation
- **Mockup:** Uses static `stab` class for tab buttons.
- **Implementation:** Uses conditional className logic for active/inactive state.
- Classes comparison:
  - Active tab -- Mockup: `text-white/70 bg-white/5 font-medium` | Implementation: same. **MATCH**
  - Inactive tab -- Mockup: `text-white/30 hover:text-white/60 hover:bg-white/[0.03]` | Implementation: same. **MATCH**
  - Both use `w-full text-left px-3 py-2 rounded-lg text-[12px] transition-all`
  - Implementation adds `border-0 cursor-pointer bg-transparent`. **MINOR**

#### 10.3 Geral (General) tab

##### 10.3.1 Recordings directory input
- **Mockup:** Has a "Procurar" (Browse) button next to the input: `<div class="flex gap-2"><input ... class="flex-1 ..."><button class="glass-heavy px-4 py-2.5 text-[11px] rounded-xl text-white/40 hover:text-white/70 transition-all">Procurar</button></div>`
- **Implementation:** Just a plain `<input>` with no browse button.

---

- **CRITICAL** -- The stagger entrance animations are completely broken. The CSS class `stagger` exists on elements but there is no CSS rule to trigger the animation. In the mockup, `stagger` only animates when a parent `.view` becomes `.active`. Since the React implementation does not use `.view.active` (it uses Framer Motion), the stagger animations never execute.

---

### 12. SUMMARY OF ALL DIFFERENCES


---

#### CRITICAL (must fix -- breaks the look or functionality)

1. **Chat bubble max-width (ChatPanel.tsx line 221):** Template literal `max-w-[${...}%]` will not be compiled by Tailwind JIT. Messages will have no max-width constraint, stretching full width. **Fix:** Use ternary with complete class strings, e.g., `msg.role === "user" ? "max-w-[75%]" : "max-w-[80%]"`.

2. **Stagger animations non-functional:** The `stagger` class is applied to ~20+ elements across all views, but the CSS rule `.view.active .stagger` from the mockup was never ported to `app.css`. The `stagger-in` keyframe exists but is never triggered. Elements that should animate in with staggered delays simply appear instantly. **Fix:** Either add equivalent CSS rules that work with the React component structure, or implement stagger via Framer Motion's `staggerChildren`.


---

11. **`stagger` class missing on many elements** where it exists in the mockup (record button container, toggle label, gallery sidebar button). Moot point since stagger is broken anyway (see CRITICAL #2).

12. **All buttons add `border-0 bg-transparent cursor-pointer`** compared to the mockup. This is correct CSS reset for `<button>` elements in React and does not change the appearance.

13. **`disabled:opacity-40 disabled:cursor-not-allowed`** added to interactive buttons. Justified for state management.

14. **`dark` class missing from `<body>`** in `index.html`. Not needed since Tailwind v4 is used and no `dark:` prefixes are in the codebase.

15. **`antialiased` added** to `<body>`. An improvement for text rendering.

16. **Transcription tab "Provider" -> "Provedor":** Translated to Portuguese for consistency.

17. **RAG/Chat tab has additional fields** (Chat URL, Chat API Key, Chat Model) not in the mockup. Feature enhancement.

18. **Settings model section** has an enhanced list view with download/activate buttons when model data is available. Falls back to mockup-style select when unavailable.

19. **Meeting page chat button** adds `text-left` class and `bg-transparent` for proper button rendering.

20. **ExportDialog format buttons** use conditional class application instead of mockup's JS toggle. Adds `cursor-pointer bg-transparent`.

---

### RELEVANT FILES

- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/docs/design-mockup-v2.html` -- the spec

---

- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx` -- transitions (CRITICAL #2, IMPORTANT #6, #7)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx` -- line 126 (`mb-8`), line 146 (`text-sm text-white/40`)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx` -- line 221 (broken `max-w-` template literal)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/styles/app.css` -- missing `.stagger` trigger rule
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx` -- missing "Procurar" button around line 195

---

- `loadAll()` uses `Promise.all` with `.catch()` fallbacks on non-critical fetches (encoders, models, active model), so a failure in one does not block the whole page.
- The loading and error-state early returns are well-structured.
- Model download and activate flows call the correct backend commands and refresh the model list afterward.
- The provider toggle (local vs. API) conditionally shows the correct sub-form, and the fallback `<select>` when the models list is empty is a reasonable degraded UX.

---


---

### CRITICAL Issues (Must Fix)

**1. Stale state mutation in `update()` -- closure captures current `settings` reference**

File: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx`, line 67-71.

```tsx
function update(fn: (s: AppSettings) => AppSettings) {
    if (!settings) return;
    setSaved(false);
    setSettings(fn(settings));  // BUG: reads `settings` from closure, not prev state
}
```

This reads `settings` from the outer closure scope rather than using React's functional updater form (`setSettings(prev => fn(prev!))`). When multiple `update()` calls are batched in the same render cycle (unlikely with user inputs, but possible programmatically), intermediate updates are lost because each call reads the same stale `settings` snapshot. The fix is trivial:

```tsx
setSettings(prev => prev ? fn(prev) : prev);
```


---

This is critical because it is a correctness bug in the core state-update path.

**2. `error` state is global and never cleared across tabs or across different operations**

The single `error` state variable is shared by `loadAll`, `handleSave`, `handleDownloadModel`, and `handleSetActiveModel`. But only `handleSave` clears it (`setError(null)` at line 75). The other three do not clear `error` on success. Consequences:

- If a model download fails, then the user switches to the "Geral" tab, the error from the download is displayed next to the "Salvar" button on an unrelated tab.
- If `handleSetActiveModel` fails and the user later changes a setting and saves successfully, the stale error from `handleSetActiveModel` persists because `handleSave` only clears error at the start -- but `saved` and `error` render side-by-side, so a successful save shows both "Salvo!" and the old error simultaneously (line 251-252 pattern repeats on every tab).
- `handleDownloadModel` and `handleSetActiveModel` should each clear `error` at the start.

**3. `Number(e.target.value)` produces `NaN` for empty input fields**

Lines 313, 346-349, 414-418, 709, 724: numeric inputs use `Number(e.target.value)`. When the user clears the input (backspacing to empty), `Number("")` produces `NaN`, which will be serialized as `null` or cause a serialization error when sent to Rust (which expects `u32`). This will crash `updateSettings` on the backend. Every `Number()` call needs a guard, e.g. `Number(e.target.value) || 0` or `parseInt(e.target.value, 10) || existingValue`.

---


---

| 1 | Critical | `update()` reads stale `settings` closure instead of using functional updater |

---

| 2 | Critical | `error` state is global, never cleared by download/activate flows, bleeds across tabs |

---

| 3 | Critical | `Number("")` produces `NaN` on empty numeric inputs, will crash backend serialization |

---

**Floating widget** -- MISSING (Critical).
The spec explicitly describes a floating widget (`widget.rs`) as a minimal always-on-top Tauri window showing recording timer, pulsing indicator, and stop button. Neither the file nor any widget-related code exists. The recorder `mod.rs` does not declare a `widget` module.


---

### Critical Issues (must fix)

| # | Module | Issue | Description |
|---|--------|-------|-------------|
| 1 | Recorder | MISSING: Floating widget | Spec explicitly requires `widget.rs` with always-on-top Tauri window (timer, stop button, red indicator). No code exists. Users have no way to control recording without the main window. |
| 2 | Recorder | DEVIATED: audio-only missing system track | `build_audio_only` captures mic only, but `stop_recording` reports 2 tracks (mic + system). Metadata claims system audio exists when it does not. |
| 3 | Recorder | DEVIATED: ignores user settings | `start_recording` uses hardcoded `VideoConfig::default()` and `AudioConfig::default()` instead of reading from the user's Settings. The settings system is fully built but the recorder does not use it. |


---

#### CRITICAL

**1. `Content-Type` header is redundant and risks conflict with reqwest's `.json()` method (line 117)**

When you call `.json(&body)` on a reqwest `RequestBuilder`, reqwest automatically sets `Content-Type: application/json`. Explicitly setting `Content-Type` before `.json()` means there will be two `Content-Type` headers, or the explicit one will be overwritten by reqwest's internal logic depending on the version and header map behavior. In reqwest 0.12, `.json()` calls `.header(CONTENT_TYPE, "application/json")` which uses the `HeaderMap::insert` method, so it will overwrite. The result is the same, but this is misleading code that suggests the author thought it was necessary.

This is not a functional bug in the current version, but it is a code smell that indicates a misunderstanding of the reqwest API and could become a real bug if someone changes the order of chained calls, or switches to a different serialization approach.

**Recommendation**: Remove line 117.

```rust
// Before:
.header("Content-Type", "application/json")
.json(&body)

// After:
.json(&body)
```

---

**2. Error response body text is lost when deserialization fails (lines 124-129)**

When the API returns a non-success status, the code tries to parse the body as `ApiErrorBody`. If that deserialization fails, the error message falls back to just `"HTTP {status_code}"`. However, the response body has already been consumed by `response.json()`, so the original error text from the API is permanently lost.

Many OpenAI-compatible APIs return error messages in non-standard formats (plain text, HTML, or different JSON structures). Losing that text makes debugging significantly harder.

**Recommendation**: Read the body as text first, then try to parse it as JSON.

```rust
let status_code = status.as_u16();
let body_text = response.text().await.unwrap_or_default();
let message = serde_json::from_str::<ApiErrorBody>(&body_text)
    .ok()
    .and_then(|b| b.error)
    .and_then(|e| e.message)
    .unwrap_or_else(|| {
        if body_text.is_empty() {
            format!("HTTP {}", status_code)
        } else {
            body_text.chars().take(500).collect()
        }
    });
```

---


---

For embedding-heavy workloads (e.g., indexing a long transcript in 50-chunk batches), this is not critical since the same `EmbeddingsClient` instance is reused within the batch loop. But across multiple calls it creates unnecessary overhead.

**Recommendation**: Either store the `Client` in `RagState`, or use `once_cell::sync::Lazy<Client>` / `std::sync::LazyLock<Client>` as a module-level singleton.

---

**6. `embed_batch` takes `&[String]` instead of `&[&str]` or `&[impl AsRef<str>]` (line 100)**

The signature `pub async fn embed_batch(&self, texts: &[String]) -> ...` forces callers to own `String` values. This is unnecessarily restrictive. The `embed_one` method already does `text.to_string()` to create a `Vec<String>` (line 89), which shows the friction this causes. Callers in `commands.rs` line 263 also have to `.map(|c| c.text.clone()).collect()` to build a `Vec<String>`.

However, since `.json(&body)` will serialize anyway, and `EmbeddingRequest.input` is `Vec<String>`, changing the signature alone requires also changing the request type. This is a design consideration rather than a bug.

**Recommendation**: Accept `&[impl AsRef<str>]` or at minimum `&[&str]`, and build the owned `Vec<String>` internally only when constructing the request body.

---


---

| 1 | Critical | Redundant `Content-Type` header with `.json()` | 117 |

---

| 2 | Critical | Error response body text lost on parse failure | 124-129 |

---

#### CRITICAL -- Stream index `0:a:0` is fragile and may select the wrong track

On line 69, the FFmpeg command uses `-map 0:a:0` to select "the first audio stream." Looking at `pipeline.rs`, the GStreamer muxer links tracks in this order:

1. **Video** encoder linked to mux (line 147)
2. **Mic** encoder linked to mux (line 152)
3. **System audio** encoder linked to mux (line 157)

For video recordings, `0:a:0` correctly maps to the mic because it is the first *audio* stream (video is stream `0:v:0`, not `0:a:0`). For audio-only recordings (`build_audio_only`), there is only one audio track, so `0:a:0` is also correct.

**Verdict:** The `0:a:0` mapping is actually correct. The `a` qualifier filters to audio-only streams, so the video stream is excluded from the index. The mic is always linked to the muxer before system audio, making it `0:a:0` in both pipeline configurations. No issue here.


---

**Severity:** Important (not critical because the current sole caller does check, but the orchestrator is a public function).


---

No critical issues were found. The FFmpeg invocation is correct in argument order, codec names, and output format. The code is clean, well-documented, and architecturally sound. The important issues are all defensive-programming concerns that would matter if the codebase grows or if error diagnostics need to be clearer.

---

## CRITICAL ISSUES

None found. There are no known deprecated crates or security-vulnerable versions in this dependency set as of May 2025.


---

### CRITICAL Issues (user gets stuck or loses data)

**C1. Recording view is a dead end if recording fails to start**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx`, lines 33-37 and 92

When `isRecordingView` is true, App.tsx has already navigated to the `"recording"` view. If `handleStart()` fails (line 62-64), the error is displayed but `recording` remains `false`. Because of the condition on line 92 (`if (isRecordingView || recording)`), the recording UI still renders -- but it shows a stop button for a recording that never started. There is **no back button and no way to return home**. The user is trapped on a screen that says "Gravando" with a "Parar" button that will attempt to stop a non-existent recording (which will also fail). The only escape is closing the entire application.

**C2. Stopping a never-started recording navigates to gallery with potential data loss**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx`, lines 69-83

If `handleStop` is called when no recording was actually started (following from C1), `stopRecording()` will throw. The error is shown, but the user is still stuck on the recording view with no way out.

**C3. Chat history is lost on navigation and cannot be recovered**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx`, line 20

Chat messages live in local component state (`useState<ChatMsg[]>([])`). When the user navigates back to MeetingPage and returns to Chat, the entire conversation is wiped. There is no persistence. If a user spent significant time chatting about a meeting, pressing the back button destroys all of that work with no warning.

**C4. Settings changes are silently lost when navigating away without saving**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx`

The user can change any number of settings across multiple tabs, then press the back arrow without saving. There is no "unsaved changes" warning or auto-save. Every modification is lost silently. This is especially bad because the save button is at the bottom of each tab -- a user could easily change settings, switch tabs (which shows a fresh save button), and navigate away thinking the previous tab's changes were saved.

---


---

| CRITICAL | 4 | Dead-end recording view, silent data loss (chat, settings) |

---

## CRITICAL Issues

### 1. Silently swallowed emit errors on every stream event (lines 181, 184, 190)

```rust
let _ = app.emit_to("main", "chat-stream-chunk", &text);
// ...
let _ = app.emit_to("main", "chat-stream-error", e.to_string());
// ...
let _ = app.emit_to("main", "chat-stream-done", ());
```

All three `emit_to` calls discard the `Result` with `let _`. If the window label `"main"` does not match (e.g., user opened a second window, or label was changed in `tauri.conf.json`), the entire stream runs server-side but the frontend receives nothing -- no chunks, no done signal, no error. The user sees "..." forever with no indication of what happened. The `chat_message` command returns `Ok(())` even though the frontend received nothing.

**Recommendation:** At minimum, log a warning on failure. For the `chat-stream-done` case, consider returning an error to the command caller so the frontend's `catch` block fires. Alternatively, use `app.emit(event, payload)` (broadcasts to all windows) instead of `emit_to`, since this is a single-window application -- it avoids the label coupling entirely.

### 2. Race condition: concurrent `chat_message` calls share the same global event names (lines 178-191)

The frontend listens on `"chat-stream-chunk"`, `"chat-stream-done"`, and `"chat-stream-error"` as globally-named events. If two `chat_message` calls are in flight simultaneously (e.g., user sends a second message before the first finishes, or two different meetings are chatted in quick succession), chunks from both streams are interleaved into the same listener, and a `"chat-stream-done"` from the first stream will terminate the second stream's listener prematurely.

**Recommendation:** Include a unique correlation ID (UUID or the `meeting_id`) in the event name or payload. For example: `"chat-stream-chunk-{meeting_id}"` or send a structured payload `{ "stream_id": "...", "text": "..." }`. The frontend already serializes sends via the `sending` state, but that is a UI-level guard and not enforced at the backend.

### 3. `do_index_meeting` skips `ensure_ready` when `embeddings` is empty after a non-empty chunk list (lines 271-276)

```rust
let dimension = embeddings
    .first()
    .map(|e| e.len())
    .unwrap_or(0);
// ...
store.ensure_ready(&config.embeddings_model, dimension)?;
```

If the embeddings API returns an empty vector for every chunk (a malformed API response that passes the batch-level checks), `dimension` will be `0`. Then `ensure_ready` is called with `dimension = 0`, which on a fresh database will call `init_vector_table` creating `float[0]` -- a nonsensical virtual table. On a database that already has a non-zero dimension, this might silently pass as a `ModelStatus::Match`.

**Recommendation:** Add an explicit guard:

```rust
if dimension == 0 {
    return Err(RagCommandError::Other(
        "Embedding API returned zero-dimension vectors".to_string(),
    ));
}
```

---


---

| Critical | 3 | Silent emit failures, race on global event names, zero-dimension vector table |

---

### CRITICAL Issues (must fix)

**C1. No `Drop` implementation -- pipeline resources leak if `RecordingPipeline` is dropped without calling `stop()`**

There is no `impl Drop for RecordingPipeline`. If the struct is dropped due to a panic, an error path, or being replaced in the `Mutex<Option<...>>` without calling `stop()`, the GStreamer pipeline will never be set to `Null` state. This leaks native GStreamer resources, PipeWire connections, file handles, and the file descriptor for the muxed output. GStreamer pipelines MUST be moved to `Null` state before being freed.

Recommendation:
```rust
impl Drop for RecordingPipeline {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
```

**C2. `pipewiresrc` for audio-only pipeline (`build_audio_only`) has no `stream-properties` configuration**

On line 27-30, the mic source in `build_audio_only` is created as a bare `pipewiresrc` with zero properties set. Unlike the video+audio path (line 104-111) which sets `media.class` to `"Audio/Source"`, the audio-only path provides no routing hints to PipeWire. On many PipeWire configurations, a bare `pipewiresrc` without `media.class` set will either fail to connect or will capture the wrong stream (e.g., a monitor/loopback rather than the microphone). This is inconsistent with `build_with_video` and likely results in broken audio-only recording.

Recommendation: Add the same `stream-properties` with `media.class = "Audio/Source"` as in `build_with_video`.

**C3. System audio source uses incorrect `media.class` value `"Audio/Sink"`**

On line 117-119, the system audio source sets `media.class` to `"Audio/Sink"`. This is semantically wrong. `"Audio/Sink"` means "I am an audio output device" (like a speaker). To capture system audio (desktop audio loopback), the correct PipeWire media class is `"Stream/Output/Audio"` (to capture what other applications are outputting). With `"Audio/Sink"`, PipeWire will either reject the stream or connect it to the wrong node, meaning system audio capture is effectively broken.

Recommendation: Use `"Stream/Output/Audio"` for the system audio capture source. However, note that capturing system audio via `pipewiresrc` with `stream-properties` alone is not straightforward -- you typically need to use a PipeWire loopback or specify a target node. This design needs rethinking for system audio capture.

**C4. `output_path` uses `to_string_lossy()` which silently corrupts non-UTF-8 paths**

On lines 41 and 133, `output_path.to_string_lossy().to_string()` is used to set the `location` property on `filesink`. If the output path contains non-UTF-8 bytes (common on Linux with certain locale configurations), `to_string_lossy()` will replace invalid bytes with the Unicode replacement character, resulting in a silently wrong file path. The file will either be created at a wrong location or `filesink` will fail.

Recommendation: Use `output_path.to_str().ok_or("Path is not valid UTF-8")?.to_string()` to fail explicitly, or use the raw OS string if GStreamer accepts it. Given that GStreamer's `filesink` location property takes a UTF-8 string, failing early is the correct behavior.

---


---

| C1 | Critical | Resource Cleanup | No `Drop` impl -- pipeline leaks if dropped without `stop()` |

---

| C2 | Critical | PipeWire Config | Audio-only `pipewiresrc` has no `stream-properties` |

---

| C3 | Critical | PipeWire Config | System audio uses wrong `media.class` (`Audio/Sink`) |

---

| C4 | Critical | File Path | `to_string_lossy()` silently corrupts non-UTF-8 paths |

---

## CRITICAL Issues (Must Fix)

### 1. Tailwind v4 dynamic `max-w-[...]` in template literal -- classes will NOT be generated

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx`, line 221

```jsx
<div className={`rounded-2xl px-4 py-3 max-w-[${msg.role === "user" ? "75" : "80"}%] ${...}`}>
```

This produces either `max-w-[75%]` or `max-w-[80%]` at runtime. Tailwind (both v3 and v4) performs static analysis of source files to detect class names. It cannot detect classes assembled from template interpolation. Neither `max-w-[75%]` nor `max-w-[80%]` will be present in the final CSS output. The element will have no max-width constraint, causing chat bubbles to stretch to 100% width.

**Fix:** Use complete, statically-detectable class strings:

```jsx
<div className={`rounded-2xl px-4 py-3 ${msg.role === "user" ? "max-w-[75%]" : "max-w-[80%]"} ${...}`}>
```

This way both full strings `max-w-[75%]` and `max-w-[80%]` appear as literal text in the source file, and Tailwind will extract and generate both.

### 2. Race condition: listeners registered AFTER `chatMessage` could be called

**File:** Lines 90-118

The three `listen()` calls (lines 90, 100, 104) are `await`ed sequentially. Each `listen()` is an async call that must round-trip to the Tauri core to register. Only after all three are registered does `chatMessage` fire (line 118). However, consider the execution order:

1. `unlistenChunk = await listen("chat-stream-chunk", ...)` -- registered
2. `unlistenDone = await listen("chat-stream-done", ...)` -- registered
3. `unlistenError = await listen("chat-stream-error", ...)` -- registration starts
4. `chatMessage(meetingId, msg)` -- fires AFTER step 3 completes

This ordering is actually safe for the initial call since `chatMessage` is `await`ed after all listeners are set up. However, there is a subtler problem: if the Rust backend emits `chat-stream-done` or `chat-stream-error` synchronously during the `chatMessage` invoke (before the JS side resumes), those events could fire before `await streamFinished` is reached, but the Promise is already created so `resolveDone` would already be assigned. This is fine.


---

On closer inspection, the ordering is correct. Downgrading this from critical. No action needed.

### 3. `streamBufferRef` is accumulated but never read

**File:** Line 91

```js
streamBufferRef.current += event.payload;
```

The buffer is written to on every chunk but is never read anywhere in the component. It is reset at line 82 (`streamBufferRef.current = ""`) before each send but its value is never consumed. This is dead code. If it was intended as a fallback or for debugging, it should be documented or removed.


---

| 1 | **Critical** | Dynamic template literal breaks Tailwind class extraction for `max-w-[...]` | 221 |

---

The **critical item** (issue 1) is the `max-w-[...]` template literal -- those chat bubble width constraints are definitely not being applied in the built output. That should be fixed immediately.

---

# Critical Issues (Must Fix)

None. All handlers exist and map to real Tauri IPC commands.


---

### CRITICAL Issues

#### 1. Bitrate property types are wrong for most encoders -- will panic at runtime

On line 49, 52, and 55, the code does:

```rust
encoder.set_property("bitrate", config.bitrate / 1000);
```

`config.bitrate` is `u32`, so `config.bitrate / 1000` is `u32`. The GStreamer `set_property` method is generic and resolves the GObject property type at runtime. The actual expected types vary by encoder:

| Encoder | `bitrate` property type | Unit |
|---------|------------------------|------|
| `vaapih264enc` / `vaapih265enc` | **`u32`** | **kbps** |
| `nvh264enc` / `nvh265enc` | **`u32`** | **kbps** |
| `x264enc` | **`u32`** | **kbps** |
| `x265enc` | **`u32`** | **kbps** |
| `svtav1enc` | **`u32`** | **kbps** |


---

So the **type** (`u32`) happens to be correct for these specific encoders. However, there is still a subtle issue: `gstreamer-rs` `set_property` does a `glib::Value` conversion and will check the GObject property spec at runtime. If the GObject property is registered as `guint` (which maps to `u32`), passing a Rust `u32` is fine. But some NVIDIA encoder plugin versions expose `bitrate` as `guint` while others use `guint64`. If you hit a version mismatch, this panics. **This is borderline critical -- it works on common setups but can panic on edge cases.**


---

The SVT-AV1 GStreamer element (`svtav1enc`) does **not** have a property named `"bitrate"`. Its bitrate property is named `"target-bitrate"` and is in **kbps**. Line 55 will cause a runtime panic (GLib critical / Rust panic via `set_property`) when the Software + AV1 combination is used.

**Recommendation**: The `create_video_encoder` function needs to dispatch on both `backend` **and** `codec`, or at least on the element name, to set the correct property. For example:

```rust
match element_name {
    "svtav1enc" => encoder.set_property("target-bitrate", config.bitrate / 1000),
    _ => encoder.set_property("bitrate", config.bitrate / 1000),
}
```

#### 3. Nonexistent GStreamer element names for Vulkan and NVIDIA AV1

In `types.rs` lines 78-80 and 77:

- `"vulkanh264enc"`, `"vulkanh265enc"`, `"vulkanav1enc"` -- **These elements do not exist** in any released version of GStreamer (as of 1.24.x). GStreamer's Vulkan plugin provides `vulkanh264enc` and `vulkanh265enc` only starting from GStreamer 1.24, and they are experimental. `vulkanav1enc` does not exist at all.
- `"nvav1enc"` -- This element does **not exist** in GStreamer's NVENC plugin. The NVIDIA AV1 hardware encoder element is either not available at all, or in very recent versions (1.24+) is still named differently. The standard NVENC plugin only provides `nvh264enc` and `nvh265enc`.

**Impact**: `probe_available()` will simply never find these elements (so they are dead code). But `create_video_encoder` can be called directly with these combinations, leading to an error message that mentions a nonexistent element -- confusing to debug.

**Recommendation**: Either remove the nonexistent element names or gate them behind a GStreamer version check. At minimum, add a comment documenting which GStreamer version is required for each element. For NVIDIA AV1, the correct element name in GStreamer 1.26+ would likely be `nvav1enc` -- but this needs verification against the actual plugin registry.

---


---

| 1 | CRITICAL | Vulkan backend silently skips bitrate; `set_property` can panic on type mismatch |

---

| 2 | CRITICAL | `svtav1enc` property is `target-bitrate`, not `bitrate` -- runtime panic |

---

| 3 | CRITICAL | Several GStreamer element names (`vulkanav1enc`, `nvav1enc`) do not exist |

---

The three critical issues (#1, #2, #3) should be addressed before this code is exercised in production. Issues #2 and #3 will cause hard panics or confusing errors for users who trigger specific backend+codec combinations.

---

## CRITICAL Issues (Must Fix)

### 1. TOCTOU Race in `delete_meeting_dir` (line 36-39)

```rust
pub fn delete_meeting_dir(&self, meeting_dir: &Path) -> std::io::Result<()> {
    if meeting_dir.exists() {
        fs::remove_dir_all(meeting_dir)?;
    }
    Ok(())
}
```


---

| 1 | Critical | TOCTOU race in `delete_meeting_dir` | 36-39 |

---

| 2 | Critical | TOCTOU race in `delete_media_files` | 42-52 |

---

| 3 | Critical | No path containment check -- directory traversal possible | 15-18, 25-28, 35-39 |

---

The three critical issues (1, 2, 3) should be addressed before this code handles any user-facing data. Issues 5 and 7 are the most impactful of the important tier -- a crash during transcript save could corrupt data, and symlink following during delete could destroy unrelated files.

---

## CRITICAL Issues (Must Fix)

### 1. SRT: Trailing blank line creates a double-blank at EOF -- many parsers tolerate this, but the real problem is empty segments

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/transcript.rs`, line 85.

The format string `"{}\n{} --> {}\n{}\n\n"` produces a correct SRT block: index, timestamp line, text, blank separator line. This is correct per spec. However, if `segment.text.trim()` is empty (e.g., a silence segment or a segment with only whitespace), you emit a subtitle block with no visible text:

```
3
00:00:10,000 --> 00:00:12,000


4
...
```

SRT players handle this inconsistently. Some render a blank subtitle; others error.

**Recommendation:** Skip segments where `text.trim().is_empty()` to produce cleaner output.

### 2. Negative or NaN `f64` timestamps cause wrapping/panic in `format_srt_timestamp`

**File:** line 200-209.

`(seconds * 1000.0).round() as u64` -- if `seconds` is negative (which can happen from buggy upstream data or floating point drift like `-0.001`), this cast to `u64` on Rust stable wraps to an astronomically large number (saturating to `u64::MAX` on newer Rust editions). If `seconds` is `NaN`, the behavior is similarly undefined/saturating.

```rust
let total_ms = (seconds * 1000.0).round() as u64;  // line 201
```

The same issue applies to `format_readable_timestamp` at line 214.

**Recommendation:** Clamp to zero: `let total_ms = (seconds.max(0.0) * 1000.0).round() as u64;` and add a test for negative input.

---


---

| 1 | Critical | Empty `text.trim()` emits blank SRT subtitle block | 83-85 |

---

| 2 | Critical | Negative/NaN f64 causes u64 wrapping in timestamp formatting | 201, 214 |

---

## CRITICAL Issues

### C1. VAAPI element names are deprecated/wrong for modern GStreamer

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/types.rs`, lines 72-74

The VAAPI plugin (`gstreamer-vaapi`) was deprecated in GStreamer 1.22+ and removed from the main distribution. Modern GStreamer uses the `va` plugin instead. The element names `vaapih264enc`, `vaapih265enc`, `vaapiav1enc` will not be found in the registry on most current systems. The correct names are:

- `vah264enc` (or `vah264lpenc` for low-power)
- `vah265enc` (or `vah265lpenc`)
- `vaav1enc` (available from GStreamer 1.24+)

Since gstreamer-rs 0.25 corresponds to GStreamer 1.26.x, the `vaapi*` elements are almost certainly absent. This means the VAAPI backend will always fail to probe, and the fallback chain will skip hardware encoding entirely even on systems with VA-API support.

### C2. System audio capture with `media.class = "Audio/Sink"` will not work

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 117-124

Setting `stream-properties` with `media.class = "Audio/Sink"` on a `pipewiresrc` element does not capture system/desktop audio. `pipewiresrc` is a PipeWire source node (it produces data), and `Audio/Sink` describes a sink node (it consumes data). To capture desktop audio on PipeWire, you need to create a stream with `media.class = "Stream/Input/Audio"` that targets a monitor of the system output, or use the portal-based approach. As written, the system audio source will either fail to negotiate or capture silence, or connect to the wrong node.

### C3. Muxer linking uses `link()` which only works for always-pads, but `matroskamux` has request pads

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 53, 147, 152, 157

`matroskamux` uses request pads (`audio_%u`, `video_%u`), not always-pads. The `link()` method works here because GStreamer's `gst_element_link()` internally handles request pads by requesting a compatible pad. However, calling `link()` on a second and third encoder to the same mux (video_enc, mic_enc, sys_enc all link to `mux`) is risky -- each `link()` call requests a new pad, so it should technically work. But the real problem is that when multiple streams link to a mux, the mux will wait for data on ALL linked pads before it starts muxing. If the system audio source fails (see C2), the entire pipeline will hang indefinitely because `matroskamux` is waiting for data on that pad. There is no timeout or error handling for this scenario.

---


---

`nvh264enc` and `nvh265enc` are from the older `nvcodec` plugin. In GStreamer 1.24+, the preferred NVIDIA encoders are `nvh264enc` (still valid) but also `nvh264device0enc` or `nvautogpuh264enc`. More critically, `nvav1enc` does not exist -- the correct name would be `nvav1enc` only on very recent builds; on most systems it would not be present. This is less severe than the VAAPI issue since the older names still work if the nvcodec plugin is installed, but AV1 NVIDIA encoding availability is questionable.

### I3. `stop_recording` always reports two audio tracks even for audio-only recordings

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs`, lines 100-108

The `TrackInfo` vector is hardcoded with both "mic" and "system" tracks regardless of whether the recording was audio-only (which only has mic) or video (which has mic + system). For audio-only recordings, this metadata is inaccurate. The track info should be derived from `pipeline.has_video()` or a more explicit track count.

### I4. No `Drop` implementation for `RecordingPipeline` -- resource leak on panic/error paths

**Files:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`

If `start_recording` succeeds but `stop_recording` is never called (application crash, panic during recording, user closes window), the GStreamer pipeline remains in `Playing` state. The pipeline holds open file handles to the PipeWire connection and the output file. While the OS will reclaim resources on process exit, the MKV file will be corrupt (no proper trailer). A `Drop` impl that sends EOS + sets state to Null would be safer:

```rust
impl Drop for RecordingPipeline {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
```

### I5. Bitrate type mismatch risk in `create_video_encoder`

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/encode.rs`, line 49

`config.bitrate` is `u32` (default 2,000,000). The code does `config.bitrate / 1000` yielding `u32`. However, GStreamer encoder `bitrate` properties have varying types -- some expect `u32`, some expect `i32`, some expect `u64`. Using `set_property("bitrate", config.bitrate / 1000)` with automatic type inference may fail at runtime if the encoder expects a different integer type. The VA (formerly VAAPI) encoders typically expect `bitrate` as `u32` in kbps, which would work, but x264enc expects `bitrate` as `u32` in kbps while x265enc expects it as `u32` in kbps too. The real risk is if the runtime type does not match -- GStreamer will panic in debug builds or silently fail.

### I6. `pipewiresrc` `fd` property type mismatch

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, line 76

The `fd` property on `pipewiresrc` expects an `i32` (that is its GObject property type), but `PipeWireSource.fd` is `std::os::fd::RawFd` which is also `i32` on Linux. This should work, but it is worth being explicit with `.property("fd", screen_source.fd as i32)` to make the intent clear and avoid surprises on platforms where `RawFd` might differ.

---


---

| CRITICAL | 3 | VAAPI element names wrong for GStreamer 1.26; system audio capture will not work; mux hangs if system audio fails |

---

### CRITICAL Issues

**1. Residual buffer data is silently discarded when the stream ends (line 141-198)**


---

| 1 | **Critical** | Residual buffer data silently lost when stream ends without trailing newline | 141-198 |

---

| 2 | **Critical** | `from_utf8_lossy` corrupts multi-byte chars split across TCP chunks | 146 |

---

The two critical issues (1 and 2) are the ones most likely to produce user-visible bugs in production -- specifically, truncated final tokens and corrupted Unicode characters in LLM responses. I recommend addressing those first.

---

## Critical Issues (Must Fix)

**1. Settings are ignored by the recorder** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` lines 40-41)

The `start_recording` command creates `VideoConfig::default()` and `AudioConfig::default()` instead of reading from the user's saved settings. The entire Settings > Video and Settings > Audio panels are non-functional.

**Fix:** Load settings via `load_settings()` at the start of `start_recording` and construct `VideoConfig`/`AudioConfig` from the user's configured values.

**2. Media playback is broken** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx` lines 85-109)

Chromium WebView does not support MKV container or H.265 codec. The `<video>` and `<audio>` elements will fail to play. Additionally, loading entire files into memory via `readFile` is not scalable.

**Fix:** Either (a) use a Tauri asset protocol / `convertFileSrc` to stream the file via a local URL and rely on system codecs, or (b) transcode to a web-compatible format (WebM with VP9/Opus or MP4 with H.264/AAC) for preview playback, or (c) at minimum, use `<audio>` with an Opus-in-OGG extraction for audio-only meetings.

**3. Hardcoded track count in `stop_recording`** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` lines 104-107)

Audio-only recordings create 1 track but report 2 tracks in metadata. This is inaccurate.

**Fix:** Derive the track list from the actual pipeline configuration. For audio-only, report 1 track (mic). For video, report the actual streams that were created.

---


---

## CRITICAL Issues

### 1. `emit_to` uses global `listen`, causing event leaks in multi-concurrent-chat scenarios

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/commands.rs` (lines 181, 184, 190)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx` (lines 90, 100, 104)

The backend emits `chat-stream-chunk`, `chat-stream-done`, `chat-stream-error` via `emit_to("main", ...)`. The frontend uses `listen(...)` which is a **global** listener -- not filtered by meeting ID. If the user sends a second chat message before the first stream finishes, the `listen` calls from both invocations will overlap: the second message's chunk handler will receive chunks from the first message's stream, and vice versa.

This is architecturally fragile. The current UI appears to serialize chat sends (the `sending` state guard prevents it), but a race could still occur if the user navigates away and back quickly. Consider either:
- Including the meeting ID in the event name (e.g., `chat-stream-chunk-{meetingId}`)
- Or adding a correlation ID payload to each event and filtering in the listener

**Verdict:** Currently mitigated by UI state guard, but the pattern is brittle. Elevating because a navigation-based race could cause garbled chat output.

---


---

| 1 | CRITICAL | Events | `emit_to`/`listen` for chat streaming has no correlation ID; race-prone |

---

**Overall verdict:** The Tauri v2 integration is fundamentally sound -- command registration, state management, plugin setup, capabilities, and frontend-backend type alignment are all correct. The critical item (event race) is mitigated by current UI flow but should be addressed. The important items around blocking commands (issues 4 and 5) will cause real UX problems for any non-trivial file sizes and should be prioritized.

---

## CRITICAL Issues

### 1. `prepare_meeting` orphans a directory on filesystem if recording never finalizes

**Location:** Lines 58-75

If `prepare_meeting` is called but `finalize_meeting` is never called (app crash, user cancels, recording error), the entry stays in the `prepared` HashMap only as long as the process lives, and the created directory on disk lives forever. There is no cleanup mechanism -- no timeout, no sweep, no startup reconciliation.

**Impact:** Disk leak. Over time, abandoned directories accumulate in the recordings folder. Since the `prepared` map is purely in-memory, a process restart loses all knowledge of these orphans.

**Recommendation:** Add a `cleanup_stale_prepared` method (or run it on startup) that scans the recordings directory for directories not referenced by any meeting in the database, and removes them. Alternatively, defer directory creation to `finalize_meeting` and only reserve the ID in `prepare_meeting`.

### 2. `delete_meeting` with `DeleteMode::Everything` -- filesystem deletion before DB deletion creates inconsistency on failure

**Location:** Lines 172-174

```rust
DeleteMode::Everything => {
    self.fs.delete_meeting_dir(&meeting.dir_path)?;  // line 173
    db.delete_meeting(id)?;                           // line 174
}
```

If `fs.delete_meeting_dir` succeeds but `db.delete_meeting` fails (e.g., SQLite disk I/O error), the meeting row persists in the database but its `dir_path` no longer exists on disk. Every subsequent operation on this meeting that touches the filesystem (get_meeting_detail, read_artifact, save_artifact) will fail with confusing IO errors.

**Recommendation:** Reverse the order -- delete from DB first, then from filesystem. A DB row without files is recoverable (the UI can show "media deleted"). Files without a DB row are orphans, which are harder to reconcile. Alternatively, wrap both in a best-effort cleanup that logs but does not early-return on FS failure.

### 3. `delete_meeting` with `DeleteMode::MediaOnly` -- same ordering problem

**Location:** Lines 177-178

```rust
DeleteMode::MediaOnly => {
    self.fs.delete_media_files(&meeting.dir_path)?;   // line 177
    db.update_media_status(id, MediaStatus::Deleted)?; // line 178
}
```

If the FS deletion succeeds but the status update fails, the media files are gone but `media_status` still reads `Present`. The UI will offer playback for files that no longer exist.

**Recommendation:** Same as above -- update DB status first (to `Deleted`), then delete files. If file deletion fails, the status is already `Deleted` which is the safer inconsistency (user sees "deleted" but files still exist -- recoverable, and you can retry deletion).

---


---

| 1 | CRITICAL | Orphaned directories on disk if finalize never called | 58-75 |

---

| 2 | CRITICAL | FS-before-DB deletion order in `Everything` mode | 172-174 |

---

| 3 | CRITICAL | FS-before-DB deletion order in `MediaOnly` mode | 177-178 |

---

The three critical issues (#1, #2, #3) should be addressed before this code handles real user data. Issues #2 and #3 are simple ordering swaps. Issue #1 requires a design decision about cleanup strategy.

---

### CRITICAL Issues

**1. `ApiProvider::transcribe` calls `reqwest::blocking` inside `spawn_blocking` -- this works, but `create_provider` reads settings on the async task, while the provider itself runs on the blocking thread. The real problem is that `ApiProvider` performs a *network I/O* call (which could block for minutes on large files uploading to a remote API) inside `spawn_blocking`. This is technically sound for tokio's blocking threadpool, but `spawn_blocking` has a limited pool (default 512 threads). For a single-user desktop app this is not a practical concern, but it is worth documenting the assumption.**

Severity: Informational -- no fix required for a desktop app, but good to note.

**2. No guard against concurrent transcription of the same meeting (lines 45-114)**

If the frontend fires `transcribe_meeting` twice rapidly for the same `id`, both calls will pass the `Processing` status check (both read `Pending`, both set `Processing`), and two blocking threads will run FFmpeg + Whisper concurrently on the same file. This leads to:
- Double resource consumption (CPU/GPU for Whisper, disk for temp WAV)
- A race condition on `save_artifact` -- both threads write `transcript.json` and `transcript.txt` for the same meeting, with the last writer winning
- The temp WAV file `_temp_mic.wav` in the meeting directory will be written/deleted by both threads simultaneously, which is a data corruption risk in `extract_mic_to_wav` (both invoke `ffmpeg -y` to the same output path)

The status check on line 67 sets `Processing`, but it does not guard against a second caller that has already passed `get_meeting` on line 53 before the first caller sets `Processing`. This is a classic TOCTOU race.

**Recommendation**: Check the current status before setting it to `Processing`, and reject if it is already `Processing`:

```rust
let meeting = library.get_meeting(&id).map_err(|e| ...)?;
if meeting.transcription_status == TranscriptionStatus::Processing {
    return Err("Transcription is already in progress".to_string());
}
```

Even better, make `update_transcription_status` return the previous status atomically (a `UPDATE ... WHERE status != 'processing' RETURNING ...` pattern), so the check-and-set is a single database operation.


---

Severity: **Critical** -- can cause temp file corruption and wasted resources.

---


---

| 2 | No guard against concurrent transcription of same meeting | Critical | 45-74 |

---

The codebase has three FFmpeg invocation points. The overall quality is solid: argument ordering is correct, `Command::new` + `.arg()` eliminates injection risks, exit codes are checked, and stderr is captured. However, there are several issues ranging from a critical correctness bug to useful suggestions.

---

### FILE 1: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/orchestrator.rs`

**What it does:** Extracts mic audio (first audio stream) from MKV into 16 kHz mono WAV for Whisper transcription.

**Reconstructed command:**
```
ffmpeg -y -i recording.mkv -map 0:a:0 -ac 1 -ar 16000 -codec:a pcm_s16le _temp_mic.wav
```

**Verdict: CORRECT.** This is textbook FFmpeg usage for Whisper input preparation.

- `-y` before `-i` -- correct.
- `-map 0:a:0` selects the first audio stream from input 0 -- correct for grabbing the mic track (mic is the first audio stream linked to the muxer in `pipeline.rs`).
- `-ac 1 -ar 16000 -codec:a pcm_s16le` -- standard Whisper requirements, all correct.
- Output path at the end -- correct.
- Exit status checked, stderr captured -- correct.
- `map_err` on `.output()` catches "ffmpeg not found" (the `io::Error` will say something like "No such file or directory") -- acceptable, though the error message is generic.


---

**CRITICAL 1: The `amerge` filter input pad specification `[0:a]` selects only the first audio stream, not both.**

At line 70 in `audio.rs`:
```rust
.arg("[0:a]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]")
```

The filter `[0:a]amerge=inputs=2` is wrong. The `amerge` filter with `inputs=2` expects two input pads, but `[0:a]` feeds only the first audio stream (or, depending on FFmpeg version, all audio streams matched by `0:a` as a single link). The correct syntax to merge two specific audio streams is:

```
[0:a:0][0:a:1]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]
```

With the current filter, FFmpeg may either: (a) fail outright because `amerge` receives only one input when it expects two, or (b) silently treat `[0:a]` as matching all audio streams and produce unexpected results depending on the FFmpeg version. In most FFmpeg versions this will produce an error like "Too few inputs specified for amerge."

**Fix:**
```rust
cmd.arg("-filter_complex")
    .arg("[0:a:0][0:a:1]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]")
```


---

| 1 | **CRITICAL** | `audio.rs:70` | `amerge` filter uses `[0:a]` instead of `[0:a:0][0:a:1]` -- will fail or produce wrong output |

---

**The one item requiring immediate action is CRITICAL 1** -- the `amerge` filter will not work as written and will cause audio export failures whenever a 2-track MKV is involved. The fix is a single-line change from `[0:a]amerge=inputs=2,...` to `[0:a:0][0:a:1]amerge=inputs=2,...` at line 70 of `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/audio.rs`.

---

In the meantime, I can provide my review based on my existing knowledge of the whisper-rs 0.14 API, but I want to be transparent that one critical claim in the code -- the timestamp unit -- needs source-level verification. Let me proceed with what I know and flag the confidence level on each finding.

---

# Deep Review: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/local.rs`

## What Was Done Well

- Clean, well-structured code with proper separation of concerns
- Good error handling throughout -- every fallible whisper-rs call is mapped to a descriptive `String` error
- The `load_wav_samples` function correctly handles both integer and float WAV sample formats
- Special token filtering (lines 140-146) properly skips `[...]` and `<|...|>` tokens
- The `TranscriptionProvider` trait implementation is clean and fulfills the contract
- Model existence check before attempting load is good defensive programming

---


---

## CRITICAL Issues (Must Fix)

### 1. Timestamp unit is almost certainly WRONG (line 120-122, 149-150)

```rust
// whisper-rs times are in centiseconds (hundredths of a second).
let seg_start = t0 as f64 / 100.0;
```

The comment says "centiseconds" and divides by 100. However, in the whisper.cpp C API (which whisper-rs wraps), `whisper_full_get_segment_t0` and `whisper_full_get_segment_t1` return timestamps in **10-millisecond units** (also called "centiseconds" or "hundredths of a second" -- these are actually the same thing). So dividing by 100 converts to seconds, which is correct.

**However**, the `token_data.t0` and `token_data.t1` fields on `WhisperTokenData` are in a **different unit**. In whisper.cpp, `whisper_token_data::t0` and `t1` are defined as `int64_t` representing time in **milliseconds** when token timestamps are enabled, not in the same 10ms units as segment timestamps.

**Verdict**: Lines 149-150 divide `token_data.t0` and `token_data.t1` by 100.0, but if these are in milliseconds, the correct divisor is **1000.0**. This would cause word-level timestamps to be 10x too large. **I cannot verify this with 100% certainty without reading the source** -- this is the item that requires Bash/Glob/WebFetch permission to confirm. The segment-level timestamps (dividing by 100) are almost certainly correct.

**Confidence**: High that this is a bug, but I need source access to be definitive. Please verify by checking `WhisperTokenData` in the whisper-rs source or by running a test and inspecting whether word timestamps exceed segment timestamps.

**Recommendation**: Verify empirically or from source, and if confirmed, change lines 149-150:
```rust
start: token_data.t0 as f64 / 1000.0,  // if t0/t1 are milliseconds
end: token_data.t1 as f64 / 1000.0,
```

### 2. `set_language(None)` may not compile or behave as expected (line 86)

```rust
params.set_language(None); // auto-detect
```

In whisper-rs 0.14, `set_language` expects `Option<&str>`. Passing `None` should work for auto-detection. However, in some versions of whisper-rs, the function signature is `set_language(Some("auto"))` or `set_language(Some("en"))` etc., and `None` may not be the way to trigger auto-detect -- it might instead leave the language unset which could cause undefined behavior in whisper.cpp.

**Confidence**: Medium. The code compiles (presumably), so the type is correct. But the semantic behavior of `None` vs `Some("auto")` should be verified.

---


---

| 1 | CRITICAL | Token-level timestamp units may be wrong (divide by 100 vs 1000) | 149-150 |

---

| 2 | CRITICAL | `set_language(None)` semantic behavior needs verification | 86 |

---

**I was unable to verify the whisper-rs 0.14 API from source because Bash, Glob, Grep, WebFetch, WebSearch, and Context7 permissions were all denied.** The most critical finding -- issue #1 (token timestamp units) -- requires reading the `WhisperTokenData` struct definition in the whisper-rs source to confirm. 

To unblock this, please either:
1. Grant **Bash** permission so I can locate and read files in `~/.cargo/registry/src/`
2. Grant **Context7** or **WebFetch** permission for online docs
3. Or manually check: in the whisper-rs source, look at the `t0` and `t1` fields of `WhisperTokenData` and confirm whether they are in the same 10ms units as segment timestamps or in milliseconds

---

## CRITICAL

### C1. No `Drop` implementation for `RecordingPipeline` -- GStreamer pipeline resource leak

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`

`RecordingPipeline` holds a `gst::Pipeline` but has no `Drop` implementation. If a `RecordingPipeline` is dropped without `stop()` being called (e.g., on error during `start_recording`, or if `stop_recording` panics after `take()`), the GStreamer pipeline remains in `Playing` state and is never set to `Null`. This leaks GStreamer resources, file handles, and the PipeWire connection. The file being written to may also be left in a corrupt/incomplete state.

Similarly, in `start_recording` (commands.rs line 66-69), if `pipeline.start()` fails, the pipeline was already built and linked but is dropped without being set to `Null`.

**Recommendation:** Add a `Drop` impl:

```rust
impl Drop for RecordingPipeline {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
```

### C2. Multiple independent Mutex locks in `start_recording` -- inconsistent state on partial failure

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` (lines 72-73)

After `pipeline.start()` succeeds, three separate mutex locks are acquired sequentially:
```
*recorder.pipeline.lock()... = Some(pipeline);   // line 72
*recorder.capture.lock()...  = Some(capture);     // line 55 (earlier, only in video path)
*recorder.current_meeting_id.lock()... = Some(prepared.id.clone());  // line 73
```

If the lock on `current_meeting_id` (line 73) fails (poisoned), the pipeline is already stored but there is no meeting ID associated with it. A subsequent `stop_recording` would find the pipeline, stop it, but then fail at line 97 with "No meeting ID", losing the recording permanently -- it would never be finalized into the library.

**Recommendation:** Either use a single Mutex wrapping a struct with all three fields, or handle the error case by rolling back: if the meeting-ID lock fails, take the pipeline back out and stop it.

---


---

### CRITICAL issues (must fix)

**C1. Blob URL is leaked when `meeting` object changes identity but `id` and `media_status` stay the same (line 109)**

The effect dependency is `[meeting?.id, meeting?.media_status]`. However, `loadMeeting` (called after retranscribe, reindex, delete-media-only) does `setMeeting(data)` which creates a brand-new object. If `id` and `media_status` are unchanged, the effect does NOT re-run, so the blob URL from the previous render survives -- that part is fine. But the more dangerous scenario is the reverse: when the user calls `handleDelete("media_only")`, the backend deletes the media, `loadMeeting` fetches the new state where `media_status` becomes `"deleted"`, the effect re-runs (because `media_status` changed), the cleanup revokes the old URL, and `loadMedia` early-returns because `media_status !== "present"`. But `setMediaBlobUrl` is never called with `null` in this path, so the component still holds the now-revoked blob URL in state and passes it to the `<video>`/`<audio>` element. The media element will show a broken source.

**Fix:** Add `setMediaBlobUrl(null)` at the top of `loadMedia` before the early return, or in the cleanup function:

```ts
return () => {
  revoked = true;
  if (url) URL.revokeObjectURL(url);
  setMediaBlobUrl(null); // reset state when effect re-runs or unmounts
};
```

**C2. Entire media file is loaded into browser memory via `readFile` (line 93)**

`readFile` from `@tauri-apps/plugin-fs` reads the entire file into a `Uint8Array` in the webview's JS heap. For a meeting recording, this could easily be hundreds of megabytes or multiple gigabytes. This will cause the tab to freeze or crash on any non-trivial recording. The project's own design principle says "low disk usage" and "simplicity" -- holding the full file in RAM contradicts this.

**Fix:** Use Tauri's `convertFileSrc` (from `@tauri-apps/api`) or the asset protocol to get a streamable URL directly from the filesystem without loading the entire file into JS memory. This avoids the blob entirely:

```ts
import { convertFileSrc } from "@tauri-apps/api/core";
// ...
const streamUrl = convertFileSrc(mediaPath);
setMediaBlobUrl(streamUrl);
```

This eliminates the need for blob creation and revocation entirely.

**C3. Play/pause button icon never updates to reflect current state (line 291-298)**


---

The play button always shows the "play" triangle icon (`<path d="M8 5v14l11-7z" />`). There is no state tracking whether the media is playing or paused, and the SVG never switches to a pause icon. The user has no visual feedback about the current playback state. Since the media element is hidden (`className="hidden"` on line 333), this button is the sole interface for controlling playback -- making this a critical usability gap.

**Fix:** Add a `playing` state driven by the media element's `play`/`pause` events, and toggle the icon:

```ts
const [playing, setPlaying] = useState(false);

// In the media element:
<audio ref={setMediaRef} src={mediaBlobUrl}
  onPlay={() => setPlaying(true)}
  onPause={() => setPlaying(false)} />

// In the button icon:
{playing
  ? <path d="M6 4h4v16H6zM14 4h4v16h-4z" />   // pause bars
  : <path d="M8 5v14l11-7z" />}                  // play triangle
```

---


---

| C1 | Critical | Blob cleanup | Revoked blob URL stays in state after media deletion |

---

| C2 | Critical | Memory | Entire media file loaded into JS heap via `readFile` |

---

| C3 | Critical | UX | Play/pause icon never reflects actual playback state |

---

The three critical items (C1, C2, C3) should be addressed before this component is considered production-ready. C2 in particular (using `convertFileSrc` instead of `readFile`) would also eliminate C1 and I4 entirely, since there would be no blob URL to manage.

---

All Portuguese text in the app consistently lacks diacritical marks. Here are the affected strings:

| File:Line | Text | Correct Portuguese |
|-----------|------|-------------------|
| `App.tsx:76` | `reunioes` | `reunioes` (consistently unaccented) |
| `Gallery.tsx:62` | `reunioes` | `reunioes` |
| `Gallery.tsx:98` | `reuniao encontrada` / `reuniao gravada` | `reuniao` |
| `Gallery.tsx:114` | `reunioes` | `reunioes` |
| `MeetingPage.tsx:222` | `Reuniao nao encontrada` | `Reuniao nao` |
| `MeetingPage.tsx:401` | `exclusao` | `exclusao` |
| `ChatPanel.tsx:152` | `indexacao` | `indexacao` |
| `ChatPanel.tsx:154` | `reuniao` | `reuniao` |
| `ChatPanel.tsx:160` | `reuniao` | `reuniao` |
| `ChatPanel.tsx:216` | `Faca uma pergunta` | `Faca` |
| `ChatPanel.tsx:242` | `reuniao` | `reuniao` |
| `SettingsPage.tsx:24` | `Transcricao` | `Transcricao` |
| `SettingsPage.tsx:123,165` | `Configuracoes` | `Configuracoes` |
| `SettingsPage.tsx:147` | `configuracoes` | `configuracoes` |
| `SettingsPage.tsx:194` | `gravacoes` | `gravacoes` |
| `SettingsPage.tsx:208` | `automatico` | `automatico` |
| `SettingsPage.tsx:220` | `Variaveis disponiveis` | `Variaveis disponiveis` |
| `TranscriptView.tsx:70,84,105,136,168` | `Transcricao` | `Transcricao` |
| `TranscriptView.tsx:87` | `transcricao` | `transcricao` |
| `ExportDialog.tsx:184` | `Transcricao` | `Transcricao` |
| `RecordButton.tsx:148` | `reuniao` | `reuniao` |

This is consistent in that accents are missing everywhere -- no mixed accented/unaccented text was found. This appears to be a deliberate choice (ASCII-only Portuguese), so it is internally consistent but visually incorrect for Portuguese readers.

---

### 18. Mixed Label Language: "API Key" vs "Chave da API"

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx:622` -- `"API Key"` (English label) | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx:555` -- `"Chave da API"` (Portuguese label) | Same concept ("API key") uses English in one place and Portuguese in another. |
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx:673` -- `"Chave chat"` | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx:622` -- `"API Key"` | "Chave chat" is Portuguese, "API Key" is English -- for the same type of field. |

---

### 19. MeetingCard Glass Class vs MeetingPage Chat Button Glass Class

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingCard.tsx:48` -- `glass-card rounded-2xl` | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx:354` -- `glass-card rounded-2xl` (chat button) | These are consistent. |
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx:293` -- play button uses `glass-heavy` with `rounded-xl` | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx:357` -- chat icon container uses `bg-brand-500/10` with `rounded-xl` | Both are icon containers in rounded-xl boxes, but one uses `glass-heavy` and the other uses `bg-brand-500/10`. Different glass treatment for same element type (icon holder). |

---

### 20. Gallery Footer Height vs Gallery Header Height

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx:51` -- header `h-12` (48px) | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx:112` -- footer `h-10` (40px) | Header and footer have different heights (48px vs 40px). This may be intentional hierarchy, but they are the only component with a footer, making it a one-off. |

---

### 21. Icon Container Size Inconsistency

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/App.tsx:70` -- Gallery sidebar icon: `w-10 h-10 rounded-xl` with `w-5 h-5` icon | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx:293` -- Play button: `w-12 h-12 rounded-xl` with `w-5 h-5` icon | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx:357` -- Chat icon: `w-10 h-10 rounded-xl` with `w-5 h-5` icon | Three icon-in-rounded-box patterns with two different container sizes (`w-10 h-10` vs `w-12 h-12`) but all holding `w-5 h-5` icons. |

---

### 22. Focus Ring Inconsistency on Inputs

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx:70` -- search input: `focus:outline-none focus:border-white/20` | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx:247` -- chat input: `focus:outline-none focus:border-brand-500/30 focus:shadow-[0_0_0_3px_rgba(244,63,94,0.08)]` | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx:110` -- settings input: `focus:outline-none focus:border-white/20` | Search input focuses to `border-white/20`; Chat input focuses to `border-brand-500/30` with a brand-colored shadow ring; Settings focuses to `border-white/20`. Chat input has a distinctly different (branded) focus state. |

---

### 23. Spinner Size Consistency

All spinners across the app use the same `w-6 h-6 border-[3px] border-white/10 border-t-brand-500 rounded-full animate-[spin_0.7s_linear_infinite]` pattern. This is consistent -- good.

---

### 24. ExportDialog Bottom Padding vs ChatPanel Bottom Padding

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ExportDialog.tsx:209` -- `p-5` bottom bar | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx:238` -- `p-4` bottom bar | Both are footer action bars with `border-t border-white/5 shrink-0`, but ExportDialog uses `p-5` and ChatPanel uses `p-4`. |

---

### 25. "Tentar novamente" Button Inconsistency Across Components

| Location A | Location B | Issue |
|-----------|-----------|-------|
| `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx:89` -- `glass-heavy px-4 py-2 text-[11px] rounded-xl text-white/40 hover:text-white/70 ... border-0` | `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/TranscriptView.tsx:89` -- `glass-heavy px-4 py-2 text-[11px] rounded-xl text-white/40 hover:text-white/70 ... border-0 bg-transparent` | Almost identical, but TranscriptView adds explicit `bg-transparent` while Gallery does not. Minor, but technically different. |

---

### Prioritized Summary


---

**Critical (should fix for visual polish):**

1. **Error text uses 4 different font sizes** (`text-sm`, `text-xs`, `text-[11px]`, `text-[12px]`) and 2 different colors (`text-red-500` vs `text-red-400/80`). Pick one size and one color.

2. **Primary buttons have 3 different vertical paddings** (`py-2`, `py-2.5`, `py-3`), 2 different text sizes (`text-[11px]` vs `text-[12px]`), and 2 different background styles (solid vs gradient). Standardize.

3. **Audio/Video type badge** is `text-[8px]` in MeetingCard:70 but `text-[9px]` in MeetingPage:302.

4. **Status badges** have `uppercase` in MeetingPage:318,323 but not in MeetingCard:98,103.

5. **CHAT_BADGE `not_indexed`** is defined differently in MeetingCard:24 (hidden) vs MeetingPage:47 (shown as "Nao indexado"). Duplicated constant with divergent values.


---

15. **All Portuguese text** consistently omits diacritical marks (no cedilla, no tilde, no acute accents). This is internally consistent but linguistically incorrect for a PT-BR app. Consider adding proper accents if targeting Portuguese-speaking users.

---

### CRITICAL Issues (must fix)

**1. `duration_secs` type mismatch: TS `number` vs Rust `f64` -- OK in practice, but `file_size` is wrong: TS `number` vs Rust `u64`**

This is a **silent data corruption risk**. JavaScript's `number` is an IEEE 754 double, which can only represent integers exactly up to 2^53. Rust's `u64` goes up to 2^64. For `file_size`, recordings larger than ~9 PB would lose precision, which is practically impossible, so this is technically safe but worth documenting. No action needed.


---

**Verdict: No actual risk for this application. Not critical after all.**

**2. `get_transcription_status` return type mismatch**

- **TypeScript** (`api.ts` line 211): `Promise<string>`
- **Rust** (`transcription/commands.rs` line 131-138): `Result<TranscriptionStatus, String>`

The Rust command returns `TranscriptionStatus`, which is a `#[serde(rename_all = "snake_case")]` enum. When serialized by Tauri, this becomes a JSON string like `"pending"`, `"processing"`, `"done"`, or `"failed"`. So `Promise<string>` will work at runtime, but the TypeScript type loses the union constraint. The return type should be:

```typescript
Promise<"pending" | "processing" | "done" | "failed">
```

Or better yet, reuse the `TranscriptionStatus` type already defined within `MeetingSummary`. This would give callers proper type narrowing.

**3. `get_chat_status` return type mismatch**

- **TypeScript** (`api.ts` line 251): `Promise<string>`
- **Rust** (`rag/commands.rs` line 196-221): `Result<ChatStatus, RagCommandError>`

Same issue. The Rust returns `ChatStatus` enum, which serializes to `"not_indexed"`, `"indexing"`, `"ready"`, or `"failed"`. The return type should be:

```typescript
Promise<"not_indexed" | "indexing" | "ready" | "failed">
```

**4. `export_audio` / `export_video` / `export_transcript` return type mismatch**

- **TypeScript** (`api.ts` lines 271-281): All return `Promise<string>`
- **Rust** (`export/commands.rs`): All return `Result<PathBuf, ExportError>`

`PathBuf` serializes as a string via serde, so it works at runtime. However, the semantic intent is a file path, not an arbitrary string. This is a minor typing concern, not a runtime bug. Still, documenting it via a type alias like `type FilePath = string` would improve clarity.

---


---

| 2 | Critical | `api.ts:211` | `getTranscriptionStatus` return type should be `TranscriptionStatus` union, not `string` |

---

| 3 | Critical | `api.ts:251` | `getChatStatus` return type should be `ChatStatus` union, not `string` |

---

### CRITICAL Issues

#### 1. `amerge` filter is hardcoded to `inputs=2` but stream count can be > 2

**Line 70:**
```
[0:a]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]
```

The code checks `stream_count >= 2` on line 67, meaning this branch fires for 2, 3, 4, ... streams. But the `amerge` filter's `inputs=` parameter **must match the actual number of input streams** fed to it. When `inputs=2` is specified but the file contains 3 or more audio streams, ffmpeg will either silently ignore the extra streams (merging only the first two) or error out depending on the version.

**Fix:** Interpolate the actual stream count into the filter string:
```rust
let filter = format!(
    "[0:a]amerge=inputs={},pan=stereo|c0<c0+c1|c1<c0+c1[aout]",
    stream_count
);
cmd.arg("-filter_complex").arg(&filter);
```

#### 2. `amerge` input pad syntax is wrong for multiple streams from a single file

**Line 70:**
```
[0:a]amerge=inputs=2,...
```

The `amerge` filter requires **separate labeled input pads**, one per stream. The shorthand `[0:a]` as a single input to amerge does not correctly feed multiple distinct streams into the filter when `inputs` > 1. The correct syntax requires each stream to be addressed individually:

For 2 streams:
```
[0:a:0][0:a:1]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]
```

For N streams, each stream `[0:a:0]`, `[0:a:1]`, ..., `[0:a:N-1]` must be listed as separate inputs to the filter.

**Fix:** Dynamically build the input pad labels:
```rust
let inputs: String = (0..stream_count)
    .map(|i| format!("[0:a:{}]", i))
    .collect();
let filter = format!(
    "{}amerge=inputs={},pan=stereo|c0<c0+c1|c1<c0+c1[aout]",
    inputs, stream_count
);
```

#### 3. `pan` filter channel mapping is incorrect for > 2 input channels

**Line 70:**
```
pan=stereo|c0<c0+c1|c1<c0+c1
```

The `pan` filter's channel references (`c0`, `c1`) refer to the **input channels of the amerge output**. When amerge combines N mono streams, the output has N channels (c0 through cN-1). The current formula `c0<c0+c1|c1<c0+c1` only sums the first two input channels and ignores any beyond that.

For a meeting recorder that captures multiple participants, each as a separate mono audio stream, the pan formula must sum **all** input channels.

**Fix:** Dynamically build the pan spec:
```rust
let all_channels: String = (0..stream_count)
    .map(|i| format!("c{}", i))
    .collect::<Vec<_>>()
    .join("+");
let filter = format!(
    "{}amerge=inputs={},pan=stereo|c0<{}|c1<{}[aout]",
    inputs, stream_count, all_channels, all_channels
);
```

This produces e.g. `pan=stereo|c0<c0+c1+c2|c1<c0+c1+c2` for 3 streams, which mixes all participants into both stereo channels equally.

---


---

| 1 | CRITICAL | `amerge=inputs=2` hardcoded; fails or drops streams when count > 2 |

---

| 2 | CRITICAL | `[0:a]amerge` is wrong pad syntax; each stream needs its own `[0:a:N]` label |

---

| 3 | CRITICAL | `pan` formula only references c0+c1, ignoring channels beyond the first two |

---

### WRONG (Critical): `CreateSessionOptions` import location

Line 2 imports `CreateSessionOptions` from `ashpd::desktop`:
```rust
use ashpd::desktop::{CreateSessionOptions, PersistMode};
```

In ashpd 0.13, `CreateSessionOptions` was removed as a standalone type. The `create_session()` method on the `Screencast` proxy no longer takes options -- it simply creates a session directly. The correct call in 0.13 is:
```rust
let session = proxy.create_session().await?;
```
without any arguments. The `CreateSessionOptions` struct existed in ashpd 0.10 but was removed in the 0.11+ rewrite.


---

### WRONG (Critical): `SelectSourcesOptions` builder methods

Line 33-37 builds `SelectSourcesOptions` with chained setters:
```rust
let select_sources_options = SelectSourcesOptions::default()
    .set_cursor_mode(CursorMode::Embedded)
    .set_sources(SourceType::Monitor | SourceType::Window)
    .set_multiple(false)
    .set_persist_mode(PersistMode::DoNot);
```

In ashpd 0.13, the `SelectSourcesOptions` does NOT have `set_` prefixed methods. The builder methods are named without the `set_` prefix:
```rust
SelectSourcesOptions::default()
    .cursor_mode(CursorMode::Embedded)
    .types(SourceType::Monitor | SourceType::Window)
    .multiple(false)
    .persist_mode(PersistMode::DoNot)
```
Also note: the method is `types()` not `sources()`.


---

### WRONG (Critical): `select_sources` and `start` method signatures

Lines 39-45:
```rust
proxy.select_sources(&session, select_sources_options).await?;
let response = proxy.start(&session, None, StartCastOptions::default()).await?;
```

In ashpd 0.13, the `select_sources` method signature changed. The screencast proxy methods use a builder pattern on the session itself rather than passing options as a second argument. Additionally, `start()` does not take `StartCastOptions` -- it takes an optional window identifier. The 0.13 API is:
```rust
proxy.select_sources(&session).types(SourceType::Monitor | SourceType::Window)
    .cursor_mode(CursorMode::Embedded)
    .multiple(false)
    .send().await?;

let response = proxy.start(&session, None::<&WindowIdentifier>).await?;
```


---

### WRONG (Critical): `response.response()` and `.streams()`

Lines 47-51:
```rust
let streams_response = response.response()?;
let stream = streams_response.streams().first()?;
```

In ashpd 0.13, `start()` returns the streams directly (a `Vec<Stream>`), not a response wrapper that needs `.response()` unwrapping. The correct pattern is:
```rust
let streams = proxy.start(&session, None::<&WindowIdentifier>).await?;
let stream = streams.first().ok_or("No stream")?;
```

### CORRECT: `open_pipe_wire_remote` and `pipe_wire_node_id()`
These method names are correct in 0.13.


---

**Verdict: WRONG** -- The ashpd usage follows the 0.10 API, not the 0.13 API. This code will not compile with ashpd 0.13.5. This is the most critical finding.

---

## 3. whisper-rs (0.14.4)

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/local.rs`

### CORRECT: WhisperContext::new_with_params
```rust
WhisperContext::new_with_params(path, WhisperContextParameters::default())
```
This is the correct 0.14 API. The older `WhisperContext::new(path)` was removed in favor of `new_with_params`.

### CORRECT: create_state and full
```rust
let mut state = ctx.create_state()?;
state.full(params, &samples)?;
```
In whisper-rs 0.14, inference runs on a `WhisperState` object created from the context, and `full()` takes `FullParams` and a `&[f32]` sample buffer. This is correct.

### CORRECT: FullParams construction
```rust
FullParams::new(SamplingStrategy::Greedy { best_of: 1 })
```
This is the correct constructor for 0.14.

### CORRECT: FullParams setter methods
`set_print_progress`, `set_print_realtime`, `set_print_timestamps`, `set_token_timestamps`, `set_language` -- all correct for 0.14.


---

| ashpd | 0.13.5 | **WRONG (Critical)** | Uses 0.10-era API: `CreateSessionOptions`, `set_` prefixed builders, `StartCastOptions`, `response.response()` |
| whisper-rs | 0.14.4 | **CORRECT** | Context, State, FullParams, segment/token extraction all correct |
| rusqlite | 0.32.1 | **CORRECT** | Connection, execute, query, params!, load_extension, unchecked_transaction all correct |
| reqwest | 0.12.28 | **CORRECT** | blocking + async client, multipart, streaming, json all correct |
| hound | 3.5 | **CORRECT** | WavReader, spec, sample format handling correct |
| genpdf | 0.2.0 | **CORRECT** | Document, fonts, elements, render_to_file all correct |
| chrono | 0.4 | **CORRECT** | DateTime<Utc>, serde, RFC 3339 conversion correct |
| motion | latest | **CORRECT** | Import path, AnimatePresence, motion.div all correct |

---


---

## Critical Issues (Must Fix)

**1. ashpd 0.13 API mismatch in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/capture.rs`**

The entire `request_screen` method uses the ashpd 0.10 API surface. With ashpd 0.13.5 (as resolved in Cargo.lock), this code will fail to compile. The required changes are:

- Remove `CreateSessionOptions` import -- it no longer exists
- Remove `StartCastOptions` import -- it no longer exists
- `PersistMode` may have moved to the screencast module
- `create_session()` takes no arguments
- `select_sources` uses a builder pattern on the proxy, not a separate options struct
- `start()` returns streams directly, not a response wrapper
- Builder methods do not have `set_` prefixes; `set_sources()` is now `types()`

This is the only compilation-blocking issue found across all libraries.

---


---

#### CRITICAL -- Issue 1: `chunk_size = 0` causes an infinite-like loop or produces one chunk per segment with no guard

On line 43, the condition is `current_tokens > 0 && current_tokens + seg_tokens > chunk_size`. If `chunk_size` is `0`, then after the first segment is accumulated (`current_tokens > 0`), every subsequent segment triggers a flush because `current_tokens + seg_tokens > 0` is always true. This is technically not an infinite loop, but it silently produces one chunk per segment, which is likely a misuse the function should reject or at least document. More dangerously, `chunk_size` comes from user-configurable settings (`RagSettings`), so a `0` value is plausible.

**Recommendation:** Add a guard at the top of `chunk_transcript`:

```rust
assert!(chunk_size > 0, "chunk_size must be greater than zero");
// or: if chunk_size == 0 { return Vec::new(); }
```

---


---

| 1 | **Critical** | `chunk_size = 0` not guarded; produces degenerate output from user-configurable input |

---

### CRITICAL Issues (must fix)

**C1. TOML deserialization is all-or-nothing -- adding a new field breaks existing configs**

Every field on every struct lacks `#[serde(default)]`. When a user already has a `config.toml` on disk and you add a new field (e.g., a future `theme` field to `GeneralSettings`), `toml::from_str` will return `Err(toml::de::Error)` and the app will fail to load settings. This is not a hypothetical problem -- it will bite on any schema evolution.

The fix is to add `#[serde(default)]` at the struct level for each settings struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSettings { ... }
```

This tells serde to fill any missing fields from the `Default` impl (which you already provide in `defaults.rs`). Apply this to all six structs (`AppSettings`, `GeneralSettings`, `AudioSettings`, `VideoSettings`, `TranscriptionSettings`, `RagSettings`).

**C2. `api_key` fields are serialized in plain text to a world-readable file**

`TranscriptionSettings::api_key`, `RagSettings::embeddings_api_key`, and `RagSettings::chat_api_key` are stored as plain strings in `~/.config/hlusra/config.toml`. There is no file permission restriction (`fs::write` creates files with the default umask, typically `0644`). At minimum:
- Set file permissions to `0600` after writing (on Unix).
- Better yet, consider using the OS keychain or at least marking these fields with `#[serde(skip_serializing_if = "String::is_empty")]` so empty keys are not written.

---


---

| C1 | Critical | No `#[serde(default)]` -- any schema evolution breaks existing configs |

---

| C2 | Critical | API keys stored in plain text with no file permission restriction |

---

### CRITICAL Issues

**1. "large" model URL resolves to a 404 -- download will always fail**

The catalogue in `types.rs` defines the model name as `"large"`, which produces the download URL `https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin`. That file does not exist on Hugging Face. The repository only contains versioned large models: `ggml-large-v1.bin`, `ggml-large-v2.bin`, `ggml-large-v3.bin`, and `ggml-large-v3-turbo.bin`.

The HTTP status check on line 65 will catch this and return an error, so it will not corrupt anything, but the "large" model is effectively un-downloadable. You need to decide which large variant(s) to support and update the catalogue accordingly (e.g., `"large-v3"` producing `ggml-large-v3.bin`). Consider also adding `large-v3-turbo` as it is significantly smaller and faster than the full v3.

Files involved:
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/types.rs` lines 49-57 (`all_models()`)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/models.rs` line 59 (consumes `download_url()`)

**2. `.part` file is never cleaned up on failure**

If the download fails mid-stream (network error, disk full, process killed), the `.part` file is left on disk. On the next call to `download_model`, a new `.part` file is created (truncating the old one), so this is not a correctness bug for retries. However, if the process exits after a partial download but before the error path completes, a multi-gigabyte orphan `.part` file remains on disk permanently -- contrary to the project's "low disk usage" goal.

The fix is straightforward: clean up the `.part` file on any error after it has been created. For example:

```rust
let result = (|| {
    let mut response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to start download for {model_name}: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("Download failed for {model_name}: HTTP {}", response.status()));
    }
    let mut file = fs::File::create(&part_path)
        .map_err(|e| format!("Failed to create temp file: {e}"))?;
    io::copy(&mut response, &mut file)
        .map_err(|e| format!("Failed to download {model_name}: {e}"))?;
    Ok(())
})();

if let Err(e) = &result {
    let _ = fs::remove_file(&part_path); // best-effort cleanup
    return Err(e.clone());
}

fs::rename(&part_path, &dest)
    .map_err(|e| format!("Failed to finalize model file: {e}"))?;
```

**3. `set_active_model` uses a hardcoded filename format instead of `WhisperModel::filename()`**

Line 107 constructs the filename with an inline `format!("ggml-{model_name}.bin")` rather than looking up the model in the catalogue and calling `model.filename()`. This duplicates the naming convention and will silently diverge if `filename()` ever changes (e.g., to support quantized variants like `ggml-small-q5_1.bin`). More immediately, it bypasses the catalogue validation -- you could pass `model_name = "../../etc/passwd"` and the function would check for the existence of an arbitrary path.

Recommended fix: look up the model in `all_models()` first (just like `download_model` does), then use `model.filename()`:

```rust
pub fn set_active_model(model_name: &str) -> Result<(), String> {
    let catalogue = all_models();
    let model = catalogue
        .iter()
        .find(|m| m.name == model_name)
        .ok_or_else(|| format!("Unknown model: {model_name}"))?;

    let dir = models_dir()?;
    if !dir.join(model.filename()).exists() {
        return Err(format!("Model '{model_name}' has not been downloaded yet"));
    }

    let active_file = dir.join(ACTIVE_MODEL_FILE);
    fs::write(&active_file, model_name)
        .map_err(|e| format!("Failed to persist active model selection: {e}"))?;
    Ok(())
}
```

---


---

| 1 | **Critical** | `"large"` model URL is a 404 -- `ggml-large.bin` does not exist on HF |

---

| 2 | **Critical** | `.part` file never cleaned up on download failure |

---

| 3 | **Critical** | `set_active_model` uses hardcoded filename format, bypasses catalogue (path traversal risk) |

---

## CRITICAL Issues

### 1. `init_vector_table` is NOT transactional -- data loss risk

Lines 181-207. This method performs four distinct mutations (DROP, DELETE, CREATE, two SET_META calls) without wrapping them in a transaction. If the process crashes after `DELETE FROM chunks` but before `CREATE VIRTUAL TABLE`, the database is left with no chunks AND no virtual table -- a corrupted state with silent data loss.

```rust
// Current: each statement commits independently
self.conn.execute_batch("DROP TABLE IF EXISTS chunks_vec;")?;
self.conn.execute_batch("DELETE FROM chunks;")?;
// ... crash here = data gone, no virtual table
let create_sql = format!("CREATE VIRTUAL TABLE ...");
self.conn.execute_batch(&create_sql)?;
self.set_meta("embedding_model", model)?;
self.set_meta("embedding_dimension", &dimension.to_string())?;
```

**Fix**: Wrap the entire method body in a transaction. Note: sqlite-vec virtual tables support transactional DDL within SQLite, so `DROP TABLE IF EXISTS` and `CREATE VIRTUAL TABLE` inside a transaction is valid.

```rust
let tx = self.conn.unchecked_transaction()?;
tx.execute_batch("DROP TABLE IF EXISTS chunks_vec;")?;
tx.execute_batch("DELETE FROM chunks;")?;
tx.execute_batch(&create_sql)?;
tx.execute("INSERT OR REPLACE INTO rag_meta ...", ...)?;
tx.execute("INSERT OR REPLACE INTO rag_meta ...", ...)?;
tx.commit()?;
```

### 2. `insert_chunk` (singular) is NOT transactional -- chunks/vec table desync

Lines 237-265. The single-chunk insert writes to `chunks` and then `chunks_vec` as two independent statements. If the second `INSERT` fails (e.g., sqlite-vec extension not loaded, dimension mismatch), the row exists in `chunks` but not in `chunks_vec`. This creates a phantom chunk: it will never appear in vector search results, but `is_meeting_indexed` returns `true`.

**Fix**: Wrap in a transaction:

```rust
pub fn insert_chunk(&self, chunk: &Chunk, embedding: &[f32]) -> Result<(), VectorStoreError> {
    let embedding_blob = embedding_to_blob(embedding);
    let tx = self.conn.unchecked_transaction()?;
    tx.execute("INSERT OR REPLACE INTO chunks ...", params![...])?;
    tx.execute("INSERT OR REPLACE INTO chunks_vec ...", params![...])?;
    tx.commit()?;
    Ok(())
}
```

### 3. Search query has incorrect sqlite-vec syntax -- `WHERE k = ?2` after `AND`

Lines 345-354. The query is:

```sql
WHERE v.embedding MATCH ?1
  AND k = ?2
  AND c.meeting_id = ?3
```

In sqlite-vec's `vec0` virtual table, the `k` parameter is a **constraint on the virtual table itself**, not a regular SQL column filter. When you add `AND c.meeting_id = ?3` as an additional WHERE clause after the vec0 constraints, SQLite's query planner may or may not push the `c.meeting_id` filter down correctly, depending on how the JOIN is structured.

The more significant problem: the `INNER JOIN` with a filter on `c.meeting_id` **after** the vec0 `MATCH` means sqlite-vec will return `fetch_limit` nearest neighbors across ALL meetings, and then the JOIN filters out non-matching ones. If the target meeting has very few chunks and the database has many meetings, you could get **zero results** even when relevant chunks exist, because all `fetch_limit` results came from other meetings.

The current mitigation (`fetch_limit = top_k * 10`) is fragile. If a meeting has 5 chunks but the database has 50,000 chunks from other meetings, a 10x over-fetch is nowhere near sufficient.

**Fix options**:
- (A) Pre-filter by retrieving chunk_ids for the meeting, then query `chunks_vec` with those IDs only. This is the most reliable approach.
- (B) Use a much larger over-fetch ratio or remove the limit on the vec0 side entirely and rely solely on the outer LIMIT. But this is inefficient.
- (C) Create per-meeting virtual tables (over-engineered for this use case).

Recommended approach (A):

```rust
pub fn search(
    &self,
    meeting_id: &str,
    query_embedding: &[f32],
    top_k: usize,
) -> Result<Vec<Chunk>, VectorStoreError> {
    let query_blob = embedding_to_blob(query_embedding);

    // Get total chunk count for this meeting to use as k
    let meeting_chunk_count: i64 = self.conn.query_row(
        "SELECT COUNT(*) FROM chunks WHERE meeting_id = ?1",
        params![meeting_id],
        |row| row.get(0),
    )?;

    // Ask vec0 for all chunks from this meeting (worst case), then take top_k
    let sql = "SELECT c.id, c.meeting_id, c.text, c.start_time, c.end_time, c.chunk_index
         FROM chunks_vec v
         INNER JOIN chunks c ON c.id = v.chunk_id
         WHERE v.embedding MATCH ?1
           AND k = ?2
           AND c.meeting_id = ?3
         ORDER BY v.distance
         LIMIT ?4";

    // Use total DB chunk count for k, not a small multiple
    // ... or restructure the query to avoid vec0 k limit entirely
}
```

The cleanest fix is to query the total number of chunks in the DB and use that as `k`, then filter and LIMIT in the outer query. This guarantees no relevant results are lost.

---


---

| 1 | CRITICAL | `init_vector_table` not wrapped in a transaction -- data loss on crash |

---

| 2 | CRITICAL | `insert_chunk` (singular) not transactional -- chunks/vec desync |

---

| 3 | CRITICAL | Search query over-fetch strategy is unreliable -- can return zero results for valid meetings |

---

### CRITICAL Issues

**1. `from_str` methods shadow the `FromStr` trait -- silent, confusing, and non-idiomatic (lines 134, 156, 180)**

All three status enums define an inherent method `pub fn from_str(s: &str) -> Self`. This is the exact signature of the standard `std::str::FromStr` trait, but it is *not* a trait implementation. This causes multiple problems:

- It prevents ever `impl FromStr` for these types (the compiler won't error, but callers using `.parse::<MediaStatus>()` won't find these methods).
- It silently swallows invalid input by defaulting (e.g., `MediaStatus::Present` for garbage input). The `FromStr` trait returns `Result`, which is the correct approach.
- The `eprintln!` "WARNING" on unknown values (lines 139, 163, 187) means corrupted data goes undetected in production. A bad database row will silently become `Present`/`Pending`/`NotIndexed` instead of surfacing an error.

Recommendation: Replace these with proper `impl FromStr for MediaStatus` (and the other two), returning `Result<Self, SomeError>`. Callers in `db.rs` (lines 117-119, 189-191) should propagate the error rather than silently defaulting.

**2. `ArtifactKind` is missing `#[serde(rename_all = "snake_case")]` (line 102-108)**

Every other enum in this file has `#[serde(rename_all = "snake_case")]`, but `ArtifactKind` does not. This means its serde representation uses PascalCase: `"Recording"`, `"TranscriptJson"`, etc. While `ArtifactKind` is currently only used server-side (never serialized over the Tauri command boundary), this inconsistency is a latent bug -- the moment it appears in a command argument or return type, the frontend will need to send PascalCase while every other enum uses snake_case. Either add the attribute for consistency, or add a comment explaining the deliberate omission.

---


---

| 1 | CRITICAL | `from_str` shadows `FromStr` trait, silently swallows bad data |

---

| 2 | CRITICAL | `ArtifactKind` missing `#[serde(rename_all = "snake_case")]` (inconsistent with all other enums) |

---

## CRITICAL Issues (must fix -- will not compile or will crash)

### C1. `SelectSourcesOptions` builder methods use wrong names

**Lines 33-37:**
```rust
let select_sources_options = SelectSourcesOptions::default()
    .set_cursor_mode(CursorMode::Embedded)
    .set_sources(SourceType::Monitor | SourceType::Window)
    .set_multiple(false)
    .set_persist_mode(PersistMode::DoNot);
```

In ashpd 0.13, `SelectSourcesOptions` uses a **builder pattern without `set_` prefixes**. The correct method names are:

- `cursor_mode(...)` -- not `set_cursor_mode(...)`
- `types(...)` -- not `set_sources(...)`
- `multiple(...)` -- not `set_multiple(...)`
- `persist_mode(...)` -- not `set_persist_mode(...)`

Additionally, the method to set source types is called `types`, not `sources` or `set_sources`.

**Fix:**
```rust
let select_sources_options = SelectSourcesOptions::default()
    .cursor_mode(CursorMode::Embedded)
    .types(SourceType::Monitor | SourceType::Window)
    .multiple(false)
    .persist_mode(PersistMode::DoNot);
```

### C2. `CreateSessionOptions` does not exist in `ashpd::desktop`

**Line 2 and 29:**
```rust
use ashpd::desktop::{CreateSessionOptions, PersistMode};
// ...
let session = proxy.create_session(CreateSessionOptions::default())
```

In ashpd 0.13, `Screencast::create_session()` takes **no arguments** -- it is a parameterless method. There is no `CreateSessionOptions` type. The `create_session` signature is simply:

```rust
pub async fn create_session(&self) -> Result<Session<'_, Self>, Error>
```

**Fix:**
```rust
// Remove CreateSessionOptions from the import on line 2
use ashpd::desktop::PersistMode;

// Line 29: call with no arguments
let session = proxy.create_session()
    .await
    .map_err(|e| format!("Failed to create session: {}", e))?;
```

### C3. `StartCastOptions` does not exist; `start()` takes only 2 arguments

**Line 1 and 43:**
```rust
use ashpd::desktop::screencast::{..., StartCastOptions};
// ...
let response = proxy.start(&session, None, StartCastOptions::default())
```

In ashpd 0.13, the `Screencast::start()` method signature does not take a `StartCastOptions` parameter. Its signature is:

```rust
pub async fn start(&self, session: &Session<'_, Self>, identifier: Option<&WindowIdentifier>) -> Result<Request<Vec<Stream>>, Error>
```

It takes only the session and an optional window identifier. There is no `StartCastOptions` type in ashpd 0.13.

**Fix:**
```rust
// Remove StartCastOptions from the import on line 1
use ashpd::desktop::screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType};

// Line 43-45:
let response = proxy.start(&session, None)
    .await
    .map_err(|e| format!("Failed to start: {}", e))?;
```

### C4. `start()` return type handling -- `response.response()` is wrong

**Lines 43-50:**
```rust
let response = proxy.start(&session, None, StartCastOptions::default())
    .await
    .map_err(|e| format!("Failed to start: {}", e))?;

let streams_response = response.response()
    .map_err(|e| format!("Failed to get response: {}", e))?;

let stream = streams_response.streams().first()
```

In ashpd 0.13, `start()` returns `Result<Request<Vec<Stream>>, Error>`. The `Request<T>` must be `.await`ed or `.response().await`ed to get the portal response. Then the result is a `Vec<Stream>` directly. There is no `.streams()` method on the response -- the response **is** the Vec of streams.

However, the exact handling depends on how `Request` is used. In ashpd 0.13, `Request<T>` implements `Future`, so you can `.await` it to get `Result<T, Error>`. Alternatively you call `response().await`. The current code calls `.response()` synchronously (no `.await`), which would be a compile error.

**Fix:**
```rust
let streams = proxy.start(&session, None)
    .await
    .map_err(|e| format!("Failed to start: {}", e))?
    .response()
    .await
    .map_err(|e| format!("Failed to get response: {}", e))?;

let stream = streams.first()
    .ok_or("No stream returned from portal")?;
```

Alternatively, if `Request` in ashpd 0.13 resolves directly:
```rust
let streams = proxy.start(&session, None)
    .await
    .map_err(|e| format!("Failed to start: {}", e))?;

let stream = streams.first()
    .ok_or("No stream returned from portal")?;
```

You should verify which pattern works in 0.13.5 -- the key point is the current code is certainly wrong because it calls `.response()` without `.await` and then calls `.streams()` which does not exist as a method on `Vec<Stream>`.

---


---

| C1 | Critical | 33-37 | `SelectSourcesOptions` builder methods have wrong names (`set_` prefix, `set_sources` vs `types`) |

---

| C2 | Critical | 2, 29 | `CreateSessionOptions` does not exist; `create_session()` takes no args |

---

| C3 | Critical | 1, 43 | `StartCastOptions` does not exist; `start()` takes 2 args not 3 |

---

| C4 | Critical | 47-50 | `response.response()` missing `.await`, `.streams()` does not exist on result type |

---

**Bottom line:** The four Critical issues (C1-C4) mean this file almost certainly does not compile against ashpd 0.13.5. The API method names, type names, and call signatures are all inconsistent with the ashpd 0.13 public API. These must be fixed before the code can work.

**Caveat:** I was unable to verify against the actual ashpd 0.13.5 source code or docs.rs (all external verification tools were denied). My analysis is based on strong familiarity with ashpd 0.13 from training data. I recommend running `cargo check` to confirm which issues produce actual compilation errors, and consulting `docs.rs/ashpd/0.13.5` for the definitive API surface.

---

## CRITICAL Issues


---

### 1. CRITICAL: `chat_message` event race -- listeners registered AFTER the command is invoked

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx`, lines 90-118

The `handleSend` function first sets up `listen()` calls (lines 90-107), then invokes `chatMessage()` (line 118). However, `listen()` is asynchronous -- it returns a Promise. All three listeners ARE awaited before `chatMessage` is called, so the ordering is actually: register chunk listener, register done listener, register error listener, then invoke the command.

The real problem is subtler: the Rust `chat_message` command (at `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/commands.rs` line 133) runs asynchronously. It calls `emit_to("main", ...)` for each streaming chunk. But the command is only invoked at line 118 (`await chatMessage(...)`) AFTER listeners are registered, so this is actually fine for the initial registration. However, there is a race condition on the `streamFinished` promise:

```typescript
const unlistenDone = await listen<void>("chat-stream-done", () => {
  resolveDone();
});
```

If the chat response is extremely fast (e.g., an error from the API), the `chat-stream-done` or `chat-stream-error` event could be emitted by the backend before the `chatMessage` invoke even returns, but since the listeners are registered before the invoke, this should be fine.


---

**Revised assessment:** This is actually correctly ordered. Downgrading from CRITICAL.


---

### 2. CRITICAL: `export_audio`/`export_video`/`export_transcript` are blocking synchronous commands that will freeze the UI

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/commands.rs`, lines 13, 31, 51
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ExportDialog.tsx`, line 68

All three export commands are declared as synchronous (`pub fn`, not `pub async fn`). In Tauri v2, synchronous commands block the main thread. These commands call FFmpeg for encoding, which can take a long time. During this time, the entire application UI will be frozen and unresponsive. The user sees "Exportando..." but cannot interact with the window at all. On some platforms/WMs, the OS may mark the window as "not responding."

**Recommendation:** Make these commands `async` and use `tokio::task::spawn_blocking` to run the heavy FFmpeg work, similar to what `download_model` and `transcribe_meeting` already do.


---

### 3. CRITICAL: `stop_recording` is a blocking synchronous command that waits up to 5 seconds for EOS

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs`, line 79
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs`, lines 176-203

`stop_recording` is `pub fn` (synchronous). Inside, `pipeline.stop()` calls `bus.timed_pop_filtered()` with a 5-second timeout. This blocks the Tauri main thread for up to 5 seconds, freezing the UI completely while the user sees "Parando...".

**Recommendation:** Make `stop_recording` async and move the EOS wait into a `spawn_blocking` task.

---


---

| CRITICAL | 2 | Blocking sync commands freeze UI (exports, stop_recording) |

---

The two critical items are the most impactful: `export_audio`, `export_video`, `export_transcript`, and `stop_recording` are all synchronous Tauri commands that perform heavy blocking I/O (FFmpeg encoding, GStreamer EOS wait). These will freeze the entire application UI for seconds to minutes. Converting them to async commands with `spawn_blocking` -- following the pattern already used by `transcribe_meeting` and `download_model` -- would fix this.


---

## CRITICAL Issues (must fix)

### 1. `renderView()` return type is `undefined` for exhaustiveness -- silent at runtime

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, line 47-138

The `switch` in `renderView()` has no `default` case. TypeScript currently infers the return type as `JSX.Element | undefined`. If a future developer adds a new value to the `ViewKind` union and forgets to add a case, `renderView()` will silently return `undefined`, and the `motion.div` will render nothing with no error.

**Recommendation:** Add exhaustiveness checking:

```typescript
default: {
  const _exhaustive: never = view;
  return _exhaustive;
}
```

This will produce a compile-time error if any `ViewKind` value is unhandled.

### 2. Window resize races and missing cleanup

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, lines 37-45

```typescript
useEffect(() => {
  (async () => {
    try {
      const win = getCurrentWindow();
      const [w, h] = HOME_VIEWS.includes(view) ? [800, 400] : [800, 600];
      await win.setSize(new LogicalSize(w, h));
    } catch {}
  })();
}, [view]);
```

Two problems here:

**(a) Race condition:** When the user navigates rapidly (e.g., `home -> gallery -> meeting`), two async IIFE calls will be in-flight concurrently. The `setSize(800,600)` for "gallery" could resolve *after* the `setSize(800,600)` for "meeting", which is harmless by coincidence in this case but architecturally fragile. If the sizes ever differ between non-home views, this becomes a real visual bug.

**(b) Silent error swallowing:** The `catch {}` block discards every error unconditionally. If `setSize` fails (e.g., the Tauri window handle is invalid, or the app runs in a test context), you will have zero diagnostic information.

**Recommendation:** Use an `AbortController`-style cancelled flag and at minimum log errors:

```typescript
useEffect(() => {
  let cancelled = false;
  const win = getCurrentWindow();
  const [w, h] = HOME_VIEWS.includes(view) ? [800, 400] : [800, 600];
  win.setSize(new LogicalSize(w, h)).catch((e) => {
    if (!cancelled) console.warn("[App] setSize failed:", e);
  });
  return () => { cancelled = true; };
}, [view]);
```

### 3. `meetingId` ref is stale when navigating to `chat` or `export` from a second meeting

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, lines 25, 31-34, 103-122

`meetingId` is stored as a `useRef`, and it is set inside `go()` only when the caller passes an `id` argument. The problem is on lines 108, 109, 119:

```typescript
onChat={(ctx) => { setMeetingCtx(ctx); go("chat"); }}
onExport={(ctx) => { setMeetingCtx(ctx); go("export"); }}
onBack={() => go("meeting")}   // in ChatPanel
```

When `go("chat")` is called, no `id` is passed, so `meetingId.current` retains its previous value. This *works* today because the `id` was already set when navigating to "meeting". However, when `ChatPanel`'s `onBack` calls `go("meeting")`, it also passes no `id`. If a future code path navigates directly to "chat" without going through "meeting" first, `meetingId.current` will be stale or empty.


---

| 1 | CRITICAL | No exhaustiveness check in `renderView()` switch | 47-138 |

---

| 2 | CRITICAL | Window resize race condition + silent error swallowing | 37-45 |

---

| 3 | CRITICAL | `meetingId` as ref is fragile, no guarantees against stale reads | 25, 31-34 |

---

### CRITICAL

**1. Missing `file_name()` on the multipart Part -- OpenAI API will reject the request**

Lines 65-66:
```rust
let file_part = multipart::Part::file(audio_path)
    .map_err(|e| format!("Failed to read audio file for upload: {e}"))?;
```


---

| 1 | CRITICAL | Missing `.mime_str()` / `.file_name()` on multipart Part |

---

The file is well-structured, concise, and the core rusqlite API usage is correct. The test coverage is decent for the happy paths. However, I found several issues ranging from critical to suggestions.

---


---

## CRITICAL Issues

### 1. Migration bootstrap is a chicken-and-egg race condition (line 53-58)

```rust
let current: u32 = self.conn
    .query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_version",
        [],
        |row| row.get(0),
    )
    .unwrap_or(0);
```

On the very first run, the `schema_version` table does not exist yet. The `query_row` call will fail with "no such table: schema_version", which is silently swallowed by `.unwrap_or(0)`. This **works by accident** -- the error is not "no rows returned" but an actual SQLite error (SQLITE_ERROR), and `unwrap_or` catches it. While functionally correct today, this is fragile:

- It conflates "table doesn't exist" with any other query error (disk I/O error, corruption, etc.). A corrupted database could silently re-run all migrations instead of surfacing the real error.
- Proper fix: explicitly check whether `schema_version` exists first using `SELECT name FROM sqlite_master WHERE type='table' AND name='schema_version'`, or create the `schema_version` table unconditionally before querying it.

### 2. `open_in_memory()` does not set PRAGMAs (lines 45-49)


---

| 1 | Critical | Migration bootstrap swallows all errors via `unwrap_or(0)` | 53-59 |

---

| 2 | Critical | `open_in_memory()` skips `PRAGMA foreign_keys = ON` | 45-49 |

---

| 3 | Critical | Migrations not wrapped in transactions | 64 |

---

## CRITICAL Issues (Must Fix)

### 1. VAAPI encoder names are WRONG -- the `gstreamer-vaapi` plugin is deprecated

Starting from GStreamer 1.22, the old `gstreamer-vaapi` plugin (which provided `vaapih264enc`, `vaapih265enc`, etc.) has been deprecated in favor of the new `va` plugin. The `gstreamer-vaapi` plugin was fully removed from GStreamer mono-repo builds starting with GStreamer 1.24, and is completely gone in 1.26.

**Lines 72-74 -- all three VAAPI names are wrong:**

| Line | Current (WRONG) | Correct (GStreamer 1.22+ `va` plugin) |
|------|-----------------|---------------------------------------|
| 72   | `"vaapih264enc"` | `"vah264enc"` |
| 73   | `"vaapih265enc"` | `"vah265enc"` |
| 74   | `"vaapiav1enc"`   | `"vaav1enc"` |

The `vaapi*` element names belong to the old deprecated `gstreamer-vaapi` plugin. The new `va` plugin uses the `va` prefix. On GStreamer 1.26, `vaapih264enc` simply does not exist anymore unless someone has separately compiled and installed the legacy plugin.

### 2. NVIDIA/CUDA encoder names are WRONG -- `nvh264enc`/`nvh265enc` are deprecated

Starting from GStreamer 1.24, the old NVENC elements (`nvh264enc`, `nvh265enc`) from the `nvcodec` plugin have been deprecated in favor of the newer elements with the `nv` prefix but different naming convention:

**Lines 75-77 -- CUDA names need updating:**

| Line | Current (deprecated) | Correct (GStreamer 1.24+) |
|------|---------------------|---------------------------|
| 75   | `"nvh264enc"`  | `"nvh264enc"` -- This one actually still works, BUT the preferred modern element is `nvh264enc` from the CUDA-based encoder. However, the issue is more nuanced here. |
| 77   | `"nvav1enc"`   | This element does NOT exist in GStreamer. NVIDIA AV1 encoding was never exposed as `nvav1enc`. There is no upstream GStreamer element for NVIDIA AV1 encoding as of GStreamer 1.26. |


---

**Line 77 is a critical bug**: `"nvav1enc"` is a fabricated element name. It does not exist in any GStreamer release. While NVIDIA GPUs (Ada Lovelace+) support AV1 encoding via NVENC, GStreamer has not shipped an `nvav1enc` element. The combination `(Cuda, Av1)` should either return an error/`None` or this combination should be explicitly unsupported.

### 3. Vulkan encoder names are WRONG -- they do NOT exist in GStreamer

**Lines 78-80 -- all three Vulkan names are fabricated:**

| Line | Current (DOES NOT EXIST) |
|------|--------------------------|
| 78   | `"vulkanh264enc"` |
| 79   | `"vulkanh265enc"` |
| 80   | `"vulkanav1enc"`  |

GStreamer's `vulkan` plugin as of 1.26 provides **no video encoder elements**. The Vulkan Video extensions for encoding are still under development in the Vulkan/GStreamer ecosystem. GStreamer's Vulkan plugin provides upload/download/format conversion elements, but **zero encoder elements**. None of these three element names exist.

The entire `Vulkan` variant of `EncoderBackend` is unsupported in GStreamer 1.26 and should be removed or gated behind a feature flag with clear documentation that it is aspirational/future support.

---


---

| 1 | CRITICAL | VAAPI names wrong (`vaapi*` -> `va*`) | 72-74 |

---

| 2 | CRITICAL | `nvav1enc` does not exist | 77 |

---

| 3 | CRITICAL | All Vulkan encoder names are fabricated | 78-80 |

---

The three critical issues (wrong VAAPI names, non-existent NVIDIA AV1 element, non-existent Vulkan elements) mean that **7 out of 12 encoder combinations will fail at runtime**. Only the 3 software encoders (`x264enc`, `x265enc`, `svtav1enc`) and the 2 NVIDIA H264/H265 encoders (`nvh264enc`, `nvh265enc`) reference real GStreamer elements. This needs immediate correction before any recording functionality can work on VA-API or Vulkan systems.

---

### CRITICAL Issues (must fix)

#### 1. Opus audio stream-copy into MP4 container will fail or produce an unplayable file

**Lines 49-50:**
```rust
cmd.arg("-codec:a").arg("copy");
```

The source `recording.mkv` uses Opus audio (confirmed by `defaults.rs` line 45: `codec: "opus"`). The MP4 container (ISO BMFF / MPEG-4 Part 14) has **limited and fragile support for Opus**. The relevant facts:

- Opus-in-MP4 was standardized late (RFC 6716 + ISO/IEC 14496-12:2022 Annex E). While ffmpeg can technically mux Opus into MP4 using `-strict experimental` (or recent builds where it is no longer experimental), **most players do not support Opus-in-MP4**. VLC and mpv handle it, but Chrome, Safari, Windows Media Player, QuickTime, and many hardware decoders do not.
- Even in recent ffmpeg versions, depending on the build, stream-copying Opus into MP4 without `-strict experimental` may produce the error: `Opus in MP4 support is experimental, add '-strict -2' if you want to use it.`
- The code does not pass `-strict -2` or `-strict experimental`, so for many ffmpeg builds this will simply fail with a non-zero exit code.

**This affects both `VideoFormat::Mp4H264` and `VideoFormat::Mp4H265`** -- any time the output container is MP4.

**Recommendation:** For MP4 output, transcode the audio from Opus to AAC (`-codec:a aac`). AAC is the standard, universally-supported audio codec for MP4. Only use `-codec:a copy` when the output container is MKV (Matroska supports Opus natively). Example fix:

```rust
match format {
    VideoFormat::Mp4H264 | VideoFormat::Mp4H265 => {
        // MP4 does not reliably support Opus; transcode to AAC
        cmd.arg("-codec:a").arg("aac").arg("-b:a").arg("128k");
    }
    VideoFormat::MkvH264 | VideoFormat::MkvH265 => {
        // MKV supports Opus natively; stream copy is safe
        cmd.arg("-codec:a").arg("copy");
    }
}
```

---

#### 2. Hardcoded assumption that source is always H.265 -- stream-copy decision is wrong when user changes recording codec

**Lines 36-47 and the doc comment on line 9:**
```
/// Transcodes from the source MKV (H.265) to the target codec and container.
```

The recording system supports **three video codecs**: `H264`, `H265`, and `Av1` (see `src/recorder/types.rs` lines 14-18, and the `VideoCodec` enum). The user's settings allow changing `video.codec` to any of these (it is a free `String` in `config.rs` line 24). If a user records in H.264 and then exports to `Mp4H265` or `MkvH265`, the code will stream-copy the H.264 stream and label the output as H.265, which is silently wrong -- the container metadata will not match the actual stream, and some players will fail or show corruption.

Conversely, if a user records in AV1 and exports to `Mp4H264`, the code will invoke `libx264` which cannot decode AV1 input -- ffmpeg does not chain decode+encode like that. Actually, ffmpeg *will* decode the input and re-encode with libx264, so the H.265-to-H.264 transcode path works generically. But the stream-copy path (lines 39-42) is wrong: it copies the bitstream regardless of what codec it actually is.

**The real bug is in the stream-copy decision.** The code assumes "if the target is H.265, the source must be H.265 too, so copy." This is only true if the source was actually recorded with H.265. If the source is H.264 or AV1 but the user selects `MkvH265`, the output file will contain the wrong codec.

**Recommendation:** Either (a) probe the source video codec with `ffprobe` before deciding to stream-copy, or (b) accept a `source_codec` parameter from the caller (the meeting metadata could store which codec was used). Only stream-copy when source codec == target codec. A minimal approach:

```rust
fn probe_video_codec(source: &Path) -> Result<String, ExportError> {
    let output = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "v:0",
               "-show_entries", "stream=codec_name", "-of", "csv=p=0"])
        .arg(source)
        .output()?;
    if !output.status.success() {
        return Err(ExportError::FfmpegFailed(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
```

Then use the result (`"hevc"`, `"h264"`, `"av1"`) to decide stream-copy vs transcode.

---


---

| 1 | **CRITICAL** | Opus audio stream-copied into MP4 will fail or produce unplayable file |

---

| 2 | **CRITICAL** | Stream-copy decision assumes source is always H.265; wrong if user changed recording codec |

---

### CRITICAL Issues

**1. Auto-start race condition and double-fire (line 33-37)**

```tsx
useEffect(() => {
  if (isRecordingView && !recording && !starting) {
    handleStart();
  }
}, [isRecordingView]);
```

This is the most serious problem in the component. Multiple issues compound here:

- **Missing ESLint exhaustive-deps.** The dependency array lists only `[isRecordingView]`, but the effect closure captures `recording`, `starting`, `handleStart`, and indirectly `withVideo`, `onRecordingStart`. React's rules of hooks require all referenced values in the dependency array. In practice this means the stale-closure check `!recording && !starting` reads the values from the initial render, not the current state. If React strict-mode double-fires the effect (development mode), or if the component is remounted due to `AnimatePresence` keying (which it is -- `App.tsx` line 143 keys on `view`), the guard check can pass twice before `starting` has been set to `true`, causing `handleStart()` to fire twice concurrently. Two concurrent `startRecording` Tauri invocations against the backend would be a race condition.

- **`handleStart` is a plain `async function`, not memoized with `useCallback`.** Every render creates a new `handleStart`, so even if it were listed in the dependency array, the effect would re-run on every render. The function must be stabilized with `useCallback` or the effect must use a ref-based guard.

- **Fix recommendation:** Use a ref-based "has started" guard that survives re-renders and strict-mode double-invocations:

```tsx
const hasAutoStarted = useRef(false);

useEffect(() => {
  if (isRecordingView && !hasAutoStarted.current) {
    hasAutoStarted.current = true;
    handleStart();
  }
}, [isRecordingView]);
```

**2. Polling interval starts without checking for a pre-existing interval (line 51)**

```tsx
pollRef.current = setInterval(async () => { ... }, 1000);
```

If `handleStart` is called twice (which is possible given issue #1), or if the user somehow triggers it while an interval is already running, the old interval is leaked -- it is overwritten without being cleared first. Each leaked interval continues firing `getRecordingStatus` indefinitely.

- **Fix:** Call `clearPoll()` at the top of `handleStart`, before assigning a new interval.

---


---

| 1 | **Critical** | Auto-start useEffect has wrong dependency array and no idempotency guard -- can double-fire |

---

| 2 | **Critical** | Polling interval leaks if handleStart is called twice |

---

## CRITICAL Issues

### 1. `stop_recording` is a synchronous `fn` but calls `pipeline.stop()` which blocks the thread waiting for EOS (up to 5 seconds)

**File:** `commands.rs` line 78-79; **Related:** `pipeline.rs` lines 176-203

`stop_recording` is declared as `pub fn` (synchronous). In Tauri 2, synchronous commands run on the main thread. `pipeline.stop()` calls `bus.timed_pop_filtered(ClockTime::from_seconds(5), ...)`, which blocks the calling thread for up to 5 seconds.

This will freeze the entire UI for the duration of the EOS wait. The function should be `async` and the blocking work should be offloaded via `tokio::task::spawn_blocking` or similar, or the command itself should be made `async` so Tauri runs it off the main thread.

In contrast, `start_recording` is already correctly declared `async`.

**Recommendation:** Make `stop_recording` async, or wrap the blocking body in `spawn_blocking`.

### 2. Hardcoded track metadata does not match the actual pipeline for audio-only recordings

**File:** `commands.rs` lines 104-107

The `stop_recording` function unconditionally emits two tracks:

```rust
tracks: vec![
    TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
    TrackInfo { index: 1, label: "system".to_string(), codec: "opus".to_string() },
],
```

However, when `with_video == false`, the `build_audio_only` pipeline (in `pipeline.rs` lines 20-62) creates only **one** audio track (mic). There is a TODO comment in that function explicitly stating: "TODO: Add system audio capture as second track once pipewire-rs device selection is implemented. For MVP, only mic is captured."

This means the persisted metadata claims a "system" track at index 1 exists when it does not. Any downstream consumer (export, transcription, UI track selector) that trusts this metadata will attempt to access a non-existent track.

**Recommendation:** Conditionally build the tracks vec based on `pipeline.has_video()` or, better, have the pipeline itself report its actual tracks. For the immediate fix:

```rust
let tracks = if pipeline.has_video() {
    // build_with_video creates mic + system tracks
    vec![
        TrackInfo { index: 0, label: "mic".into(), codec: "opus".into() },
        TrackInfo { index: 1, label: "system".into(), codec: "opus".into() },
    ]
} else {
    // build_audio_only creates only mic
    vec![
        TrackInfo { index: 0, label: "mic".into(), codec: "opus".into() },
    ]
};
```

### 3. Track indices are wrong when video is present

**File:** `commands.rs` lines 104-107

When `has_video == true`, the `build_with_video` pipeline produces three streams muxed into MKV: video (track 0), mic audio (track 1), system audio (track 2). But the hardcoded metadata says mic is index 0 and system is index 1, omitting the video track entirely from the track list and assigning incorrect indices to the audio tracks.

This will cause misalignment if any component uses `TrackInfo.index` to demux or seek to specific tracks (e.g., the export module extracting specific audio streams by index).

**Recommendation:** When video is present, the tracks should be:

```rust
vec![
    TrackInfo { index: 0, label: "video".into(), codec: "<actual_codec>".into() },
    TrackInfo { index: 1, label: "mic".into(), codec: "opus".into() },
    TrackInfo { index: 2, label: "system".into(), codec: "opus".into() },
]
```

Ideally the actual video codec name should come from the pipeline (since fallback encoding may change it), not be hardcoded.

---


---

| 1 | CRITICAL | `stop_recording` is synchronous but blocks up to 5 seconds, freezing the UI |

---

| 2 | CRITICAL | Audio-only recordings claim 2 tracks but only have 1 |

---

| 3 | CRITICAL | Track indices wrong when video is present (audio tracks listed as 0,1 instead of 1,2; video track omitted) |

---

The three critical issues (1, 2, 3) will cause user-visible bugs: UI freezes on stop, incorrect track metadata persisted to the database, and wrong track indices for any downstream processing. I recommend addressing those first.
========== bq1emblt7.output ==========
========== bwy56h8os.output ==========

---

