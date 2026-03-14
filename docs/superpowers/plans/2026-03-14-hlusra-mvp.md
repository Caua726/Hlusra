# Hlusra MVP Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a minimal desktop meeting recorder for Linux (Wayland/Hyprland) that captures audio/video, transcribes, and enables RAG chat per meeting.

**Architecture:** Tauri v2 + Rust backend + React frontend. Five Rust modules (Library, Recorder, Transcription, RAG/Chat, Settings/Export) communicating through the Library as central data service. GStreamer for recording pipeline, FFmpeg for export, PipeWire for capture.

**Tech Stack:** Tauri v2, Rust, React + TypeScript, GStreamer (gstreamer-rs), FFmpeg (ffmpeg-next), rusqlite + sqlite-vec, whisper-rs, PipeWire, ashpd

**Specs:** `docs/superpowers/specs/2026-03-13-recorder-module-design.md`, `docs/superpowers/specs/2026-03-14-library-module-design.md`, `docs/superpowers/specs/2026-03-14-transcription-module-design.md`, `docs/superpowers/specs/2026-03-14-rag-chat-module-design.md`, `docs/superpowers/specs/2026-03-14-settings-export-module-design.md`

---

## Chunk 1: Project Scaffolding + Library Module

This chunk sets up the Tauri project and implements the Library module — the foundation all other modules depend on.

### Task 1: Initialize Tauri + React project

**Files:**
- Create: `package.json`
- Create: `tsconfig.json`
- Create: `vite.config.ts`
- Create: `index.html`
- Create: `src/main.tsx`
- Create: `src/App.tsx`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create Tauri + React project**

```bash
cargo create-tauri-app hlusra-init --template react-ts
```

Copy the generated files into the project root. If `create-tauri-app` is not installed:

```bash
cargo install create-tauri-app
```

Alternatively, initialize manually:

```bash
npm init -y
npm install react react-dom @tauri-apps/api @tauri-apps/plugin-dialog @tauri-apps/plugin-fs
npm install -D typescript @types/react @types/react-dom vite @vitejs/plugin-react
```

- [ ] **Step 2: Configure Cargo.toml with all dependencies**

`src-tauri/Cargo.toml`:
```toml
[package]
name = "hlusra"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
rusqlite = { version = "0.32", features = ["bundled"] }
thiserror = "2"
dirs = "6"
toml = "0.8"
```

- [ ] **Step 3: Configure tauri.conf.json**

`src-tauri/tauri.conf.json`:
```json
{
  "$schema": "https://raw.githubusercontent.com/nicedoc/tauri/refs/heads/dev/crates/tauri-schema-generator/schemas/config.schema.json",
  "productName": "Hlusra",
  "version": "0.1.0",
  "identifier": "com.hlusra.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Hlusra",
        "width": 1024,
        "height": 768,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

- [ ] **Step 4: Create minimal lib.rs and main.rs**

`src-tauri/src/lib.rs`:
```rust
mod library;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

`src-tauri/src/main.rs`:
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    hlusra::run();
}
```

- [ ] **Step 5: Create minimal React shell**

`src/App.tsx`:
```tsx
function App() {
  return (
    <div>
      <h1>Hlusra</h1>
      <p>Meeting Recorder</p>
    </div>
  );
}
export default App;
```

- [ ] **Step 6: Verify build compiles**

```bash
cd src-tauri && cargo check
npm run tauri dev
```

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: initialize Tauri + React project scaffold"
```

---

### Task 2: Library types

**Files:**
- Create: `src-tauri/src/library/mod.rs`
- Create: `src-tauri/src/library/types.rs`

- [ ] **Step 1: Write types tests**

