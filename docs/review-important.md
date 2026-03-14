# Hlusra Code Review — IMPORTANT Issues

> 245 important issue sections from 44 code reviewers

---

- **IMPORTANT** -- The bottom margin is `mb-8` (32px) in the implementation vs `mb-6` (24px) in the mockup. This shifts the button 8px higher relative to the text below.
- **MINOR** -- Missing `stagger` class on the pulse ring container.

#### 4.4 Record button element
- **Mockup:** `class="rec-btn relative w-20 h-20 rounded-full bg-gradient-to-br from-brand-500 to-brand-700 flex items-center justify-center glow-brand cursor-pointer group"`
- **Implementation:** `className="rec-btn relative w-20 h-20 rounded-full bg-gradient-to-br from-brand-500 to-brand-700 flex items-center justify-center glow-brand cursor-pointer group border-0 disabled:opacity-40 disabled:cursor-not-allowed"`
- **MINOR** -- Added `border-0 disabled:opacity-40 disabled:cursor-not-allowed` -- justified additions for interactive state handling. No visual difference in normal state.

#### 4.5 Text below button
- **Mockup:** `<p class="text-[11px] text-white/20 mb-6 stagger">Clique para gravar</p>`
- **Implementation (RecordButton.tsx line 143-149):** `<button className="text-sm text-white/40 hover:text-white/70 transition-colors mb-6 cursor-pointer bg-transparent border-0 disabled:cursor-not-allowed">Gravar reuniao</button>`

| Difference | Mockup | Implementation | Severity |
|---|---|---|---|
| Element | `<p>` | `<button>` | MINOR -- Makes it clickable, justified |

---

| Font size | `text-[11px]` | `text-sm` (14px) | **IMPORTANT** -- 3px larger text |

---

| Text color | `text-white/20` | `text-white/40` | **IMPORTANT** -- Twice the opacity (more visible) |
| Text content | "Clique para gravar" | "Gravar reuniao" | MINOR -- Different label |
| `stagger` class | Present | Missing | MINOR |
| Added classes | -- | `hover:text-white/70 transition-colors cursor-pointer bg-transparent border-0` | MINOR -- Interactive state |

#### 4.6 Screen toggle
- **Mockup:** `<label class="flex items-center gap-2.5 cursor-pointer select-none stagger">`
- **Implementation:** `<label className="flex items-center gap-2.5 cursor-pointer select-none">`
- **MINOR** -- Missing `stagger` class.
- Toggle internals (divs, peer classes) are **identical** between mockup and implementation.

#### 4.7 Gallery sidebar button
- **Mockup:** `class="w-48 h-full glass-heavy flex flex-col items-center justify-center gap-3 cursor-pointer group hover:bg-white/[0.06] transition-all duration-300 shrink-0 stagger"`
- **Implementation:** `className="w-48 h-full glass-heavy flex flex-col items-center justify-center gap-3 cursor-pointer group hover:bg-white/[0.06] transition-all duration-300 shrink-0 border-0"`
- **MINOR** -- `stagger` replaced by `border-0`. Missing stagger animation; added border reset.

#### 4.8 Gallery sidebar content
- **Mockup:** `<span class="text-[10px] text-white/8 ...">3 reunioes</span>`
- **Implementation:** `<span className="text-[10px] text-white/8 ...">reunioes</span>` -- dynamic count removed (just "reunioes"), since it is not a static mockup count.
- **MINOR** -- Acceptable.

#### 4.9 SVG icons
- Gallery icon SVG path: **MATCH** -- identical.
- Microphone icon SVG path: **MATCH** -- identical.

---

### 5. RECORDING VIEW

**Mockup (lines 137-152):**

#### 5.1 Container
- **Mockup:** `style="display:flex;flex-direction:column;align-items:center;justify-content:center"`
- **App.tsx:** `className="w-full h-full flex flex-col items-center justify-center relative"`
- **MINOR** -- Implementation adds `w-full h-full relative`. The mockup relies on absolute positioning of `.view`. Functionally equivalent.

#### 5.2 Ambient glow -- **MATCH** (identical)

#### 5.3 Recording indicator
- Classes on all elements: **MATCH** -- identical Tailwind classes.

#### 5.4 Timer
- **Mockup:** `class="text-6xl font-mono font-extralight text-white tabular-nums tracking-widest mb-3 stagger"`
- **Implementation:** Same classes. **MATCH**

#### 5.5 File info line
- **Mockup:** `class="text-[11px] text-white/15 mb-10 stagger"`
- **Implementation:** `className="text-[11px] text-white/15 mb-10 stagger"` **MATCH**

#### 5.6 Stop button
- **Mockup:** `class="rec-btn glass-heavy px-8 py-3 rounded-2xl text-sm text-white/60 hover:text-brand-400 hover:border-brand-500/30 transition-all duration-300 active:scale-95 stagger"`
- **Implementation:** `className="rec-btn glass-heavy px-8 py-3 rounded-2xl text-sm text-white/60 hover:text-brand-400 hover:border-brand-500/30 transition-all duration-300 active:scale-95 stagger cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"`
- **MINOR** -- Adds `cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed`. Justified additions.

---

### 6. GALLERY VIEW

#### 6.1 Header
- **Mockup:** `<header class="glass shrink-0 border-b border-white/5">` -- **MATCH**
- Inner div: `class="px-5 h-12 flex items-center justify-between"` -- **MATCH**

#### 6.2 Back button
- **Mockup:** `class="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5"`
- **Implementation:** `className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer"`
- **MINOR** -- Adds `border-0 bg-transparent cursor-pointer` -- justified for button reset in React.
- This same pattern applies to **every back button** in every view. I will not repeat this for each view.

#### 6.3 Search input
- **Mockup and implementation** classes are identical. **MATCH**

#### 6.4 Content area
- `class="flex-1 overflow-y-auto p-5"` -- **MATCH**
- `class="grid grid-cols-3 gap-4"` -- **MATCH**

#### 6.5 MeetingCard
- **Mockup card container:**
  `class="glass-card rounded-2xl overflow-hidden cursor-pointer group hover:border-white/15 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-black/30 stagger"`
- **Implementation:**
  `className="glass-card rounded-2xl overflow-hidden cursor-pointer group hover:border-white/15 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-black/30 stagger"`
- **MATCH** -- identical.

#### 6.6 Card preview area (video type)
- **Mockup:** `class="aspect-[16/10] bg-gradient-to-br from-white/[0.03] to-transparent relative"` with play icon wrapped in `<div class="absolute inset-0 flex items-center justify-center">`
- **Implementation:** `className="aspect-[16/10] relative flex items-center justify-center bg-gradient-to-br from-white/[0.03] to-transparent"` -- the `flex items-center justify-center` is on the outer div rather than an inner wrapper.
- **MINOR** -- Structural simplification. The SVG is placed directly inside the aspect container instead of an absolute-positioned inner wrapper. Visual result is the same.

#### 6.7 Card badge positioning
- **Mockup:** `<div class="absolute top-2.5 left-2.5"><span class="...">Video</span></div>`
- **Implementation:** `<div className="absolute top-2.5 left-2.5"><span className="...">Video</span></div>` -- **MATCH**

#### 6.8 Audio waveform component
- Waveform bar heights in mockup: `30%, 60%, 45%, 80%, 55%, 90%, 40%, 70%, 50%, 85%, 35%, 65%`
- Implementation `AudioWaveform` heights array: `[30, 60, 45, 80, 55, 90, 40, 70, 50, 85, 35, 65]` -- **MATCH**

#### 6.9 Card info section
- All classes identical. **MATCH**

#### 6.10 Footer
- **Mockup:** `<footer class="glass shrink-0 border-t border-white/5">` -- **MATCH**
- Settings gear icon SVG: **MATCH** -- identical path.

---

### 7. MEETING VIEW

#### 7.1 Header
- All header elements match. The edit icon, title, and layout are **identical** to the mockup.

#### 7.2 Export/Delete buttons
- **Mockup Export:** `class="glass-input px-3 py-1.5 text-[10px] rounded-lg text-white/35 hover:text-white/60 transition-all"`
- **Implementation:** `className="glass-input px-3 py-1.5 text-[10px] rounded-lg text-white/35 hover:text-white/60 transition-all cursor-pointer disabled:opacity-40"`
- **MINOR** -- Adds interactive state classes.

- **Mockup Delete:** `class="px-3 py-1.5 text-[10px] rounded-lg text-red-400/50 hover:text-red-400 hover:bg-red-500/10 border border-white/5 hover:border-red-500/20 transition-all"`
- **Implementation:** adds `bg-transparent cursor-pointer disabled:opacity-40`
- **MINOR** -- Button reset additions.

#### 7.3 Info card
- Play button: **MATCH** -- same classes.
- Type badge, date, duration, size, tracks: **MATCH** -- identical classes.
- Status badges: **MATCH**

#### 7.4 Transcript section
- **Mockup:** Inline in the meeting view HTML.
- **Implementation:** Extracted to `TranscriptView.tsx` component.
- Transcript header: `class="px-5 py-3 flex items-center justify-between border-b border-white/5"` with `<h2 class="text-[11px] font-semibold text-white/50 uppercase tracking-wider">Transcricao</h2>` -- **MATCH**
- Retranscribe button classes in header: **MATCH** -- `text-[10px] text-white/20 hover:text-white/50 transition-colors` (implementation adds `bg-transparent border-0 cursor-pointer`)
- Segment container: `class="max-h-[180px] overflow-y-auto p-5 space-y-3"` -- **MATCH**
- Segment time button: `class="text-[10px] font-mono text-brand-400/60 hover:text-brand-400 tabular-nums transition-colors"` -- **MATCH** (implementation adds `bg-transparent border-0 cursor-pointer`)
- Segment text: `class="text-[12px] text-white/40 mt-1 leading-relaxed ..."` -- **MATCH**

#### 7.5 Chat button
- **Mockup:** `class="w-full glass-card rounded-2xl p-4 flex items-center justify-between group hover:border-white/15 hover:bg-white/[0.05] transition-all duration-300 cursor-pointer stagger"`
- **Implementation:** `className="w-full glass-card rounded-2xl p-4 flex items-center justify-between group hover:border-white/15 hover:bg-white/[0.05] transition-all duration-300 cursor-pointer stagger bg-transparent"`
- **MINOR** -- Adds `bg-transparent`.

- Chat button inner content:
  - **Mockup:** `<div>` wrapping the text lines
  - **Implementation:** `<div className="text-left">` wrapping the text lines
  - **MINOR** -- Adds `text-left` to ensure text alignment inside button (defensive, good practice).

- All SVG icons: **MATCH**

#### 7.6 Action buttons (Retranscrever / Reindexar)
- **Mockup:** `class="glass-input px-3 py-2 text-[10px] rounded-xl text-white/30 hover:text-white/60 transition-all flex-1"`
- **Implementation:** `className="glass-input px-3 py-2 text-[10px] rounded-xl text-white/30 hover:text-white/60 transition-all flex-1 cursor-pointer bg-transparent disabled:opacity-40"`
- **MINOR** -- Adds interactive state classes.

#### 7.7 Delete confirmation overlay
- **Not in mockup** -- This is a React-only addition for the delete confirmation dialog. It uses `glass-heavy rounded-2xl p-7` styling consistent with the design language.
- **N/A** -- Justified addition for real functionality.

---

### 8. CHAT VIEW

#### 8.1 Header
- **Mockup:** `<div class="px-5 h-12 flex items-center justify-between">` with back, title, subtitle, and status indicator.
- **Implementation:** **MATCH** -- identical structure and classes.
- Status indicator: `<div class="w-1.5 h-1.5 rounded-full bg-emerald-500/70 animate-pulse">` -- **MATCH**

#### 8.2 Messages area
- `class="flex-1 overflow-y-auto p-5 space-y-4"` -- **MATCH**

#### 8.3 User message bubble
- **Mockup:** `<div class="bg-brand-500/8 border border-brand-500/10 rounded-2xl rounded-br-md px-4 py-3 max-w-[75%]">`
- **Implementation (line 221-224):** Uses template literal with `max-w-[${msg.role === "user" ? "75" : "80"}%]`

---

- **IMPORTANT** -- Missing "Procurar" (Browse) button for directory selection. This changes the layout from a flex row with input + button to just a full-width input.

##### 10.3.2 Save button
- **Mockup:** `class="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm"`
- **Implementation:** Same + `border-0 cursor-pointer disabled:opacity-40`
- **MINOR** -- Interactive state additions.

- **Mockup save section:** Just `<div class="pt-4 border-t border-white/5">` with button only.
- **Implementation:** `<div class="pt-4 border-t border-white/5 flex items-center gap-3">` with button + saved/error feedback.
- **MINOR** -- Adds `flex items-center gap-3` for inline feedback. Justified improvement.

#### 10.4 Video tab
- All input/select classes: **MATCH**
- Grid layout: **MATCH** (`grid grid-cols-2 gap-4`)

#### 10.5 Audio tab
- **MATCH** -- identical layout and classes.

#### 10.6 Transcription tab

##### 10.6.1 Provider label
- **Mockup:** `<span class="text-[12px] text-white/50">Provider</span>`
- **Implementation:** `<span className="text-[12px] text-white/50">Provedor</span>`
- **MINOR** -- Label translated from English "Provider" to Portuguese "Provedor". This is actually more consistent with the rest of the UI being in Portuguese.

##### 10.6.2 Provider toggle buttons
- **Mockup:** `class="px-4 py-1.5 text-[10px] rounded-lg bg-brand-500 text-white font-medium transition-all"` (active) and `class="px-4 py-1.5 text-[10px] rounded-lg text-white/30 hover:text-white/50 transition-colors"` (inactive)
- **Implementation:** Active: adds `border-0 cursor-pointer`. Inactive: uses `transition-all` instead of `transition-colors`, adds `bg-transparent hover:text-white/50 border-0 cursor-pointer`.
- **MINOR** -- `transition-all` vs `transition-colors` on inactive button is a negligible difference.

##### 10.6.3 Model listing (enhanced in implementation)
- **Mockup:** Simple `<select>` dropdown with model options.
- **Implementation:** When `models` array is populated from the API, renders a list of model cards with download/activate buttons.
- **N/A** -- Feature enhancement beyond the static mockup. Falls back to the `<select>` dropdown when models are unavailable, matching the mockup.

#### 10.7 RAG / Chat tab

##### 10.7.1 Chat URL section
- **Mockup** is missing the "URL do Chat" field that the implementation has.
- **Implementation** adds a separate "URL do Chat" input and "Chave chat" / "Modelo chat" fields in a 2-column grid.
- **N/A** -- Feature additions beyond the mockup. The mockup only shows embeddings URL, API key, model, chunk size, and top-k. The implementation extends it with chat-specific URL/key/model. This is a spec improvement.

---

### 11. TRANSITIONS / ANIMATIONS

#### 11.1 View transitions
- **Mockup:** Uses CSS classes `.view`, `.view.active`, `.morph-from-*` with CSS transitions (`transition: opacity 0.35s, transform 0.35s`).
- **Implementation:** Uses `motion/react` (Framer Motion/Motion) with `AnimatePresence`:
  ```
  initial={{ opacity: 0, scale: 0.97, y: 8 }}
  animate={{ opacity: 1, scale: 1, y: 0 }}
  exit={{ opacity: 0, scale: 0.97, y: -8 }}
  transition={{ duration: 0.2, ease: [0.16, 1, 0.3, 1] }}
  ```

---

- **IMPORTANT** -- The transition is fundamentally different:
  - **Duration:** 0.2s vs 0.35s in the mockup (faster).
  - **Direction-aware morphs are lost:** The mockup has `morph-from-right`, `morph-from-left`, `morph-from-center`, `morph-from-bottom` with different transform-origins and translates. The implementation uses a single uniform `y: 8 -> 0` (enter) / `y: 0 -> -8` (exit) transition for ALL views, losing the directional context of navigation (e.g., gallery slides in from right, settings from bottom).

---

  - **IMPORTANT** -- This is a significant animation fidelity loss. The mockup carefully designed direction-aware transitions that give spatial context to navigation.

#### 11.2 Stagger animations
- **Mockup:** `.view.active .stagger { opacity: 0; animation: stagger-in 0.3s forwards; }` with nth-child delays (0.04s, 0.08s, 0.12s, ...).
- **Implementation:** The `stagger` CSS class is defined in `app.css` via the `stagger-in` keyframe, but the `.view.active .stagger` parent selector is NOT defined in `app.css`. The `stagger` class is applied to many elements, but without the `.view.active .stagger` CSS rule, the stagger animations **never fire**.

---

#### IMPORTANT (should fix -- noticeable visual differences)

3. **Home "Gravar reuniao" text size:** `text-sm` (14px) instead of `text-[11px]` (11px). The text is 27% larger than the mockup specifies. File: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx` line 146.

4. **Home "Gravar reuniao" text opacity:** `text-white/40` instead of `text-white/20`. The text is twice as visible as the mockup. File: same as above.

5. **Record button container margin:** `mb-8` (32px) vs mockup's `mb-6` (24px). 8px extra space between the button and the label. File: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx` line 126.

6. **Directional transitions lost:** All views use the same `y: 8` slide-up animation instead of the mockup's direction-aware morphs (from-right, from-left, from-center, from-bottom). Navigation loses its spatial context. File: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx` lines 143-149.

7. **Transition duration:** 0.2s vs mockup's 0.35s. Transitions are ~43% faster than designed. File: same as above.

