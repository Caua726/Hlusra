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
    CREATE INDEX IF NOT EXISTS idx_meetings_created_at ON meetings(created_at DESC);
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
        // WAL and synchronous are not meaningful for in-memory databases.
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
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
                let tx = self.conn.unchecked_transaction()?;
                tx.execute_batch(sql)?;
                tx.commit()?;
            }
        }
        Ok(())
    }

    pub fn insert_meeting(&self, meeting: &Meeting, tracks: &[TrackInfo]) -> rusqlite::Result<()> {
        let tracks_json = serde_json::to_string(tracks).unwrap_or_else(|_| "[]".to_string());
        self.conn.execute(
            "INSERT INTO meetings (id, title, created_at, duration_secs, has_video, file_size, dir_path, tracks_json, media_status, transcription_status, chat_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                meeting.id,
                meeting.title,
                meeting.created_at.to_rfc3339(),
                meeting.duration_secs,
                meeting.has_video as i32,
                meeting.file_size as i64,
                meeting.dir_path.to_string_lossy().to_string(),
                tracks_json,
                meeting.media_status.as_str(),
                meeting.transcription_status.as_str(),
                meeting.chat_status.as_str(),
            ],
        )?;
        Ok(())
    }

    pub fn get_meeting(&self, id: &str) -> rusqlite::Result<Meeting> {
        self.conn.query_row(
            "SELECT id, title, created_at, duration_secs, has_video, file_size, dir_path, tracks_json, media_status, transcription_status, chat_status
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
                media_status: row.get::<_, String>(6)?.parse::<MediaStatus>().unwrap_or_default(),
                transcription_status: row.get::<_, String>(7)?.parse::<TranscriptionStatus>().unwrap_or_default(),
                chat_status: row.get::<_, String>(8)?.parse::<ChatStatus>().unwrap_or_default(),
            })
        })?;

        meetings.collect()
    }

    pub fn update_title(&self, id: &str, title: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET title = ?1 WHERE id = ?2",
            params![title, id],
        )?;
        if self.conn.changes() == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn update_transcription_status(&self, id: &str, status: TranscriptionStatus) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET transcription_status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        if self.conn.changes() == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn update_chat_status(&self, id: &str, status: ChatStatus) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET chat_status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        if self.conn.changes() == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn update_media_status(&self, id: &str, status: MediaStatus) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE meetings SET media_status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        if self.conn.changes() == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn delete_meeting(&self, id: &str) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM meetings WHERE id = ?1", params![id])?;
        if self.conn.changes() == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn row_to_meeting(row: &rusqlite::Row) -> rusqlite::Result<Meeting> {
        let tracks_json_str: String = row.get(7)?;
        let tracks: Vec<TrackInfo> = serde_json::from_str(&tracks_json_str).unwrap_or_default();
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
            tracks,
            media_status: row.get::<_, String>(8)?.parse::<MediaStatus>().unwrap_or_default(),
            transcription_status: row.get::<_, String>(9)?.parse::<TranscriptionStatus>().unwrap_or_default(),
            chat_status: row.get::<_, String>(10)?.parse::<ChatStatus>().unwrap_or_default(),
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
            tracks: vec![],
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        }
    }

    #[test]
    fn test_insert_and_get() {
        let db = LibraryDb::open_in_memory().unwrap();
        let meeting = make_test_meeting("test-1");
        let tracks = vec![
            TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
        ];
        db.insert_meeting(&meeting, &tracks).unwrap();
        let retrieved = db.get_meeting("test-1").unwrap();
        assert_eq!(retrieved.id, "test-1");
        assert_eq!(retrieved.title, "Meeting test-1");
        assert!(retrieved.has_video);
        assert_eq!(retrieved.tracks.len(), 1);
        assert_eq!(retrieved.tracks[0].label, "mic");
    }

    #[test]
    fn test_list_meetings_ordered() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("a"), &[]).unwrap();
        db.insert_meeting(&make_test_meeting("b"), &[]).unwrap();
        let list = db.list_meetings().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_update_title() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("t1"), &[]).unwrap();
        db.update_title("t1", "New Title").unwrap();
        let m = db.get_meeting("t1").unwrap();
        assert_eq!(m.title, "New Title");
    }

    #[test]
    fn test_update_statuses() {
        let db = LibraryDb::open_in_memory().unwrap();
        db.insert_meeting(&make_test_meeting("s1"), &[]).unwrap();

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
        db.insert_meeting(&make_test_meeting("d1"), &[]).unwrap();
        db.delete_meeting("d1").unwrap();
        assert!(db.get_meeting("d1").is_err());
    }
}