`src-tauri/src/library/types.rs` — define all types with basic serialization tests:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub dir_path: PathBuf,
    pub media_status: MediaStatus,
    pub transcription_status: TranscriptionStatus,
    pub chat_status: ChatStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingSummary {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub media_status: MediaStatus,
    pub transcription_status: TranscriptionStatus,
    pub chat_status: ChatStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingDetail {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub dir_path: PathBuf,
    pub media_status: MediaStatus,
    pub transcription_status: TranscriptionStatus,
    pub chat_status: ChatStatus,
    pub tracks: Vec<TrackInfo>,
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub index: usize,
    pub label: String,
    pub codec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedMeeting {
    pub id: String,
    pub dir_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingInfo {
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub tracks: Vec<TrackInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaStatus {
    Present,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionStatus {
    Pending,
    Processing,
    Done,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatStatus {
    NotIndexed,
    Indexing,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteMode {
    Everything,
    MediaOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactKind {
    Recording,
    Thumbnail,
    TranscriptJson,
    TranscriptTxt,
}

impl ArtifactKind {
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Recording => "recording.mkv",
            Self::Thumbnail => "thumbnail.jpg",
            Self::TranscriptJson => "transcript.json",
            Self::TranscriptTxt => "transcript.txt",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingUpdate {
    pub title: Option<String>,
}

impl MediaStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Deleted => "deleted",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "deleted" => Self::Deleted,
            _ => Self::Present,
        }
    }
}

impl TranscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Done => "done",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "processing" => Self::Processing,
            "done" => Self::Done,
            "failed" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

impl ChatStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotIndexed => "not_indexed",
            Self::Indexing => "indexing",
            Self::Ready => "ready",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "indexing" => Self::Indexing,
            "ready" => Self::Ready,
            "failed" => Self::Failed,
            _ => Self::NotIndexed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_status_roundtrip() {
        assert_eq!(MediaStatus::from_str(MediaStatus::Present.as_str()), MediaStatus::Present);
        assert_eq!(MediaStatus::from_str(MediaStatus::Deleted.as_str()), MediaStatus::Deleted);
    }

    #[test]
    fn test_artifact_kind_filenames() {
        assert_eq!(ArtifactKind::Recording.filename(), "recording.mkv");
        assert_eq!(ArtifactKind::Thumbnail.filename(), "thumbnail.jpg");
        assert_eq!(ArtifactKind::TranscriptJson.filename(), "transcript.json");
        assert_eq!(ArtifactKind::TranscriptTxt.filename(), "transcript.txt");
    }

    #[test]
    fn test_meeting_serialization() {
        let meeting = Meeting {
            id: "test-id".to_string(),
            title: "Test Meeting".to_string(),
            created_at: Utc::now(),
            duration_secs: 3600.0,
            has_video: true,
            file_size: 1024,
            dir_path: PathBuf::from("/tmp/test"),
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        };
        let json = serde_json::to_string(&meeting).unwrap();
        let deserialized: Meeting = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test-id");
    }
}
```

- [ ] **Step 2: Create mod.rs**

`src-tauri/src/library/mod.rs`:
```rust
pub mod types;
pub mod db;
pub mod fs;
pub mod api;

pub use api::Library;
pub use types::*;
```

- [ ] **Step 3: Run tests**

```bash
cd src-tauri && cargo test library::types
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/library/
git commit -m "feat(library): add type definitions"
```

---

### Task 3: Library DB layer

**Files:**
- Create: `src-tauri/src/library/db.rs`

- [ ] **Step 1: Write DB module with schema and migrations**

`src-tauri/src/library/db.rs`:
```rust
use rusqlite::{Connection, params};
use std::path::Path;
use crate::library::types::*;

pub struct LibraryDb {
    conn: Connection,
}

const MIGRATIONS: &[&str] = &[
    // v1: meetings table
    "CREATE TABLE IF NOT EXISTS meetings (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        created_at TEXT NOT NULL,
        duration_secs REAL NOT NULL,
        has_video INTEGER NOT NULL,
        file_size INTEGER NOT NULL,
        dir_path TEXT NOT NULL,
        tracks_json TEXT NOT NULL DEFAULT '[]',
        media_status TEXT NOT NULL DEFAULT 'present',
        transcription_status TEXT NOT NULL DEFAULT 'pending',
        chat_status TEXT NOT NULL DEFAULT 'not_indexed'
    );
    CREATE TABLE IF NOT EXISTS schema_version (
        version INTEGER PRIMARY KEY,
        applied_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    INSERT OR IGNORE INTO schema_version (version) VALUES (1);",
];

impl LibraryDb {
    pub fn open(db_path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
        )?;

        let db = LibraryDb { conn };
        db.run_migrations()?;
        Ok(db)
    }

    pub fn open_in_memory() -> rusqlite::Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = LibraryDb { conn };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> rusqlite::Result<()> {
        let current: u32 = self.conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        for (i, sql) in MIGRATIONS.iter().enumerate() {
            let version = (i + 1) as u32;
            if version > current {
                self.conn.execute_batch(sql)?;
            }
        }
        Ok(())
    }

    pub fn insert_meeting(&self, meeting: &Meeting) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO meetings (id, title, created_at, duration_secs, has_video, file_size, dir_path, media_status, transcription_status, chat_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                meeting.id,
                meeting.title,
                meeting.created_at.to_rfc3339(),
                meeting.duration_secs,
                meeting.has_video as i32,
                meeting.file_size as i64,
                meeting.dir_path.to_string_lossy().to_string(),
                meeting.media_status.as_str(),
                meeting.transcription_status.as_str(),
                meeting.chat_status.as_str(),
            ],
        )?;
        Ok(())
    }

    pub fn get_meeting(&self, id: &str) -> rusqlite::Result<Meeting> {
        self.conn.query_row(
            "SELECT id, title, created_at, duration_secs, has_video, file_size, dir_path, media_status, transcription_status, chat_status
             FROM meetings WHERE id = ?1",
            params![id],
            |row| Self::row_to_meeting(row),
        )
    }

    pub fn list_meetings(&self) -> rusqlite::Result<Vec<MeetingSummary>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, created_at, duration_secs, has_video, file_size, media_status, transcription_status, chat_status
             FROM meetings ORDER BY created_at DESC"
        )?;

        let meetings = stmt.query_map([], |row| {
            Ok(MeetingSummary {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                duration_secs: row.get(3)?,
                has_video: row.get::<_, i32>(4)? != 0,
                file_size: row.get::<_, i64>(5)? as u64,
                media_status: MediaStatus::from_str(&row.get::<_, String>(6)?),
                transcription_status: TranscriptionStatus::from_str(&row.get::<_, String>(7)?),
                chat_status: ChatStatus::from_str(&row.get::<_, String>(8)?),
            })
        })?;

        meetings.collect()
    }

    pub fn update_title(&self, id: &str, title: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET title = ?1 WHERE id = ?2",
            params![title, id],
        )?;
        Ok(())
    }

    pub fn update_transcription_status(&self, id: &str, status: TranscriptionStatus) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET transcription_status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    pub fn update_chat_status(&self, id: &str, status: ChatStatus) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET chat_status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    pub fn update_media_status(&self, id: &str, status: MediaStatus) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET media_status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    pub fn delete_meeting(&self, id: &str) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM meetings WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn row_to_meeting(row: &rusqlite::Row) -> rusqlite::Result<Meeting> {
        Ok(Meeting {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                .unwrap_or_default()
                .with_timezone(&chrono::Utc),
            duration_secs: row.get(3)?,
            has_video: row.get::<_, i32>(4)? != 0,
            file_size: row.get::<_, i64>(5)? as u64,
            dir_path: std::path::PathBuf::from(row.get::<_, String>(6)?),
            media_status: MediaStatus::from_str(&row.get::<_, String>(7)?),
            transcription_status: TranscriptionStatus::from_str(&row.get::<_, String>(8)?),
            chat_status: ChatStatus::from_str(&row.get::<_, String>(9)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;

    fn make_test_meeting(id: &str) -> Meeting {
        Meeting {
            id: id.to_string(),
            title: format!("Meeting {}", id),
            created_at: Utc::now(),
            duration_secs: 3600.0,
            has_video: true,
            file_size: 1024 * 1024,
            dir_path: PathBuf::from(format!("/tmp/{}", id)),
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        }
    }

    #[test]
    fn test_insert_and_get() {
        let db = LibraryDb::open_in_memory().unwrap();
        let meeting = make_test_meeting("test-1");
        db.insert_meeting(&meeting).unwrap();
        let retrieved = db.get_meeting("test-1").unwrap();
        assert_eq!(retrieved.id, "test-1");
        assert_eq!(retrieved.title, "Meeting test-1");
        assert!(retrieved.has_video);
    }

    #[test]
    fn test_list_meetings_ordered() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("a")).unwrap();
        db.insert_meeting(&make_test_meeting("b")).unwrap();
        let list = db.list_meetings().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_update_title() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("t1")).unwrap();
        db.update_title("t1", "New Title").unwrap();
        let m = db.get_meeting("t1").unwrap();
        assert_eq!(m.title, "New Title");
    }

    #[test]
    fn test_update_statuses() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("s1")).unwrap();

        db.update_transcription_status("s1", TranscriptionStatus::Done).unwrap();
        db.update_chat_status("s1", ChatStatus::Ready).unwrap();
        db.update_media_status("s1", MediaStatus::Deleted).unwrap();

        let m = db.get_meeting("s1").unwrap();
        assert_eq!(m.transcription_status, TranscriptionStatus::Done);
        assert_eq!(m.chat_status, ChatStatus::Ready);
        assert_eq!(m.media_status, MediaStatus::Deleted);
    }

    #[test]
    fn test_delete_meeting() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("d1")).unwrap();
        db.delete_meeting("d1").unwrap();
        assert!(db.get_meeting("d1").is_err());
    }
}
```

- [ ] **Step 2: Run tests**

```bash
cd src-tauri && cargo test library::db
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/library/db.rs
git commit -m "feat(library): add SQLite database layer"
```

---

### Task 4: Library filesystem layer

**Files:**
- Create: `src-tauri/src/library/fs.rs`

- [ ] **Step 1: Write filesystem manager**

`src-tauri/src/library/fs.rs`:
```rust
use std::fs;
use std::path::{Path, PathBuf};
use crate::library::types::ArtifactKind;

pub struct LibraryFs {
    base_dir: PathBuf,
}

impl LibraryFs {
    pub fn new(base_dir: PathBuf) -> std::io::Result<Self> {
        fs::create_dir_all(&base_dir)?;
        Ok(LibraryFs { base_dir })
    }

