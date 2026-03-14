use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

use crate::rag::types::Chunk;

#[derive(Debug, thiserror::Error)]
pub enum VectorStoreError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Model changed: stored={stored}, configured={configured}. Reindex required.")]
    ModelChanged { stored: String, configured: String },
    #[error("No embedding dimension stored — database needs initialisation")]
    NoDimension,
    #[error("sqlite-vec extension not available: {0}")]
    VecNotAvailable(String),
}

impl serde::Serialize for VectorStoreError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Result of checking whether the stored embedding model matches the configured one.
#[derive(Debug)]
pub enum ModelStatus {
    /// No model stored yet (fresh database).
    Fresh,
    /// Stored model matches the configured model.
    Match,
    /// Stored model differs — reindex is needed.
    Changed { stored: String },
}

/// Manages the separate SQLite database for RAG chunks and vector search.
///
/// Schema:
/// - `chunks`: stores chunk text, metadata, and raw embedding blob
/// - `chunks_vec`: sqlite-vec virtual table for approximate nearest-neighbour search
/// - `rag_meta`: key-value table storing `embedding_model` and `embedding_dimension`
pub struct VectorStore {
    conn: Connection,
    vec_available: bool,
}

impl VectorStore {
    /// Open (or create) the RAG database at the default path
    /// `~/.local/share/hlusra/rag.db`.
    pub fn open_default() -> Result<Self, VectorStoreError> {
        let path = Self::default_db_path();
        Self::open(&path)
    }