8. **Settings "Procurar" browse button missing:** The recordings directory input in the mockup has a "Procurar" button alongside it for opening a file picker. The implementation shows only a plain text input. File: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx` line 195-206.

#### MINOR (barely visible or justified differences)

9. **Home title wrapper:** Mockup wraps `<h1>` in `<div class="mb-10 stagger">`. Implementation puts `mb-10` on the `<h1>` directly and drops `stagger`.

10. **Home "Gravar reuniao" label text:** "Clique para gravar" (mockup) vs "Gravar reuniao" (implementation).


---

### IMPORTANT Issues (Should Fix)

**4. No input validation before save**

`handleSave` sends `settings` to the backend with no client-side validation. Examples of values that would pass through unchecked:
- `fps: 0` or `fps: -5` or `fps: NaN`
- `bitrate: 0`
- `recordings_dir: ""` (empty string)
- `chunk_size: 0` or `top_k: 0`

If the Rust backend does not validate these either (and looking at `commands.rs` line 12, `update_settings` just calls `save_settings` which serializes and writes -- no validation), invalid TOML config will be persisted and cause runtime failures elsewhere.

**5. `saved` indicator persists across tab switches and is misleading**

When the user saves on the "Geral" tab, `saved` becomes `true`. If they switch to the "Video" tab, the "Salvo!" indicator appears there too, implying the video settings were saved when they were not necessarily changed. The `saved` flag should either be tab-scoped or cleared on tab change.

**6. Active model selection is disconnected from `settings.transcription.model`**

The `activeModelName` state (line 41) and `settings.transcription.model` are completely independent. When the user clicks "Usar" on a model, `handleSetActiveModel` updates `activeModelName` and calls the backend `set_active_model`, but it does NOT update `settings.transcription.model`. Conversely, the fallback `<select>` (line 517-532) binds to `settings.transcription.model` but does not call `setActiveModel`. These are two separate flows that should converge. The backend `create_provider` function (in `commands.rs` line 30) uses `models::get_active_model()` for local mode -- it ignores `settings.transcription.model` entirely for local provider. This means the fallback `<select>` writes a model name to the TOML config that is never read by anything.

**7. Download blocks the UI thread with no progress indication**

`handleDownloadModel` (line 88) awaits `downloadModel(name)` which is a blocking download (the Rust side uses `reqwest::blocking::get`). For large models (1.5 GB for medium, 3 GB for large), this will block for minutes. The only feedback is the button text changing to "Baixando..." -- but there is no progress bar, no cancel option, and no timeout. If the user navigates away and back, the `downloadingModel` state is lost. This is not a bug per se, but models up to 3 GB with no progress is a poor user experience.

**8. `saved` indicator has no auto-dismiss**

After a successful save, "Salvo!" stays permanently until the user changes another field. A `setTimeout` to clear `saved` after 2-3 seconds would be standard UX.

**9. Save button is duplicated five times with identical markup**

Lines 244-253, 372-382, 423-432, 587-597, 733-743 all repeat the exact same save button + saved/error indicator block. This should be extracted into a shared component or rendered once outside the tab content area. The current approach means:
- Any styling or behavior change must be replicated in five places.
- The save button is semantically confusing -- it saves ALL settings, not just the current tab's settings, but it appears inside each tab as if it is tab-scoped.

---


---

| 4 | Important | No client-side validation before save (zero/negative/NaN values pass through) |

---

| 5 | Important | `saved` flag persists across tab switches, misleading the user |

---

| 6 | Important | `activeModelName` and `settings.transcription.model` are disconnected; fallback select writes to a field nothing reads |

---

| 7 | Important | Multi-GB downloads block with no progress, no cancel, no timeout |

---

| 8 | Important | "Salvo!" indicator never auto-dismisses |

---

| 9 | Important | Save button + feedback duplicated 5 times; should be extracted or rendered once |

---

**Thumbnail generation on finalize** -- MISSING (Important).
The spec says thumbnails should be auto-generated when `has_video == true` during `finalize_meeting`. There is no `thumbnail.rs` in the library module (confirmed by glob). The `finalize_meeting` method in `api.rs` does not call any thumbnail generation logic. The `ArtifactKind::Thumbnail` type exists and the filesystem layer supports it, but no code ever generates a thumbnail.

**Capture interface** -- DEVIATED.
The spec defines four public methods: `Capture::screen()`, `Capture::mic()`, `Capture::system_audio()`, `Capture::list_audio_devices()`. The implementation only has `ScreenCapture::request_screen()`. Mic and system audio sources are created inline within the pipeline builder rather than through the Capture layer. This deviates from the spec's clean layering but is functionally acceptable for MVP.

**Recording states** -- DEVIATED.
The spec defines 3 states: `Idle -> Recording -> Stopped`. The implementation has the enum with all 3 values in `types.rs`, but `get_recording_status` only ever returns `Idle` or `Recording` -- `Stopped` is never set because the pipeline is `take()`-en on stop and the lock becomes `None` (= Idle). The `Stopped` state is effectively unused.


---

**VideoConfig not read from Settings** -- DEVIATED (Important).
`start_recording` in `commands.rs` line 40-41 creates `VideoConfig::default()` and `AudioConfig::default()` hardcoded rather than loading from the user's Settings. The spec says configurable parameters come from Settings (`[video]` and `[audio]` sections), and the settings system is fully built. The recorder ignores user settings entirely.

---

## 2. LIBRARY MODULE

### File Structure

| Spec file | Code file | Status |
|-----------|-----------|--------|
| `mod.rs` | Present | MATCHES |
| `db.rs` | Present | MATCHES |
| `fs.rs` | Present | MATCHES |
| `api.rs` | Present | MATCHES |
| `types.rs` | Present | MATCHES |
| `thumbnail.rs` | (none) | MISSING |
| `commands.rs` | Present | Not in spec but needed for Tauri IPC |

### Feature-by-Feature

**prepare_meeting / finalize_meeting flow** -- MATCHES.
Implemented exactly as specified. `prepare_meeting()` generates UUID, creates directory, returns `PreparedMeeting { id, dir_path }`. `finalize_meeting()` writes metadata to SQLite. The `prepared` HashMap tracks in-flight meetings.

**DeleteMode (Everything + MediaOnly)** -- MATCHES.
Both modes are implemented correctly. `Everything` removes the directory and DB record. `MediaOnly` deletes `.mkv` + thumbnail, updates `media_status` to `Deleted`, preserves text artifacts and the DB record.

**All artifact types** -- MATCHES.
`ArtifactKind` enum has `Recording`, `Thumbnail`, `TranscriptJson`, `TranscriptTxt` with correct filenames.

**MeetingDetail with tracks + transcript** -- MATCHES.
`MeetingDetail` struct includes `tracks: Vec<TrackInfo>` and `transcript: Option<String>`. The `get_meeting_detail` method reads transcript.json with a fallback to transcript.txt.


---

**Thumbnail generation** -- MISSING (Important).
As noted above, there is no `thumbnail.rs` and no code to extract a frame via FFmpeg. The spec says "auto-generated on `create_meeting` when `has_video == true`" using `ffmpeg-next`.

**Tauri Commands** -- MATCHES.
All five spec commands are registered: `list_meetings`, `get_meeting`, `update_meeting_title`, `delete_meeting`, `get_thumbnail`. The `get_thumbnail` command correctly reads the thumbnail artifact if it exists.

**SQLite schema** -- DEVIATED (minor).
The spec uses `duration REAL` while the code uses `duration_secs REAL`. The spec has separate columns; the implementation stores tracks as `tracks_json TEXT`. This is a reasonable deviation -- JSON-in-column avoids a separate `tracks` table, which is simpler for MVP.

**MeetingUpdate type** -- DEVIATED (minor).
The spec defines `Library::update_meeting(id, MeetingUpdate)` as a generic update method. The implementation has `update_title(id, title)` as a specific method instead. The `MeetingUpdate` struct exists in `types.rs` but is unused. Functionally equivalent for MVP since title is the only updatable field.

---

## 3. TRANSCRIPTION MODULE

### File Structure

| Spec file | Code file | Status |
|-----------|-----------|--------|
| `mod.rs` | Present | MATCHES |
| `orchestrator.rs` | Present | MATCHES |
| `provider.rs` | Present | MATCHES |
| `local.rs` | Present | MATCHES |
| `api.rs` | Present | MATCHES |
| `models.rs` | Present | MATCHES |
| `types.rs` | Present | MATCHES |
| `commands.rs` | Present | Not in spec layout but needed |

### Feature-by-Feature

**TranscriptionProvider trait with 2 implementations** -- MATCHES.
`provider.rs` defines `trait TranscriptionProvider: Send + Sync` with `fn transcribe(&self, audio_path: &Path) -> Result<TranscriptResult, String>`. `LocalProvider` and `ApiProvider` both implement it.

**LocalProvider (whisper-rs) with word-level timestamps** -- MATCHES.
Implemented in `local.rs`. Uses `whisper-rs` with `set_token_timestamps(true)`, extracts per-token data with `full_get_token_data`, constructs `Word` structs with start/end/confidence. Language auto-detection works via `full_lang_id_from_state`.

**ApiProvider (OpenAI-compatible)** -- MATCHES.
Implemented in `api.rs`. Sends multipart form to `/v1/audio/transcriptions` with `response_format: verbose_json` and `timestamp_granularities[]: word`. Parses the verbose JSON response into the internal `TranscriptResult` format. Bearer auth is conditional on non-empty key.

**Orchestrator flow (extract mic -> transcribe -> save)** -- MATCHES.
`orchestrator.rs` implements exactly the described flow: extract mic track (stream 0:a:0) via FFmpeg CLI to temp WAV (16 kHz mono PCM s16le), transcribe via provider, serialize to JSON + TXT, clean up temp file. Uses `tokio::task::spawn_blocking` for the heavy work.

**Model management (download/list/select)** -- MATCHES.
All five functions from the spec are implemented: `list_available_models`, `get_downloaded_models`, `download_model`, `get_active_model`, `set_active_model`. Models stored in `~/.local/share/hlusra/models/` (uses `dirs::data_dir()`). Downloads from Hugging Face with atomic `.part` rename. Active model persisted in `.active_model` file.

**User-prompted (not automatic)** -- MATCHES.
The `transcribe_meeting` command is a Tauri command invoked by the frontend. There is no auto-transcription on recording stop.

**All Tauri commands** -- MATCHES.
All 8 spec commands are registered in `lib.rs`: `transcribe_meeting`, `retranscribe_meeting`, `get_transcription_status`, `list_available_models`, `get_downloaded_models`, `download_model`, `get_active_model`, `set_active_model`.

**TranscriptResult / Segment / Word types** -- MATCHES.
All three types match the spec exactly, including the `confidence: f32` field on `Word`.

**FFmpeg usage** -- DEVIATED (minor, acceptable).
The spec lists `ffmpeg-next` crate for audio extraction. The implementation uses FFmpeg CLI via `std::process::Command` instead. This is pragmatic and avoids linking complexity, but diverges from the spec's stated dependency.

---

## 4. RAG/CHAT MODULE

### File Structure

| Spec file | Code file | Status |
|-----------|-----------|--------|
| `mod.rs` | Present | MATCHES |
| `chunker.rs` | Present | MATCHES |
| `embeddings.rs` | Present | MATCHES |
| `vector_store.rs` | Present | MATCHES |
| `chat.rs` | Present | MATCHES |
| `prompt.rs` | Present | MATCHES |
| `types.rs` | Present | MATCHES |
| `commands.rs` | Present | Not in spec layout but needed |

### Feature-by-Feature

**Separate SQLite DB for vectors** -- MATCHES.
`VectorStore::default_db_path()` returns `~/.local/share/hlusra/rag.db`. The RAG database is independent of the Library's `library.db`.

**sqlite-vec integration** -- MATCHES (with caveat).
The code loads the `vec0` extension via `conn.load_extension("vec0", ...)`. The `chunks_vec` virtual table is created with the correct syntax. However, there is a significant caveat: the `load_sqlite_vec` function (line 104) silently continues if the extension cannot be loaded, logging only a warning. This means vector search will fail at query time rather than at startup. The TODO at line 102-103 acknowledges this: "Bundle the sqlite-vec shared library with the application."

**Chunker preserving timestamps** -- MATCHES.
`chunker.rs` groups consecutive segments until reaching the configurable size limit, preserves `start_time` of first segment and `end_time` of last segment, and never splits mid-segment. Well-tested with 5 unit tests.

**Embeddings client (OpenRouter-focused)** -- MATCHES.
`embeddings.rs` implements `EmbeddingsClient` with `embed_one` and `embed_batch` methods. Uses OpenAI-compatible format. Sorts results by index to guarantee order. Validates consistent embedding dimensions.

**Chat client with streaming** -- MATCHES.
`chat.rs` implements `ChatClient` with `chat_stream` (SSE streaming) and `chat_once` (non-streaming). The `chat_message` command in `commands.rs` uses streaming and forwards chunks to the frontend via Tauri events (`chat-stream-chunk`, `chat-stream-done`, `chat-stream-error`).

**Chat per-meeting only, ephemeral** -- MATCHES.
Vector search is scoped by `meeting_id` in the `search` method. No chat history is persisted. Each `chat_message` call builds a fresh prompt from scratch.

**Embedding dimension strategy** -- MATCHES.
`ensure_ready` checks the stored model against the configured model. On fresh DB, initializes. On model change, returns `ModelChanged` error. The `rag_meta` table stores `embedding_model` and `embedding_dimension`.

**All Tauri commands** -- MATCHES.
`index_meeting`, `reindex_meeting`, `chat_message`, `get_chat_status` are all registered in `lib.rs`. The `reindex_meeting` command deletes existing chunks before re-indexing.

**chat_message return type** -- DEVIATED (minor, justified).
The spec says `chat_message -> Stream<String>`. The implementation returns `Result<(), RagCommandError>` and streams via Tauri events instead. This is a justified deviation -- Tauri commands cannot return streams directly, so event-based streaming is the correct Tauri pattern.

---

## 5. SETTINGS/EXPORT MODULE

### File Structure

| Spec file | Code file | Status |
|-----------|-----------|--------|
| `settings/mod.rs` | Present | MATCHES |
| `settings/config.rs` | Present | MATCHES |
| `settings/defaults.rs` | Present | MATCHES |
| `settings/commands.rs` | Present | Not in spec layout but needed |
| `export/mod.rs` | Present | MATCHES |
| `export/audio.rs` | Present | MATCHES |
| `export/video.rs` | Present | MATCHES |
| `export/transcript.rs` | Present | MATCHES |
| `export/types.rs` | Present | MATCHES |
| `export/commands.rs` | Present | Not in spec layout but needed |

### Feature-by-Feature

**TOML config at ~/.config/hlusra/** -- MATCHES.
`config_path()` uses `dirs::config_dir()` to return `~/.config/hlusra/config.toml`. The load/save cycle works with auto-creation of defaults.

**All config sections (general/audio/video/transcription/rag)** -- MATCHES.
All five sections are implemented as separate structs matching the spec's TOML layout exactly. Default values match the spec: `recordings_dir = ~/Hlusra/recordings`, `codec = opus`, `bitrate = 64000`, `backend = vaapi`, `fps = 15`, `resolution = 720p`, `provider = local`, `model = base`, `chunk_size = 500`, `top_k = 5`.

**Audio export with multi-track mixing** -- MATCHES.
`audio.rs` uses ffprobe to count audio streams, then applies an amerge filter to mix multiple tracks when the target format requires mixdown (MP3, WAV, OGG). Opus export preserves tracks. All four audio formats are supported.

**Video export with codec transcode** -- MATCHES.
`video.rs` handles all four format combinations (MP4/MKV x H.264/H.265). Uses stream copy when source codec matches target, transcodes via libx264 otherwise.

**Transcript export (TXT/JSON/SRT/PDF)** -- MATCHES.
All four formats are implemented in `transcript.rs`. TXT and JSON copy existing files. SRT is generated from segments with proper `HH:MM:SS,mmm` timestamps. PDF is generated via `genpdf` with LiberationSans/DejaVuSans fallback fonts.

**Save + Save As modes** -- MATCHES.
`SaveMode` enum with `Save` (meeting directory) and `SaveAs { path }` (user-chosen). `resolve_output_path` handles both modes.

**All Tauri commands** -- MATCHES.
`get_settings`, `update_settings`, `export_audio`, `export_video`, `export_transcript` are all registered in `lib.rs`.

**FFmpeg usage** -- DEVIATED (same pattern as transcription).
Spec says `ffmpeg-next` crate. Implementation uses FFmpeg CLI via `std::process::Command`. Consistent deviation across all modules -- pragmatic choice for MVP.

---

## SUMMARY TABLE


---

### Important Issues (should fix)

| # | Module | Issue | Description |
|---|--------|-------|-------------|
| 4 | Library | MISSING: Thumbnail generation | No `thumbnail.rs` exists. No code extracts a video frame on finalize. The gallery will never show thumbnails for video meetings. |
| 5 | RAG | sqlite-vec not bundled | Extension loading silently fails if `vec0` is not installed on the host. Vector search will break at query time with a confusing error rather than a clear "not installed" message at startup. |
| 6 | Recorder | DEVIATED: Capture layer incomplete | Spec defines `Capture::mic()`, `Capture::system_audio()`, `Capture::list_audio_devices()`. Only `ScreenCapture::request_screen()` exists. Audio sources are hardcoded in the pipeline builder, making future device selection harder. |


---

#### IMPORTANT

**3. No request timeout configured (line 75, `Client::new()`)**

`reqwest::Client::new()` creates a client with no explicit timeout. Reqwest 0.12 does set a default connect timeout of 30 seconds, but has no default total request timeout. If the embedding API hangs (server accepts connection but never responds), this call will block the Tauri async runtime indefinitely.

Given that the caller (in `commands.rs`) holds a `Mutex` lock around the `RagConfig` and the `VectorStore`, a hung request could cascade into a deadlocked UI.

**Recommendation**: Set an explicit timeout.

```rust
pub fn new(config: &RagConfig) -> Self {
    Self {
        client: Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap_or_else(|_| Client::new()),
        // ...
    }
}
```

---

**4. No validation that the number of returned embeddings matches the number of inputs (line 138-157)**

After sorting by index, the code validates that all embeddings have the same dimension, but never checks that `data.len() == texts.len()`. If the API returns fewer embeddings than inputs (a partial response), the caller in `commands.rs` (line 268) will silently have `embeddings.len() < chunks.len()`, and the subsequent `insert_chunks(&chunks, &embeddings)` call will likely panic or silently drop data.

**Recommendation**: Add a count validation after the empty check.

```rust
if resp.data.len() != texts.len() {
    return Err(EmbeddingsError::CountMismatch {
        expected: texts.len(),
        got: resp.data.len(),
    });
}
```

(Add a corresponding variant to `EmbeddingsError`.)

---

**5. `Client` is created per `EmbeddingsClient` instance, not reused (line 75)**

In `commands.rs`, a new `EmbeddingsClient` is created for every `index_meeting` and `chat_message` invocation (lines 157 and 262). Each construction calls `Client::new()`, which allocates a new connection pool and TLS context. The reqwest documentation explicitly recommends reusing a single `Client` across the application for connection pooling benefits.


---

| 3 | Important | No request timeout -- can hang indefinitely | 75 |

---

| 4 | Important | No validation that returned embedding count matches input count | 138-157 |

---

| 5 | Important | `reqwest::Client` created per instance, not reused | 75 |

---

| 6 | Important | `embed_batch` takes `&[String]` instead of `&[&str]` | 100 |

---

#### IMPORTANT -- No validation that the input MKV file exists

The `extract_mic_to_wav` function and `run_transcription_pipeline` do not check that `input.mkv_path` exists before invoking FFmpeg. If the file is missing, FFmpeg will fail with a cryptic error from stderr. The caller in `commands.rs` (line 59) does perform this check, but the orchestrator's public API has no guard. If another caller is added later, the error message will be unhelpful.

**Recommendation:** Add an existence check at the top of `run_transcription_pipeline` for defense-in-depth:

```rust
if !input.mkv_path.exists() {
    return Err(format!("MKV file not found: {}", input.mkv_path.display()));
}
```


---

#### IMPORTANT -- No validation that `meeting_dir` exists before writing temp file

Line 38 computes `temp_wav` as `input.meeting_dir.join("_temp_mic.wav")`. If `meeting_dir` does not exist (e.g., the directory was deleted between preparation and execution), FFmpeg will fail with a confusing "No such file or directory" error on the *output* path, which is harder to diagnose than a clear "meeting directory missing" message.

**Recommendation:** Validate the directory exists at the start of `run_transcription_pipeline`.


---

#### IMPORTANT -- Temp file name collision risk on concurrent transcriptions

The temp file is always named `_temp_mic.wav` (line 38). If two transcription pipelines run concurrently for recordings in the same `meeting_dir` (unlikely but not prevented by the API), they would write to the same temp file, causing data corruption. While the current UI flow probably serializes this, nothing in the orchestrator enforces it.

**Recommendation:** Use a unique temp file name, for example by appending a random suffix or using `tempfile::NamedTempFile` scoped to the meeting directory:

```rust
let temp_wav = input.meeting_dir.join(format!("_temp_mic_{}.wav", std::process::id()));
```

Or better, use the `tempfile` crate which handles atomicity and cleanup automatically.


---

#### IMPORTANT -- Temp file left behind if the process is killed between FFmpeg and cleanup

If the process is killed (SIGKILL, crash, power loss) after FFmpeg completes (line 39) but before `remove_file` runs (line 45), the temp WAV file is orphaned. For a long meeting, this could be hundreds of megabytes of 16kHz 16-bit PCM (roughly 1.9 MB/minute, so a 2-hour meeting would leave a ~230 MB orphan).

**Recommendation:** This is inherent to the current approach and hard to fully solve, but two mitigations help:
1. Document the `_temp_mic` prefix as a convention so that a startup cleanup routine can sweep orphaned temp files.
2. Consider using `tempfile::NamedTempFile` which provides better cleanup guarantees (though still not against SIGKILL).


---

| 1 | Important | No existence check on `mkv_path` within the orchestrator's public API |

---

| 2 | Important | No existence check on `meeting_dir` before writing temp file |

---

| 3 | Important | Fixed temp file name `_temp_mic.wav` risks collision on concurrent runs |

---

| 4 | Important | Temp file orphaned on process kill (inherent, but mitigatable) |

---

## IMPORTANT ISSUES

### 1. TypeScript version is pinned too tightly
- **File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/package.json` line 27
- **Current:** `~5.7.0` (locks to 5.7.x only)
- **Recommendation:** Change to `~5.8.0` or `~5.9.0` (whatever is latest stable). TypeScript 5.8 shipped in March 2025, and 5.9 likely shipped by now. The `~` prefix is actually fine for TypeScript (minor versions can have breaking type-checking changes), but you should bump the minor version.

### 2. `genpdf` crate is low-maintenance
- **File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/Cargo.toml` line 35
- **Current:** `"0.2"`
- **Concern:** `genpdf` has not seen significant development. For a meeting recorder that exports PDF transcripts, consider evaluating alternatives like `printpdf` (more actively maintained, 0.8+) or `typst` (if you want rich formatting). However, if `genpdf` meets your needs for simple text-based PDF export, keeping it is pragmatically fine.

### 3. `rusqlite` -- verify 0.33 availability
- **File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/Cargo.toml` line 18
- **Current:** `"0.32"`
- **Note:** rusqlite 0.32 was current as of May 2025, but this crate has a regular release cadence. A 0.33 or 0.34 may exist by March 2026. `[VERIFY]`

---


---

### IMPORTANT Issues (confusing but still usable)

**I1. Window resize silently fails**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, lines 37-45

The `setSize` call has an empty `catch {}` block. If the window fails to resize (e.g., Wayland/tiling WM restrictions), the user gets no feedback. The content will be laid out for an 800x400 or 800x600 window but may be in a different-sized container, causing overflow or empty space.

**I2. Play/Pause button has no visual state indicator**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 290-299

The play button always shows a play icon (triangle). When audio/video is playing, there is no pause icon, no progress bar, no time indicator. The user has no idea if media is playing, how far in they are, or when it will end. The media element is literally in a `<div className="hidden">` (line 333). For a meeting recorder app, this is a significant media playback gap.

**I3. Model download has no progress indication**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/SettingsPage.tsx`, lines 88-99

When downloading a Whisper model (which can be 75 MB to 3 GB), the button text changes to "Baixando..." but there is no progress bar, percentage, or download speed. For a 3 GB "large" model, the user has no idea if it will take 30 seconds or 30 minutes, and cannot tell if the download is stalled.

**I4. Export "Salvar" gives no hint about where the file will be saved**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ExportDialog.tsx`, lines 90-91 and 210-216

The "Salvar" button calls `doExport({ mode: "save" })` which saves to a default location. But the user is never told what that default location is before clicking. After saving, the path is shown, but only as a small green text that could easily be missed. The "Salvar como..." button is clearer since it opens a file dialog.

**I5. "Retranscrever" and "Reindexar" buttons give no warning about overwriting previous results**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 374-391

These buttons trigger potentially long operations (retranscription, reindexation) that overwrite existing data. There is no confirmation dialog, unlike the delete button which properly has one. A user might click "Retranscrever" accidentally and lose their current transcript while the new one processes.

**I6. Chat panel allows sending messages when transcript is not ready**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 352-370

The "Conversar sobre esta reuniao" button in MeetingPage is always visible regardless of transcription status. A user can navigate to Chat even when the meeting has no transcript (status "pending"). The ChatPanel will then either try to index a meeting with no content, or confuse the user by requiring indexing that may fail because there is nothing to index yet.

**I7. Shared `actionBusy` state causes unrelated buttons to appear disabled**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 60, 267-278, 376-390

A single `actionBusy` boolean controls the disabled state of Export, Delete, Retranscribe, and Reindex buttons. If the user triggers Retranscribe, the Export and Delete buttons become disabled too, with no explanation. This is defensive but confusing -- the user sees grayed-out buttons and does not understand why.

**I8. Indexing status on ChatPanel does not poll for completion**
`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/ChatPanel.tsx`, lines 58-73

After calling `indexMeeting()` and then `getChatStatus()`, if the status comes back as `"indexing"` (because it is async), the component shows the "Indexando reuniao..." spinner forever. There is no polling to detect when indexing finishes. The user has to navigate away and come back to check if it completed.

---


---

| IMPORTANT | 8 | Missing playback controls, no progress on downloads, stale indexing state |

---

## IMPORTANT Issues

### 4. `reindex_meeting` does not call `refresh_rag_config` before deleting chunks (lines 113-126)

`reindex_meeting` calls `store.delete_meeting_chunks()` immediately, then delegates to `index_meeting` which does call `refresh_rag_config`. But if `refresh_rag_config` (or the subsequent config validation) fails inside `index_meeting`, the chunks have already been deleted. The meeting is now in a state where it has no chunks and the Library still says whatever status it had before. The `get_chat_status` cross-check will eventually correct this to `NotIndexed`, but between the delete and that check, the status is stale.

**Recommendation:** Move the config refresh and validation to the top of `reindex_meeting`, before the delete. If RAG is not configured, fail fast before destroying existing data.

### 5. `index_meeting` sets `ChatStatus::Indexing` but has no timeout or cancellation mechanism (lines 91-108)

If the embedding API hangs (network timeout not configured on `reqwest::Client::new()`), the meeting stays in `ChatStatus::Indexing` indefinitely. The `reqwest::Client::new()` default has no connect or read timeout. The user sees "Indexando..." forever with no way to cancel.

**Recommendation:** Configure timeouts on the `reqwest::Client`. For example:

```rust
Client::builder()
    .connect_timeout(Duration::from_secs(10))
    .timeout(Duration::from_secs(120))
    .build()
    .unwrap_or_default()
```

This applies to both `EmbeddingsClient::new` and `ChatClient::new` (in their respective files), but the impact surfaces here in `commands.rs`.

### 6. `chat_message` holds the Mutex lock while performing an async network call (lines 156-167)

```rust
let (config, relevant_chunks) = {
    let config = rag.config.lock()...clone();
    // ...
    let query_embedding = emb_client.embed_one(&message).await?;  // NETWORK CALL
    let store = rag.store.lock()...;                                // SECOND LOCK
    let chunks = store.search(...)?;
    (config, chunks)
};
```

The `config` MutexGuard is dropped by the `.clone()` call (the binding shadows it with the owned `RagConfig`), so that specific lock is fine. However, the `emb_client.embed_one(&message).await?` call happens before `rag.store.lock()`, so the store lock is not held during the network call -- that is correct. But the entire block is inside a single scope, and if the code were refactored to move the `store.lock()` above the `await`, it would deadlock on `std::sync::Mutex`. This is fragile.

**Recommendation:** Consider splitting this into two explicit blocks for clarity, with a comment explaining why:

```rust
// 1. Clone config and embed query (no locks held during await)
let config = { rag.config.lock()...clone() };
let query_embedding = emb_client.embed_one(&message).await?;

// 2. Search vector store (brief lock)
let relevant_chunks = {
    let store = rag.store.lock()...;
    store.search(&meeting_id, &query_embedding, config.top_k)?
};
```

### 7. `do_index_meeting` does not handle `VectorStoreError::ModelChanged` gracefully (line 284)

`ensure_ready` returns `VectorStoreError::ModelChanged` when the user changes embedding models, but `do_index_meeting` just propagates this as a generic error. The user sees `"Model changed: stored=X, configured=Y. Reindex required."` and the meeting status is set to `Failed`. But this is not actually a failure -- it means the user needs to reinitialize the vector table for the new model. There is no code path that calls `init_vector_table` and retries.

**Recommendation:** Either (a) catch `ModelChanged` in `do_index_meeting` and automatically call `store.init_vector_table()` then retry, or (b) surface a specific error variant to the frontend so it can prompt the user and offer a "Reinitialize & Reindex" button.

### 8. `std::sync::Mutex` used in async context -- potential for blocking the Tokio runtime (throughout)

`RagState` uses `std::sync::Mutex` for both `store` and `config`. In a Tauri v2 app, commands marked `async` run on the Tokio runtime. While the locks are brief, if the `VectorStore` operations (SQLite queries, especially `insert_chunks` with large transactions) take significant time, they block the async executor thread.

**Recommendation:** Consider `tokio::sync::Mutex` for `store` (which would allow `.await`-friendly locking), or use `tokio::task::spawn_blocking` around the SQLite-heavy sections.

---


---

| Important | 5 | Reindex deletes before validation, no timeouts, ModelChanged not handled, std::sync::Mutex in async |

---

The file is well-organized and demonstrates good Rust/Tauri patterns overall. The most urgent fixes are around the event streaming reliability (issues 1-2) and the defensive guard against zero-dimension embeddings (issue 3). The important issues around `reindex_meeting` ordering (issue 4) and `ModelChanged` handling (issue 7) represent real user-facing failure modes that should be addressed before the feature is considered stable.

---

### IMPORTANT Issues (should fix)