    pub fn create_meeting_dir(&self, dir_name: &str) -> std::io::Result<PathBuf> {
        let path = self.base_dir.join(dir_name);
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn get_artifact_path(&self, meeting_dir: &Path, kind: &ArtifactKind) -> PathBuf {
        meeting_dir.join(kind.filename())
    }

    pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> std::io::Result<PathBuf> {
        let path = self.get_artifact_path(meeting_dir, kind);
        fs::write(&path, data)?;
        Ok(path)
    }

    pub fn has_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> bool {
        self.get_artifact_path(meeting_dir, kind).exists()
    }

    pub fn delete_meeting_dir(&self, meeting_dir: &Path) -> std::io::Result<()> {
        if meeting_dir.exists() {
            fs::remove_dir_all(meeting_dir)?;
        }
        Ok(())
    }

    pub fn delete_media_files(&self, meeting_dir: &Path) -> std::io::Result<()> {
        let recording = self.get_artifact_path(meeting_dir, &ArtifactKind::Recording);
        if recording.exists() {
            fs::remove_file(&recording)?;
        }
        let thumbnail = self.get_artifact_path(meeting_dir, &ArtifactKind::Thumbnail);
        if thumbnail.exists() {
            fs::remove_file(&thumbnail)?;
        }
        Ok(())
    }

    pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> std::io::Result<Vec<u8>> {
        let path = self.get_artifact_path(meeting_dir, kind);
        fs::read(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_meeting_dir() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("2026-03-14_abc123").unwrap();
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    #[test]
    fn test_save_and_read_artifact() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("test").unwrap();

        let data = b"test transcript content";
        lib_fs.save_artifact(&dir, &ArtifactKind::TranscriptTxt, data).unwrap();

        assert!(lib_fs.has_artifact(&dir, &ArtifactKind::TranscriptTxt));
        assert!(!lib_fs.has_artifact(&dir, &ArtifactKind::Recording));

        let read = lib_fs.read_artifact(&dir, &ArtifactKind::TranscriptTxt).unwrap();
        assert_eq!(read, data);
    }

    #[test]
    fn test_delete_media_files() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("test").unwrap();

        lib_fs.save_artifact(&dir, &ArtifactKind::Recording, b"video").unwrap();
        lib_fs.save_artifact(&dir, &ArtifactKind::Thumbnail, b"thumb").unwrap();
        lib_fs.save_artifact(&dir, &ArtifactKind::TranscriptTxt, b"text").unwrap();

        lib_fs.delete_media_files(&dir).unwrap();

        assert!(!lib_fs.has_artifact(&dir, &ArtifactKind::Recording));
        assert!(!lib_fs.has_artifact(&dir, &ArtifactKind::Thumbnail));
        assert!(lib_fs.has_artifact(&dir, &ArtifactKind::TranscriptTxt));
    }

    #[test]
    fn test_delete_meeting_dir() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("deleteme").unwrap();
        lib_fs.save_artifact(&dir, &ArtifactKind::Recording, b"data").unwrap();

        lib_fs.delete_meeting_dir(&dir).unwrap();
        assert!(!dir.exists());
    }
}
```

- [ ] **Step 2: Add tempfile dev-dependency**

In `src-tauri/Cargo.toml`:
```toml
[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 3: Run tests**

```bash
cd src-tauri && cargo test library::fs
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/library/fs.rs src-tauri/Cargo.toml
git commit -m "feat(library): add filesystem layer"
```

---

### Task 5: Library API layer

**Files:**
- Create: `src-tauri/src/library/api.rs`

- [ ] **Step 1: Write Library API that combines DB + FS**

`src-tauri/src/library/api.rs`:
```rust
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use chrono::Utc;
use uuid::Uuid;
use crate::library::db::LibraryDb;
use crate::library::fs::LibraryFs;
use crate::library::types::*;

#[derive(Debug, thiserror::Error)]
pub enum LibraryError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Meeting not found: {0}")]
    NotFound(String),
}

impl serde::Serialize for LibraryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LibraryError>;

pub struct Library {
    db: Mutex<LibraryDb>,
    fs: LibraryFs,
    prepared: Mutex<std::collections::HashMap<String, PathBuf>>,
}

impl Library {
    pub fn new(db_path: &Path, recordings_dir: PathBuf) -> Result<Self> {
        let db = LibraryDb::open(db_path)?;
        let fs = LibraryFs::new(recordings_dir)?;
        Ok(Library {
            db: Mutex::new(db),
            fs,
            prepared: Mutex::new(std::collections::HashMap::new()),
        })
    }

    #[cfg(test)]
    pub fn new_in_memory(recordings_dir: PathBuf) -> Result<Self> {
        let db = LibraryDb::open_in_memory()?;
        let fs = LibraryFs::new(recordings_dir)?;
        Ok(Library {
            db: Mutex::new(db),
            fs,
            prepared: Mutex::new(std::collections::HashMap::new()),
        })
    }

    pub fn prepare_meeting(&self) -> Result<PreparedMeeting> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let dir_name = format!("{}_{}", now.format("%Y-%m-%d"), &id[..8]);
        let dir_path = self.fs.create_meeting_dir(&dir_name)?;

        // Track prepared meeting internally
        self.prepared.lock().unwrap().insert(id.clone(), dir_path.clone());

        Ok(PreparedMeeting { id, dir_path })
    }

    /// Library tracks prepared meetings internally to avoid redundant dir_path param
    pub fn finalize_meeting(&self, id: &str, info: RecordingInfo) -> Result<Meeting> {
        let now = Utc::now();
        let title = format!("Reunião {}", now.format("%Y-%m-%d %H:%M"));

        let dir_path = self.prepared.lock().unwrap().remove(id)
            .ok_or(LibraryError::NotFound(format!("No prepared meeting with id {}", id)))?;

        let meeting = Meeting {
            id: id.to_string(),
            title,
            created_at: now,
            duration_secs: info.duration_secs,
            has_video: info.has_video,
            file_size: info.file_size,
            dir_path,
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        };

        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        db.insert_meeting(&meeting)?;

        Ok(meeting)
    }

    pub fn get_meeting(&self, id: &str) -> Result<Meeting> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        db.get_meeting(id).map_err(|_| LibraryError::NotFound(id.to_string()))
    }

    pub fn list_meetings(&self) -> Result<Vec<MeetingSummary>> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.list_meetings()?)
    }

    pub fn update_title(&self, id: &str, title: &str) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.update_title(id, title)?)
    }

    pub fn delete_meeting(&self, id: &str, mode: DeleteMode) -> Result<()> {
        // Single lock acquisition to avoid TOCTOU race
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        let meeting = db.get_meeting(id).map_err(|_| LibraryError::NotFound(id.to_string()))?;

        match mode {
            DeleteMode::Everything => {
                self.fs.delete_meeting_dir(&meeting.dir_path)?;
                db.delete_meeting(id)?;
            }
            DeleteMode::MediaOnly => {
                self.fs.delete_media_files(&meeting.dir_path)?;
                db.update_media_status(id, MediaStatus::Deleted)?;
            }
        }
        Ok(())
    }

    pub fn update_transcription_status(&self, id: &str, status: TranscriptionStatus) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.update_transcription_status(id, status)?)
    }

    pub fn update_chat_status(&self, id: &str, status: ChatStatus) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.update_chat_status(id, status)?)
    }

    pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> Result<PathBuf> {
        Ok(self.fs.save_artifact(meeting_dir, kind, data)?)
    }

    pub fn get_artifact_path(&self, meeting_dir: &Path, kind: &ArtifactKind) -> PathBuf {
        self.fs.get_artifact_path(meeting_dir, kind)
    }

    pub fn has_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> bool {
        self.fs.has_artifact(meeting_dir, kind)
    }

    pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> Result<Vec<u8>> {
        Ok(self.fs.read_artifact(meeting_dir, kind)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup() -> (Library, TempDir) {
        let tmp = TempDir::new().unwrap();
        let lib = Library::new_in_memory(tmp.path().to_path_buf()).unwrap();
        (lib, tmp)
    }

    #[test]
    fn test_prepare_and_finalize_meeting() {
        let (lib, _tmp) = setup();

        let prepared = lib.prepare_meeting().unwrap();
        assert!(!prepared.id.is_empty());
        assert!(prepared.dir_path.exists());

        let info = RecordingInfo {
            duration_secs: 1800.0,
            has_video: true,
            file_size: 512 * 1024,
            tracks: vec![
                TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
                TrackInfo { index: 1, label: "system".to_string(), codec: "opus".to_string() },
            ],
        };

        let meeting = lib.finalize_meeting(&prepared.id, info).unwrap();
        assert_eq!(meeting.id, prepared.id);
        assert!(meeting.has_video);
        assert_eq!(meeting.media_status, MediaStatus::Present);
    }

    #[test]
    fn test_list_meetings() {
        let (lib, _tmp) = setup();

        let p1 = lib.prepare_meeting().unwrap();
        lib.finalize_meeting(&p1.id, RecordingInfo {
            duration_secs: 100.0, has_video: false, file_size: 1024,
            tracks: vec![],
        }).unwrap();

        let p2 = lib.prepare_meeting().unwrap();
        lib.finalize_meeting(&p2.id, RecordingInfo {
            duration_secs: 200.0, has_video: true, file_size: 2048,
            tracks: vec![],
        }).unwrap();

        let list = lib.list_meetings().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_delete_everything() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();
        lib.finalize_meeting(&p.id, RecordingInfo {
            duration_secs: 60.0, has_video: false, file_size: 512,
            tracks: vec![],
        }).unwrap();

        lib.delete_meeting(&p.id, DeleteMode::Everything).unwrap();
        assert!(lib.get_meeting(&p.id).is_err());
        assert!(!p.dir_path.exists());
    }

    #[test]
    fn test_delete_media_only() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();
        lib.save_artifact(&p.dir_path, &ArtifactKind::Recording, b"video").unwrap();
        lib.save_artifact(&p.dir_path, &ArtifactKind::TranscriptTxt, b"text").unwrap();

        lib.finalize_meeting(&p.id, RecordingInfo {
            duration_secs: 60.0, has_video: true, file_size: 512,
            tracks: vec![],
        }).unwrap();

        lib.delete_meeting(&p.id, DeleteMode::MediaOnly).unwrap();

        let m = lib.get_meeting(&p.id).unwrap();
        assert_eq!(m.media_status, MediaStatus::Deleted);
        assert!(!lib.has_artifact(&p.dir_path, &ArtifactKind::Recording));
        assert!(lib.has_artifact(&p.dir_path, &ArtifactKind::TranscriptTxt));
    }

    #[test]
    fn test_artifacts() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();

        lib.save_artifact(&p.dir_path, &ArtifactKind::TranscriptJson, b"{\"text\":\"hello\"}").unwrap();
        assert!(lib.has_artifact(&p.dir_path, &ArtifactKind::TranscriptJson));

        let data = lib.read_artifact(&p.dir_path, &ArtifactKind::TranscriptJson).unwrap();
        assert_eq!(data, b"{\"text\":\"hello\"}");
    }
}
```

- [ ] **Step 2: Run tests**

```bash
cd src-tauri && cargo test library::api
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/library/api.rs
git commit -m "feat(library): add API layer combining DB and filesystem"
```

---

### Task 6: Library Tauri commands

**Files:**
- Modify: `src-tauri/src/library/mod.rs`
- Create: `src-tauri/src/library/commands.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write Tauri command handlers**

`src-tauri/src/library/commands.rs`:
```rust
use tauri::State;
use crate::library::api::{Library, LibraryError};
use crate::library::types::*;

type Result<T> = std::result::Result<T, LibraryError>;

#[tauri::command]
pub fn list_meetings(library: State<'_, Library>) -> Result<Vec<MeetingSummary>> {
    library.list_meetings()
}

#[tauri::command]
pub fn get_meeting(library: State<'_, Library>, id: String) -> Result<Meeting> {
    library.get_meeting(&id)
}

#[tauri::command]
pub fn update_meeting_title(library: State<'_, Library>, id: String, title: String) -> Result<()> {
    library.update_title(&id, &title)
}

#[tauri::command]
pub fn delete_meeting(library: State<'_, Library>, id: String, mode: DeleteMode) -> Result<()> {
    library.delete_meeting(&id, mode)
}

#[tauri::command]
pub fn get_thumbnail(library: State<'_, Library>, id: String) -> Result<Option<Vec<u8>>> {
    let meeting = library.get_meeting(&id)?;
    if library.has_artifact(&meeting.dir_path, &ArtifactKind::Thumbnail) {
        let data = library.read_artifact(&meeting.dir_path, &ArtifactKind::Thumbnail)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}
```

- [ ] **Step 2: Update mod.rs**

`src-tauri/src/library/mod.rs`:
```rust
pub mod types;
pub mod db;
pub mod fs;
pub mod api;
pub mod commands;

pub use api::Library;
pub use types::*;
```

- [ ] **Step 3: Register commands and Library state in lib.rs**

`src-tauri/src/lib.rs`:
```rust
mod library;

use library::Library;
use std::path::PathBuf;

fn get_recordings_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Hlusra")
        .join("recordings")
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
    let library = Library::new(
        &get_db_path(),
        get_recordings_dir(),
    ).expect("Failed to initialize library");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(library)
        .invoke_handler(tauri::generate_handler![
            library::commands::list_meetings,
            library::commands::get_meeting,
            library::commands::update_meeting_title,
            library::commands::delete_meeting,
            library::commands::get_thumbnail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 4: Verify compilation**

```bash
cd src-tauri && cargo check
```

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/library/commands.rs src-tauri/src/library/mod.rs src-tauri/src/lib.rs
git commit -m "feat(library): add Tauri commands and app initialization"
```

---

### Task 7: Basic frontend shell with gallery

**Files:**
- Create: `src/lib/api.ts`
- Create: `src/components/Gallery.tsx`
- Create: `src/components/MeetingCard.tsx`
- Modify: `src/App.tsx`
- Create: `src/styles/app.css`

- [ ] **Step 1: Create TypeScript API bindings**

`src/lib/api.ts`:
```typescript
import { invoke } from "@tauri-apps/api/core";

export interface MeetingSummary {
  id: string;
  title: string;
  created_at: string;
  duration_secs: number;
  has_video: boolean;
  file_size: number;
  media_status: "present" | "deleted";
  transcription_status: "pending" | "processing" | "done" | "failed";
  chat_status: "not_indexed" | "indexing" | "ready" | "failed";
}

export interface Meeting extends MeetingSummary {
  dir_path: string;
  tracks: TrackInfo[];
  transcript: string | null;
}

export interface TrackInfo {
  index: number;
  label: string;
  codec: string;
}

export async function listMeetings(): Promise<MeetingSummary[]> {
  return invoke("list_meetings");
}

export async function getMeeting(id: string): Promise<Meeting> {
  return invoke("get_meeting", { id });
}

export async function updateMeetingTitle(id: string, title: string): Promise<void> {
  return invoke("update_meeting_title", { id, title });
}

export async function deleteMeeting(id: string, mode: "everything" | "media_only"): Promise<void> {
  return invoke("delete_meeting", { id, mode });
}
```

- [ ] **Step 2: Create Gallery and MeetingCard components**

`src/components/MeetingCard.tsx`:
```tsx
import { MeetingSummary } from "../lib/api";

interface Props {
  meeting: MeetingSummary;
  onClick: (id: string) => void;
}

function formatDuration(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m ${s}s`;
  return `${s}s`;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

export default function MeetingCard({ meeting, onClick }: Props) {
  return (
    <div className="meeting-card" onClick={() => onClick(meeting.id)}>
      <div className="meeting-card-header">
        <span className="meeting-type">{meeting.has_video ? "Video" : "Audio"}</span>
        <span className="meeting-duration">{formatDuration(meeting.duration_secs)}</span>
      </div>
      <h3 className="meeting-title">{meeting.title}</h3>
      <div className="meeting-meta">
        <span>{new Date(meeting.created_at).toLocaleDateString()}</span>
        <span>{formatSize(meeting.file_size)}</span>
      </div>
      <div className="meeting-status">
        <span className={`status-badge ${meeting.transcription_status}`}>
          {meeting.transcription_status}
        </span>
      </div>
    </div>
  );
}
```

`src/components/Gallery.tsx`:
```tsx
import { useState, useEffect } from "react";
import { MeetingSummary, listMeetings } from "../lib/api";
import MeetingCard from "./MeetingCard";

interface Props {
  onSelectMeeting: (id: string) => void;
}

export default function Gallery({ onSelectMeeting }: Props) {
  const [meetings, setMeetings] = useState<MeetingSummary[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadMeetings();
  }, []);

  async function loadMeetings() {
    try {
      const list = await listMeetings();
      setMeetings(list);
    } catch (err) {
      console.error("Failed to load meetings:", err);
    } finally {
      setLoading(false);
    }
  }

  if (loading) return <div className="loading">Loading...</div>;

  return (
    <div className="gallery">
      <h2>Reuniões</h2>
      {meetings.length === 0 ? (
        <p className="empty">Nenhuma reunião gravada ainda.</p>
      ) : (
        <div className="meeting-grid">
          {meetings.map((m) => (
            <MeetingCard key={m.id} meeting={m} onClick={onSelectMeeting} />
          ))}
        </div>
      )}
    </div>
  );
}
```

- [ ] **Step 3: Update App.tsx with basic routing**

`src/App.tsx`:
```tsx
import { useState } from "react";
import Gallery from "./components/Gallery";
import "./styles/app.css";

type View = { kind: "home" } | { kind: "gallery" } | { kind: "meeting"; id: string };

function App() {
  const [view, setView] = useState<View>({ kind: "home" });

  return (
    <div className="app">
      {view.kind === "home" && (
        <div className="home">
          <h1>Hlusra</h1>
          <div className="home-actions">
            <button className="btn-primary btn-large" disabled>
              Começar a gravar
            </button>
            <label className="toggle">
              <input type="checkbox" disabled />
              <span>Gravar tela</span>
            </label>
          </div>
          <button className="btn-secondary" onClick={() => setView({ kind: "gallery" })}>
            Galeria
          </button>
        </div>
      )}

      {view.kind === "gallery" && (
        <div>
          <button className="btn-back" onClick={() => setView({ kind: "home" })}>
            ← Voltar
          </button>
          <Gallery onSelectMeeting={(id) => setView({ kind: "meeting", id })} />
        </div>
      )}

      {view.kind === "meeting" && (
        <div>
          <button className="btn-back" onClick={() => setView({ kind: "gallery" })}>
            ← Voltar
          </button>
          <p>Meeting: {view.id}</p>
        </div>
      )}
    </div>
  );
}

export default App;
```

- [ ] **Step 4: Add basic styles**

`src/styles/app.css`: Create minimal dark-theme CSS for the app shell.

- [ ] **Step 5: Verify the app runs**

```bash
npm run tauri dev
```

- [ ] **Step 6: Commit**

```bash
git add src/
git commit -m "feat(frontend): add gallery shell and API bindings"
```

---

## Chunk 2: Recorder Module

This chunk implements the Recorder module — GStreamer pipeline with PipeWire capture.

### Task 8: Recorder types and module scaffold

**Files:**
- Create: `src-tauri/src/recorder/mod.rs`
- Create: `src-tauri/src/recorder/types.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Define recorder types**

`src-tauri/src/recorder/types.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncoderBackend {
    Vaapi,
    Cuda,
    Vulkan,
    Software,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoCodec {
    H264,
    H265,
    Av1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingState {
    Idle,
    Recording,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingStatus {
    pub state: RecordingState,
    pub duration_secs: f64,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub codec: VideoCodec,
    pub backend: EncoderBackend,
    pub bitrate: u32,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub bitrate: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            codec: VideoCodec::H265,
            backend: EncoderBackend::Vaapi,
            bitrate: 2_000_000,
            fps: 15,
            width: 1280,
            height: 720,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self { bitrate: 64_000 }
    }
}

impl EncoderBackend {
    pub fn gst_element_name(&self, codec: &VideoCodec) -> &'static str {
        match (self, codec) {
            (Self::Vaapi, VideoCodec::H264) => "vaapih264enc",
            (Self::Vaapi, VideoCodec::H265) => "vaapih265enc",
            (Self::Vaapi, VideoCodec::Av1) => "vaapiav1enc",
            (Self::Cuda, VideoCodec::H264) => "nvh264enc",
            (Self::Cuda, VideoCodec::H265) => "nvh265enc",
            (Self::Cuda, VideoCodec::Av1) => "nvav1enc",
            (Self::Vulkan, VideoCodec::H264) => "vulkanh264enc",
            (Self::Vulkan, VideoCodec::H265) => "vulkanh265enc",
            (Self::Vulkan, VideoCodec::Av1) => "vulkanav1enc",
            (Self::Software, VideoCodec::H264) => "x264enc",
            (Self::Software, VideoCodec::H265) => "x265enc",
            (Self::Software, VideoCodec::Av1) => "svtav1enc",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_element_names() {
        assert_eq!(EncoderBackend::Vaapi.gst_element_name(&VideoCodec::H265), "vaapih265enc");
        assert_eq!(EncoderBackend::Cuda.gst_element_name(&VideoCodec::H264), "nvh264enc");
        assert_eq!(EncoderBackend::Software.gst_element_name(&VideoCodec::Av1), "svtav1enc");
        assert_eq!(EncoderBackend::Vulkan.gst_element_name(&VideoCodec::Av1), "vulkanav1enc");
    }
}
```

- [ ] **Step 2: Create mod.rs**

`src-tauri/src/recorder/mod.rs`:
```rust
pub mod types;
pub mod capture;
pub mod encode;
pub mod pipeline;
pub mod commands;
```

- [ ] **Step 3: Add GStreamer dependencies to Cargo.toml**

Add to `src-tauri/Cargo.toml`:
```toml
gstreamer = "0.23"
gstreamer-video = "0.23"
gstreamer-audio = "0.23"
ashpd = { version = "0.10", features = ["tokio"] }
pipewire = "0.8"
```

- [ ] **Step 4: Register recorder module in lib.rs**

Add `mod recorder;` to `src-tauri/src/lib.rs`.

- [ ] **Step 5: Run tests and verify compilation**

```bash
cd src-tauri && cargo test recorder::types && cargo check
```

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/recorder/ src-tauri/Cargo.toml src-tauri/src/lib.rs
git commit -m "feat(recorder): add types and module scaffold"
```

---

### Task 9: Encode layer

**Files:**
- Create: `src-tauri/src/recorder/encode.rs`

- [ ] **Step 1: Write encode layer — probe and factory**

`src-tauri/src/recorder/encode.rs`:
```rust
use std::collections::HashMap;
use gstreamer as gst;
use gstreamer::prelude::*;
use crate::recorder::types::*;

const BACKENDS: &[EncoderBackend] = &[
    EncoderBackend::Vaapi,
    EncoderBackend::Cuda,
    EncoderBackend::Vulkan,
    EncoderBackend::Software,
];

const CODECS: &[VideoCodec] = &[
    VideoCodec::H264,
    VideoCodec::H265,
    VideoCodec::Av1,
];

pub fn probe_available() -> HashMap<EncoderBackend, Vec<VideoCodec>> {
    let mut result = HashMap::new();
    for &backend in BACKENDS {
        let mut codecs = Vec::new();
        for &codec in CODECS {
            let name = backend.gst_element_name(&codec);
            if gst::ElementFactory::find(name).is_some() {
                codecs.push(codec);
            }
        }
        if !codecs.is_empty() {
            result.insert(backend, codecs);
        }
    }
    result
}

pub fn create_video_encoder(
    backend: EncoderBackend,
    codec: VideoCodec,
    config: &VideoConfig,
) -> Result<gst::Element, String> {
    let element_name = backend.gst_element_name(&codec);
    let encoder = gst::ElementFactory::make(element_name)
        .build()
        .map_err(|e| format!("Failed to create encoder {}: {}", element_name, e))?;

    // Set bitrate — property name varies by encoder
    match backend {
        EncoderBackend::Vaapi => {
            encoder.set_property("bitrate", config.bitrate / 1000); // kbps
        }
        EncoderBackend::Cuda => {
            encoder.set_property("bitrate", config.bitrate / 1000); // kbps
        }
        EncoderBackend::Software => {
            encoder.set_property("bitrate", config.bitrate / 1000); // kbps
        }
        EncoderBackend::Vulkan => {
            // Vulkan encoder properties vary, set if available
        }
    }

    Ok(encoder)
}

pub fn create_audio_encoder(config: &AudioConfig) -> Result<gst::Element, String> {
    let encoder = gst::ElementFactory::make("opusenc")
        .property("bitrate", config.bitrate as i32)
        .build()
        .map_err(|e| format!("Failed to create opusenc: {}", e))?;
    Ok(encoder)
}

/// Try to create a video encoder with fallback chain.
/// Order: requested backend → next available → software.
pub fn create_video_encoder_with_fallback(
    preferred: EncoderBackend,
    codec: VideoCodec,
    config: &VideoConfig,
) -> Result<(gst::Element, EncoderBackend), String> {
    // Try preferred first
    if let Ok(enc) = create_video_encoder(preferred, codec, config) {
        return Ok((enc, preferred));
    }

    // Try others in order
    let available = probe_available();
    for &backend in BACKENDS {
        if backend == preferred {
            continue;
        }
        if let Some(codecs) = available.get(&backend) {
            if codecs.contains(&codec) {
                if let Ok(enc) = create_video_encoder(backend, codec, config) {
                    return Ok((enc, backend));
                }
            }
        }
    }

    Err(format!("No encoder available for {:?}", codec))
}
```

- [ ] **Step 2: Run tests**

```bash
cd src-tauri && cargo check
```

Note: Full tests require GStreamer installed. Unit tests for probe logic can be added but element creation tests are integration-level.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/recorder/encode.rs
git commit -m "feat(recorder): add encode layer with probe and fallback"
```

---

### Task 10: Capture layer

**Files:**
- Create: `src-tauri/src/recorder/capture.rs`

- [ ] **Step 1: Write capture layer — portal + PipeWire**

`src-tauri/src/recorder/capture.rs`:
```rust
use ashpd::desktop::screencast::{CursorMode, PersistMode, Screencast, SourceType};
use ashpd::WindowIdentifier;

#[derive(Debug, Clone)]
pub struct PipeWireSource {
    pub node_id: u32,
    pub fd: std::os::fd::RawFd,
}

pub struct ScreenCapture {
    node_id: Option<u32>,
    fd: Option<std::os::unix::io::OwnedFd>,
}

impl ScreenCapture {
    pub fn new() -> Self {
        Self { node_id: None, fd: None }
    }

    /// Opens the XDG Desktop Portal screen picker and returns a PipeWire source.
    pub async fn request_screen(&mut self) -> Result<PipeWireSource, String> {
        let proxy = Screencast::new()
            .await
            .map_err(|e| format!("Failed to create screencast proxy: {}", e))?;

        let session = proxy.create_session()
            .await
            .map_err(|e| format!("Failed to create session: {}", e))?;

        proxy.select_sources(
            &session,
            CursorMode::Embedded,
            SourceType::Monitor | SourceType::Window,
            false,
            None,
            PersistMode::DoNot,
        )
        .await
        .map_err(|e| format!("Failed to select sources: {}", e))?;

        let response = proxy.start(&session, &WindowIdentifier::default())
            .await
            .map_err(|e| format!("Failed to start: {}", e))?;

        let stream = response.streams().first()
            .ok_or("No stream returned from portal")?;

        let fd = response.fd()
            .map_err(|e| format!("Failed to get fd: {}", e))?;

        let node_id = stream.pipe_wire_node_id();

        use std::os::fd::AsRawFd;
        let raw_fd = fd.as_raw_fd();

        self.node_id = Some(node_id);
        self.fd = Some(fd);

        Ok(PipeWireSource { node_id, fd: raw_fd })
    }
}
```

Note: Audio capture via PipeWire defaults are handled directly in the GStreamer pipeline with `pipewiresrc`. Device enumeration will be added when the Settings module is implemented.

- [ ] **Step 2: Verify compilation**

```bash
cd src-tauri && cargo check
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/recorder/capture.rs
git commit -m "feat(recorder): add capture layer with XDG portal"
```

---

### Task 11: Pipeline layer

**Files:**
- Create: `src-tauri/src/recorder/pipeline.rs`

- [ ] **Step 1: Write pipeline assembly and state management**

`src-tauri/src/recorder/pipeline.rs`:
```rust
use gstreamer as gst;
use gstreamer::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::recorder::capture::PipeWireSource;
use crate::recorder::encode;
use crate::recorder::types::*;

pub struct RecordingPipeline {
    pipeline: gst::Pipeline,
    start_time: Instant,
    output_path: PathBuf,
    has_video: bool,
}

impl RecordingPipeline {
    /// Build an audio-only pipeline (2 PipeWire sources → Opus → MKV)
    pub fn build_audio_only(
        output_path: PathBuf,
        audio_config: &AudioConfig,
    ) -> Result<Self, String> {
        gst::init().map_err(|e| format!("GStreamer init failed: {}", e))?;

        let pipeline = gst::Pipeline::new();

        // Mic source
        let mic_src = gst::ElementFactory::make("pipewiresrc")
            .name("mic_src")
            .build()
            .map_err(|e| format!("pipewiresrc: {}", e))?;

        let mic_queue = gst::ElementFactory::make("queue").name("mic_queue").build().map_err(|e| e.to_string())?;
        let mic_convert = gst::ElementFactory::make("audioconvert").name("mic_convert").build().map_err(|e| e.to_string())?;
        let mic_enc = encode::create_audio_encoder(audio_config)?;
        mic_enc.set_name("mic_enc").ok();

        // System audio source
        let sys_src = gst::ElementFactory::make("pipewiresrc")
            .name("sys_src")
            .build()
            .map_err(|e| format!("pipewiresrc: {}", e))?;

        let sys_queue = gst::ElementFactory::make("queue").name("sys_queue").build().map_err(|e| e.to_string())?;
        let sys_convert = gst::ElementFactory::make("audioconvert").name("sys_convert").build().map_err(|e| e.to_string())?;
        let sys_enc = encode::create_audio_encoder(audio_config)?;
        sys_enc.set_name("sys_enc").ok();

        // Muxer + sink
        let mux = gst::ElementFactory::make("matroskamux").name("mux").build().map_err(|e| e.to_string())?;
        let filesink = gst::ElementFactory::make("filesink")
            .name("filesink")
            .property("location", output_path.to_string_lossy().to_string())
            .build()
            .map_err(|e| e.to_string())?;

        pipeline.add_many(&[
            &mic_src, &mic_queue, &mic_convert, &mic_enc,
            &sys_src, &sys_queue, &sys_convert, &sys_enc,
            &mux, &filesink,
        ]).map_err(|e| e.to_string())?;

        // Link mic path
        gst::Element::link_many(&[&mic_src, &mic_queue, &mic_convert, &mic_enc])
            .map_err(|e| format!("Link mic: {}", e))?;
        mic_enc.link(&mux).map_err(|e| format!("Link mic→mux: {}", e))?;

        // Link system path
        gst::Element::link_many(&[&sys_src, &sys_queue, &sys_convert, &sys_enc])
            .map_err(|e| format!("Link sys: {}", e))?;
        sys_enc.link(&mux).map_err(|e| format!("Link sys→mux: {}", e))?;

        // Link mux → sink
        mux.link(&filesink).map_err(|e| format!("Link mux→sink: {}", e))?;

        Ok(Self {
            pipeline,
            start_time: Instant::now(),
            output_path,
            has_video: false,
        })
    }

    /// Build a video + audio pipeline
    pub fn build_with_video(
        output_path: PathBuf,
        screen_source: &PipeWireSource,
        video_config: &VideoConfig,
        audio_config: &AudioConfig,
    ) -> Result<Self, String> {
        gst::init().map_err(|e| format!("GStreamer init failed: {}", e))?;

        let pipeline = gst::Pipeline::new();

        // Screen source
        let screen_src = gst::ElementFactory::make("pipewiresrc")
            .name("screen_src")
            .property("fd", screen_source.fd)
            .property("path", screen_source.node_id.to_string())
            .build()
            .map_err(|e| format!("pipewiresrc screen: {}", e))?;

        let video_queue = gst::ElementFactory::make("queue").name("video_queue").build().map_err(|e| e.to_string())?;
        let videoconvert = gst::ElementFactory::make("videoconvert").name("videoconvert").build().map_err(|e| e.to_string())?;

        let (video_enc, _actual_backend) = encode::create_video_encoder_with_fallback(
            video_config.backend, video_config.codec, video_config,
        )?;
        video_enc.set_name("video_enc").ok();

        // Audio sources (same as audio-only)
        let mic_src = gst::ElementFactory::make("pipewiresrc").name("mic_src").build().map_err(|e| e.to_string())?;
        let mic_queue = gst::ElementFactory::make("queue").name("mic_queue").build().map_err(|e| e.to_string())?;
        let mic_convert = gst::ElementFactory::make("audioconvert").name("mic_convert").build().map_err(|e| e.to_string())?;
        let mic_enc = encode::create_audio_encoder(audio_config)?;

        let sys_src = gst::ElementFactory::make("pipewiresrc").name("sys_src").build().map_err(|e| e.to_string())?;
        let sys_queue = gst::ElementFactory::make("queue").name("sys_queue").build().map_err(|e| e.to_string())?;
        let sys_convert = gst::ElementFactory::make("audioconvert").name("sys_convert").build().map_err(|e| e.to_string())?;
        let sys_enc = encode::create_audio_encoder(audio_config)?;

        let mux = gst::ElementFactory::make("matroskamux").name("mux").build().map_err(|e| e.to_string())?;
        let filesink = gst::ElementFactory::make("filesink")
            .name("filesink")
            .property("location", output_path.to_string_lossy().to_string())
            .build()
            .map_err(|e| e.to_string())?;

        pipeline.add_many(&[
            &screen_src, &video_queue, &videoconvert, &video_enc,
            &mic_src, &mic_queue, &mic_convert, &mic_enc,
            &sys_src, &sys_queue, &sys_convert, &sys_enc,
            &mux, &filesink,
        ]).map_err(|e| e.to_string())?;

        // Link video
        gst::Element::link_many(&[&screen_src, &video_queue, &videoconvert, &video_enc])
            .map_err(|e| format!("Link video: {}", e))?;
        video_enc.link(&mux).map_err(|e| format!("Link video→mux: {}", e))?;

        // Link mic
        gst::Element::link_many(&[&mic_src, &mic_queue, &mic_convert, &mic_enc])
            .map_err(|e| format!("Link mic: {}", e))?;
        mic_enc.link(&mux).map_err(|e| format!("Link mic→mux: {}", e))?;

        // Link system
        gst::Element::link_many(&[&sys_src, &sys_queue, &sys_convert, &sys_enc])
            .map_err(|e| format!("Link sys: {}", e))?;
        sys_enc.link(&mux).map_err(|e| format!("Link sys→mux: {}", e))?;

        mux.link(&filesink).map_err(|e| format!("Link mux→sink: {}", e))?;

        Ok(Self {
            pipeline,
            start_time: Instant::now(),
            output_path,
            has_video: true,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.start_time = Instant::now();
        self.pipeline.set_state(gst::State::Playing)
            .map_err(|e| format!("Failed to start pipeline: {:?}", e))?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.pipeline.send_event(gst::event::Eos::new());
        // Wait for EOS to propagate
        let bus = self.pipeline.bus().ok_or("No bus")?;
        let _msg = bus.timed_pop_filtered(
            gst::ClockTime::from_seconds(5),
            &[gst::MessageType::Eos, gst::MessageType::Error],
        );
        self.pipeline.set_state(gst::State::Null)
            .map_err(|e| format!("Failed to stop pipeline: {:?}", e))?;
        Ok(())
    }

    pub fn duration_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    pub fn output_path(&self) -> &PathBuf {
        &self.output_path
    }

    pub fn has_video(&self) -> bool {
        self.has_video
    }

    pub fn file_size(&self) -> u64 {
        std::fs::metadata(&self.output_path)
            .map(|m| m.len())
            .unwrap_or(0)
    }
}
```

- [ ] **Step 2: Verify compilation**

```bash
cd src-tauri && cargo check
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/recorder/pipeline.rs
git commit -m "feat(recorder): add pipeline layer with GStreamer assembly"
```

---

### Task 12: Recorder commands and integration

**Files:**
- Create: `src-tauri/src/recorder/commands.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write recorder Tauri commands**

`src-tauri/src/recorder/commands.rs`:
```rust
use std::sync::Mutex;
use tauri::State;
use crate::library::api::Library;
use crate::library::types::{RecordingInfo, TrackInfo, ArtifactKind};
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
    let prepared = library.prepare_meeting().map_err(|e| e.to_string())?;
    let output_path = prepared.dir_path.join("recording.mkv");

    let video_config = VideoConfig::default();
    let audio_config = AudioConfig::default();

    let mut pipeline = if with_video {
        let mut capture = ScreenCapture::new();
        let source = capture.request_screen().await?;
        let p = RecordingPipeline::build_with_video(output_path, &source, &video_config, &audio_config)?;
        // Store capture in RecorderState so OwnedFd outlives the pipeline
        *recorder.capture.lock().unwrap() = Some(capture);
        p
    } else {
        RecordingPipeline::build_audio_only(output_path, &audio_config)?
    };

    pipeline.start()?;

    *recorder.pipeline.lock().unwrap() = Some(pipeline);
    *recorder.current_meeting_id.lock().unwrap() = Some(prepared.id.clone());

    Ok(prepared.id)
}

#[tauri::command]
pub fn stop_recording(
    library: State<'_, Library>,
    recorder: State<'_, RecorderState>,
) -> Result<crate::library::types::Meeting, String> {
    let mut pipeline_lock = recorder.pipeline.lock().unwrap();
    let pipeline = pipeline_lock.take().ok_or("No active recording")?;

    pipeline.stop()?;

    // Release screen capture fd
    *recorder.capture.lock().unwrap() = None;

    let meeting_id = recorder.current_meeting_id.lock().unwrap().take()
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
) -> RecordingStatus {
    let pipeline_lock = recorder.pipeline.lock().unwrap();
    match pipeline_lock.as_ref() {
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
    }
}

#[tauri::command]
pub fn probe_encoders() -> std::collections::HashMap<String, Vec<String>> {
    gstreamer::init().ok();
    let available = crate::recorder::encode::probe_available();
    available.into_iter()
        .map(|(backend, codecs)| {
            (format!("{:?}", backend).to_lowercase(),
             codecs.iter().map(|c| format!("{:?}", c).to_lowercase()).collect())
        })
        .collect()
}
```

- [ ] **Step 2: Register recorder state and commands in lib.rs**

Update `src-tauri/src/lib.rs` to add:
```rust
mod recorder;
```

And in the builder:
```rust
.manage(recorder::commands::RecorderState::new())
.invoke_handler(tauri::generate_handler![
    // library commands...
    library::commands::list_meetings,
    library::commands::get_meeting,
    library::commands::update_meeting_title,
    library::commands::delete_meeting,
    library::commands::get_thumbnail,
    // recorder commands...
    recorder::commands::start_recording,
    recorder::commands::stop_recording,
    recorder::commands::get_recording_status,
    recorder::commands::probe_encoders,
])
```

- [ ] **Step 3: Verify compilation**

```bash
cd src-tauri && cargo check
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/recorder/ src-tauri/src/lib.rs
git commit -m "feat(recorder): add commands and Library integration"
```

---

### Task 13: Recording UI in frontend

**Files:**
- Create: `src/components/RecordButton.tsx`
- Modify: `src/App.tsx`
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add recorder API bindings**

Add to `src/lib/api.ts`:
```typescript
export interface RecordingStatus {
  state: "idle" | "recording" | "stopped";
  duration_secs: number;
  file_size: number;
}

export async function startRecording(withVideo: boolean): Promise<string> {
  return invoke("start_recording", { withVideo });
}

export async function stopRecording(): Promise<void> {
  return invoke("stop_recording");
}

export async function getRecordingStatus(): Promise<RecordingStatus> {
  return invoke("get_recording_status");
}

export async function probeEncoders(): Promise<Record<string, string[]>> {
  return invoke("probe_encoders");
}
```

- [ ] **Step 2: Create RecordButton component**

`src/components/RecordButton.tsx`:
```tsx
import { useState, useEffect, useRef } from "react";
import { startRecording, stopRecording, getRecordingStatus, RecordingStatus } from "../lib/api";

interface Props {
  onRecordingFinished: () => void;
}

function formatTimer(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  return [h, m, s].map((v) => String(v).padStart(2, "0")).join(":");
}

export default function RecordButton({ onRecordingFinished }: Props) {
  const [recording, setRecording] = useState(false);
  const [withVideo, setWithVideo] = useState(false);
  const [status, setStatus] = useState<RecordingStatus | null>(null);
  const [error, setError] = useState<string | null>(null);
  const intervalRef = useRef<number | null>(null);

  useEffect(() => {
    if (recording) {
      intervalRef.current = window.setInterval(async () => {
        const s = await getRecordingStatus();
        setStatus(s);
      }, 1000);
    }
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [recording]);

  async function handleStart() {
    try {
      setError(null);
      await startRecording(withVideo);
      setRecording(true);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleStop() {
    try {
      await stopRecording();
      setRecording(false);
      setStatus(null);
      onRecordingFinished();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="record-section">
      {!recording ? (
        <>
          <button className="btn-primary btn-large" onClick={handleStart}>
            Começar a gravar
          </button>
          <label className="toggle">
            <input
              type="checkbox"
              checked={withVideo}
              onChange={(e) => setWithVideo(e.target.checked)}
            />
            <span>Gravar tela</span>
          </label>
        </>
      ) : (
        <div className="recording-active">
          <div className="recording-indicator" />
          <span className="timer">{formatTimer(status?.duration_secs ?? 0)}</span>
          <button className="btn-danger" onClick={handleStop}>
            Parar
          </button>
        </div>
      )}
      {error && <p className="error">{error}</p>}
    </div>
  );
}
```

- [ ] **Step 3: Update App.tsx to use RecordButton**

Replace the disabled button in the home view with `<RecordButton onRecordingFinished={() => setView({ kind: "gallery" })} />`.

- [ ] **Step 4: Verify the app runs**

```bash
npm run tauri dev
```

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "feat(frontend): add recording UI with start/stop"
```

---

## Chunk 3: Transcription Module

### Task 14: Transcription types and provider trait

**Files:**
- Create: `src-tauri/src/transcription/mod.rs`
- Create: `src-tauri/src/transcription/types.rs`
- Create: `src-tauri/src/transcription/provider.rs`

- [ ] **Step 1: Define types**

`src-tauri/src/transcription/types.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptResult {
    pub language: String,
    pub segments: Vec<Segment>,
    pub full_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperModel {
    pub name: String,
    pub size_bytes: u64,
    pub downloaded: bool,
}
```

- [ ] **Step 2: Define provider trait**

`src-tauri/src/transcription/provider.rs`:
```rust
use std::path::Path;
use crate::transcription::types::TranscriptResult;

pub trait TranscriptionProvider: Send + Sync {
    fn transcribe(&self, audio_path: &Path) -> Result<TranscriptResult, String>;
}
```

- [ ] **Step 3: Create mod.rs**

`src-tauri/src/transcription/mod.rs`:
```rust
pub mod types;
pub mod provider;
pub mod local;
pub mod api;
pub mod orchestrator;
pub mod models;
pub mod commands;
```

- [ ] **Step 4: Add dependencies to Cargo.toml**

```toml
whisper-rs = "0.12"
reqwest = { version = "0.12", features = ["json"] }
```

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/transcription/
git commit -m "feat(transcription): add types and provider trait"
```

---

### Task 15: Local provider (whisper-rs)

**Files:**
- Create: `src-tauri/src/transcription/local.rs`
- Create: `src-tauri/src/transcription/models.rs`

- [ ] **Step 1: Write model management**

`src-tauri/src/transcription/models.rs`: Handles model download, storage in `~/.local/share/hlusra/models/`, listing available/downloaded models.

- [ ] **Step 2: Write LocalProvider**

`src-tauri/src/transcription/local.rs`: Implements `TranscriptionProvider` using `whisper-rs`. Loads model, processes audio, extracts word-level timestamps.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/transcription/local.rs src-tauri/src/transcription/models.rs
git commit -m "feat(transcription): add local provider with whisper-rs"
```

---

### Task 16: API provider

**Files:**
- Create: `src-tauri/src/transcription/api.rs`

- [ ] **Step 1: Write ApiProvider**

`src-tauri/src/transcription/api.rs`: Implements `TranscriptionProvider` using `reqwest` HTTP client. Sends audio to OpenAI-compatible endpoint, parses response into `TranscriptResult`.

- [ ] **Step 2: Commit**

```bash
git add src-tauri/src/transcription/api.rs
git commit -m "feat(transcription): add API provider for OpenAI-compatible endpoints"
```

---

### Task 17: Orchestrator and commands

**Files:**
- Create: `src-tauri/src/transcription/orchestrator.rs`
- Create: `src-tauri/src/transcription/commands.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write orchestrator**

`src-tauri/src/transcription/orchestrator.rs`: Extracts mic track from MKV via FFmpeg (ffmpeg-next), sends to provider, saves results via Library.

Add to Cargo.toml:
```toml
ffmpeg-next = "7"
```

- [ ] **Step 2: Write Tauri commands**

`src-tauri/src/transcription/commands.rs`: `transcribe_meeting`, `retranscribe_meeting`, `get_transcription_status`, model management commands.

- [ ] **Step 3: Register in lib.rs**

Add `mod transcription;` and register commands.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/transcription/ src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat(transcription): add orchestrator and Tauri commands"
```

---

### Task 18: Transcription UI

**Files:**
- Create: `src/components/MeetingPage.tsx`
- Create: `src/components/TranscriptView.tsx`
- Modify: `src/lib/api.ts`
- Modify: `src/App.tsx`

- [ ] **Step 1: Add transcription API bindings**

- [ ] **Step 2: Create MeetingPage component** — shows media player, transcript, and actions

- [ ] **Step 3: Create TranscriptView component** — renders transcript with timestamps

- [ ] **Step 4: Wire MeetingPage into App.tsx navigation**

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "feat(frontend): add meeting page with transcript view"
```

---

## Chunk 4: RAG/Chat Module

### Task 19: RAG types and vector store

**Files:**
- Create: `src-tauri/src/rag/mod.rs`
- Create: `src-tauri/src/rag/types.rs`
- Create: `src-tauri/src/rag/vector_store.rs`

- [ ] **Step 1: Define types**

`src-tauri/src/rag/types.rs`: `Chunk`, `RagConfig` structs. Note: reuse `ChatStatus` from `library::types`, do NOT redefine it here.

- [ ] **Step 2: Write VectorStore**

`src-tauri/src/rag/vector_store.rs`: Separate SQLite DB at `~/.local/share/hlusra/rag.db`. Uses `rusqlite` + `sqlite-vec`. CRUD for chunks with embeddings. Scoped search per meeting.

Add to Cargo.toml:
```toml
sqlite-vec = "0.1"
```

- [ ] **Step 3: Write tests**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/rag/
git commit -m "feat(rag): add vector store with sqlite-vec"
```

---

### Task 20: Chunker and embeddings client

**Files:**
- Create: `src-tauri/src/rag/chunker.rs`
- Create: `src-tauri/src/rag/embeddings.rs`

- [ ] **Step 1: Write chunker**

`src-tauri/src/rag/chunker.rs`: Splits `TranscriptResult` into chunks based on segments, respects configurable chunk size (~500 tokens), preserves timestamps.

- [ ] **Step 2: Write embeddings client**

`src-tauri/src/rag/embeddings.rs`: HTTP client for OpenAI-compatible embeddings endpoint. Batch embedding of chunks.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/rag/chunker.rs src-tauri/src/rag/embeddings.rs
git commit -m "feat(rag): add chunker and embeddings client"
```

---

### Task 21: Chat client and prompt assembly

**Files:**
- Create: `src-tauri/src/rag/chat.rs`
- Create: `src-tauri/src/rag/prompt.rs`

- [ ] **Step 1: Write prompt assembly**

`src-tauri/src/rag/prompt.rs`: Builds system prompt + relevant chunks + user question.

- [ ] **Step 2: Write chat client**

`src-tauri/src/rag/chat.rs`: HTTP client for OpenAI-compatible chat endpoint. Streaming response.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/rag/chat.rs src-tauri/src/rag/prompt.rs
git commit -m "feat(rag): add chat client with streaming and prompt assembly"
```

---

### Task 22: RAG commands and chat UI

**Files:**
- Create: `src-tauri/src/rag/commands.rs`
- Create: `src/components/ChatPanel.tsx`
- Modify: `src/components/MeetingPage.tsx`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write Tauri commands**: `index_meeting`, `reindex_meeting`, `chat_message`, `get_chat_status`

- [ ] **Step 2: Create ChatPanel component**: Input + message list + streaming display

- [ ] **Step 3: Add ChatPanel to MeetingPage**

- [ ] **Step 4: Register commands in lib.rs**

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/rag/ src/ src-tauri/src/lib.rs
git commit -m "feat(rag): add commands and chat UI"
```

---

## Chunk 5: Settings/Export Module

### Task 23: Settings config (TOML)

**Files:**
- Create: `src-tauri/src/settings/mod.rs`
- Create: `src-tauri/src/settings/config.rs`
- Create: `src-tauri/src/settings/defaults.rs`
- Create: `src-tauri/src/settings/commands.rs`

- [ ] **Step 1: Define AppSettings struct with serde + toml**

`src-tauri/src/settings/config.rs`: Full `AppSettings` struct matching the TOML spec. Load from `~/.config/hlusra/config.toml`, save on change, create with defaults if missing.

- [ ] **Step 2: Define defaults**

`src-tauri/src/settings/defaults.rs`: Default values for all settings.

- [ ] **Step 3: Write Tauri commands**

`src-tauri/src/settings/commands.rs`: `get_settings`, `update_settings`.

- [ ] **Step 4: Register in lib.rs**

- [ ] **Step 5: Wire settings into Recorder** — update `start_recording` to read `VideoConfig` and `AudioConfig` from `AppSettings` instead of using defaults. Also update Library initialization to use `recordings_dir` from settings.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/settings/ src-tauri/src/lib.rs src-tauri/src/recorder/commands.rs
git commit -m "feat(settings): add TOML config with defaults, wire into recorder"
```

---

### Task 24: Export module

**Files:**
- Create: `src-tauri/src/export/mod.rs`
- Create: `src-tauri/src/export/audio.rs`
- Create: `src-tauri/src/export/video.rs`
- Create: `src-tauri/src/export/transcript.rs`
- Create: `src-tauri/src/export/types.rs`
- Create: `src-tauri/src/export/commands.rs`

- [ ] **Step 1: Define export types**

`src-tauri/src/export/types.rs`: `AudioFormat`, `VideoFormat`, `TranscriptFormat`, `SaveMode` enums.

- [ ] **Step 2: Write audio export**

`src-tauri/src/export/audio.rs`: FFmpeg-based export. Extract and mix multi-track to single when needed (MP3, WAV, OGG). Keep tracks for Opus.

- [ ] **Step 3: Write video export**

`src-tauri/src/export/video.rs`: FFmpeg-based transcode. MKV→MP4, H.265→H.264 conversions.

- [ ] **Step 4: Write transcript export**

`src-tauri/src/export/transcript.rs`: TXT (copy), JSON (copy), SRT (generate from segments), PDF (via `genpdf`).

Add to Cargo.toml:
```toml
genpdf = "0.3"
```

- [ ] **Step 5: Write Tauri commands**

`src-tauri/src/export/commands.rs`: `export_audio`, `export_video`, `export_transcript`.

- [ ] **Step 6: Register in lib.rs**

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/export/ src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat(export): add audio, video, and transcript export"
```

---

### Task 25: Settings and Export UI

**Files:**
- Create: `src/components/SettingsPage.tsx`
- Create: `src/components/ExportDialog.tsx`
- Modify: `src/components/MeetingPage.tsx`
- Modify: `src/App.tsx`

- [ ] **Step 1: Create SettingsPage** — sections for general, audio, video, transcription, RAG

- [ ] **Step 2: Create ExportDialog** — format selection, Save/Save As buttons

- [ ] **Step 3: Add ExportDialog to MeetingPage**

- [ ] **Step 4: Add Settings navigation to App.tsx**

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "feat(frontend): add settings page and export dialog"
```

---

## Chunk 6: Integration and Polish

### Task 26: Floating recording widget

**Files:**
- Create: `src/widget.html`
- Create: `src/widget.tsx`
- Modify: `src-tauri/tauri.conf.json`
- Modify: `src-tauri/src/recorder/commands.rs`

- [ ] **Step 1: Create widget window** — separate HTML entry point with timer + stop button

- [ ] **Step 2: Configure Tauri for multi-window** — add widget window config with `always_on_top`, no decorations, minimal size

- [ ] **Step 3: Open/close widget on recording start/stop** via Tauri window API

- [ ] **Step 4: Commit**

```bash
git add src/ src-tauri/
git commit -m "feat(recorder): add floating recording widget"
```

---

### Task 27: Thumbnail generation

**Files:**
- Create: `src-tauri/src/library/thumbnail.rs`
- Modify: `src-tauri/src/library/api.rs`
- Modify: `src-tauri/src/library/mod.rs`

- [ ] **Step 1: Write thumbnail extraction** — uses `ffmpeg-next` to extract a frame (~10s) from MKV, save as JPEG

- [ ] **Step 2: Call thumbnail generation in `finalize_meeting`** when `has_video == true`

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/library/
git commit -m "feat(library): add automatic thumbnail generation"
```

---

### Task 28: End-to-end integration test

- [ ] **Step 1: Manual test** — record audio-only, verify appears in gallery, transcribe, chat, export

- [ ] **Step 2: Manual test** — record with video, verify MKV output, thumbnail, export to MP4

- [ ] **Step 3: Fix any integration issues found**

- [ ] **Step 4: Final commit**

```bash
git add -A
git commit -m "fix: integration fixes from end-to-end testing"
```