    /// Open (or create) the RAG database at a specific path.
    pub fn open(db_path: &Path) -> Result<Self, VectorStoreError> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)?;

        // Try to load the sqlite-vec extension for vector search support.
        let vec_available = Self::load_sqlite_vec(&conn);

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        // Create the chunks table and rag_meta table (these are safe to
        // CREATE IF NOT EXISTS regardless of dimension).
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY,
                meeting_id TEXT NOT NULL,
                text TEXT NOT NULL,
                start_time REAL NOT NULL,
                end_time REAL NOT NULL,
                chunk_index INTEGER NOT NULL,
                embedding BLOB NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_chunks_meeting
                ON chunks(meeting_id);

            CREATE TABLE IF NOT EXISTS rag_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )?;

        Ok(VectorStore { conn, vec_available })
    }

    /// Try to load the sqlite-vec extension.  If it fails (e.g. the shared
    /// library is not installed on this system), log a warning and continue
    /// without vector search — the rest of VectorStore (chunk storage, meta)
    /// still works.
    ///
    /// TODO: Bundle the sqlite-vec shared library with the application so it
    /// is always available regardless of the host system.
    fn load_sqlite_vec(conn: &Connection) -> bool {
        unsafe {
            if let Err(e) = conn.load_extension_enable() {
                tracing::warn!("could not enable SQLite extension loading: {e}");
                return false;
            }
            let ok = match conn.load_extension("vec0", None::<&str>) {
                Ok(()) => true,
                Err(e) => {
                    tracing::warn!(
                        "could not load sqlite-vec (vec0) extension: {e}. \
                         Vector search will not be available."
                    );
                    false
                }
            };
            let _ = conn.load_extension_disable();
            ok
        }
    }

    /// Returns `~/.local/share/hlusra/rag.db`.
    pub fn default_db_path() -> PathBuf {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("hlusra");
        data_dir.join("rag.db")
    }

    // -----------------------------------------------------------------------
    // Meta helpers
    // -----------------------------------------------------------------------

    /// Get a value from `rag_meta` by key.
    fn get_meta(&self, key: &str) -> Result<Option<String>, VectorStoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM rag_meta WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }

    /// Set a value in `rag_meta`.
    fn set_meta(&self, key: &str, value: &str) -> Result<(), VectorStoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO rag_meta (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    /// Check whether the stored embedding model matches the configured model.
    pub fn check_model(&self, configured_model: &str) -> Result<ModelStatus, VectorStoreError> {
        match self.get_meta("embedding_model")? {
            None => Ok(ModelStatus::Fresh),
            Some(stored) if stored == configured_model => Ok(ModelStatus::Match),
            Some(stored) => Ok(ModelStatus::Changed { stored }),
        }
    }

    /// Get the stored embedding dimension, if any.
    pub fn get_dimension(&self) -> Result<Option<usize>, VectorStoreError> {
        match self.get_meta("embedding_dimension")? {
            Some(d) => {
                let dim = d.parse::<usize>().map_err(|e| {
                    VectorStoreError::Db(rusqlite::Error::InvalidParameterName(
                        format!("invalid embedding_dimension '{}': {}", d, e),
                    ))
                })?;
                Ok(Some(dim))
            }
            None => Ok(None),
        }
    }

    // -----------------------------------------------------------------------
    // Initialisation / reindex support
    // -----------------------------------------------------------------------

    /// Returns whether the sqlite-vec extension is loaded and vector search
    /// is available.
    pub fn vec_available(&self) -> bool {
        self.vec_available
    }

    /// Initialise (or reinitialise) the vector table for a given model and
    /// embedding dimension.  Drops the old `chunks_vec` table if it exists,
    /// clears all chunks, and creates a new virtual table with the right
    /// dimension.
    ///
    /// Requires sqlite-vec to be loaded.
    pub fn init_vector_table(
        &self,
        model: &str,
        dimension: usize,
    ) -> Result<(), VectorStoreError> {
        if !self.vec_available {
            return Err(VectorStoreError::VecNotAvailable(
                "cannot create vector table without sqlite-vec (vec0) extension".to_string(),
            ));
        }

        let tx = self.conn.unchecked_transaction()?;

        // Drop old virtual table if present.
        tx.execute_batch("DROP TABLE IF EXISTS chunks_vec;")?;

        // Delete all stored chunks (embeddings are now invalid).
        tx.execute_batch("DELETE FROM chunks;")?;

        // Create the sqlite-vec virtual table with the new dimension.
        let create_sql = format!(
            "CREATE VIRTUAL TABLE chunks_vec USING vec0(
                chunk_id TEXT PRIMARY KEY,
                embedding float[{dimension}]
            );"
        );
        tx.execute_batch(&create_sql)?;

        // Store the model and dimension in meta.
        tx.execute(
            "INSERT OR REPLACE INTO rag_meta (key, value) VALUES (?1, ?2)",
            params!["embedding_model", model],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO rag_meta (key, value) VALUES (?1, ?2)",
            params!["embedding_dimension", &dimension.to_string()],
        )?;

        tx.commit()?;
        Ok(())
    }

    /// Ensure the vector table exists and matches the given model/dimension.
    /// If the database is fresh, creates it.  If the model changed, returns
    /// `ModelChanged` error — the caller should prompt the user and call
    /// `init_vector_table` + reindex.
    pub fn ensure_ready(
        &self,
        model: &str,
        dimension: usize,
    ) -> Result<(), VectorStoreError> {
        match self.check_model(model)? {
            ModelStatus::Fresh => {
                self.init_vector_table(model, dimension)?;
                Ok(())
            }
            ModelStatus::Match => Ok(()),
            ModelStatus::Changed { stored } => Err(VectorStoreError::ModelChanged {
                stored,
                configured: model.to_string(),
            }),
        }
    }

    // -----------------------------------------------------------------------
    // CRUD
    // -----------------------------------------------------------------------

    /// Insert a chunk and its embedding into both the `chunks` table and the
    /// `chunks_vec` virtual table.
    ///
    /// If sqlite-vec is not available, the chunk is stored in `chunks` but
    /// skipped in `chunks_vec`.
    pub fn insert_chunk(
        &self,
        chunk: &Chunk,
        embedding: &[f32],
    ) -> Result<(), VectorStoreError> {
        let embedding_blob = embedding_to_blob(embedding);

        let tx = self.conn.unchecked_transaction()?;

        tx.execute(
            "INSERT OR REPLACE INTO chunks (id, meeting_id, text, start_time, end_time, chunk_index, embedding)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                chunk.id,
                chunk.meeting_id,
                chunk.text,
                chunk.start_time,
                chunk.end_time,
                chunk.chunk_index as i64,
                embedding_blob,
            ],
        )?;

        if self.vec_available {
            tx.execute(
                "INSERT OR REPLACE INTO chunks_vec (chunk_id, embedding)
                 VALUES (?1, ?2)",
                params![chunk.id, embedding_blob],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Insert multiple chunks with their embeddings in a single transaction.
    pub fn insert_chunks(
        &self,
        chunks: &[Chunk],
        embeddings: &[Vec<f32>],
    ) -> Result<(), VectorStoreError> {
        if chunks.len() != embeddings.len() {
            return Err(VectorStoreError::Db(rusqlite::Error::InvalidParameterCount(
                chunks.len(),
                embeddings.len(),
            )));
        }

        let tx = self.conn.unchecked_transaction()?;
        for (chunk, emb) in chunks.iter().zip(embeddings.iter()) {
            let blob = embedding_to_blob(emb);

            tx.execute(
                "INSERT OR REPLACE INTO chunks (id, meeting_id, text, start_time, end_time, chunk_index, embedding)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    chunk.id,
                    chunk.meeting_id,
                    chunk.text,
                    chunk.start_time,
                    chunk.end_time,
                    chunk.chunk_index as i64,
                    blob,
                ],
            )?;

            if self.vec_available {
                tx.execute(
                    "INSERT OR REPLACE INTO chunks_vec (chunk_id, embedding)
                     VALUES (?1, ?2)",
                    params![chunk.id, blob],
                )?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Delete all chunks for a given meeting.
    pub fn delete_meeting_chunks(&self, meeting_id: &str) -> Result<(), VectorStoreError> {
        let tx = self.conn.unchecked_transaction()?;

        if self.vec_available {
            // Get chunk IDs first so we can remove from the virtual table.
            let chunk_ids: Vec<String> = {
                let mut stmt = tx
                    .prepare("SELECT id FROM chunks WHERE meeting_id = ?1")?;
                let rows = stmt.query_map(params![meeting_id], |row| row.get(0))?;
                rows.collect::<Result<Vec<String>, _>>()?
            };

            for cid in &chunk_ids {
                tx.execute(
                    "DELETE FROM chunks_vec WHERE chunk_id = ?1",
                    params![cid],
                )?;
            }
        }

        tx.execute(
            "DELETE FROM chunks WHERE meeting_id = ?1",
            params![meeting_id],
        )?;
        tx.commit()?;
        Ok(())
    }

    /// Search for the top-k most similar chunks to a query embedding,
    /// scoped to a specific meeting.
    ///
    /// Returns `Vec<(Chunk, f32)>` where the `f32` is the distance score
    /// from sqlite-vec (lower = more similar).
    ///
    /// Returns an empty result set if sqlite-vec is not available.
    pub fn search(
        &self,
        meeting_id: &str,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<Vec<(Chunk, f32)>, VectorStoreError> {
        if !self.vec_available {
            return Ok(Vec::new());
        }

        let query_blob = embedding_to_blob(query_embedding);

        // sqlite-vec returns rows ordered by distance.  We use the total
        // chunk count as k so that the virtual table returns all rows,
        // then filter by meeting_id and LIMIT to top_k.
        let total_chunks: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM chunks",
            [],
            |row| row.get(0),
        )?;

        let sql = format!(
            "SELECT c.id, c.meeting_id, c.text, c.start_time, c.end_time, c.chunk_index, v.distance
             FROM chunks_vec v
             INNER JOIN chunks c ON c.id = v.chunk_id
             WHERE v.embedding MATCH ?1
               AND k = ?2
               AND c.meeting_id = ?3
             ORDER BY v.distance
             LIMIT ?4"
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(
            params![
                query_blob,
                total_chunks,
                meeting_id,
                top_k as i64
            ],
            |row| {
                let chunk = Chunk {
                    id: row.get(0)?,
                    meeting_id: row.get(1)?,
                    text: row.get(2)?,
                    start_time: row.get(3)?,
                    end_time: row.get(4)?,
                    chunk_index: row.get::<_, i64>(5)? as usize,
                };
                let distance: f32 = row.get(6)?;
                Ok((chunk, distance))
            },
        )?;

        let results: Vec<(Chunk, f32)> = rows.collect::<Result<Vec<_>, _>>()?;
        Ok(results)
    }

    /// Check whether a meeting has been indexed (has any chunks).
    pub fn is_meeting_indexed(&self, meeting_id: &str) -> Result<bool, VectorStoreError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM chunks WHERE meeting_id = ?1",
            params![meeting_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a `&[f32]` to a byte slice suitable for sqlite-vec.
///
/// This is zero-copy — it reinterprets the f32 slice as bytes via `bytemuck`.
/// On little-endian platforms (x86, ARM) the layout matches what sqlite-vec
/// expects.  (All supported Tauri targets are little-endian.)
fn embedding_to_blob(embedding: &[f32]) -> &[u8] {
    bytemuck::cast_slice(embedding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_embedding_to_blob_roundtrip() {
        let original = vec![1.0f32, 2.5, -3.14, 0.0];
        let blob = embedding_to_blob(&original);
        assert_eq!(blob.len(), 16); // 4 floats * 4 bytes

        // Read back via bytemuck (zero-copy round-trip).
        let recovered: &[f32] = bytemuck::cast_slice(blob);
        assert_eq!(original.as_slice(), recovered);
    }

    // --- S40: VectorStore tests using in-memory SQLite ---

    /// Helper: open a VectorStore in a temp directory.
    fn open_temp_store() -> (VectorStore, TempDir) {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("test_rag.db");
        let store = VectorStore::open(&db_path).unwrap();
        (store, tmp)
    }

    /// Helper: create a test Chunk.
    fn make_chunk(id: &str, meeting_id: &str, index: usize) -> Chunk {
        Chunk {
            id: id.to_string(),
            meeting_id: meeting_id.to_string(),
            text: format!("chunk text {}", id),
            start_time: index as f64 * 5.0,
            end_time: (index as f64 + 1.0) * 5.0,
            chunk_index: index,
        }
    }

    #[test]
    fn test_open_and_schema_creation() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("test_rag.db");
        let store = VectorStore::open(&db_path).unwrap();

        // sqlite-vec will typically not be available in test environment,
        // but the store should open successfully either way.
        let _ = store.vec_available();

        // Verify the chunks table exists by running a query against it.
        let count: i64 = store
            .conn
            .query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        // Verify the rag_meta table exists.
        let meta_count: i64 = store
            .conn
            .query_row("SELECT COUNT(*) FROM rag_meta", [], |row| row.get(0))
            .unwrap();
        assert_eq!(meta_count, 0);
    }

    #[test]
    fn test_open_creates_parent_dirs() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("nested").join("dirs").join("test_rag.db");
        let _store = VectorStore::open(&db_path).unwrap();
        assert!(db_path.exists());
    }

    #[test]
    fn test_insert_and_query_chunks() {
        let (store, _tmp) = open_temp_store();
        let chunk = make_chunk("c1", "meeting-A", 0);
        let embedding = vec![1.0f32, 2.0, 3.0];

        store.insert_chunk(&chunk, &embedding).unwrap();

        // Verify the chunk was inserted into the chunks table.
        let (text, meeting_id): (String, String) = store
            .conn
            .query_row(
                "SELECT text, meeting_id FROM chunks WHERE id = ?1",
                params!["c1"],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(text, "chunk text c1");
        assert_eq!(meeting_id, "meeting-A");

        // Verify the embedding blob was stored correctly.
        let stored_blob: Vec<u8> = store
            .conn
            .query_row(
                "SELECT embedding FROM chunks WHERE id = ?1",
                params!["c1"],
                |row| row.get(0),
            )
            .unwrap();
        let expected_blob = embedding_to_blob(&embedding);
        assert_eq!(stored_blob, expected_blob);
    }

    #[test]
    fn test_insert_chunks_batch() {
        let (store, _tmp) = open_temp_store();
        let chunks = vec![
            make_chunk("c1", "meeting-A", 0),
            make_chunk("c2", "meeting-A", 1),
            make_chunk("c3", "meeting-A", 2),
        ];
        let embeddings = vec![
            vec![1.0f32, 0.0, 0.0],
            vec![0.0f32, 1.0, 0.0],
            vec![0.0f32, 0.0, 1.0],
        ];

        store.insert_chunks(&chunks, &embeddings).unwrap();

        let count: i64 = store
            .conn
            .query_row(
                "SELECT COUNT(*) FROM chunks WHERE meeting_id = ?1",
                params!["meeting-A"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_insert_chunks_mismatched_lengths() {
        let (store, _tmp) = open_temp_store();
        let chunks = vec![make_chunk("c1", "m1", 0)];
        let embeddings: Vec<Vec<f32>> = vec![vec![1.0], vec![2.0]]; // 2 != 1

        let result = store.insert_chunks(&chunks, &embeddings);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_meeting_chunks() {
        let (store, _tmp) = open_temp_store();

        // Insert chunks for two different meetings.
        let chunk_a1 = make_chunk("a1", "meeting-A", 0);
        let chunk_a2 = make_chunk("a2", "meeting-A", 1);
        let chunk_b1 = make_chunk("b1", "meeting-B", 0);
        let emb = vec![1.0f32, 2.0, 3.0];

        store.insert_chunk(&chunk_a1, &emb).unwrap();
        store.insert_chunk(&chunk_a2, &emb).unwrap();
        store.insert_chunk(&chunk_b1, &emb).unwrap();

        // Delete meeting-A's chunks.
        store.delete_meeting_chunks("meeting-A").unwrap();

        // meeting-A chunks should be gone.
        let count_a: i64 = store
            .conn
            .query_row(
                "SELECT COUNT(*) FROM chunks WHERE meeting_id = ?1",
                params!["meeting-A"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_a, 0);

        // meeting-B chunks should still be present.
        let count_b: i64 = store
            .conn
            .query_row(
                "SELECT COUNT(*) FROM chunks WHERE meeting_id = ?1",
                params!["meeting-B"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_b, 1);
    }

    #[test]
    fn test_is_meeting_indexed() {
        let (store, _tmp) = open_temp_store();

        assert!(!store.is_meeting_indexed("meeting-X").unwrap());

        let chunk = make_chunk("c1", "meeting-X", 0);
        store.insert_chunk(&chunk, &[1.0f32, 2.0]).unwrap();

        assert!(store.is_meeting_indexed("meeting-X").unwrap());
    }

    #[test]
    fn test_meta_get_set_roundtrip() {
        let (store, _tmp) = open_temp_store();

        // Initially empty.
        assert_eq!(store.get_meta("some_key").unwrap(), None);

        // Set a value.
        store.set_meta("some_key", "some_value").unwrap();
        assert_eq!(
            store.get_meta("some_key").unwrap(),
            Some("some_value".to_string())
        );

        // Overwrite.
        store.set_meta("some_key", "updated").unwrap();
        assert_eq!(
            store.get_meta("some_key").unwrap(),
            Some("updated".to_string())
        );
    }

    #[test]
    fn test_check_model_fresh() {
        let (store, _tmp) = open_temp_store();
        match store.check_model("text-embedding-3-small").unwrap() {
            ModelStatus::Fresh => {} // expected
            other => panic!("expected Fresh, got {:?}", other),
        }
    }

    #[test]
    fn test_check_model_match() {
        let (store, _tmp) = open_temp_store();
        store
            .set_meta("embedding_model", "text-embedding-3-small")
            .unwrap();
        match store.check_model("text-embedding-3-small").unwrap() {
            ModelStatus::Match => {} // expected
            other => panic!("expected Match, got {:?}", other),
        }
    }

    #[test]
    fn test_check_model_changed() {
        let (store, _tmp) = open_temp_store();
        store.set_meta("embedding_model", "old-model").unwrap();
        match store.check_model("new-model").unwrap() {
            ModelStatus::Changed { stored } => {
                assert_eq!(stored, "old-model");
            }
            other => panic!("expected Changed, got {:?}", other),
        }
    }

    #[test]
    fn test_get_dimension() {
        let (store, _tmp) = open_temp_store();

        // No dimension stored yet.
        assert_eq!(store.get_dimension().unwrap(), None);

        store.set_meta("embedding_dimension", "384").unwrap();
        assert_eq!(store.get_dimension().unwrap(), Some(384));
    }

    #[test]
    fn test_search_returns_empty_without_vec() {
        let (store, _tmp) = open_temp_store();

        // When sqlite-vec is not loaded, search should return empty vec.
        if !store.vec_available() {
            let results = store.search("meeting-A", &[1.0, 2.0, 3.0], 5).unwrap();
            assert!(results.is_empty());
        }
    }

    #[test]
    fn test_embedding_to_blob_empty() {
        let blob = embedding_to_blob(&[]);
        assert!(blob.is_empty());
    }

    #[test]
    fn test_embedding_to_blob_single_value() {
        let original = [42.0f32];
        let blob = embedding_to_blob(&original);
        assert_eq!(blob.len(), 4);
        let recovered = f32::from_le_bytes(blob.try_into().unwrap());
        assert_eq!(recovered, 42.0);
    }
}