**I1. `start_time` is initialized in the constructor and then overwritten in `start()` -- misleading if `start()` is not called or called late**


---

On line 58 (and 163), `start_time: Instant::now()` is set during `build_*()`. Then on line 170, `self.start_time = Instant::now()` resets it in `start()`. If someone calls `duration_secs()` between building and starting, they get a duration that starts from build time. More importantly, after `stop()` is called, `duration_secs()` continues to accumulate time because it just calls `self.start_time.elapsed()`. The duration keeps growing after the recording ends.

Recommendation: Either store the elapsed duration as a fixed value when `stop()` is called, or switch to `Option<Instant>` to make the state explicit. A `stopped_duration: Option<f64>` field that gets set in `stop()` and is returned by `duration_secs()` when present would be more correct.

**I2. `stop()` takes `&self` but should take `&mut self` or consume `self`**

The `stop()` method on line 176 takes `&self`, meaning it can be called multiple times. Sending EOS to an already-stopped pipeline and waiting on its bus is undefined behavior in GStreamer terms (the bus may be flushing, the pipeline may already be in Null). The method should either consume `self` (take ownership) or at minimum take `&mut self` and set a flag to prevent double-stop.

Additionally, in `commands.rs` line 85, `pipeline_lock.take()` removes the pipeline from the option before calling `stop()`, which means the pipeline is dropped after `stop()` returns. Since there is no `Drop` impl (see C1), this is fine for now but fragile.

Recommendation: Change `stop` to `pub fn stop(self) -> Result<(), String>` to enforce single-use, or add a `stopped: bool` guard field.

**I3. `stop()` on EOS success path does not check the return value of `set_state(Null)` rigorously**

On line 201-202, after successfully receiving EOS, the code calls `self.pipeline.set_state(gst::State::Null)` and maps the error. However, `set_state()` returns `Result<StateChangeSuccess, StateChangeError>`. A return of `Ok(StateChangeSuccess::Async)` means the state change has not completed yet -- the pipeline is still transitioning. The code does not call `get_state()` to wait for the transition to complete. For the `Null` state this is usually synchronous, but it is not guaranteed.

Recommendation: After setting state to Null, call `self.pipeline.state(gst::ClockTime::from_seconds(5))` to wait for the state change to actually complete.

**I4. No bus watch for asynchronous errors during recording**

The pipeline has no bus watch or error callback installed. If a GStreamer element posts an error during recording (e.g., PipeWire source disconnects, disk full, encoder crash), the error goes unnoticed until `stop()` is called. By that point, the recording may be corrupted. The application should install a bus watch at `start()` time to detect errors and surface them to the UI proactively.

Recommendation: Install a bus callback or spawn a thread that monitors the bus for `Error` and `Warning` messages, and updates the `RecorderState` accordingly (e.g., setting an `error` field that `get_recording_status` can report).

**I5. `build_with_video` screen source sets `fd` property as `RawFd` (i32) -- type mismatch risk**

On line 76, `screen_src.property("fd", screen_source.fd)` passes a `RawFd` (which is `i32` on Linux). The `pipewiresrc` element's `fd` property expects an `i32`, so this works. However, `RawFd` is a type alias and there is no compile-time guarantee that GStreamer's property type matches. If the GStreamer bindings change or on a platform where `RawFd` is not `i32`, this silently compiles but fails at runtime.

Recommendation: Explicitly cast: `.property("fd", screen_source.fd as i32)` to document the intent and ensure correctness.

**I6. Queue elements have no size limits configured**

The `queue` elements (lines 32, 81, 112, 125) are created with default properties. GStreamer `queue` defaults to a maximum of 200 buffers, 10MB, and 1 second. For a multi-track recording, the default limits may cause the queue to block if one track produces data faster than another, leading to deadlocks or dropped frames. Alternatively, if queues are too small, the `matroskamux` could starve.

Recommendation: Set explicit `max-size-time`, `max-size-buffers`, and `max-size-bytes` properties on each queue. For a recording pipeline, generous time-based limits (e.g., 3-5 seconds) prevent deadlocks. Example:
```rust
let mic_queue = gst::ElementFactory::make("queue")
    .name("mic_queue")
    .property("max-size-time", 5_000_000_000u64) // 5 seconds
    .property("max-size-buffers", 0u32)           // unlimited
    .property("max-size-bytes", 0u32)             // unlimited
    .build()
    .map_err(|e| e.to_string())?;
```

---


---

| I1 | Important | State Mgmt | Duration keeps growing after `stop()` |

---

| I2 | Important | API Safety | `stop(&self)` allows double-stop |

---

| I3 | Important | State Mgmt | No wait for Null state transition to complete |

---

| I4 | Important | Error Handling | No async bus watch for runtime errors |

---

| I5 | Important | Type Safety | `RawFd` passed as property without explicit cast |

---

| I6 | Important | Pipeline Design | Queue elements have no explicit size limits |

---

The most urgent items are C1-C3, which represent correctness bugs that will cause resource leaks or completely broken audio capture at runtime. C4 is a data corruption risk on non-trivial paths. I4 (no bus watch) is the most impactful "important" item because without it, errors during recording go entirely undetected.

---

**Severity:** Important (should fix) -- dead code that adds confusion and minor memory overhead for long streams.

---


---

## IMPORTANT Issues (Should Fix)

### 4. No guard against double-invocation of `handleSend`

**File:** Line 75-77

```js
async function handleSend() {
  const msg = input.trim();
  if (!msg || sending) return;
```

The `sending` guard works at the state level, but `setState` is asynchronous. If a user double-clicks the send button fast enough, two calls to `handleSend` can both see `sending === false` before the first `setSending(true)` at line 81 is committed. This would register duplicate listeners and send the message twice.

**Fix:** Use a ref-based guard in addition to the state check:

```js
const sendingRef = useRef(false);

async function handleSend() {
  const msg = input.trim();
  if (!msg || sending || sendingRef.current) return;
  sendingRef.current = true;
  // ... rest of function
  // In finally block:
  sendingRef.current = false;
}
```

### 5. `handleIndex` has a stale closure over `onStatusChange`

**File:** Line 58-73

`handleIndex` captures `onStatusChange` from the render scope, but `onStatusChange` is passed as a prop that could change between renders. Since `handleIndex` is a plain `async function` (not wrapped in `useCallback`), it re-creates each render and always captures the latest prop, so this is technically fine. However, note that in `App.tsx` line 120, `onStatusChange` is `() => {}` -- a no-op. This means the `onStatusChange()` call at line 66 does nothing. If this is intentional dead wiring, consider removing it or connecting it properly.

### 6. The `stagger` CSS class on line 220 has no corresponding CSS rule

**File:** Line 220

```jsx
<div key={i} className={`flex ${...} stagger`}>
```

The CSS file at `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/styles/app.css` defines a `@keyframes stagger-in` animation (line 89) and animation classes `animate-pulse-rec` and `animate-pulse-ring` (line 94), but there is no `.stagger` CSS rule. A grep for `\.stagger\b` in CSS files returns zero matches. The class name `stagger` is applied across multiple components (MeetingCard, MeetingPage, ExportDialog, RecordButton, TranscriptView) but has no styling effect. Either the CSS rule was deleted at some point, or it was never created.

**Fix:** Either remove the `stagger` class from all components, or add a CSS rule that applies the `stagger-in` animation. For example in `app.css`:

```css
.stagger {
  animation: stagger-in 0.3s ease-out both;
}
```

### 7. Using array index as `key` for message list

**File:** Line 219

```jsx
{messages.map((msg, i) => (
  <div key={i} ...>
```

Using the array index as a React key is problematic when items can be inserted, deleted, or reordered. In this chat, messages are only appended, and the last assistant message is mutated in-place (via the functional updater in `appendToLastAssistant`). Index keys will not cause visible bugs here because messages are append-only and never removed. However, if delete/edit functionality is added later, this will silently break reconciliation.

**Fix (minor):** Add an `id` field to `ChatMsg` (e.g., a monotonic counter or `crypto.randomUUID()`) and use it as the key.

### 8. `chatMessage` Promise rejection could leave `streamFinished` unresolved

**File:** Lines 117-125

```js
try {
  await chatMessage(meetingId, msg);
  await streamFinished;
} catch (e) {
  if (mountedRef.current) setError(formatError(e));
} finally {
  cleanup();
```

If `chatMessage` rejects (line 118), the `catch` block runs and `cleanup()` is called. However, `streamFinished` is never resolved or rejected -- the Promise just leaks. The `resolveDone` callback is captured in the `chat-stream-done` and `chat-stream-error` listeners, which are unregistered in `cleanup()`. The Promise and its closure hang in memory until garbage-collected. This is not a functional bug (execution continues correctly through catch/finally), but it does create a minor memory leak for each failed send.

**Fix:** Call `resolveDone()` in the catch block before the error handling, or restructure so `streamFinished` does not leak.

---


---

| 3 | Important | `streamBufferRef` is written but never read (dead code) | 27, 82, 91 |

---

| 4 | Important | Possible double-invocation of `handleSend` due to async setState | 75-77 |

---

| 5 | Important | `onStatusChange` is wired as a no-op in `App.tsx` | 66, App.tsx:120 |

---

| 6 | Important | `stagger` CSS class has no corresponding rule | 220 |

---

| 7 | Important | Array index used as React key | 219 |

---

| 8 | Important | Leaked `streamFinished` Promise on `chatMessage` rejection | 117-125 |

---

#### IMPORTANT: Dual system messages may confuse some models

Lines 39-50. The function sends **two** consecutive messages with `role: "system"` -- one for instructions and one for context. While the OpenAI API technically accepts multiple system messages, this is not standard practice and can cause issues:

1. **OpenRouter routing** -- Some models behind OpenRouter (especially open-source models like Mistral, Llama) only honor the first or last system message, or merge them unpredictably. Since this project is OpenRouter-focused (per the project memory), this is a real risk.
2. **Token accounting** -- Some providers count system messages differently for billing/context-window purposes; two separate system messages may be counted as two "turns" instead of one continuous instruction block.

**Recommendation**: Merge the context into the single system message, or deliver the context as a `"user"` prefixed message (a common RAG pattern where context is injected as a pseudo-user message before the real question). The safest OpenAI-compatible approach for broad model support:

```rust
// Option A: merge into one system message
messages.push(ChatMessage {
    role: "system".to_string(),
    content: format!("{}\n\n{}", SYSTEM_PROMPT, context),
});

// Option B: use role "user" for context (clearly labeled)
messages.push(ChatMessage {
    role: "user".to_string(),
    content: format!("[CONTEXT]\n{}\n[/CONTEXT]", context),
});
```


---

#### IMPORTANT: No truncation or token budget guard

The `build_context` function concatenates all chunks without limit. The caller in `commands.rs` (line 165) passes `config.top_k` chunks (default 5), but there is no guard here against the total prompt exceeding the model's context window. If chunks are large (the chunk size target is ~500 tokens each, so 5 chunks could be ~2500 tokens of context alone), and if `top_k` is ever increased or chunk sizes grow, the prompt could silently exceed the model limit and get truncated or rejected.

**Recommendation**: Add a `max_context_tokens` or `max_context_chars` parameter and truncate chunks when the budget is exhausted. Even a rough character-based limit (e.g., 1 token ~= 4 chars) would be a meaningful safety net.


---

#### IMPORTANT: `format_timestamp` silently truncates fractional seconds

Line 6: `let total_secs = seconds as u64;` performs a truncating cast. A timestamp of `59.9` seconds becomes `00:59`, not `01:00`. This is correct behavior for a timestamp display (you want the floor, not a round), but it is worth noting that:

- **Negative values** are silently cast to 0 (or to `u64::MAX` on some targets) due to undefined float-to-int casting behavior. Rust saturates in debug mode (to 0 for negatives) but in release mode prior to Rust 1.45 this was UB. Since Rust 1.45+ it saturates, so negative f64 becomes 0. This is fine but undocumented.
- **NaN or infinity** values would also produce 0 (saturating cast). No validation is performed.

**Recommendation**: Add a `debug_assert!(seconds.is_finite() && seconds >= 0.0)` or clamp explicitly:
```rust
let total_secs = seconds.max(0.0) as u64;
```


---

| **Important** | Dual system messages may break on OpenRouter models | 39-50 |

---

| **Important** | No truncation/token budget guard on context size | 66-84 |

---

| **Important** | `format_timestamp` does not validate input (NaN, negative, infinity) | 6 |

---

# Important Issues (Should Fix)

1. **Play/Pause button never reflects state** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, line 291). The icon is hardcoded to a play triangle. When the user clicks it and media starts playing, they see no visual change. They cannot tell whether clicking will play or pause. Recommendation: add a `playing` state, listen to the media element's `play`/`pause` events, and toggle the SVG icon accordingly.

2. **Title `<h1>` is not keyboard accessible** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, line 252). Add `tabIndex={0}`, `role="button"`, and `onKeyDown` for Enter/Space to match the keyboard pattern already correctly implemented on MeetingCard.

3. **Delete modal has no Escape key handler** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, line 399). Add a `useEffect` with a `keydown` listener for Escape that calls `setShowDeleteConfirm(false)`.

4. **TranscriptView "Retranscrever" header button missing `disabled` prop** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/TranscriptView.tsx`, line 137). The button at line 139 has `onClick={handleTranscribe}` but no `disabled={transcribing}`, unlike the identical buttons at lines 91 and 112 in the same file. This is an inconsistency that could lead to double-submit.


---

More importantly: the code **silently ignores the Vulkan backend** (lines 57-59), meaning Vulkan-encoded video will use whatever default bitrate the encoder ships with (often 0 or an unusably low value). This is a functional bug.

**Recommendation**: At minimum, attempt to set bitrate for Vulkan encoders too. Additionally, use `try_set_property` or check `element.has_property("bitrate", ...)` before calling `set_property` to avoid panics on unexpected property types:

```rust
if encoder.has_property("bitrate", None) {
    let _ = encoder.try_set_property("bitrate", &(config.bitrate / 1000));
}
```

#### 2. `svtav1enc` bitrate property name is `target-bitrate`, not `bitrate` -- will panic


---

### IMPORTANT Issues

#### 4. `create_video_encoder_with_fallback` does not fall back across codecs

The fallback chain (lines 75-101) only falls back across **backends** for the same codec. If H265 is requested and no H265 encoder is available on any backend, the function returns `Err` even if H264 encoders are available everywhere. This may be intentional, but it is worth calling out: a user whose system only has `x264enc` installed (common on minimal setups) will get an error if the default config requests H265 (which it does -- `VideoConfig::default()` uses `VideoCodec::H265`).

**Recommendation**: Consider either (a) adding a codec fallback chain (H265 -> H264 -> AV1), or (b) validating at startup that the default config's codec is actually available and adjusting it. The current behavior will give a cryptic error on systems with only x264.

#### 5. `probe_available` is called redundantly in the fallback function

On line 81, `create_video_encoder_with_fallback` first tries `create_video_encoder(preferred, ...)`. If that fails, it calls `probe_available()` (line 86) which re-probes the entire registry. But the initial failure of `create_video_encoder` already tells us the preferred combination does not work. Meanwhile, `ElementFactory::make(...).build()` is heavier than `ElementFactory::find(...)` -- it actually instantiates the element.

The inconsistency is: the first attempt (line 81) calls `create_video_encoder` which does `ElementFactory::make().build()` without first checking `find()`. If the element does not exist, `make().build()` will fail, but it is doing more work than necessary. Conversely, the fallback loop (lines 87-97) does probe with `find()` first (via `probe_available`), then calls `create_video_encoder` which calls `make().build()` again -- so for fallback elements, both `find()` AND `make().build()` run.

**Recommendation**: Call `probe_available()` once at the top of `create_video_encoder_with_fallback`, check if the preferred backend+codec is in the result, and only then attempt to create it. This avoids the unnecessary instantiation attempt on the preferred backend when it is not even registered:

```rust
let available = probe_available();
// Try preferred first
if available.get(&preferred).map_or(false, |c| c.contains(&codec)) {
    if let Ok(enc) = create_video_encoder(preferred, codec, config) {
        return Ok((enc, preferred));
    }
}
// Then try others...
```

#### 6. `opusenc` bitrate is cast to `i32` -- truncation risk and incorrect type

On line 67:

```rust
.property("bitrate", config.bitrate as i32)
```

`opusenc`'s `"bitrate"` property is of GObject type `gint` (i.e., `i32`) and is in **bits per second**. The default `AudioConfig::bitrate` is `64_000` which fits in `i32` fine. However, the `as i32` cast on a `u32` is a **truncating cast** -- if someone sets `bitrate` to a value above `i32::MAX` (2,147,483,647, i.e., ~2 Gbps), it would silently wrap to a negative number, which `opusenc` would reject or interpret incorrectly.

While 2 Gbps for audio is absurd, `as i32` is a code smell. Use `i32::try_from(config.bitrate)` with proper error handling, or at minimum validate the bitrate range in `AudioConfig`.

**Recommendation**:

```rust
let bitrate: i32 = config.bitrate.try_into()
    .map_err(|_| format!("Audio bitrate {} out of range", config.bitrate))?;
```

#### 7. Video encoder bitrate units are not documented or validated

`VideoConfig::bitrate` default is `2_000_000` (line 55 in types.rs). The code divides by 1000 before passing to encoders, so it assumes `config.bitrate` is in **bits per second** and the encoders expect **kbps**. But:

- There is no documentation on the `bitrate` field stating the unit.
- There is no validation that the bitrate is non-zero or within a reasonable range.
- A bitrate of 0 would silently produce `0 / 1000 = 0`, which some encoders interpret as "use default" and others as "produce garbage."

**Recommendation**: Add a doc comment to `VideoConfig::bitrate` and `AudioConfig::bitrate` stating the unit (bps). Add validation in the encoder creation functions.

---


---

| 4 | IMPORTANT | No codec fallback; default config (H265) fails on x264-only systems |

---

| 5 | IMPORTANT | Redundant probing; `make().build()` called before `find()` for preferred backend |

---

| 6 | IMPORTANT | `as i32` truncation on audio bitrate is unsafe |

---

| 7 | IMPORTANT | Bitrate unit undocumented; no validation for zero or negative values |

---

The `exists()` check followed by `remove_dir_all()` is a classic time-of-check-time-of-use race. Between the check and the removal, the directory could be deleted by another thread (e.g., a concurrent delete request from the UI). More importantly, this pattern silently swallows the case where the path was never a valid directory to begin with -- if the caller passes a bogus path, this returns `Ok(())` silently, masking bugs.

**Recommendation:** Remove the `exists()` guard and handle the specific `NotFound` error instead:

```rust
pub fn delete_meeting_dir(&self, meeting_dir: &Path) -> std::io::Result<()> {
    match fs::remove_dir_all(meeting_dir) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}
```

This is atomic with respect to the filesystem call and handles concurrent deletion gracefully.

### 2. TOCTOU Race in `delete_media_files` (lines 42-52)

Same pattern:

```rust
if recording.exists() {
    fs::remove_file(&recording)?;
}
```

**Recommendation:** Same fix -- attempt the delete, tolerate `NotFound`:

```rust
fn remove_file_if_exists(path: &Path) -> std::io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}
```

### 3. No Path Containment Validation -- Directory Traversal

`create_meeting_dir` takes an arbitrary `&str` and joins it directly:

```rust
pub fn create_meeting_dir(&self, dir_name: &str) -> std::io::Result<PathBuf> {
    let path = self.base_dir.join(dir_name);
    fs::create_dir_all(&path)?;
    Ok(path)
}
```

If `dir_name` contains `..` or `/`, this creates directories outside `base_dir`. While the caller in `api.rs` currently constructs `dir_name` from a date and UUID slice (line 61: `format!("{}_{}", now.format("%Y-%m-%d"), &id[..8])`), `LibraryFs` itself has no defense. If any other caller is ever added or the format changes, this is a silent path traversal vulnerability.

**Recommendation:** Validate that the resolved path is still under `base_dir`:

```rust
pub fn create_meeting_dir(&self, dir_name: &str) -> std::io::Result<PathBuf> {
    let path = self.base_dir.join(dir_name);
    let canonical_base = self.base_dir.canonicalize()?;
    // create first, then canonicalize (canonicalize requires the path to exist)
    fs::create_dir_all(&path)?;
    let canonical_path = path.canonicalize()?;
    if !canonical_path.starts_with(&canonical_base) {
        fs::remove_dir_all(&canonical_path).ok(); // clean up
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("directory name would escape base directory: {}", dir_name),
        ));
    }
    Ok(canonical_path)
}
```

Similarly, `delete_meeting_dir`, `save_artifact`, `read_artifact`, `has_artifact`, and `delete_media_files` all accept an arbitrary `&Path` for `meeting_dir` with no validation that it resides under `base_dir`. A caller could pass any absolute path (e.g., `/etc`) and the methods would happily operate on it.

---


---

## IMPORTANT Issues (Should Fix)

### 4. `save_artifact` Does Not Ensure Parent Directory Exists (line 25-29)

```rust
pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> std::io::Result<PathBuf> {
    let path = self.get_artifact_path(meeting_dir, kind);
    fs::write(&path, data)?;
    Ok(path)
}
```

If the meeting directory was deleted or never created (e.g., `create_meeting_dir` was skipped or the dir was externally removed), `fs::write` will fail with a confusing "No such file or directory" error pointing at the artifact file, not the missing parent. This is especially relevant because `delete_media_files` deletes files inside a directory that might later have artifacts saved to it again.

**Recommendation:** Either add a check with a clear error message, or call `fs::create_dir_all` on the parent before writing:

```rust
pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> std::io::Result<PathBuf> {
    let path = self.get_artifact_path(meeting_dir, kind);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, data)?;
    Ok(path)
}
```

### 5. `save_artifact` Is Not Atomic -- Partial Writes on Disk Full / Crash

`fs::write` performs a single `open + write + close`. If the disk fills up during the write, or the process crashes, the file will be left in a partial/corrupted state. For recording artifacts (MKV files) this may be tolerable since they are written by GStreamer directly, but for `transcript.json` and `transcript.txt` which are written through `save_artifact`, a partial JSON file would be unrecoverable.

**Recommendation:** Write to a temporary file in the same directory, then `rename()` (which is atomic on Linux/ext4/btrfs):

```rust
pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> std::io::Result<PathBuf> {
    let path = self.get_artifact_path(meeting_dir, kind);
    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, data)?;
    fs::rename(&tmp_path, &path)?;
    Ok(path)
}
```

### 6. `read_artifact` Loads Entire File Into Memory (line 54-57)

```rust
pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> std::io::Result<Vec<u8>> {
    let path = self.get_artifact_path(meeting_dir, kind);
    fs::read(&path)
}
```

This is used to read `Recording` artifacts (MKV video files) through the same interface as transcript text. Looking at the callers, `api.rs` line 211 (`read_artifact`) is exposed to Tauri commands. If someone calls `read_artifact` with `ArtifactKind::Recording` on a 2GB MKV file, this will attempt to allocate 2GB of memory in a single `Vec<u8>`, which will either OOM-kill the process or cause severe memory pressure.

The current callers in `api.rs` only read transcript files, and `rag/commands.rs` only reads `TranscriptJson`. But the API permits reading any artifact kind, making this a latent footgun.

**Recommendation:** Either restrict `read_artifact` to non-media types, or add a size guard:

```rust
pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> std::io::Result<Vec<u8>> {
    let path = self.get_artifact_path(meeting_dir, kind);
    let metadata = fs::metadata(&path)?;
    const MAX_READ_SIZE: u64 = 50 * 1024 * 1024; // 50 MB
    if metadata.len() > MAX_READ_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("artifact too large to read into memory: {} bytes", metadata.len()),
        ));
    }
    fs::read(&path)
}
```

### 7. Symlink Following in `delete_meeting_dir`

`fs::remove_dir_all` follows symlinks. If `meeting_dir` contains a symlink pointing to, say, `/home/user/Documents`, `remove_dir_all` will delete the *target* directory's contents, not just the symlink. This is a data-loss risk.

On Linux, a malicious or buggy state where a symlink ends up in the meeting directory could cause `delete_meeting_dir` to destroy unrelated user data.

**Recommendation:** Before deletion, verify the path is not a symlink and is a real directory:

```rust
pub fn delete_meeting_dir(&self, meeting_dir: &Path) -> std::io::Result<()> {
    let metadata = match fs::symlink_metadata(meeting_dir) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    if metadata.is_symlink() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "refusing to delete symlink as meeting directory",
        ));
    }
    fs::remove_dir_all(meeting_dir)
}
```

### 8. `has_artifact` Follows Symlinks Silently (line 31-33)

`Path::exists()` follows symlinks. If a symlink named `transcript.txt` points to a nonexistent target, `exists()` returns `false` even though a filesystem entry is present. If it points to a valid target outside the meeting directory, `has_artifact` returns `true` and subsequent `read_artifact` will read that external file.

This is minor given the app's usage but worth noting for correctness.

---


---

No test verifies that `read_artifact` returns an error for a missing file. This is important edge-case documentation:

```rust
#[test]
fn test_read_missing_artifact_fails() {
    let tmp = TempDir::new().unwrap();
    let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
    let dir = lib_fs.create_meeting_dir("test").unwrap();
    let result = lib_fs.read_artifact(&dir, &ArtifactKind::Recording);
    assert!(result.is_err());
}
```

### 12. `delete_media_files` Only Handles Two Artifact Kinds

The method hardcodes `Recording` and `Thumbnail` as the media files to delete. If a new media artifact kind is added to `ArtifactKind` in the future (e.g., a waveform image, or an audio-only export), this method will not know about it. Consider adding a method on `ArtifactKind` like `is_media() -> bool` and iterating over all media kinds, or at minimum leave a comment noting the coupling.

### 13. No Logging on Filesystem Operations

The `api.rs` layer has `eprintln!` logging, but `fs.rs` has none. When debugging disk-full errors or permission failures, having the path printed in the error chain would help. Consider wrapping `std::io::Error` with `map_err` to include the path:

```rust
fs::write(&path, data).map_err(|e| {
    std::io::Error::new(e.kind(), format!("{}: {}", path.display(), e))
})?;
```

---

## Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 4 | Important | `save_artifact` does not ensure parent directory exists | 25-29 |

---

| 5 | Important | `save_artifact` is not atomic (partial writes on crash/disk-full) | 25-29 |

---

| 6 | Important | `read_artifact` can OOM on large media files | 54-57 |

---

| 7 | Important | `delete_meeting_dir` follows symlinks into unrelated directories | 35-39 |

---

| 8 | Important | `has_artifact` follows symlinks silently | 31-33 |

---

## IMPORTANT Issues (Should Fix)

### 3. `TranscriptSegment` silently ignores the `words` field -- potential data model drift

**File:** lines 10-15.

The canonical data type `transcription::types::Segment` has four fields: `start`, `end`, `text`, and `words: Vec<Word>`. The export module defines its own `TranscriptSegment` with only `start`, `end`, `text`.

Because serde by default ignores unknown fields, deserialization works. However, this creates a parallel type that can silently diverge. If someone adds a `speaker` field to `Segment` and the SRT output needs it, this type will silently drop it.

**Recommendation:** Either reuse `transcription::types::Segment` directly (it already derives `Deserialize`), or add `#[serde(deny_unknown_fields)]` to force a compile-time/test-time signal when the schema drifts. Reusing the canonical type is strongly preferred:

```rust
use crate::transcription::types::Segment as TranscriptSegment;
```

### 4. `TranscriptFile` field types do not match `TranscriptResult` exactly

**File:** lines 170-177.

`TranscriptFile` declares `language: Option<String>` and `full_text: Option<String>`, but `TranscriptResult` declares them as non-optional `String`. This means:
- If someone edits `TranscriptResult` to truly make a field optional, `TranscriptFile` already tolerates it -- accidental tolerance.

---

- More importantly, this is a signal that the two types are already drifting. If `language` or `full_text` is ever missing from the JSON, `TranscriptResult` deserialization will fail elsewhere in the app, but the export module will silently succeed -- inconsistent behavior.

**Recommendation:** Reuse `TranscriptResult` directly instead of `TranscriptFile`:

```rust
use crate::transcription::types::TranscriptResult;
// In load_segments:
let transcript: TranscriptResult = serde_json::from_str(&content)...;
Ok(transcript.segments)
```

This eliminates an entire redundant struct and guarantees schema consistency.

### 5. PDF font loading: empty string `""` as first path is a silent no-op at best

**File:** line 97.

```rust
genpdf::fonts::from_files("", "LiberationSans", None)
```

Passing an empty string as the font directory means genpdf will look for `LiberationSans-Regular.ttf` in the current working directory. For a Tauri desktop app, the CWD is unpredictable (often `/` or the user's home). This first attempt will virtually never succeed and just adds noise.

**Recommendation:** Remove the empty-string attempt. Start directly with the system font paths.

### 6. PDF margins are very tight at 10 points

**File:** line 136.

`decorator.set_margins(10)` sets 10-point margins on all sides. That is approximately 3.5mm, which is below the printable area of most printers (typically 5-6mm minimum). Text will be clipped when printed.

**Recommendation:** Use at least 20-25 points (roughly 7-9mm), or the more conventional 36 points (~12.7mm / half inch).

### 7. `eprintln!` debug logging left in production code

**File:** lines 192 and 195.

```rust
eprintln!("[export] BUG-FIX: failed to parse transcript.json as TranscriptFile: {}", e);
eprintln!("[export] load_segments: parsed {} segments from transcript.json", transcript.segments.len());
```

The first one labels itself "BUG-FIX" which reads like a development debugging leftover. The second logs on every successful invocation. Both write to stderr. In a Tauri app, stderr is typically invisible to the user, so these are just noise. If logging is desired, use the `log` crate (or Tauri's built-in `log` plugin) with appropriate levels.

**Recommendation:** Replace with `tracing::debug!` / `tracing::warn!`, or remove entirely.

---


---

| 3 | Important | Duplicate `TranscriptSegment` type diverges from canonical `Segment` | 10-15 |

---

| 4 | Important | `TranscriptFile` field optionality mismatches `TranscriptResult` | 170-177 |

---

| 5 | Important | Empty string `""` font path is dead code | 97 |

---

| 6 | Important | 10-point PDF margins clip on print | 136 |

---

| 7 | Important | `eprintln!` debug logging in production code | 192, 195 |

---

## IMPORTANT Issues

### I1. Vulkan encoder element names are fictional

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/types.rs`, lines 78-80

The element names `vulkanh264enc`, `vulkanh265enc`, `vulkanav1enc` do not exist in GStreamer. GStreamer's Vulkan plugin provides `vulkanoverlaycompositor`, `vulkanupload`, `vulkandownload`, etc., but no Vulkan-based video encoders. The Vulkan Video encoding support in GStreamer is still experimental/nonexistent as of 1.26. These names will never resolve, making the `Vulkan` backend permanently unavailable. The probe will silently skip it, so this is not a crash risk, but it is dead code that provides a false option to users.

### I2. NVIDIA encoder element names may be wrong for gstreamer 0.25

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/types.rs`, lines 75-77


---

| IMPORTANT | 6 | Vulkan encoders fictional; hardcoded track info; no Drop impl; bitrate type risk; fd type clarity; NVIDIA AV1 naming |

---

When the `while let Some(chunk_result) = byte_stream.next().await` loop exits naturally (the HTTP body is fully consumed), any data remaining in `buffer` that does not end with `\n` is silently dropped. The final SSE chunk from some providers may arrive without a trailing newline, meaning the last `data: [DONE]` or -- more importantly -- the last `data: {"choices":[...]}` line is lost.

This means the last token of an LLM response can be silently swallowed.

Fix: After the `while let Some(...)` loop exits (after line 198's closing brace, but before the spawned task returns), process whatever remains in `buffer` using the same line-parsing logic:

```rust
// After the while loop ends, process any remaining buffered data.
if !buffer.is_empty() {
    let line = buffer.trim().to_string();
    if !line.is_empty() && !line.starts_with(':') {
        if let Some(data) = line.strip_prefix("data: ") {
            let data = data.trim();
            if data != "[DONE]" {
                if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                    for choice in &chunk.choices {
                        if let Some(ref content) = choice.delta.content {
                            if !content.is_empty() {
                                let _ = tx.send(Ok(content.clone())).await;
                            }
                        }
                    }
                }
            }
        }
    }
}
```

**2. `String::from_utf8_lossy` silently replaces invalid UTF-8 with replacement characters (line 146)**

If the HTTP response contains a multi-byte UTF-8 character that is split across two `Bytes` chunks (which is entirely possible -- TCP framing makes no guarantees about character boundaries), `from_utf8_lossy` will replace the partial bytes with U+FFFD in the first chunk and again in the second chunk. This corrupts the data.

The correct approach is to maintain a byte buffer (`Vec<u8>`) instead of a `String` buffer, and only decode complete lines (where you know the newline boundary gives you a complete code point sequence), or use something like `String::from_utf8` on each extracted line and handle the error. At minimum, switch from `String` buffer to `Vec<u8>` and split on the byte `b'\n'`.

---


---

### IMPORTANT Issues

**3. Lines not prefixed with `data:` are silently ignored (line 159)**

The SSE specification defines `event:`, `id:`, and `retry:` fields in addition to `data:`. While OpenAI-compatible APIs typically only use `data:`, some providers (notably OpenRouter, which is explicitly the target per the project docs) can send `event: error` lines followed by `data:` containing error details. When `event:` lines are silently skipped, the parser may misinterpret an error event's `data:` payload as a normal content chunk.

Recommendation: At minimum, track the most recent `event:` value. If `event: error` is seen, treat the next `data:` payload as an error rather than a content chunk.

**4. No timeout on the HTTP request or stream consumption (lines 109-116, 138-198)**

The `reqwest::Client` is created with `Client::new()` (line 88) which uses reqwest's defaults: no connect timeout and no overall timeout. If the API server hangs during connection or stops sending data mid-stream, the spawned task will block forever, leaking the task and holding the channel open indefinitely. The downstream `commands.rs` consumer at line 178 (`while let Some(...) = rx.recv().await`) will also hang forever, blocking the Tauri command.

Recommendation: Configure the client with a connect timeout and a read timeout:

```rust
client: Client::builder()
    .connect_timeout(std::time::Duration::from_secs(10))
    .timeout(std::time::Duration::from_secs(120))
    .build()
    .unwrap_or_default(),
```

Alternatively, or additionally, wrap the stream reads in `tokio::time::timeout`.

**5. `ChatMessage.role` is a bare `String` rather than an enum (line 10)**

The OpenAI chat API accepts a fixed set of roles: `"system"`, `"user"`, `"assistant"`, and optionally `"tool"`. Using a bare `String` means any typo (e.g., `"assitant"`) compiles and runs but produces an API error at runtime. A `Role` enum with `Serialize`/`Deserialize` would give compile-time safety. The downstream `prompt.rs` already hard-codes role strings -- an enum would make those usages safer.

**6. `chat_once` returns empty string on missing choices instead of an error (lines 252-257)**

```rust
Ok(resp.choices.into_iter().next()
    .map(|c| c.message.content)
    .unwrap_or_default())
```

If the API returns a 200 with an empty `choices` array (which can happen with content-filtered responses), this silently returns `""`. The caller has no way to distinguish "the LLM said nothing" from "the response was filtered/malformed." This should return a `ChatError::Stream("No choices in response")` or similar.

---


---

| 3 | Important | SSE `event:` field ignored; error events mishandled | 154-159 |

---

| 4 | Important | No timeouts on HTTP connect or stream reads; can hang forever | 88, 109-116 |

---

| 5 | Important | `role` as bare String lacks compile-time validation | 10 |

---

| 6 | Important | Empty choices silently returns "" instead of error | 252-257 |

---

Both `EmbeddingsClient` and `ChatClient` post directly to `self.url` without appending any path suffix. The settings placeholder says `https://openrouter.ai/api/v1` which means the user must provide the full endpoint URL (e.g., `https://openrouter.ai/api/v1/embeddings`). This is an important UX consideration since the placeholder is misleading.

Now I have all the information needed for a comprehensive review. Here is my analysis:

---

# Hlusra End-to-End Functionality Review

## Flow 1: Record Audio

**Trace:** RecordButton.tsx `handleStart()` --> `startRecording(withVideo)` --> `invoke("start_recording", { withVideo })` --> `recorder/commands.rs start_recording(with_video: bool, ...)` --> Library `prepare_meeting()` --> GStreamer pipeline --> polling --> `stopRecording()` --> `stop_recording` --> `pipeline.stop()` --> `finalize_meeting` --> gallery

### Parameter naming: WORKS
Tauri v2 automatically converts camelCase (`withVideo`) to snake_case (`with_video`). This is correct.

### Library `prepare_meeting()`: WORKS
Creates UUID, builds dir name `YYYY-MM-DD_<uuid8>`, calls `LibraryFs::create_meeting_dir()` which does `create_dir_all`. Records the `(id, dir_path)` in an in-memory HashMap for later `finalize_meeting`. Sound approach.

### GStreamer pipeline (audio-only): RISKY
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs` line 27-61: The audio-only pipeline uses `pipewiresrc` without `stream-properties`, meaning it relies on PipeWire's default input device. This works on most Wayland setups but could silently capture the wrong device or fail if no default input is configured.
- The pipeline only captures **one** audio track (mic), but `stop_recording` in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` line 104-107 **hardcodes two tracks** in the `RecordingInfo` (mic + system). This is a data inconsistency: the database will say there are 2 tracks, but the MKV only contains 1 audio stream. This will confuse export operations (the `amerge=inputs=2` filter in audio export will fail when there's actually only 1 stream).

### GStreamer pipeline (video): RISKY
- Screen capture via XDG Desktop Portal (ashpd) is correctly implemented. The `fd` lifetime management is correct -- `ScreenCapture` owns the `OwnedFd` and outlives the pipeline.
- The video pipeline attempts system audio capture with `pipewiresrc` using `media.class=Audio/Sink`, which on PipeWire attempts to capture a monitor source. This **may or may not work** depending on the PipeWire configuration. Many setups require explicit monitor node selection.

### Polling: WORKS
`getRecordingStatus()` every 1 second via `setInterval`. The backend reads `Instant::now().elapsed()` and `fs::metadata` for file size. Functional and clean.

### Stop recording: WORKS
Sends GStreamer EOS event, waits up to 5 seconds for propagation, then sets pipeline to Null state. Clean shutdown. `finalize_meeting` inserts into SQLite with correct data.

### Post-recording navigation: WORKS
After stop, `onRecordingDone` fires, which increments `galleryKey` and navigates to gallery. The Gallery component re-mounts with a new key, forcing a fresh `listMeetings()` call.

### Verdict: RISKY
The hardcoded 2-track metadata in `stop_recording` for audio-only recordings is a **data integrity bug** that will cause audio export to fail when using the `amerge=inputs=2` filter on a single-track MKV. The recording itself will work, but downstream operations are affected.

---

## Flow 2: View Gallery

**Trace:** Gallery.tsx `loadMeetings()` --> `listMeetings()` --> `invoke("list_meetings")` --> `library/commands.rs list_meetings` --> `LibraryDb::list_meetings()` --> SQL query ordered by `created_at DESC`

### Data mapping: WORKS
The Rust `MeetingSummary` struct uses `#[serde(rename_all = "snake_case")]` on all status enums. The TypeScript `MeetingSummary` interface matches field names exactly. The `created_at` field is serialized as RFC3339 string by chrono, and the frontend creates `new Date(iso)` which correctly parses ISO 8601/RFC3339.

### Card rendering: WORKS
`MeetingCard.tsx` correctly displays title, date, file size, duration, badges for transcription/chat status, and audio/video type indicator. The `formatDuration` helper handles hours/minutes/seconds correctly.

### Search: WORKS
Client-side filter by `title.toLowerCase().includes(search.toLowerCase())`.

### Verdict: WORKS

---

## Flow 3: View Meeting

**Trace:** Click card --> `MeetingPage` gets `meetingId` --> `getMeeting(meetingId)` --> `invoke("get_meeting", { id })` --> `library/commands.rs get_meeting` --> `Library::get_meeting_detail()` --> returns `MeetingDetail` with transcript

### Media player loading: RISKY
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx` line 91-98: Reads the entire MKV file into memory via `readFile(mediaPath)`, converts to a `Blob`, creates an `objectURL`, and assigns it to a `<video>` or `<audio>` element.
- **Memory concern**: For long meetings, MKV files can be hundreds of MB to GB. Loading the entire file into a JS `Uint8Array`, then duplicating into a `Blob`, will consume significant memory and potentially crash the browser process.
- **Browser codec support**: The MIME type is set to `video/x-matroska` or `audio/x-matroska`. **Most browsers (Chromium/WebView) do NOT support MKV container playback natively.** This means the `<audio>` and `<video>` elements will likely fail to play. Chromium supports WebM (which is a subset of MKV/Matroska) but only with VP8/VP9/AV1 video and Vorbis/Opus audio. The recordings use H.265 video and Opus audio in MKV, which **will not play** in the Tauri WebView.
- **Audio-only playback**: Even audio-only MKV with Opus inside will likely not play because Chromium doesn't recognize `audio/x-matroska` MIME type. The user would see the play button but hear nothing.
- **Tauri FS scope**: The scope in `default.json` allows `$HOME/Hlusra/**`, which matches the default `~/Hlusra/recordings/` directory. If the user changes `recordings_dir` in settings to a path outside this scope, `readFile` will be **permission-denied**.

### Transcript display: WORKS
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/api.rs` line 126-138: Correctly reads `transcript.json` first (structured JSON with segments), falling back to `transcript.txt` (plain text).
- `TranscriptView.tsx` tries `JSON.parse(transcript)` to get the structured view with clickable timestamps, falling back to plain text display. This flow is correct.

### Seek functionality: RISKY
`handleSeek` sets `mediaElRef.current.currentTime = time` and calls `play()`. This depends on the media player actually being functional (see the MKV playback issue above).

### Verdict: BROKEN for media playback (MKV/H.265 will not play in Chromium WebView). RISKY for memory (large files). The transcript display, metadata, and actions all work correctly.

---

## Flow 4: Transcribe

**Trace:** Click "Transcrever" --> `transcribeMeeting(meetingId)` --> `invoke("transcribe_meeting", { id })` --> `transcription/commands.rs transcribe_meeting` --> spawns blocking thread --> FFmpeg extracts mic track to WAV --> whisper-rs processes --> saves `transcript.json` + `transcript.txt`

### FFmpeg extraction: WORKS
`extract_mic_to_wav` in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/orchestrator.rs` runs `ffmpeg -y -i input.mkv -map 0:a:0 -ac 1 -ar 16000 -codec:a pcm_s16le output.wav`. Correctly targets the first audio stream, resamples to 16kHz mono PCM. Requires `ffmpeg` on PATH.

### Local whisper-rs processing: WORKS
`LocalProvider` in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/local.rs` loads the WAV via `hound`, validates 16kHz mono, creates a `WhisperContext`, runs inference with greedy sampling, extracts segments and word-level timestamps. Returns `TranscriptResult` with `language`, `segments`, and `full_text`.

### API provider: WORKS
`ApiProvider` in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/transcription/api.rs` posts to `{base_url}/v1/audio/transcriptions` with multipart form data. Correctly appends the path suffix here (unlike the RAG endpoints -- see Flow 5).

### Artifact saving: WORKS
Both `transcript.json` (pretty-printed) and `transcript.txt` (full text only) are saved as library artifacts. Status is updated to `Done` on success, `Failed` on error.

### Polling for status: WORKS
`TranscriptView.tsx` starts polling `getTranscriptionStatus(meetingId)` every 2 seconds after calling `transcribeMeeting`. When status becomes "done" or "failed", clears the interval and calls `onStatusChange()` which triggers `loadMeeting()` in the parent, refreshing the transcript display.

### Model not downloaded: RISKY
If the user hasn't downloaded a whisper model, `LocalProvider::transcribe()` returns an error "Model file not found: ... Download it first." The user must navigate to Settings > Transcription > Local > download a model before transcription works. The error message is clear, but there's no in-flow guidance.

### Verdict: WORKS (assuming ffmpeg is installed and a whisper model is downloaded or API is configured)

---

## Flow 5: Chat (RAG)

**Trace:** Navigate to chat --> ChatPanel.tsx --> click "Indexar reuniao" --> `indexMeeting(meetingId)` --> `invoke("index_meeting", { id })` --> reads transcript.json --> chunks it --> embeds via API --> stores in sqlite-vec --> status becomes "ready" --> user types message --> `chatMessage(meetingId, message)` --> embeds question --> vector search --> LLM call --> stream response via Tauri events

### Indexing: WORKS
`do_index_meeting` in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/commands.rs` reads `transcript.json`, chunks it via `chunk_transcript`, embeds in batches of 50 via `EmbeddingsClient`, initializes the sqlite-vec virtual table if needed, and inserts chunks with embeddings. Status transitions: NotIndexed --> Indexing --> Ready/Failed.

### RAG config refresh: WORKS
`refresh_rag_config()` reloads from the TOML settings file before every indexing and chat operation. This means changes in Settings take effect immediately without app restart.

### Embeddings URL: RISKY -- misleading UX
The `EmbeddingsClient` posts directly to `self.url` at line 114 of `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/embeddings.rs`. The settings placeholder says `https://openrouter.ai/api/v1`, but the user must provide the **full endpoint URL** including the path (e.g., `https://openrouter.ai/api/v1/embeddings`). The same applies to the chat URL -- it must be `https://openrouter.ai/api/v1/chat/completions`. If the user enters just the base URL as the placeholder suggests, the API calls will hit the wrong endpoint and fail.

Compare with the transcription API provider which correctly appends `/v1/audio/transcriptions` to the base URL. This inconsistency is confusing.

### sqlite-vec availability: RISKY
`VectorStore::load_sqlite_vec` in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/vector_store.rs` line 104-121 tries to load the `vec0` extension. If it fails, it prints a warning but **continues without vector search**. This means `init_vector_table` will fail (it tries to `CREATE VIRTUAL TABLE ... USING vec0`), and vector search queries will fail. The error messages will be obscure SQLite errors rather than a clear "install sqlite-vec" message. The TODO at line 103 acknowledges this: "Bundle the sqlite-vec shared library with the application."

### Chat streaming: WORKS
The SSE stream parser in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/chat.rs` correctly handles `data: [DONE]`, parses `StreamChunk` JSON, and forwards text content through an `mpsc` channel. The Tauri command emits `chat-stream-chunk`, `chat-stream-done`, and `chat-stream-error` events to the `"main"` window, which matches the window label in `tauri.conf.json`.

### Frontend event handling: WORKS
`ChatPanel.tsx` sets up listeners before calling `chatMessage`, appends chunks to the last assistant message, and cleans up listeners when done. The `mountedRef` guard prevents state updates after unmount.

### Model change error: RISKY
`VectorStore::ensure_ready` returns `ModelChanged` error if the user switches embedding models between indexing operations. The error message is descriptive, but the frontend doesn't offer an automatic "reindex with new model" flow -- the user would see a generic error.

### Verdict: RISKY -- requires sqlite-vec extension to be installed on the system (not bundled), and the URL fields require full endpoint paths despite misleading placeholders

---

## Flow 6: Export

**Trace:** Select format --> `exportAudio/Video/Transcript(id, format, saveMode)` --> `invoke("export_audio/video/transcript", { id, format, saveMode })` --> FFmpeg processing or file copy --> returns output path

### SaveMode serialization: WORKS
TS sends `{ mode: "save" }` or `{ mode: "save_as", path: string }`. Rust's `SaveMode` has `#[serde(rename_all = "snake_case", tag = "mode")]` with variants `Save` and `SaveAs { path: PathBuf }`. The internally-tagged representation matches exactly.

### Audio export: RISKY
As noted in Flow 1, the `stop_recording` command hardcodes 2 tracks for audio-only recordings. The export audio function in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/audio.rs` calls `count_audio_streams` via ffprobe and applies `amerge=inputs=2` filter when count >= 2. For audio-only recordings (1 stream), this filter is skipped, so it works. But the metadata mismatch (DB says 2 tracks, file has 1) could confuse users looking at the track count.

For video recordings with 3 streams (video + mic + system), the `amerge=inputs=2` filter correctly merges the two audio tracks. If system audio capture didn't work (PipeWire routing issue) and the MKV has only 1 audio stream + video, the merge filter would fail.

### Video export: WORKS
Stream-copies when codec matches (H.265 source to H.265 target), transcodes when codec changes (H.265 to H.264). Audio is copied as-is. Uses `ffmpeg` CLI.

### Transcript export (TXT, JSON): WORKS
Simply copies `transcript.txt` or `transcript.json` from the meeting directory.

### Transcript export (SRT): WORKS
The `load_segments` function in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/transcript.rs` correctly deserializes the `TranscriptFile` wrapper object (not a bare array), extracting segments. The SRT formatting with proper timestamp format (`HH:MM:SS,mmm`) is correct.

### Transcript export (PDF): WORKS
Uses `genpdf` with fallback font detection across multiple Linux font paths. Generates a formatted document with timestamps and segment text.

### Verdict: WORKS (assuming ffmpeg is installed and the correct number of audio streams are present)

---

## Flow 7: Settings

**Trace:** Load settings --> `getSettings()` --> `invoke("get_settings")` --> `settings/commands.rs get_settings` --> `load_settings()` from TOML --> returns `AppSettings` --> Save settings --> `updateSettings(settings)` --> `invoke("update_settings", { settings })` --> `save_settings()` to TOML

### Load/Save: WORKS
TOML round-trip is tested. Default creation on first run works. All settings fields match between TS and Rust interfaces.

### Settings affect recording: BROKEN -- they do NOT
The recorder uses `VideoConfig::default()` and `AudioConfig::default()` hardcoded in `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` line 40-41. **The user's settings are completely ignored.** Changing codec, backend, bitrate, FPS, or resolution in Settings has zero effect on recordings. The Library is initialized once at startup from `get_recordings_dir()`, but the recorder pipeline never reads from the settings config.

### Settings affect Library recordings_dir: BROKEN on restart only
The `Library` is constructed once in `lib.rs` with `get_recordings_dir()` at startup. If the user changes `recordings_dir` in Settings and saves, the TOML file is updated, but the running `Library` instance still uses the old directory. Only after restarting the app would the new directory take effect. Worse, there is no indication to the user that a restart is needed.

### Verdict: BROKEN -- recording settings (codec, backend, bitrate, FPS, resolution) are never read; they are hardcoded to defaults. The `recordings_dir` change requires an app restart.

---

## Summary Table

| Flow | Status | Key Issues |
|------|--------|------------|
| **1. Record Audio** | RISKY | Hardcoded 2-track metadata for 1-track audio-only recordings; system audio capture may not work |
| **2. View Gallery** | WORKS | Clean implementation |
| **3. View Meeting** | BROKEN | MKV/H.265 will not play in Chromium WebView; entire file loaded into memory |
| **4. Transcribe** | WORKS | Requires ffmpeg + downloaded model or API config |
| **5. Chat (RAG)** | RISKY | sqlite-vec not bundled; embeddings/chat URLs require full paths despite misleading placeholders |
| **6. Export** | WORKS | Depends on ffmpeg and correct audio stream count |
| **7. Settings** | BROKEN | Recording settings are never applied (hardcoded defaults); recordings_dir change needs restart |

---


---

## Important Issues (Should Fix)

**4. RAG API URLs require full endpoint paths** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/embeddings.rs` line 114, `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/chat.rs` lines 111, 219)

Unlike the transcription API provider (which appends `/v1/audio/transcriptions`), the RAG clients post directly to the stored URL. But the settings UI placeholder says `https://openrouter.ai/api/v1`, which is a base URL that won't work without appending `/embeddings` or `/chat/completions`.

**Fix:** Either append the path suffix in the client code (like the transcription API does), or update the placeholder text to show the full URL (e.g., `https://openrouter.ai/api/v1/embeddings`).

**5. sqlite-vec not bundled** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/vector_store.rs` lines 104-121)

If `vec0` is not installed on the user's system, chat/indexing silently degrades. The `ensure_ready` call will fail with an obscure SQLite error when trying to create the virtual table.

**Fix:** Bundle the `vec0.so` shared library with the application, or at minimum provide a clear error message when the extension fails to load and chat features are attempted.

**6. FS scope too narrow for custom recordings_dir** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/capabilities/default.json` line 21)

The Tauri FS scope only allows `$HOME/Hlusra/**`. If the user changes `recordings_dir` to another location (e.g., `/data/meetings/`), `readFile` in MeetingPage.tsx will fail with a permission error.

**Fix:** Dynamically adjust the FS scope based on the configured `recordings_dir`, or add a broader scope pattern.

**7. `recordings_dir` change requires restart** (`/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/lib.rs` lines 55-58)

The `Library` is constructed once with `get_recordings_dir()`. Settings changes to `recordings_dir` are written to TOML but not applied until restart.

**Fix:** Either (a) hot-reload the Library's base directory when settings change, or (b) display a clear "restart required" message in the UI after changing the recordings directory.

---


---

## IMPORTANT Issues

### 2. `get_thumbnail` command registered but never invoked from frontend

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/commands.rs` (line 27-35)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/api.ts` -- no corresponding function

The `get_thumbnail` command is registered in `generate_handler![]` but has no frontend wrapper in `api.ts` and is never called from any component. This is dead code. Either implement the thumbnail feature end-to-end or remove the command to reduce surface area.

### 3. FS scope may be too narrow if the user customizes `recordings_dir`

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/capabilities/default.json` (lines 21-22)
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx` (line 93)

The `fs:scope` allows `$HOME/Hlusra/**`, but the Settings UI lets the user change `recordings_dir` to any path. If changed to, say, `/data/meetings/`, the `readFile` call in `MeetingPage.tsx` (line 93) will be denied by the FS scope at runtime. The user would see media loading fail silently (the catch block only logs to console).

Possible fixes:
- Dynamically add the user's configured recordings path to the FS scope at startup
- Or use the `asset:` protocol scope instead
- Or perform all file reads through Tauri commands (backend-side) where the FS plugin scope does not apply

### 4. `export_audio`, `export_video`, `export_transcript` are synchronous (non-async) commands that may block the main thread

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/commands.rs`

These three commands call FFmpeg and PDF generation synchronously. In Tauri v2, synchronous `#[tauri::command]` functions run on the main thread (unless marked `async`). FFmpeg transcoding can take minutes for large files, during which the entire app will freeze -- no UI updates, no other commands processed.

These should be made `async` and spawn the heavy work via `tokio::task::spawn_blocking`, following the same pattern already used by `transcribe_meeting` and `download_model`.

### 5. `stop_recording` is synchronous and calls `pipeline.stop()` which waits for EOS

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` (line 79)

`stop_recording` is a synchronous command. The `pipeline.stop()` method sends EOS and waits for the pipeline to flush, which can take several seconds depending on buffer sizes and encoder flush. During this time the UI will be unresponsive. This should be `async` with `spawn_blocking` around the pipeline stop.

### 6. Frontend `SaveMode` type definition does not match Rust tagged enum shape

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/lib/api.ts` (lines 155-157)

The TypeScript definition is:
```typescript
export type SaveMode =
  | { mode: "save" }
  | { mode: "save_as"; path: string };
```

The Rust definition uses `#[serde(rename_all = "snake_case", tag = "mode")]` which serializes `SaveAs { path }` as `{ "mode": "save_as", "path": "..." }`. This actually matches because Tauri deserializes the JSON from the frontend. **However**, the `path` field in Rust is `PathBuf` while the TypeScript uses `string`. This is fine for deserialization, but worth noting for type documentation accuracy.

Actually, on closer inspection this works correctly. Lowering severity -- this is not an issue.

---


---

| 2 | IMPORTANT | Commands | `get_thumbnail` registered but never invoked from frontend |

---

| 3 | IMPORTANT | FS Scope | Custom `recordings_dir` paths will be denied by FS scope |

---

| 4 | IMPORTANT | Threading | Export commands are synchronous; will block UI during FFmpeg |

---

| 5 | IMPORTANT | Threading | `stop_recording` is synchronous; blocks UI during pipeline flush |

---

## IMPORTANT Issues

### 4. TOCTOU race in artifact methods (`save_artifact`, `has_artifact`, `read_artifact`, `get_artifact_path`)

**Location:** Lines 194-212

All four artifact methods call `self.get_meeting(id)` which acquires and immediately releases the `db` mutex, then operates on the returned `dir_path` outside any lock. A concurrent `delete_meeting(..., DeleteMode::Everything)` can delete the directory between the `get_meeting` call and the filesystem operation.

For example, `save_artifact`:
1. Thread A: `save_artifact("m1", Recording, data)` -- calls `get_meeting("m1")`, gets `dir_path`, releases lock.
2. Thread B: `delete_meeting("m1", Everything)` -- acquires lock, deletes dir, deletes DB row, releases lock.
3. Thread A: `fs.save_artifact(dir_path, ...)` -- `dir_path` no longer exists, `fs::write` fails with `NotFound`.

In a Tauri app this is somewhat mitigated because most commands come from the single UI thread, but the Tauri command system runs handlers on a thread pool, so concurrent IPC calls can race.

**Recommendation:** For `save_artifact` and `read_artifact`, consider holding the db lock for the entire operation, or at minimum accepting that these can fail with IO errors and handling that gracefully at the call site. Documenting this limitation would also help.

### 5. `get_meeting_detail` does not hold the db lock while reading transcripts -- stale data risk

**Location:** Lines 118-154

`get_meeting_detail` calls `self.get_meeting(id)` (locks/unlocks db), then reads transcript files from the filesystem without any lock. Between the db read and the fs read, another thread could delete the meeting or overwrite the transcript. The `meeting` struct returned has status fields that might be stale relative to the transcript content returned.

This is the same class of issue as #4 but specifically impacts the detail view.

### 6. `finalize_meeting` acquires two different mutexes sequentially -- theoretical deadlock if lock ordering is ever violated elsewhere

**Location:** Lines 78-111

`finalize_meeting` locks `prepared` first (line 84), then locks `db` (line 107). If any future code path acquires `db` first and then `prepared`, it would deadlock. Currently no such code path exists, but the lock ordering is not documented or enforced.

**Recommendation:** Add a comment documenting the lock ordering invariant: `prepared` must always be acquired before `db`. Better yet, restructure so that the `prepared` lock is released before acquiring `db` (which is actually the current behavior -- the `MutexGuard` from the `prepared` lock is dropped after the `.remove()` call returns, since it is a temporary). Let me verify this more carefully.

Actually, looking again at lines 84-90: the lock guard is a temporary expression. The chain `.lock()?.remove(id).ok_or(...)? ` means the `MutexGuard` is dropped at the semicolon on line 90, before `db` is locked on line 107. So the two locks are NOT held simultaneously. This is fine. But this relies on Rust's temporary lifetime rules and is fragile -- a refactor that stores the guard in a variable would silently introduce the overlap. A brief comment would prevent this.

### 7. `prepared` HashMap grows unboundedly if `prepare_meeting` is called without `finalize_meeting`

**Location:** Line 33, Lines 58-75

Related to issue #1 but specifically about memory: each `prepare_meeting` call inserts into the HashMap and only `finalize_meeting` removes from it. If the UI code has a bug that calls `prepare_meeting` repeatedly without finalizing, the HashMap grows without bound for the lifetime of the process.

**Recommendation:** Consider a bounded map or a maximum size check in `prepare_meeting`. Also, add a `cancel_prepared` method that removes the entry and cleans up the directory.

---


---

| 4 | IMPORTANT | TOCTOU in artifact methods (get_meeting then fs op) | 194-212 |

---

| 5 | IMPORTANT | Stale data in `get_meeting_detail` (lock released before fs read) | 118-154 |

---

| 6 | IMPORTANT | Undocumented lock ordering between `prepared` and `db` | 78-111 |

---

| 7 | IMPORTANT | Unbounded `prepared` HashMap growth | 33, 58-75 |

---

### IMPORTANT Issues

**3. `retranscribe_meeting` sets status to `Pending` then immediately overwrites it with `Processing` (lines 118-127)**


---

The flow is: set `Pending` -> call `transcribe_meeting` -> `transcribe_meeting` sets `Processing`. The intermediate `Pending` state is visible to the frontend for a brief moment (or not at all), and performs a redundant database write. More importantly, if the goal of `retranscribe_meeting` is to re-run even when status is `Done`, the current code works, but if the concurrent-guard from issue #2 were added (reject if `Processing`), then `retranscribe_meeting` would need to set the status to something other than `Processing` before calling `transcribe_meeting`.

**Recommendation**: Either (a) remove the redundant `Pending` write and pass a flag to `transcribe_meeting` to skip the status guard, or (b) make `transcribe_meeting` accept any non-`Processing` status and set it atomically.


---

Severity: **Important** -- wasted DB write now, will become a logic bug if concurrency guard is added.

**4. `create_provider` silently falls back to local for any unrecognized provider string (line 28-33)**

The catch-all `_ =>` arm means a typo like `"apii"` or `"API"` (case mismatch) in the settings file will silently use the local provider instead of the intended API provider. The user would see no indication that their setting was ignored.

**Recommendation**: Match `"local"` explicitly and return an error for unknown values:

```rust
"local" => { ... }
other => Err(format!("Unknown transcription provider: '{other}'. Expected 'api' or 'local'."))
```


---

Severity: **Important** -- silent misconfiguration is a user-hostile failure mode.

**5. `create_provider` for the `"api"` path does not validate that `api_url` and `model` are non-empty (lines 23-26)**

If the user has `provider = "api"` but left `api_url` or `model` empty in their TOML config, the `ApiProvider` is created with empty strings. The failure will only surface later as a confusing HTTP error (request to empty URL). The `api_key` being empty is intentional (local API servers), but `api_url` and `model` should be validated eagerly.

**Recommendation**: Add validation in `create_provider`:

```rust
if url.is_empty() {
    return Err("API transcription requires 'api_url' in settings".to_string());
}
if model.is_empty() {
    return Err("API transcription requires 'model' in settings".to_string());
}
```


---

Severity: **Important** -- produces confusing downstream errors instead of a clear message.

**6. Error string from the `Err` branch of `result` is passed through without context (line 111)**

On line 111, `Err(err)` returns the raw error string from the orchestrator. Unlike every other error in the function, this one has no prefix indicating it came from the transcription pipeline. If the orchestrator returns something like `"Failed to run ffmpeg: No such file"`, that is clear enough, but if it returns a bare provider error, the frontend gets no context about which step failed.

**Recommendation**: Wrap it:

```rust
Err(err) => {
    let _ = library.update_transcription_status(&id, TranscriptionStatus::Failed);
    Err(format!("Transcription failed: {err}"))
}
```


---

Severity: **Important** -- affects debuggability.

---


---

| 3 | `retranscribe_meeting` redundant Pending write | Important | 118-127 |

---

| 4 | Silent fallback for unknown provider strings | Important | 28-33 |

---

| 5 | No validation of `api_url`/`model` for API provider | Important | 23-26 |

---

| 6 | Raw error string passed through without context prefix | Important | 111 |

---

**IMPORTANT 1: The `amerge` filter is hardcoded to exactly 2 inputs, but `count_audio_streams` only checks `>= 2`.**

If a file somehow has 3+ audio streams (unlikely in this app, but defensively), the filter still says `inputs=2` and only maps `[0:a:0][0:a:1]`, ignoring the rest. This is actually fine behavior (merge only mic + system), but the `stream_count >= 2` guard should arguably be `stream_count == 2` or the logic should be documented to clarify that only the first two streams are merged intentionally.


---

**IMPORTANT 2: Opus export with multi-track does not mix and does not map streams explicitly.**

When `format` is `AudioFormat::Opus`, `requires_mixdown()` returns `false`, so the code skips the filter entirely. The resulting command is:
```
ffmpeg -y -i recording.mkv -vn -codec:a libopus audio.opus
```

FFmpeg's default behavior with multiple audio streams and no `-map` is to select only the "best" audio stream (typically the first one). So for Opus export, only the mic track is exported and the system audio track is silently dropped. The comment in `types.rs` says "Opus can preserve tracks in supported containers," but the `.opus` container (OGG) does NOT support multiple audio streams well. If the intent is to export only mic audio for Opus, this works but should be documented. If the intent is to preserve both tracks, this needs `-map 0:a` to select all audio streams, and the output container should be `.mka` or `.mkv` instead of `.opus`.


---

The pan filter `pan=stereo|c0<c0+c1|c1<c0+c1` mixes both channels of the amerge output into both left and right, producing a mono-like stereo signal. This is fine for a meeting recorder where spatial separation is not important, but worth noting: the mic will be in both speakers equally, as will system audio. An alternative approach would be `pan=stereo|c0<c0|c1<c1` to keep mic on left and system audio on right, but the current choice is reasonable for a meeting context.


---

**IMPORTANT 3: Stream-copying H.265 into MP4 may fail or produce non-compliant files.**

At line 39-41 in `video.rs`, when `VideoFormat::Mp4H265` is selected:
```rust
VideoFormat::Mp4H265 | VideoFormat::MkvH265 => {
    cmd.arg("-codec:v").arg("copy");
}
```

The source video could be encoded by any of the GStreamer backends (VAAPI, CUDA, Vulkan, software) and could actually be H.264 or AV1, not necessarily H.265. The `export_video` function and its comment assume the source is always H.265, but the recording pipeline supports H.264 and AV1 as well (see `VideoCodec` enum in `types.rs` and the codec selection in `pipeline.rs`).

If the user recorded in H.264 and then selects "Export as Mp4H265", the code will stream-copy the H.264 stream but label/imply it as H.265. The file will play (since stream-copy preserves the actual codec), but the export name is misleading.

Conversely, if the user recorded in AV1 and selects "Export as Mp4H264," the code will try to transcode using `libx264`, which expects raw video input, not AV1. FFmpeg will decode the AV1 and re-encode to H.264, so this actually works, but it is slow and the user gets no feedback that a full transcode is happening.

The real fix here is to probe the source codec before deciding the copy-vs-transcode strategy. Either:
1. Probe the source codec with `ffprobe` and match it against the target.
2. Store the recording codec in the meeting metadata and pass it to `export_video`.


---

**IMPORTANT 4: `-codec:a copy` with Opus audio in MP4 container may fail.**

The recording pipeline uses `opusenc` (Opus codec) for audio. When exporting to MP4 with `-codec:a copy`, FFmpeg will attempt to copy the Opus stream into an MP4 container. While MP4 technically supports Opus (since FFmpeg 3.x / ISO 14496-12), many players and tools do not handle Opus-in-MP4 well. Some FFmpeg builds may even refuse this with "Could not find tag for codec opus in stream."

For MP4 output, consider transcoding audio to AAC:
```rust
match format {
    VideoFormat::Mp4H264 | VideoFormat::Mp4H265 => {
        cmd.arg("-codec:a").arg("aac");
    }
    _ => {
        cmd.arg("-codec:a").arg("copy");
    }
}
```


---

**IMPORTANT 5: The `-f` flag is redundant when the output extension matches.**

At line 53, the code explicitly sets `-f mp4` or `-f matroska`. FFmpeg infers the format from the output file extension. This is not harmful (explicit is good), but if the `SaveMode::SaveAs` path has a different extension than expected (e.g., user picks `output.mkv` but format is `Mp4H264`), the `-f mp4` flag will force MP4 muxing into a `.mkv` file, which will confuse players. This is actually a defensive correctness measure (the format flag overrides the extension), so it is arguably correct behavior -- but it means the output file extension might not match its actual format.


---

| 2 | IMPORTANT | `audio.rs:87-89` | Opus export silently drops the second audio track with no user feedback |

---

| 3 | IMPORTANT | `video.rs:38-47` | Stream-copy vs transcode decision assumes source is always H.265, but the recorder supports H.264 and AV1 |

---

| 4 | IMPORTANT | `video.rs:50` | `-codec:a copy` puts Opus into MP4, which has poor player compatibility |

---

| 5 | IMPORTANT | `audio.rs:67` | `stream_count >= 2` with hardcoded `inputs=2` does not account for 3+ streams |

---

## IMPORTANT Issues (Should Fix)

### 3. `filter_map(|s| s.ok())` silently swallows sample read errors (lines 39, 46)

```rust
reader
    .into_samples::<i32>()
    .filter_map(|s| s.ok())   // <-- silently drops errors
    .map(|s| s as f32 / max)
    .collect()
```

If a WAV file is partially corrupted, this will silently drop corrupted samples, producing shorter audio without any warning. This can lead to subtle timing misalignment between the audio and the transcript.

**Recommendation**: Either propagate the first error, or at minimum log a warning. A clean approach:
```rust
reader
    .into_samples::<i32>()
    .collect::<Result<Vec<i32>, _>>()
    .map_err(|e| format!("Failed to read WAV samples: {e}"))?
    .into_iter()
    .map(|s| s as f32 / max)
    .collect()
```

### 4. `full_n_segments()` return type -- casting `i32` to `usize` without bounds check (line 95)

```rust
let num_segments = state.full_n_segments()...?;
let mut segments = Vec::with_capacity(num_segments as usize);
```

In whisper-rs, `full_n_segments()` returns `Result<i32, ...>`. Casting a negative `i32` to `usize` with `as` would produce a huge number, causing a massive allocation or panic. While whisper.cpp should never return a negative segment count, defensive code should guard:

```rust
let num_segments = state.full_n_segments()...?;
if num_segments < 0 {
    return Err("Whisper returned negative segment count".into());
}
let num_segments = num_segments as usize;
```

The same applies to `full_n_tokens()` on line 125-127.

### 5. `get_lang_str` may not exist as a free function (line 102)

```rust
if let Some(lang) = whisper_rs::get_lang_str(lang_id) {
```

In whisper-rs 0.14, language lookup might be a method on `WhisperContext` rather than a free function. The function `whisper_rs::get_lang_str` was introduced at some point but its signature and availability in 0.14 specifically needs verification. If it does not exist, this would be a compilation error.

**Confidence**: Medium -- if the project compiles, this is fine. But worth noting for review.

### 6. `full_lang_id_from_state()` called only when `num_segments > 0`, but it does not depend on segments (lines 100-106)

```rust
if num_segments > 0 {
    if let Ok(lang_id) = state.full_lang_id_from_state() {
```

`full_lang_id_from_state()` retrieves the detected language from the whisper state, which is set during `state.full()` regardless of how many segments were produced. The guard `num_segments > 0` is technically unnecessary and could mask the detected language in edge cases where whisper detects a language but produces no segments (e.g., silence). This is minor but the guard is misleading.

---


---

| 3 | IMPORTANT | `filter_map(ok())` silently drops corrupted WAV samples | 39, 46 |

---

| 4 | IMPORTANT | Negative `i32` to `usize` cast on segment/token counts | 95, 127 |

---

| 5 | IMPORTANT | `whisper_rs::get_lang_str` existence as free function needs verification | 102 |

---

| 6 | IMPORTANT | Unnecessary `num_segments > 0` guard on language detection | 100 |

---

## IMPORTANT

### I1. Hardcoded track metadata in `stop_recording` does not match actual pipeline configuration

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/commands.rs` (lines 100-108)

`stop_recording` always reports two tracks (mic + system, both opus) regardless of whether the recording was audio-only (which has only one mic track) or video (which has three streams: video + mic + system). The `has_video` flag is correct from the pipeline, but the `tracks` metadata is always wrong for audio-only recordings:

```rust
tracks: vec![
    TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
    TrackInfo { index: 1, label: "system".to_string(), codec: "opus".to_string() },
],
```

For audio-only mode (`build_audio_only`), only one mic track exists. This metadata is stored in the database and would confuse any downstream consumer (e.g., the export audio mixdown logic that decides whether to apply `amerge` based on stream count).

**Recommendation:** Build the tracks list dynamically based on `pipeline.has_video()` and the actual pipeline configuration.

### I2. `init_vector_table` uses string interpolation for SQL DDL -- dimension injection

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/vector_store.rs` (lines 194-198)

```rust
let create_sql = format!(
    "CREATE VIRTUAL TABLE chunks_vec USING vec0(
        chunk_id TEXT PRIMARY KEY,
        embedding float[{dimension}]
    );"
);
```

The `dimension` parameter is a `usize`, so it cannot carry SQL injection payload. However, this sets a pattern of string-interpolating into SQL. Since `dimension` is derived from the first embedding vector length (from an external API response in `do_index_meeting`), a malicious or buggy embedding API returning dimension 0 would create a `float[0]` column, which may cause sqlite-vec to behave unpredictably.

**Recommendation:** Add a bounds check (e.g., `if dimension == 0 || dimension > 8192 { return Err(...) }`) before constructing the DDL. The type safety of `usize` does protect against SQL injection proper, so this is about defensive validation rather than a security vulnerability.

### I3. `insert_chunks` uses `assert_eq!` which panics in production

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/vector_store.rs` (line 274)

```rust
assert_eq!(chunks.len(), embeddings.len());
```

This will cause a panic (and crash) in production if there is ever a mismatch between chunks and embeddings counts. This could happen due to a bug in the embedding batch API response.

**Recommendation:** Return a `VectorStoreError` instead of panicking:
```rust
if chunks.len() != embeddings.len() {
    return Err(VectorStoreError::Db(rusqlite::Error::InvalidParameterCount(
        chunks.len(), embeddings.len()
    )));
}
```
Or add a new error variant to `VectorStoreError`.

### I4. `export_audio` assumes exactly 2 audio streams for mixdown

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/audio.rs` (lines 69-71)

The amerge filter is hardcoded for exactly 2 inputs:
```rust
cmd.arg("-filter_complex")
    .arg("[0:a]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]")
```

Combined with issue I1 (tracks metadata is always 2 even for audio-only), this could produce incorrect FFmpeg commands. If there is genuinely only 1 audio stream, the `stream_count >= 2` guard (line 67) would prevent this, but the approach is fragile. If a future pipeline has 3 audio streams, this would also fail.

**Recommendation:** Make the amerge filter dynamic based on the actual `stream_count`, or document the assumption clearly.

### I5. `video.rs` assumes source is always H.265 -- incorrect stream-copy decision

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/export/video.rs` (lines 38-47)

The comment says "Source is H.265/MKV" and the code stream-copies for H.265 targets. However, the recording pipeline uses `create_video_encoder_with_fallback` which can fall back to H.264 or AV1. If the fallback selected H.264 but the user exports as MkvH265, the code would stream-copy an H.264 stream labeled as H.265, producing a broken file with no error.

**Recommendation:** Either probe the source codec with ffprobe before deciding on stream-copy vs. transcode, or store the actual encoder codec used in the meeting metadata.

### I6. `start_time` is set at construction but overwritten in `start()` -- duration includes build time on error paths

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/recorder/pipeline.rs` (lines 58, 170)

`start_time` is set to `Instant::now()` in the constructor (line 58/161), then overwritten in `start()` (line 170). If `start()` is not called (error path), `duration_secs()` would return time since construction. This is minor but could produce confusing duration values in error logs.

**Recommendation:** Use `Option<Instant>` for `start_time` and only set it in `start()`.

---


---

### IMPORTANT issues (should fix)

**I1. Hardcoded file path separator and filename (line 91)**

```ts
const mediaPath = meeting.dir_path + "/recording.mkv";
```

This uses a hardcoded forward slash and a hardcoded filename. While Linux always uses `/`, this is a knowledge duplication issue -- the backend already knows the file path. If the backend ever changes the filename or adds a different container format, this breaks silently. The backend should return the full media path, or at least the filename.

**I2. The hidden media player inside `className="hidden"` may not decode on all browsers (line 333)**

Setting `display: none` (which Tailwind's `hidden` class does) on a media element means some browser engines may not allocate decoding resources or may pause the element automatically. This is browser-dependent behavior. A more reliable approach is to use `position: absolute; width: 0; height: 0; overflow: hidden` or similar offscreen technique, or (better) for audio-only recordings, simply use an `<audio>` element that is styled minimally rather than hidden.

**I3. `handleSaveTitle` is async but called from `onBlur` -- potential race condition (line 247)**

When the user presses Enter, `handleSaveTitle` is called from `handleTitleKeyDown`. Then focus moves away, triggering `onBlur`, which calls `handleSaveTitle` a second time. If the first call is still in-flight (awaiting `updateMeetingTitle`), the second call runs concurrently. While the current code would likely produce a duplicate API call rather than corruption, it is wasteful and could show a brief flicker. Add a guard:

```ts
const [savingTitle, setSavingTitle] = useState(false);

async function handleSaveTitle() {
  if (!meeting || savingTitle) return;
  // ...
}
```

Or, on Enter, blur the input and let `onBlur` be the single save trigger.

**I4. Media blob URL is not cleaned up when `meetingId` prop changes without unmounting (line 85-108)**

If the parent changes `meetingId` without unmounting `MeetingPage` (unlikely with current architecture, but possible), the effect dependency on `meeting?.id` would change. The cleanup runs and revokes the old URL, but `setMediaBlobUrl(null)` is never called (same as C1), so for a brief moment the old revoked URL is passed to the media element.

**I5. No loading indicator for media file (line 85-108)**

The media blob loads asynchronously (potentially for a long time given C2's in-memory loading). During this time, the play button is simply not shown. There is no loading spinner or "loading media..." indicator to tell the user that media is being loaded. This could be confusing for large files.

**I6. `loadMeeting` does not reset the `error` state on retry (line 68-78)**

If `loadMeeting` fails, `error` is set. If the user somehow triggers `loadMeeting` again (e.g., after retranscribe or reindex which call `loadMeeting`), and this time it succeeds, the old error is still displayed because `setError(null)` is never called at the start of `loadMeeting`.

```ts
const loadMeeting = useCallback(async () => {
  try {
    setError(null); // <-- add this
    const data = await getMeeting(meetingId);
```

**I7. Delete confirmation modal is not keyboard-accessible (line 398-428)**

The modal has no focus trap, no Escape key handler, and no `role="dialog"` / `aria-modal`. A user navigating with a keyboard cannot close it with Escape and may tab behind the overlay. At minimum, add an `onKeyDown` handler for Escape and `role="dialog"` with `aria-modal="true"`.

---


---

| I1 | Important | Coupling | Hardcoded media path should come from backend |

---

| I2 | Important | Compat | `display:none` media elements may not decode |

---

| I3 | Important | Race | `onBlur` + Enter key double-fires `handleSaveTitle` |

---

| I4 | Important | Cleanup | `mediaBlobUrl` state not nulled on effect cleanup |

---

| I5 | Important | UX | No loading indicator while media blob is loading |

---

| I6 | Important | State | `loadMeeting` does not clear previous error state |

---

| I7 | Important | A11y | Delete modal lacks focus trap, Escape, ARIA roles |

---

**Important (should fix for consistency):**

6. **Input fields** use 3 different rounding/padding/size combinations: `rounded-lg py-1.5 text-xs` (Gallery search), `rounded-xl py-2.5 text-[12px]` (Settings), `rounded-xl py-3 text-[13px]` (Chat).

7. **Focus states** differ: Gallery search and Settings use `focus:border-white/20`; Chat input uses `focus:border-brand-500/30` with a brand shadow ring.

8. **Footer padding** differs: ChatPanel uses `p-4`, ExportDialog uses `p-5`.

9. **Content area `space-y`** differs: MeetingPage/ChatPanel use `space-y-4`, ExportDialog/SettingsPage use `space-y-5`.

10. **Settings content padding** is `p-6` while all other content areas use `p-5`.

11. **Mixed language labels** in SettingsPage: "API Key" (line 622) vs "Chave da API" (line 555) vs "Chave chat" (line 673).

12. **Header-area action buttons** use `rounded-lg` (MeetingPage:268) while body action buttons use `rounded-xl` (MeetingPage:378).


---

However, the more important point: `TrackInfo.index` is `usize` in Rust but `number` in TypeScript. Same safe-in-practice situation.


---

### IMPORTANT Issues (should fix)

**5. Missing command: `get_thumbnail`**

The Rust backend registers `library::commands::get_thumbnail` in `lib.rs` line 80, and the command is defined in `library/commands.rs` lines 27-35:

```rust
pub fn get_thumbnail(library: State<'_, Library>, id: String) -> Result<Option<Vec<u8>>>
```

This command is **not exposed** in `api.ts`. If the frontend needs thumbnails (and the command exists because it was planned), this wrapper function is missing.

**6. `RagSettings.chunk_size` and `RagSettings.top_k` type mismatch**

- **TypeScript** (`api.ts` lines 135-136): `chunk_size: number`, `top_k: number`
- **Rust** (`settings/config.rs` lines 54-55): `chunk_size: u32`, `top_k: u32`

This works at runtime because `u32` fits in JavaScript `number`. However, there is an internal inconsistency in the Rust code itself: `RagConfig` (in `rag/types.rs` lines 38-40) uses `chunk_size: usize` and `top_k: usize`, while `RagSettings` uses `u32`. The `RagConfig::from_settings` method does `settings.chunk_size as usize`, which is fine. This is not a TS/Rust mismatch per se, but worth noting.

**7. `Segment.start` / `Segment.end` / `Word.start` / `Word.end` type nuance**

- **TypeScript** (`api.ts` lines 83-94): `start: number`, `end: number`
- **Rust** (`transcription/types.rs`): `start: f64`, `end: f64` for `Segment`; `start: f64`, `end: f64` for `Word`

And `Word.confidence`:
- **TypeScript**: `confidence: number`
- **Rust**: `confidence: f32`

These all work correctly since JavaScript `number` handles both `f64` and `f32`. No issue.

**8. `SaveMode` variant naming: `save_as` vs `SaveAs`**

The Rust enum uses `#[serde(rename_all = "snake_case", tag = "mode")]`, so variant `SaveAs` serializes with `"mode": "save_as"`. The TypeScript type on line 157 uses `{ mode: "save_as"; path: string }`, which correctly matches. No issue here.

However, note that the Rust `SaveAs` variant has `path: PathBuf`, not `path: String`. When deserializing from JSON, `PathBuf` accepts a JSON string. And the TypeScript sends `path: string`. This works correctly.

---


---

| 5 | Important | `api.ts` | Missing `get_thumbnail` command wrapper |

---

| 10 | Important | `format.ts:39` | `"ffmpeg"` check is case-sensitive, won't match Rust's `"FFmpeg failed"` |

---

### IMPORTANT Issues

#### 4. Opus export skips mixdown entirely -- will fail for certain containers

**Lines 64, 87-89, and types.rs line 29:**

`AudioFormat::Opus` has `requires_mixdown() -> false`, meaning multi-track files are passed through without mixing. The output file extension is `.opus`. However, an `.opus` file (bare Ogg/Opus container) does **not** support multiple audio streams. Only MKV or WebM containers support multiple Opus streams.

If the source MKV has multiple audio tracks and the user exports to Opus, ffmpeg will write only the first audio stream (silently dropping the rest) or fail, depending on the muxer.

**Recommendation:** Either:
- (a) Also set `requires_mixdown()` to `true` for Opus, consistent with the other formats, or
- (b) Use `-map 0:a` to explicitly map all audio streams and output to an `.mka` (Matroska Audio) container instead of `.opus` when multiple streams are present, or
- (c) Document this limitation clearly and accept that only the first stream is exported for Opus.

#### 5. No `-ac 2` flag to guarantee stereo output when there is only 1 audio stream

**Line 74 (the single-stream path):**

When there is only one audio stream and it happens to be mono, the output will also be mono. The doc comment on line 36 says "mixed into a single stereo stream" but the single-stream code path does not enforce stereo. For consistency, consider adding `-ac 2` in the single-stream branch for formats that explicitly target stereo output. This is a minor concern if the source is always stereo, but a meeting recorder may well have mono mic captures.

#### 6. `count_audio_streams` result of 0 is not handled

If the source file has **no** audio streams at all (e.g., a video-only file or a corrupt recording), `count_audio_streams` returns `Ok(0)`. The code then falls through to the single-stream path (since `0 < 2`), and ffmpeg will fail with a cryptic error because there is no audio to extract.

**Fix:** Add a guard:
```rust
let stream_count = count_audio_streams(&source)?;
if stream_count == 0 {
    return Err(ExportError::FfmpegFailed(
        "Source file contains no audio streams".to_string(),
    ));
}
```

#### 7. Blocking subprocess calls on the Tauri main thread

`Command::new("ffmpeg").output()` and the ffprobe call are **synchronous blocking** operations. Looking at `commands.rs`, the Tauri commands (`export_audio`, `export_video`, `export_transcript`) are not marked `async`. This means these commands will block the Tauri command handler thread during the entire ffmpeg encode, which can take seconds to minutes depending on file size.

**Recommendation:** Mark the Tauri commands as `async` and use `tokio::process::Command` or `tauri::async_runtime::spawn_blocking` to avoid blocking the main thread. This is labeled as an "MVP approach" in the doc comments, so this may be intentional for now, but it should be tracked.

---


---

| 4 | IMPORTANT | Opus export to `.opus` container cannot hold multiple streams |

---

| 5 | IMPORTANT | No `-ac 2` enforcement for single-stream mono sources |

---

| 6 | IMPORTANT | Zero audio streams not guarded; produces a confusing ffmpeg error |

---

| 7 | IMPORTANT | Blocking subprocess calls in Tauri sync command handlers |

---

### WRONG (Important): `set_language(None)` for auto-detect

Line 86:
```rust
params.set_language(None);
```

In whisper-rs 0.14, `set_language` takes `Option<&str>`. Passing `None` is syntactically valid but the actual auto-detect behavior depends on the whisper.cpp binding. This is technically correct, but worth noting that some versions expect an explicit `Some("auto")` for auto-detection. With 0.14.4, `None` should map to auto-detect correctly.

**Revised: CORRECT** -- `None` is the documented way to enable auto-detect.

### CORRECT: Segment and token extraction methods
`full_n_segments()`, `full_get_segment_text(i)`, `full_get_segment_t0(i)`, `full_get_segment_t1(i)`, `full_n_tokens(i)`, `full_get_token_data(i, j)`, `full_get_token_text(i, j)` -- all return `Result` types in 0.14. The code correctly handles them with `.map_err()`.

### CORRECT: full_lang_id_from_state and get_lang_str
```rust
state.full_lang_id_from_state()
whisper_rs::get_lang_str(lang_id)
```
These are correct 0.14 APIs. `full_lang_id_from_state()` is on `WhisperState` and returns `Result<i32>`, and `get_lang_str` is a free function.


---

### WRONG (Important): Timestamp unit interpretation

Lines 121-122:
```rust
let seg_start = t0 as f64 / 100.0;  // "centiseconds"
let seg_end = t1 as f64 / 100.0;
```

The comment says "whisper-rs times are in centiseconds (hundredths of a second)" but this is incorrect. In whisper-rs 0.14 (and whisper.cpp), `full_get_segment_t0` and `full_get_segment_t1` return timestamps in **centiseconds** (1/100th of a second, i.e., 10ms units). Actually, wait -- whisper.cpp returns timestamps in units of 10ms, which is indeed centiseconds. Dividing by 100.0 converts to seconds. The code IS correct, but the same logic applied to token data (line 149: `token_data.t0 as f64 / 100.0`) is also correct since `WhisperTokenData.t0` and `.t1` use the same time base.

**Revised: CORRECT** -- The division by 100.0 correctly converts centisecond timestamps to seconds.

**Verdict: CORRECT** -- whisper-rs 0.14 API is used correctly throughout.

---

## 4. rusqlite (0.32.1 with bundled + load_extension)

**Files:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/library/db.rs`, `.../rag/vector_store.rs`

### CORRECT: Connection::open and Connection::open_in_memory
Both are standard rusqlite 0.32 API.

### CORRECT: execute, execute_batch, query_row, prepare, query_map
All used with correct signatures. `params![]` macro usage is correct.

### CORRECT: load_extension_enable / load_extension / load_extension_disable

In vector_store.rs lines 105-121:
```rust
unsafe {
    conn.load_extension_enable()?;
    conn.load_extension("vec0", None::<&str>)?;
    conn.load_extension_disable();
}
```

In rusqlite 0.32, `load_extension_enable()` and `load_extension()` are `unsafe` methods (since loading arbitrary native code is inherently unsafe). The `load_extension` takes `(dylib_path, entry_point: Option<&str>)`. The code wraps these in an `unsafe` block correctly.

### CORRECT: unchecked_transaction
`self.conn.unchecked_transaction()` is the correct 0.32 API for creating a transaction from a non-`&mut` reference. This is appropriate here since `VectorStore` holds `conn: Connection` (not `&mut Connection`), making `transaction()` unavailable due to borrow checker constraints.

### CORRECT: query with params! and rows.next()
The `get_meta` method uses `stmt.query(params![key])` then `rows.next()`, which is correct 0.32 API.

### CORRECT: Row column access
`row.get(0)?`, `row.get::<_, String>(2)?`, `row.get::<_, i32>(4)?` are all correct rusqlite 0.32 patterns for typed column extraction.


---

## Important Issues (Should Fix)

**2. Redundant Content-Type header in embeddings.rs and chat.rs**

Files: `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/embeddings.rs` (line 116), `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/src/rag/chat.rs` (line 113)

The `.header("Content-Type", "application/json")` is redundant when using `.json(&body)` since reqwest sets it automatically.

**3. Inconsistent auth header pattern**

`api.rs` uses `.bearer_auth()` while `embeddings.rs` and `chat.rs` use manual `.header("Authorization", format!("Bearer {}"))`. Standardize on `.bearer_auth()` across all files.

---


---

#### IMPORTANT -- Issue 2: Redundant `current_start` initialization on line 34 creates a subtle dead-store

Line 34 initializes `current_start` to `transcript.segments[0].start`. Then the loop begins on line 38 by iterating over all segments starting from index 0. The first segment will never trigger the flush (because `current_tokens` is 0), so it falls through to line 63 where `current_texts.is_empty()` is true, and `current_start` is reassigned to `segment.start` -- the same value. This means line 34 is always a dead store. It works, but it signals to a reader that the first segment might be special-cased when it is not.

**Recommendation:** Initialize `current_start` to `0.0` (or any sentinel) and let the `if current_texts.is_empty()` block on line 63 handle it uniformly. This eliminates the misleading initialization and is more robust if the loop body is ever restructured.

---


---

#### IMPORTANT -- Issue 3: Whitespace-only segments produce silent token-count/text mismatch

Consider a segment whose `text` is `"   "` (only whitespace). `estimate_tokens("   ")` returns 0. The text gets pushed into `current_texts` on line 67, contributing zero tokens but adding whitespace to the joined string. When the chunk text is eventually constructed via `current_texts.join(" ").trim()`, the whitespace is trimmed away, so the final text is fine. However, the token counter never advances, which means a long sequence of whitespace-only segments would accumulate indefinitely without triggering a flush, potentially building up a very large `current_texts` vector in memory. This is unlikely in practice with Whisper output but is an edge case worth guarding.

**Recommendation:** Skip segments with empty or whitespace-only text early in the loop:

```rust
let trimmed = segment.text.trim();
if trimmed.is_empty() {
    current_end = segment.end; // still advance the timestamp
    continue;
}
```

---


---

#### IMPORTANT -- Issue 4: A single segment larger than `chunk_size` produces an oversized chunk with no warning

Line 43's condition `current_tokens > 0 && ...` means that if the buffer is empty (`current_tokens == 0`), the segment is always accepted regardless of size. If a single segment has 2000 tokens and `chunk_size` is 500, it silently produces a chunk 4x larger than intended. Since the doc-comment says "approximately `chunk_size` tokens" and the design deliberately never splits a segment, this behavior is correct by specification. However, for a RAG pipeline this can be problematic -- an oversized chunk may exceed the embedding model's context window or degrade retrieval quality.

**Recommendation:** At minimum, log a warning when a single segment exceeds `chunk_size`. Optionally, consider splitting at word boundaries for segments that exceed `2 * chunk_size`, since `Segment` already has a `words` field with per-word timestamps that could be used for sub-segment splitting:

```rust
if seg_tokens > chunk_size {
    tracing::warn!(
        "Segment with {} tokens exceeds chunk_size {}; chunk will be oversized",
        seg_tokens, chunk_size
    );
}
```

---


---

#### IMPORTANT -- Issue 5: Token estimation diverges significantly from BPE for non-English text

The `estimate_tokens` function splits on whitespace. For English this gives a rough 1.3x underestimate (BPE typically produces ~1.3 tokens per whitespace-delimited word). For languages like Chinese, Japanese, or Korean -- where words are not whitespace-delimited -- a sentence of 50 BPE tokens may contain only 1-3 whitespace-separated chunks. This means chunks for CJK languages could be 10-50x larger than intended. Given that Hlusra uses Whisper (which supports 99 languages) and stores the transcript language in `TranscriptResult.language`, this is a realistic concern.

**Recommendation:** At minimum, document this limitation. A simple improvement would be to fall back to a character-based estimate for CJK: approximately `text.len() / 4` bytes per token (rough BPE average for CJK UTF-8). Alternatively, use `text.chars().count() / 2` as a cross-language compromise.

---


---

The chunker produces strictly non-overlapping chunks. In RAG pipelines, it is common practice to include an overlap of 10-20% of `chunk_size` between consecutive chunks. Without overlap, a relevant sentence that happens to fall at a chunk boundary gets split across two chunks, and neither chunk alone may score well enough during retrieval. This is especially important for meeting transcripts where context flows continuously.

**Recommendation:** Consider adding an optional `overlap` parameter. After flushing a chunk, retain the last N tokens' worth of segments in the buffer for the next chunk.

---


---

| 2 | **Important** | Dead store of `current_start` on line 34; misleading initialization |

---

| 3 | **Important** | Whitespace-only segments accumulate without advancing token count |

---

| 4 | **Important** | Single oversized segments silently exceed `chunk_size` with no log/warning |

---

| 5 | **Important** | Token estimation breaks down for non-whitespace-delimited languages (CJK) |

---

### IMPORTANT Issues (should fix)

**I1. `recordings_dir` is `String` instead of `PathBuf`**

The `recordings_dir` field is stored as `String`, then converted to `PathBuf` everywhere it is consumed (`lib.rs` line 26: `PathBuf::from(&s.general.recordings_dir)`). TOML's `toml` crate natively supports `PathBuf` serialization/deserialization via serde. Using `PathBuf` directly would:
- Eliminate every `to_string_lossy()` call (which silently replaces invalid UTF-8 with the replacement character, potentially corrupting paths).
- Make the type system enforce correctness at the boundary.

Note: `to_string_lossy()` is used in `defaults.rs` line 35, meaning if the user's home directory contains non-UTF-8 bytes (rare but legal on Linux), the path will be silently corrupted.

**I2. `config_path()` panics via `unwrap_or_else` chain but the real panic is hidden**

The fallback logic in `config_path()` (lines 80-94) does:
1. Try `dirs::config_dir()` -- if `None`, fall back.
2. Try `dirs::home_dir()` -- if `None`, fall back.
3. Try `$HOME` env var -- if missing, use `/tmp/hlusra`.

The problem: if both `dirs::config_dir()` and `dirs::home_dir()` return `None`, and `$HOME` is not set, the config file path becomes `/tmp/hlusra/hlusra/config.toml`. This is a **silently wrong** location -- there is an `eprintln` warning, but the caller has no way to know this happened. The function returns `PathBuf`, not `Result<PathBuf>`. This is a data-loss risk: settings saved to `/tmp` will be lost on reboot.

Consider making this function return `Result<PathBuf, SettingsError>` so callers can handle the failure explicitly.

**I3. `load_settings` has a TOCTOU race condition**

Lines 103-107:
```rust
if !path.exists() {
    let defaults = AppSettings::default();
    save_settings(&defaults)?;
    return Ok(defaults);
}
```

Between the `exists()` check and the `read_to_string`, another process (or the app restarting concurrently) could create or delete the file. This is a classic time-of-check-to-time-of-use race. A more robust approach: attempt to read, and if the error is `NotFound`, create defaults.

```rust
pub fn load_settings() -> Result<AppSettings, SettingsError> {
    let path = config_path();
    match fs::read_to_string(&path) {
        Ok(content) => Ok(toml::from_str(&content)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let defaults = AppSettings::default();
            save_settings(&defaults)?;
            Ok(defaults)
        }
        Err(e) => Err(SettingsError::Io(e)),
    }
}
```

**I4. Duplicate `default_recordings_dir` logic in `lib.rs`**

`lib.rs` lines 16-21 define `default_recordings_dir()` with its own fallback logic that is slightly different from `GeneralSettings::default()` in `defaults.rs`. In `lib.rs`, the fallback for a missing home dir is `PathBuf::from(".")` (the current working directory), while in `defaults.rs` the fallback is `$HOME` env var then `/tmp/hlusra`. This inconsistency means if `dirs::home_dir()` fails, the two code paths produce different default directories. The `lib.rs` function should just delegate to `AppSettings::default().general.recordings_dir` or the defaults module.

**I5. No validation on deserialized values**

After `toml::from_str`, the loaded settings are trusted as-is. There is no validation that:
- `video.fps` is non-zero (a zero FPS would cause division-by-zero downstream).
- `video.bitrate` / `audio.bitrate` are within reasonable ranges.
- `video.codec` is one of the supported values.
- `rag.chunk_size` is non-zero.
- `recordings_dir` is an absolute path.

Consider adding a `validate()` method on `AppSettings` that is called after deserialization.

---


---

| I1 | Important | `recordings_dir` is `String` instead of `PathBuf`; `to_string_lossy` can corrupt paths |

---

| I2 | Important | `config_path()` silently falls back to `/tmp`, risking data loss |

---

| I3 | Important | TOCTOU race between `path.exists()` and `fs::read_to_string()` |

---

| I4 | Important | Duplicate and inconsistent `default_recordings_dir` logic in `lib.rs` |

---

| I5 | Important | No validation on deserialized settings values |

---

### IMPORTANT Issues

**4. No `fsync` / `File::sync_all()` before rename**

The doc comment on `download_model` says "written atomically", but the data is never flushed to disk before the rename. On a power loss between `io::copy` completing and the OS lazily flushing dirty pages, the renamed file could be truncated or zeroed. For multi-gigabyte model files that take minutes to download, this is a real risk.

Add `file.sync_all()` between `io::copy` and `fs::rename`:

```rust
io::copy(&mut response, &mut file)
    .map_err(|e| format!("Failed to download {model_name}: {e}"))?;
file.sync_all()
    .map_err(|e| format!("Failed to flush model data to disk: {e}"))?;
```

**5. `get_active_model` doc comment is misleading -- it does not fall back on invalid selection**

The doc says "Falls back to the default (tiny) if nothing has been explicitly selected, or the persisted choice is invalid." But the implementation only falls back when the `.active_model` file does not exist. If the file contains a name that is not in the catalogue (e.g., a model that was in an older version), the function returns `Err` on line 101 instead of falling back. Either update the doc to match the behavior, or implement the fallback:

```rust
let catalogue = list_available_models()?;
catalogue
    .into_iter()
    .find(|m| m.name == name)
    .or_else(|| {
        // Persisted choice is invalid; fall back to default.
        all_models().into_iter().find(|m| m.name == DEFAULT_MODEL)
    })
    .ok_or_else(|| "No valid model found".to_string())
```

**6. `list_available_models` is called redundantly inside `get_active_model`**

`get_active_model` calls `list_available_models()` on line 97, which internally calls `models_dir()` again and does a filesystem stat for every model. Since `get_active_model` already called `models_dir()` on line 86, the directory is created and statted twice. This is a minor performance issue but the real problem is that the `mut` binding on line 97 is unnecessary -- the vector is consumed by `into_iter()`, not mutated. Just use `all_models()` directly (the `downloaded` field is not needed for this lookup).

**7. No download progress reporting**

The `download_model` command is called from the frontend via Tauri. Large models (medium ~1.5 GB, large ~3 GB) will take minutes on typical connections. The function provides zero progress feedback to the UI. The user will see no indication of whether the download is at 1% or 99%. Consider using Tauri's event system (`app_handle.emit()`) to emit progress events during the download, or at minimum, add a way to query download progress (e.g., check `.part` file size vs. `Content-Length`).

**8. No download cancellation mechanism**

Related to the above: once a user starts a multi-gigabyte download, there is no way to cancel it. A `CancellationToken` or `AtomicBool` shared between the command handler and the download function would allow the UI to signal cancellation.

---


---

| 4 | **Important** | No `sync_all()` before rename -- "atomic write" claim is not fully met |

---

| 5 | **Important** | `get_active_model` doc claims fallback on invalid selection but actually errors |

---

| 6 | **Important** | Redundant `list_available_models` call in `get_active_model` |

---

| 7 | **Important** | No download progress reporting to the UI |

---

| 8 | **Important** | No download cancellation mechanism |

---

Issues 1, 2, and 3 should be fixed before this code ships. Issue 1 in particular means the "large" model simply cannot be downloaded at all. Issue 3 is a security concern (path traversal via crafted `model_name`). Issues 4-8 are important for production quality but are not showstoppers for an early iteration.

---

## IMPORTANT Issues

### 4. `&self` methods mutate the database -- should be `&mut self` or documented

All mutation methods (`insert_chunk`, `insert_chunks`, `delete_meeting_chunks`, `init_vector_table`, `set_meta`) take `&self`. rusqlite's `Connection` allows this because its `execute` methods take `&self` (using internal Cell-based mutability), but this is semantically misleading. In the caller (`commands.rs`), `VectorStore` is behind a `Mutex<VectorStore>`, so thread safety is handled there. However, the `&self` API makes it possible to accidentally share the `VectorStore` across threads without the Mutex, since nothing in the type signature prevents it. This is a design concern, not a bug, because `rusqlite::Connection` is `!Sync` (it does not implement `Sync`), which provides some protection. Still, consider documenting this explicitly.

### 5. `unchecked_transaction` vs `transaction` -- unnecessary risk

Lines 275 and 314. The code uses `unchecked_transaction()` instead of `transaction()`. The difference: `transaction()` requires `&mut self` on `Connection`, while `unchecked_transaction()` only requires `&self`. Since `VectorStore` holds `conn` as an owned `Connection` and all access goes through `Mutex<VectorStore>`, you have exclusive access. But `unchecked_transaction` skips the compile-time check that no other transaction is active. If future code accidentally nests transactions (e.g., `insert_chunks` calling `insert_chunk` which also starts a transaction), `unchecked_transaction` will silently create a savepoint rather than failing at compile time.

**Fix**: Change `conn: Connection` to allow `&mut self` methods, or at minimum add a comment explaining why `unchecked_transaction` is necessary and that nesting must be avoided.

### 6. `assert_eq!` in `insert_chunks` panics in production

Line 274:
```rust
assert_eq!(chunks.len(), embeddings.len());
```

This is a debug assertion disguised as a production guard. In release builds, `assert_eq!` is still active (unlike `debug_assert_eq!`), but panicking is the wrong behavior for a Tauri app -- it will crash the entire backend. This should return an error instead.

**Fix**:
```rust
if chunks.len() != embeddings.len() {
    return Err(VectorStoreError::Db(rusqlite::Error::InvalidParameterCount(
        chunks.len(), embeddings.len()
    )));
}
```
Or add a new `VectorStoreError` variant for this.

### 7. `get_dimension` silently swallows parse errors

Lines 166-171:
```rust
Some(d) => Ok(Some(d.parse::<usize>().unwrap_or(0))),
```

If the stored dimension value is corrupted (e.g., "abc"), this silently returns `Some(0)`, which would cause a zero-dimension virtual table to be created -- certainly an error. A dimension of 0 is never valid.

**Fix**: Return an error on parse failure:
```rust
Some(d) => match d.parse::<usize>() {
    Ok(dim) if dim > 0 => Ok(Some(dim)),
    _ => Err(VectorStoreError::NoDimension),
},
```

### 8. `load_sqlite_vec` does not check if extension is already loaded


---

Line 104-121. Every call to `VectorStore::open()` attempts to load `vec0`. If the extension is already loaded (e.g., compiled into the bundled SQLite), `load_extension("vec0", None)` may fail with an error that is then logged as a warning, making the logs noisy. More importantly, there is no way for callers to know whether the extension loaded successfully -- methods like `search`, `insert_chunk`, and `init_vector_table` that depend on `chunks_vec` will fail with cryptic SQLite errors ("no such table: chunks_vec") rather than a clear "sqlite-vec not available" message.

**Fix**: Store a `vec_available: bool` field in `VectorStore` and check it before any vec-dependent operation, returning a clear error.

### 9. PRAGMA `journal_mode = WAL` result is not checked

Lines 67-69. `PRAGMA journal_mode = WAL` returns the new journal mode as a result. Using `execute_batch` discards this result. If WAL mode cannot be activated (e.g., on a read-only filesystem, or when another connection holds the database in a different mode), the code silently continues in DELETE mode, which has different concurrency and durability characteristics. This is unlikely to cause issues in practice but violates the principle of checking operation success.

---


---

| 4 | IMPORTANT | `&self` on mutation methods is semantically misleading |

---

| 5 | IMPORTANT | `unchecked_transaction` used where compile-time safety is available |

---

| 6 | IMPORTANT | `assert_eq!` panics in production instead of returning error |

---

| 7 | IMPORTANT | `get_dimension` silently returns 0 on parse failure |

---

| 8 | IMPORTANT | No tracking of whether sqlite-vec loaded successfully |

---

| 9 | IMPORTANT | WAL pragma result discarded |

---

### IMPORTANT Issues

**3. Massive struct duplication between `Meeting`, `MeetingSummary`, and `MeetingDetail` (lines 5-47)**

These three structs share 8 identical fields. `MeetingSummary` is `Meeting` minus `dir_path` and `tracks`. `MeetingDetail` is `Meeting` plus `transcript`. This means:

- Any new field (e.g., `tags`, `language`) must be added to all three structs and kept in sync manually.
- The conversion in `api.rs` (lines 140-153) is a tedious field-by-field copy that will break silently if a field is added to one struct but forgotten in another.

Recommendation: Use a `From<Meeting>` impl for `MeetingSummary`, or consider embedding a shared inner struct. At minimum, add a compile-time test or derive macro to catch drift.

**4. `PathBuf` serialization via serde is platform-dependent (lines 13, 41, 59)**

`PathBuf`'s `Serialize` implementation serializes as a string, but it will fail on non-UTF-8 paths (returns a serde error). Since this is a Tauri app sending data to a JavaScript frontend, and `dir_path` is sent over JSON in `MeetingDetail`, this is a potential runtime panic on systems with non-UTF-8 path components.

The `db.rs` code already uses `to_string_lossy()` (line 82) when storing to SQLite, acknowledging this risk. But the serde path (used by Tauri commands) does not use lossy conversion -- it will hard-error instead. Consider adding `#[serde(serialize_with = "...")]` that uses lossy conversion, matching the DB behavior.

**5. `duration_secs: f64` -- floating-point for a time duration (lines 10, 24, 37, 64)**

Using `f64` for seconds introduces floating-point precision issues. A meeting of 1 hour 30 minutes 15 seconds could serialize as `5415.000000000001`. For a recording duration, `u64` (whole seconds) or a fixed-point representation would be more predictable. If sub-second precision is needed, consider `u64` milliseconds.

**6. `as_str()` methods duplicate what serde already does (lines 127-143, 146-168, 170-192)**

Each status enum has both `#[serde(rename_all = "snake_case")]` and a hand-written `as_str()` method that returns the same snake_case strings. These must be kept manually in sync. If a new variant is added to an enum, the developer must update both the enum definition and the `as_str()`/`from_str()` pair. This is the definition of error-prone duplication.

Recommendation: Either use serde for DB serialization too (via `serde_json::to_string` for single enum values, or a small helper), or use a macro/crate like `strum` to derive `Display`/`FromStr` from a single source of truth.

---


---

| 3 | IMPORTANT | Three near-identical meeting structs with no shared structure or conversion safety |

---

| 4 | IMPORTANT | `PathBuf` serde fails on non-UTF-8 paths (runtime error) |

---

| 5 | IMPORTANT | `f64` for duration introduces floating-point imprecision |

---

| 6 | IMPORTANT | `as_str()`/`from_str()` duplicate serde rename logic (double maintenance) |

---

## IMPORTANT Issues (should fix -- correctness/safety)

### I1. `open_pipe_wire_remote` returns `OwnedFd`, but the second argument type may be wrong

**Lines 53-58:**
```rust
let fd = proxy.open_pipe_wire_remote(
    &session,
    Default::default(),
)
.await
.map_err(|e| format!("Failed to get fd: {}", e))?;
```

In ashpd 0.13, `open_pipe_wire_remote` takes `&Session` and a `HashMap` (or similar options parameter). Using `Default::default()` should work if the type is a `HashMap` (which defaults to empty). This is likely fine, but worth confirming -- this is the one call I am least certain about.

### I2. `RawFd` escape hatch creates a dangling-fd risk

**Lines 60-68:**
```rust
use std::os::fd::AsRawFd;
let raw_fd = fd.as_raw_fd();

self.node_id = Some(node_id);
self.fd = Some(fd);

Ok(PipeWireSource { node_id, fd: raw_fd })
```

The `PipeWireSource` stores a `RawFd` (an integer copy), while the `OwnedFd` is moved into `self.fd`. The doc comment (lines 4-6) correctly notes that `ScreenCapture` must outlive any pipeline using the source. However:

- There is **no lifetime enforcement** at the type level. `PipeWireSource` has no lifetime parameter tying it to `ScreenCapture`. The safety guarantee relies entirely on the caller getting the drop order right.
- In `commands.rs` (line 55), the `ScreenCapture` is stored in `RecorderState.capture`, and the pipeline in `RecorderState.pipeline`. In `stop_recording` (line 95), the capture is released (`= None`) **after** the pipeline is stopped (line 88-91), which is correct. But nothing prevents a future maintainer from reordering these operations.

**Recommendation:** Consider making `PipeWireSource` borrow from `ScreenCapture` with a lifetime:
```rust
pub struct PipeWireSource<'a> {
    pub node_id: u32,
    pub fd: std::os::fd::BorrowedFd<'a>,
}
```
Or, if that creates lifetime complexity with the pipeline builder, at minimum use `BorrowedFd<'_>` instead of `RawFd` to get borrow-checker protection. An alternative is to `Arc<OwnedFd>` and clone it into `PipeWireSource`.

### I3. `PersistMode` import location

**Line 2:**
```rust
use ashpd::desktop::PersistMode;
```

In ashpd 0.13, `PersistMode` is re-exported from `ashpd::desktop::screencast::PersistMode`, not from `ashpd::desktop`. Verify the import compiles. It may work if there is a re-export at the `desktop` level, but the canonical location is `ashpd::desktop::screencast::PersistMode`.

---


---

| I1 | Important | 53-58 | `open_pipe_wire_remote` second arg type -- verify `Default::default()` resolves correctly |

---

| I2 | Important | 8-11, 60-68 | `RawFd` in `PipeWireSource` has no lifetime safety; dangling fd risk |

---

| I3 | Important | 2 | `PersistMode` canonical import is from `screencast`, not `desktop` |

---

## IMPORTANT Issues


---

### 4. IMPORTANT: `readFile` on MeetingPage will fail if recordings directory is not under `$HOME/Hlusra`

**Files:**
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, line 93
- `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src-tauri/capabilities/default.json`, lines 20-25

The Tauri FS scope only allows reading from:
- `$HOME/Hlusra/**`
- `$HOME/.local/share/hlusra/**`
- `$HOME/.config/hlusra/**`
- `/tmp/**`

If the user changes `recordings_dir` in settings to any path outside these scopes (e.g., `/mnt/data/recordings/`), the frontend's `readFile(mediaPath)` at line 93 of MeetingPage.tsx will fail with a permission error. The media player will silently fail (error is caught at line 100), but the user will never see their recording's audio/video.

Additionally, the "Save As" dialog export feature allows saving to an arbitrary path, but the FS scope does not cover arbitrary write paths. However, the export is done via Tauri commands (Rust-side), not the FS plugin, so writes will work. The issue is only with the frontend `readFile` call.

**Recommendation:** Either dynamically update the FS scope when settings change, or route media playback through a Tauri command that serves the file from the Rust side (e.g., as a `convertFileSrc` URL).


---

### 5. IMPORTANT: RecordButton `handleStart` is called from a `useEffect` that has a missing dependency

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx`, lines 33-37

```typescript
useEffect(() => {
  if (isRecordingView && !recording && !starting) {
    handleStart();
  }
}, [isRecordingView]);
```


---

`handleStart` is not in the dependency array, and it is not wrapped in `useCallback`. More importantly, `recording` and `starting` are also missing from the dependency array. In React Strict Mode (dev), this effect fires twice, which would call `handleStart()` twice in rapid succession. In production, the stale closure over `recording` and `starting` means the guard conditions may not prevent a second invocation if state has changed.

This could lead to two `start_recording` invocations racing, which would attempt to create two meetings and two GStreamer pipelines simultaneously. The second one would likely fail with a confusing error, or worse, the first pipeline would be orphaned (leaked) because the second `start_recording` overwrites the `RecorderState.pipeline` mutex contents.

**Recommendation:** Either use a ref to track whether start was already attempted, or make `handleStart` a `useCallback` with proper deps and include it in the effect dependency array.


---

### 6. IMPORTANT: State update after unmount in RecordButton polling interval

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/RecordButton.tsx`, lines 51-60

The polling interval calls `setElapsed` and `setFileSize` unconditionally. If the component unmounts while the interval is still active (e.g., user navigates away rapidly), the cleanup from `useEffect` on line 28-30 clears the interval, but there is a window where the interval callback has already been queued and fires after cleanup. React will log a "Can't perform a React state update on an unmounted component" warning.

Unlike ChatPanel, RecordButton has no `mountedRef` guard.

**Recommendation:** Add a `mountedRef` pattern like ChatPanel uses, or use an AbortController.


---

### 7. IMPORTANT: `MeetingPage` media blob URL leak on re-render paths

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/MeetingPage.tsx`, lines 85-109

The `useEffect` that loads media depends on `meeting?.id` and `meeting?.media_status`. When `loadMeeting()` is called again (e.g., after retranscribe or reindex at lines 169 and 183), `setMeeting` creates a new meeting object. Since `meeting?.id` has not changed, the effect will NOT re-run, which is correct. However, if `media_status` changes (e.g., from "present" to "deleted" after `handleDelete("media_only")`), the cleanup function runs and revokes the old URL, but `setMediaBlobUrl(null)` is never called -- `mediaBlobUrl` state still holds the revoked URL string. The hidden `<video>` or `<audio>` element on line 334-338 will then have a broken `src` attribute pointing to a revoked blob URL.

**Recommendation:** In the cleanup function, also call `setMediaBlobUrl(null)` before revoking. Or set it to null at the beginning of the effect.


---

### 8. IMPORTANT: `Gallery` component has no flex container parent -- header/footer may not stick

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/components/Gallery.tsx`, lines 47-128

Gallery returns a fragment (`<>...</>`) with `<header>`, `<div className="flex-1 overflow-y-auto">`, and `<footer>`. These use `shrink-0` and `flex-1`, which require a flex container parent. The parent in `App.tsx` is `<motion.div className="w-screen h-screen">`, which does NOT have `flex` or `flex-col`. This means:
- `shrink-0` on header/footer has no effect
- `flex-1` on the content div has no effect
- The gallery content may overflow the viewport without scrolling properly

The same issue applies to `MeetingPage`, `ChatPanel`, `ExportDialog`, and `SettingsPage` -- they all use fragments with flex children expecting a flex container.

**Recommendation:** Add `flex flex-col` to the `<motion.div>` wrapper in App.tsx, or wrap each view's fragment in a `<div className="flex flex-col h-full">`.

---


---

| IMPORTANT | 5 | FS scope mismatch, double-start race, state-after-unmount, blob URL leak, missing flex parent |

---

The most impactful IMPORTANT item is #8 (missing flex container parent). Without `flex flex-col` on the motion.div wrapper in App.tsx, the header/content/footer layout of Gallery, MeetingPage, ChatPanel, ExportDialog, and SettingsPage will not render correctly -- content will overflow instead of scrolling, and headers/footers will not stick.

---

More importantly, because `meetingId` is a ref, changes to it do **not** trigger re-renders. If `meetingId.current` is mutated while `MeetingPage` is already mounted, the component will not see the new value (though currently the component unmounts/remounts on every view change due to `AnimatePresence`, so this is not actively broken -- just fragile).

**Recommendation:** Either promote `meetingId` to `useState` (and pass it as a proper reactive value), or document the invariant that `go("meeting", id)` must always be called before `go("chat")` / `go("export")`.

---


---

## IMPORTANT Issues (should fix)

### 4. `AnimatePresence` exit animation is fighting the resize

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, lines 142-153 and 37-45

The `AnimatePresence mode="wait"` means: the exiting `motion.div` must finish its exit animation (200ms) **before** the entering one mounts. But the `useEffect` for window resize fires on `view` state change immediately -- meaning the window starts resizing while the old view is still animating out. If the resize changes height (400->600 or 600->400), users will see the exit animation playing inside a window that is actively changing size. This creates a jarring visual.

**Recommendation:** Either:
- Use `onExitComplete` on `AnimatePresence` to trigger the resize after the exit animation finishes, or
- Delay the resize with a `setTimeout` matching the transition duration, or
- Resize before the animation starts by calling `setSize` inside the `go()` function itself (synchronously before `setView`).

### 5. `motion.div` uses `w-screen h-screen` inside a `w-full h-full` parent

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, line 144

```typescript
<div className="w-full h-full relative overflow-hidden">
  <AnimatePresence mode="wait">
    <motion.div
      key={view}
      className="w-screen h-screen"
```

`w-screen` / `h-screen` use `100vw` / `100vh`, while the parent is `w-full h-full` (100% of its own parent). In a Tauri app with `decorations: false`, these are usually equivalent. But if the window has native title bars or any padding on the body/root, `100vw`/`100vh` will be *larger* than the parent, causing horizontal/vertical overflow. The parent has `overflow-hidden`, which clips it, but the `scale: 0.97` animation will briefly reveal content outside the intended bounds.

**Recommendation:** Change to `w-full h-full` on the `motion.div` to match the parent's sizing model, or use `size-full` (Tailwind v4 shorthand).

### 6. `onStatusChange` in `ChatPanel` is a no-op

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, line 120

```typescript
onStatusChange={() => {}}
```

`ChatPanel` calls `onStatusChange()` on line 66 of `ChatPanel.tsx` after indexing succeeds. Passing a no-op means `App` never learns that the chat status changed. When the user navigates back to `MeetingPage`, the stale `meetingCtx.chatStatus` will still say `"not_indexed"` even though indexing completed. If MeetingPage re-fetches from the backend, this is fine in practice, but the prop contract is clearly designed for the parent to be notified.

**Recommendation:** At minimum, re-fetch meeting data or update `meetingCtx`:

```typescript
onStatusChange={(newStatus) => {
  setMeetingCtx(prev => ({ ...prev, chatStatus: newStatus }));
}}
```

This requires changing the callback signature in ChatPanel to pass the new status, or having the App do a fresh fetch.

### 7. `Gallery` renders fragments (`<>...</>`) as direct children of `motion.div`

**File:** `/home/caua/Documentos/Projetos-Pessoais/Hlusra/src/App.tsx`, line 93-101, and Gallery.tsx returns `<>...</>`

`Gallery`, `MeetingPage`, `ChatPanel`, `ExportDialog`, and `SettingsPage` all return bare fragments (`<>header... content... footer...</>`). These fragments become the children of the `motion.div` on line 143. This is fine for animation purposes (motion animates the wrapper, not the children), but it means the fragment children rely on the `motion.div` being a flex container for their layout (e.g., `shrink-0`, `flex-1` on headers/footers). Currently the `motion.div` has `className="w-screen h-screen"` with **no `flex flex-col`**. 

This means `Gallery`'s `header` with `shrink-0`, its content with `flex-1`, and its `footer` with `shrink-0` are all inside a block-layout div. `flex-1` and `shrink-0` have no effect in block layout. The gallery content will overflow rather than scroll properly.

**Recommendation:** Add `flex flex-col` to the `motion.div`:

```typescript
className="w-full h-full flex flex-col"
```

This is likely the most impactful layout bug in the file.

---


---

| 4 | IMPORTANT | Exit animation fights window resize timing | 37-45, 142-153 |

---

| 5 | IMPORTANT | `w-screen h-screen` vs `w-full h-full` mismatch | 144 |

---

| 6 | IMPORTANT | `onStatusChange` is a no-op, chat status never propagates back | 120 |

---

| 7 | IMPORTANT | `motion.div` lacks `flex flex-col`, breaking fragment-child layouts | 144 |

---

`reqwest::blocking::multipart::Part::file()` reads the file contents and sets the file name from the path's file stem, but it does **not** set a MIME type. The default MIME type for a Part is `application/octet-stream`. The OpenAI Whisper API validates the file extension and MIME type. More importantly, if `audio_path` is something like `/path/to/_temp_mic.wav` (which it is -- see `orchestrator.rs` line 38), the inferred file name will be `_temp_mic.wav`. While the `.wav` extension is acceptable to OpenAI, there are two sub-issues:

- **(a)** The MIME type should be explicitly set to `audio/wav` (or `audio/mpeg`, `audio/mp4`, etc. depending on format) using `.mime_str("audio/wav")`. Some Whisper-compatible servers (especially self-hosted ones like `faster-whisper-server` or `whisper-asr-webservice`) validate MIME type strictly and will reject `application/octet-stream`.

- **(b)** If the orchestrator ever changes the temp file name to not have a `.wav` extension, or if someone calls `ApiProvider::transcribe()` with a non-WAV path (the trait signature accepts any `&Path`), the API will silently receive an incorrect content type.

**Recommendation:** Explicitly set both the file name and MIME type:
```rust
let file_part = multipart::Part::file(audio_path)
    .map_err(|e| format!("Failed to read audio file for upload: {e}"))?
    .file_name("audio.wav")
    .mime_str("audio/wav")
    .map_err(|e| format!("Invalid MIME type: {e}"))?;
```

Or better yet, derive the MIME type from the file extension dynamically.

---


---

### IMPORTANT

**2. `reqwest::blocking::Client` is constructed on every single call**

Line 74:
```rust
let client = reqwest::blocking::Client::new();
```

Creating a new `reqwest::blocking::Client` on every invocation of `transcribe()` means a new connection pool, new TLS session, and new DNS resolution each time. The `reqwest` documentation explicitly recommends reusing `Client` instances because they manage an internal connection pool.

In the current architecture, `ApiProvider` is created fresh for each `transcribe_meeting` command (see `commands.rs` line 26-27), so the client would be discarded anyway. But storing the `Client` in `ApiProvider` is still the correct pattern and would enable future reuse (e.g., batch transcriptions, retries).

**Recommendation:** Store the client in `ApiProvider`:
```rust
pub struct ApiProvider {
    base_url: String,
    api_key: String,
    model: String,
    client: reqwest::blocking::Client,
}

impl ApiProvider {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        Self {
            base_url,
            api_key,
            model,
            client: reqwest::blocking::Client::new(),
        }
    }
}
```

**3. No request timeout configured**

There is no timeout set on the client or the request. Audio transcription via an API can take a very long time (especially for long recordings), but having *no* timeout means the blocking thread could hang indefinitely if the server stops responding. `reqwest::blocking::Client::new()` defaults to **no timeout** for the overall request (only a 30-second connect timeout).

**Recommendation:** Configure a generous but finite timeout:
```rust
let client = reqwest::blocking::Client::builder()
    .timeout(std::time::Duration::from_secs(600)) // 10 minutes
    .build()
    .map_err(|e| format!("Failed to build HTTP client: {e}"))?;
```

**4. `timestamp_granularities[]` sent as a single value -- may cause issues with some servers**

Line 72:
```rust
.text("timestamp_granularities[]", "word")
```

The OpenAI API expects `timestamp_granularities[]` as an array parameter. When using `response_format=verbose_json`, the default granularity is `segment`. To also get word-level timestamps, both `"segment"` and `"word"` should be specified. Currently only `"word"` is sent. The OpenAI API documentation states:

> "There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency."

The API returns segments regardless (it always includes them in verbose_json), but explicitly requesting only `"word"` without `"segment"` is technically asking for word-only granularity. The code then reads both `segments` and `words` from the response. This works in practice with the OpenAI API (which always returns segments in verbose_json mode), but it is semantically incorrect and may fail on stricter compatible servers.

**Recommendation:** Send both granularities:
```rust
.text("timestamp_granularities[]", "word")
.text("timestamp_granularities[]", "segment")
```

**5. No validation of `audio_path` existence before attempting upload**


---

The `transcribe()` method calls `multipart::Part::file(audio_path)` which will fail with an IO error if the file does not exist, but the error message "Failed to read audio file for upload" is generic. More importantly, there is no validation that the file is non-empty or a valid audio format. While the orchestrator does create the WAV before calling this, the trait signature is public and could be called with bad paths.

**Recommendation:** Add a pre-flight check:
```rust
if !audio_path.exists() {
    return Err(format!("Audio file not found: {}", audio_path.display()));
}
```

**6. The API key is sent as a raw string reference, not redacted in error messages**

Lines 77-79:
```rust
if !self.api_key.is_empty() {
    request = request.bearer_auth(&self.api_key);
}
```

This is fine for the auth itself, but lines 82-83 and 89-90 format the full error including the response body. If the server returns an error that echoes back headers or if debug logging is enabled elsewhere, the API key could leak into logs. This is a minor concern but worth noting.

---


---

| 2 | IMPORTANT | `Client` rebuilt on every call instead of reused |

---

| 3 | IMPORTANT | No request timeout -- blocking thread can hang forever |

---

| 4 | IMPORTANT | Only `"word"` granularity sent, `"segment"` omitted |

---

| 5 | IMPORTANT | No pre-flight validation of audio file existence |

---

| 6 | IMPORTANT | Potential API key leakage in error contexts |

---

The `open()` method sets `journal_mode = WAL`, `synchronous = NORMAL`, and `foreign_keys = ON`. The `open_in_memory()` method skips all of these. While WAL and synchronous are irrelevant for in-memory databases, **`foreign_keys = ON` is semantically important** -- if you ever add foreign keys to the schema, all in-memory tests will silently skip FK enforcement. This is a correctness hazard waiting to happen.

### 3. No transaction wrapping on migrations (line 64)

```rust
self.conn.execute_batch(sql)?;
```

Each migration string in `MIGRATIONS` can contain multiple SQL statements (and migration v1 does: it creates two tables and inserts a row). If the process crashes mid-`execute_batch`, the database is left in a partially-migrated state with no way to recover -- the schema_version row may be inserted but the meetings table might be incomplete, or vice versa. Each migration should run inside an explicit transaction (`BEGIN; ... COMMIT;`), or the entire migration body should be wrapped in one.

---


---

## IMPORTANT Issues

### 4. `delete_meeting` silently succeeds when ID does not exist (line 170-173)

All the `update_*` methods check `self.conn.changes() < 1` and return `QueryReturnedNoRows` if nothing was affected. But `delete_meeting` does not. This inconsistency means callers cannot distinguish "successfully deleted" from "ID never existed." Either add the same guard or document the intentional difference.

### 5. `file_size` column is `INTEGER NOT NULL` but the Rust type is `u64` (line 17 of schema, line 6 of types.rs)


---

SQLite's INTEGER is a signed 64-bit value. Storing `u64` as `i64` (line 82: `meeting.file_size as i64`) will silently overflow and produce negative values for files larger than ~9.2 exabytes. While this is unlikely in practice, the `as i64` cast is a lossy cast with no bounds check. More importantly, when reading back (line 186: `row.get::<_, i64>(5)? as u64`), a negative stored value would wrap around to a massive `u64`. Consider at minimum adding a debug assertion, or using `i64` throughout.

### 6. `duration_secs` column is `REAL NOT NULL` but there is no validation (line 15 of schema)

`NaN` and `Infinity` are valid `f64` values in Rust but will produce undefined behavior in SQLite comparisons and ORDER BY clauses. If a recording somehow produces `NaN` for duration, the database will accept it but queries relying on ordering or comparisons will behave unpredictably.

### 7. Silent data loss in `unwrap_or_default()` calls (lines 112-113, 177, 181-183)

Multiple places silently swallow parse errors:
- Line 177: `serde_json::from_str(&tracks_json_str).unwrap_or_default()` -- if the JSON is malformed, tracks are silently dropped.
- Lines 112, 181: `DateTime::parse_from_rfc3339(...).unwrap_or_default()` -- if the date string is corrupt, it silently becomes `1970-01-01T00:00:00Z`.

These should at minimum log a warning (as the `from_str` methods in types.rs do), or better yet, propagate the error. You could map parse errors to `rusqlite::Error::InvalidColumnName` or a custom error.

### 8. `load_extension` feature is enabled but unused in db.rs

In `Cargo.toml`, `rusqlite` has the `load_extension` feature enabled. This feature is a security-sensitive capability (it allows loading arbitrary shared libraries into the SQLite process). If it is only used by `src/rag/vector_store.rs`, that is fine, but it is worth verifying that `db_unsafe_handle()` or `load_extension()` are not accidentally exposed. This is more of a Cargo.toml concern than a db.rs concern, but worth noting.

---


---

There is no test that calls `run_migrations()` twice on the same database to verify that the `CREATE TABLE IF NOT EXISTS` and version-checking logic correctly skip already-applied migrations. Given the fragility of the migration bootstrap (issue #1), this is an important coverage gap.

### 15. `test_list_meetings_ordered` does not actually verify order

The test on lines 235-241 inserts two meetings and checks `list.len() == 2`, but never asserts that the list is actually ordered by `created_at DESC`. Both meetings are created with `Utc::now()` which may produce identical timestamps, making the test meaningless for order verification.

---

## Summary Table

| # | Severity | Issue | Line(s) |
|---|----------|-------|---------|

---

| 4 | Important | `delete_meeting` does not check `changes()` | 170-173 |

---

| 5 | Important | `u64` to `i64` lossy cast for `file_size` | 82, 186 |

---

| 6 | Important | No validation for `NaN`/`Infinity` in `duration_secs` | 79 |

---

| 7 | Important | Silent data loss on parse failures (`unwrap_or_default`) | 112, 177, 181 |

---

| 8 | Important | `load_extension` feature enabled -- verify necessity | Cargo.toml:18 |

---

## IMPORTANT Issues (Should Fix)

### 4. `gst_element_name` returns `&'static str` with no fallibility

The function signature `fn gst_element_name(&self, codec: &VideoCodec) -> &'static str` makes it impossible to signal that a combination is unsupported. Given that at least 4 of the 12 combinations reference non-existent elements (`nvav1enc`, `vulkanh264enc`, `vulkanh265enc`, `vulkanav1enc`), the return type should be `Option<&'static str>` or `Result<&'static str, UnsupportedCombination>`. Returning a string for a non-existent element will cause a silent GStreamer pipeline construction failure at runtime.

### 5. Missing `Default` derive/impl for `RecordingState`

`RecordingState` (line 20-26) has no `Default` implementation. Logically, `Idle` is the obvious default. If `RecordingStatus` is ever used in a context requiring `Default`, this will be a problem. At minimum, adding `#[derive(Default)]` with `#[default]` on `Idle` (Rust 1.62+ feature) would be appropriate.

### 6. Missing `Default` for `RecordingStatus`

`RecordingStatus` (line 28-33) has no `Default` implementation. Since it contains `RecordingState` which also lacks a default, neither can derive it easily. Consider adding defaults for both.

### 7. Missing `Copy` on `VideoCodec`

`VideoCodec` (line 14) derives `Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize` -- this is correct. But `RecordingState` at line 20 also correctly has `Copy`. No issue here on closer inspection -- both are fine.

### 8. `gst_element_name` takes `&VideoCodec` instead of `VideoCodec`

Line 70: `pub fn gst_element_name(&self, codec: &VideoCodec) -> &'static str`

`VideoCodec` is `Copy` (4 bytes max). Passing it by reference is an anti-pattern for `Copy` types. It should be:

```rust
pub fn gst_element_name(&self, codec: VideoCodec) -> &'static str
```

This is a minor ergonomics/convention issue but worth fixing for idiomatic Rust.

### 9. The `VideoConfig` default sets `backend: EncoderBackend::Vaapi` unconditionally

Line 54: The default backend is hardcoded to `Vaapi`. This will fail on systems without VA-API support (NVIDIA-only, software-only). Consider runtime detection or at least documenting this assumption. This is more of a design concern than a code bug.

### 10. Missing `Hash` derive on `VideoCodec`

`EncoderBackend` (line 3) derives `Hash`, but `VideoCodec` (line 12) does not. If these types are used together as map keys (which is plausible for caching encoder availability), `VideoCodec` should also derive `Hash` for consistency.

---


---

| 4 | IMPORTANT | Return type should be `Option`/`Result` for unsupported combos | 70 |

---

| 5 | IMPORTANT | Missing `Default` for `RecordingState` | 20-26 |

---

| 6 | IMPORTANT | Missing `Default` for `RecordingStatus` | 28-33 |

---

| 7 | IMPORTANT | `&VideoCodec` should be `VideoCodec` (Copy type) | 70 |

---

| 8 | IMPORTANT | Hardcoded Vaapi default may fail on non-VA systems | 54 |

---

| 9 | IMPORTANT | Missing `Hash` on `VideoCodec` | 12 |

---

### IMPORTANT Issues (should fix)

#### 3. No `-movflags +faststart` for MP4 output

When producing MP4 files, ffmpeg places the `moov` atom (the index/metadata) at the end of the file by default. This means the file cannot be played until it is fully downloaded -- a problem for any network streaming or progressive download scenario. The standard practice is to pass `-movflags +faststart` which relocates the moov atom to the beginning of the file after writing.

**Lines 52-54:**
```rust
cmd.arg("-f").arg(format.container_name());
```

**Recommendation:** Add this flag for MP4 outputs:

```rust
if matches!(format, VideoFormat::Mp4H264 | VideoFormat::Mp4H265) {
    cmd.arg("-movflags").arg("+faststart");
}
```

#### 4. No CRF / quality parameter for H.264 transcode

**Lines 43-46:**
```rust
VideoFormat::Mp4H264 | VideoFormat::MkvH264 => {
    cmd.arg("-codec:v").arg("libx264").arg("-preset").arg("medium");
}
```


---

The code sets `-preset medium` but provides no quality target. Without `-crf` or `-b:v`, libx264 defaults to CRF 23, which is acceptable for general use but may not match user expectations for a meeting recorder (screen content with text typically benefits from CRF 18-20 for readable text). More importantly, the *intent* is undocumented -- a future maintainer cannot tell if CRF 23 was chosen deliberately or by omission.

**Recommendation:** Explicitly set a CRF value, e.g. `-crf 20`, so the quality decision is intentional and visible in code.

#### 5. AV1 source recordings have no export path at all

The `VideoFormat` enum only offers H.264 and H.265 targets. If a user records in AV1 (which the recorder supports), there is no `VideoFormat` variant that will stream-copy AV1. Every export will transcode AV1 to either H.264 or H.265, which is computationally expensive and lossy. There should at least be `Mp4Av1` / `MkvAv1` variants, or the omission should be documented as intentional with a tracking issue.

#### 6. The `-f` flag is redundant when the output filename has the correct extension

FFmpeg infers the output format from the file extension. Passing `-f mp4` alongside an output path ending in `.mp4` is harmless but redundant. However, if `SaveMode::SaveAs` provides a path with a *different* extension (e.g., the user picks `meeting.mkv` but the format is `Mp4H264`), then `-f mp4` forces MP4 muxing into a `.mkv`-named file, which is confusing. The code should either:
- Trust `-f` and not care about the extension (current behavior, acceptable), or
- Validate that the `SaveAs` path extension matches the format.

This is a minor UX footgun more than a correctness bug, but worth noting.

#### 7. `codec_name()` method in `types.rs` is defined but never used

`VideoFormat::codec_name()` (types.rs lines 54-59) returns `"libx264"` / `"libx265"` but `video.rs` hardcodes the codec strings directly in the match arms. This is a code smell -- the method exists precisely to avoid duplication, but it is not used. If someone updates the codec name in one place but not the other, they will diverge.

**Recommendation:** Use `format.codec_name()` in the transcode branch:

```rust
VideoFormat::Mp4H264 | VideoFormat::MkvH264 => {
    cmd.arg("-codec:v").arg(format.codec_name())
       .arg("-preset").arg("medium");
}
```

---


---

| 3 | IMPORTANT | Missing `-movflags +faststart` for MP4 output |

---

| 4 | IMPORTANT | No explicit CRF for libx264 transcode; relying on implicit default |

---

| 5 | IMPORTANT | No AV1 export path despite recorder supporting AV1 |

---

| 6 | IMPORTANT | SaveAs path extension can mismatch the format flag |

---

| 7 | IMPORTANT | `codec_name()` method exists but is not used; codec strings are hardcoded |

---

### IMPORTANT Issues

**3. `handleStart` and `handleStop` are not memoized (lines 39, 69)**

Both `handleStart` and `handleStop` are declared as plain `async function` declarations inside the component body. They are recreated on every render. This matters because:

- They are passed as `onClick` handlers, causing unnecessary re-renders of child elements if the component is ever wrapped in `React.memo` or if the buttons are extracted.
- `handleStart` is referenced inside a `useEffect`, making the dependency array incorrect regardless.

**Fix:** Wrap both with `useCallback` and list the correct dependencies, or convert `handleStart` to a ref-stable function.

**4. `withVideo` is always `false` during auto-start (line 43 via line 34)**

When the component mounts with `isRecordingView={true}`, the auto-start effect fires `handleStart()` immediately. At that point `withVideo` is its initial value (`false`), and the user has no opportunity to toggle it. This is by design (the toggle is only shown in the home view), but it means the user's intent from the home-view toggle is lost because a *new* `RecordButton` instance is created for the recording view (confirmed by `App.tsx` line 85-89 rendering a second `<RecordButton>`). The `withVideo` state from the home-view instance does not carry over.

Looking at `App.tsx` lines 59-62 vs 85-89: when the user clicks record in the home view, `handleStart()` runs in the home-view instance (which has the correct `withVideo`), then `onRecordingStart` transitions to the "recording" view which mounts a *new* `RecordButton` with `isRecordingView={true}`. That new instance also calls `handleStart()` -- meaning `startRecording` is called **twice**: once from the home instance, once from the auto-start in the recording instance. This is a functional bug. The second call will either fail (if the backend rejects double-start) or start a second recording.

**Fix:** The architecture needs one of two adjustments:
- (a) Do not call `handleStart` in the home-view instance -- only use it to navigate, and let the recording-view instance handle the actual start. Pass `withVideo` as a prop.
- (b) Do not auto-start in the recording-view instance -- only use it as a display. Remove the auto-start `useEffect` entirely and rely on the home-view instance having already started the recording. Pass `recording` state through a shared context or lift it to the parent.

**5. Polling callback does not check if the component is still mounted (line 51-60)**

The `setInterval` callback calls `setElapsed` and `setFileSize` asynchronously. If the component unmounts between the `getRecordingStatus()` call and the state setter, React will attempt to update state on an unmounted component. While React 18+ tolerates this without a warning, it is still a logic error that can mask bugs.

**Fix:** Use an `AbortController` or a mounted ref to skip state updates after unmount.

**6. Polling does not detect backend state transitions (line 54-57)**

The polling callback only acts when `status.state === "recording"`. If the backend transitions to `"stopped"` or `"idle"` (e.g., the recording stops due to a backend error, disk full, pipeline crash), the component never notices. It continues showing the recording UI with a stale elapsed time forever.

**Fix:** Handle non-`"recording"` states in the polling callback. At minimum, if `status.state !== "recording"`, clear the poll and update the UI accordingly.

**7. Error state is not cleared on view transitions**

If an error occurs during start or stop, the error message is displayed. However, if the user navigates away and back (e.g., via parent view switching), the error persists because the component mounts fresh. This is actually fine for fresh mounts (state resets), but within a single mount, if `handleStart` fails, the error is shown, and then the user can click the button again -- the error is cleared at the top of `handleStart` (line 40), which is correct. However, in the recording view, there is no way to retry after an error -- the stop button is the only action available, and there is no "back" or "retry" button. The user is stuck.

**Fix:** Add a way to dismiss the error or navigate back from the recording view when start fails.

---


---

| 3 | Important | handleStart/handleStop not memoized, breaks useEffect deps |

---

| 4 | Important | Double startRecording call: home instance starts, then recording-view instance auto-starts again |

---

| 5 | Important | Polling callback sets state after potential unmount |

---

| 6 | Important | Polling ignores non-"recording" backend states (disk full, crash) |

---

| 7 | Important | No recovery path from error in recording view |

---

## IMPORTANT Issues

### 4. Non-atomic multi-mutex acquisition creates inconsistent state on partial failure

**File:** `commands.rs` lines 43-75

In `start_recording`, three separate mutexes are locked sequentially: `capture` (line 55), `pipeline` (line 72), `current_meeting_id` (line 73). If the `pipeline` or `current_meeting_id` lock fails (poisoned), the function returns an error, but the `capture` mutex has already been set to `Some(capture)`, and `library.prepare_meeting()` has already created a directory on disk. There is no cleanup:

- The orphaned capture session remains in state.
- The prepared meeting directory is never finalized or removed.

Similarly, if line 73 (`current_meeting_id` lock) fails, the pipeline is already stored in `recorder.pipeline` but there is no meeting ID, so `stop_recording` would successfully stop the pipeline but then fail at line 97-98 with "No meeting ID", leaving the meeting in a permanently unfinalized state.

**Recommendation:** Consider a single `Mutex<RecorderInner>` struct that holds all three fields together, so they are always modified atomically. Alternatively, add cleanup logic on partial failure paths.

### 5. `get_recording_status` holds the mutex lock while computing `duration_secs()` and `file_size()`

**File:** `commands.rs` lines 123-139

`file_size()` performs a filesystem `metadata()` call (line 219 of `pipeline.rs`), and `duration_secs()` calls `Instant::elapsed()`. The mutex is held for the duration of both calls. While `Instant::elapsed()` is cheap, the `fs::metadata` call can block on I/O, especially on networked or slow storage. This command is likely called on a polling interval from the frontend, which means the pipeline mutex is repeatedly held for I/O during recording. If the frontend polls frequently, this could cause contention with `stop_recording` trying to acquire the same lock.

**Recommendation:** Extract the path and start_time from the pipeline while the lock is held, then release the lock before performing the I/O call.

### 6. `duration_secs()` after `stop()` returns wall-clock time, not actual recording duration

**File:** `commands.rs` line 101; **Related:** `pipeline.rs` lines 206-208

`pipeline.duration_secs()` returns `self.start_time.elapsed().as_secs_f64()`. This is called in `stop_recording` on line 101, **after** `pipeline.stop()` has already been called (which blocks for up to 5 seconds waiting for EOS). So the recorded `duration_secs` includes the EOS wait time, making it consistently ~1-5 seconds longer than the actual recording duration.

**Recommendation:** Capture the duration before calling `stop()`, or record the stop timestamp separately and compute the difference between start and stop times (not stop and elapsed).

### 7. `std::sync::Mutex` used in an `async` function risks blocking the async runtime

**File:** `commands.rs` lines 55, 72, 73 (inside `start_recording` which is `async`)

`std::sync::Mutex` is used in the `async fn start_recording`. If contention occurs (e.g., `get_recording_status` polling from the frontend holds the lock), the async task will block while holding a spot on the Tokio runtime thread pool. This can cause thread starvation under heavy polling.

**Recommendation:** Either use `tokio::sync::Mutex` for all fields accessed in async commands, or ensure the `std::sync::Mutex` locks are extremely short-lived and never held across `.await` points. Currently they are not held across await points, which is good, but the contention risk with `file_size()` I/O (issue 5) makes this worth addressing.

---


---

| 4 | IMPORTANT | Non-atomic multi-mutex acquisition; no cleanup on partial failure |

---

| 5 | IMPORTANT | `get_recording_status` holds mutex during filesystem I/O |

---

| 6 | IMPORTANT | `duration_secs()` called after `stop()` inflates duration by EOS wait time |

---

| 7 | IMPORTANT | `std::sync::Mutex` in async function risks blocking tokio runtime |

---

