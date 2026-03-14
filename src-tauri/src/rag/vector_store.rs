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
                eprintln!("WARNING: could not enable SQLite extension loading: {e}");
                return false;
            }
            let ok = match conn.load_extension("vec0", None::<&str>) {
                Ok(()) => true,
                Err(e) => {
                    eprintln!(
                        "WARNING: could not load sqlite-vec (vec0) extension: {e}. \
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

    /// Initialise (or reinitialise) the vector table for a given model and
    /// embedding dimension.  Drops the old `chunks_vec` table if it exists,
    /// clears all chunks, and creates a new virtual table with the right
    /// dimension.
    pub fn init_vector_table(
        &self,
        model: &str,
        dimension: usize,
    ) -> Result<(), VectorStoreError> {
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

        tx.execute(
            "INSERT OR REPLACE INTO chunks_vec (chunk_id, embedding)
             VALUES (?1, ?2)",
            params![chunk.id, embedding_blob],
        )?;

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

            tx.execute(
                "INSERT OR REPLACE INTO chunks_vec (chunk_id, embedding)
                 VALUES (?1, ?2)",
                params![chunk.id, blob],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Delete all chunks for a given meeting.
    pub fn delete_meeting_chunks(&self, meeting_id: &str) -> Result<(), VectorStoreError> {
        // Get chunk IDs first so we can remove from the virtual table.
        let chunk_ids: Vec<String> = {
            let mut stmt = self
                .conn
                .prepare("SELECT id FROM chunks WHERE meeting_id = ?1")?;
            let rows = stmt.query_map(params![meeting_id], |row| row.get(0))?;
            rows.collect::<Result<Vec<String>, _>>()?
        };

        let tx = self.conn.unchecked_transaction()?;
        for cid in &chunk_ids {
            tx.execute(
                "DELETE FROM chunks_vec WHERE chunk_id = ?1",
                params![cid],
            )?;
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
    pub fn search(
        &self,
        meeting_id: &str,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<Vec<Chunk>, VectorStoreError> {
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
            "SELECT c.id, c.meeting_id, c.text, c.start_time, c.end_time, c.chunk_index
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
                Ok(Chunk {
                    id: row.get(0)?,
                    meeting_id: row.get(1)?,
                    text: row.get(2)?,
                    start_time: row.get(3)?,
                    end_time: row.get(4)?,
                    chunk_index: row.get::<_, i64>(5)? as usize,
                })
            },
        )?;

        let chunks: Vec<Chunk> = rows.collect::<Result<Vec<_>, _>>()?;
        Ok(chunks)
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

/// Convert a `&[f32]` to a little-endian byte blob suitable for sqlite-vec.
fn embedding_to_blob(embedding: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(embedding.len() * 4);
    for &val in embedding {
        bytes.extend_from_slice(&val.to_le_bytes());
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_to_blob_roundtrip() {
        let original = vec![1.0f32, 2.5, -3.14, 0.0];
        let blob = embedding_to_blob(&original);
        assert_eq!(blob.len(), 16); // 4 floats * 4 bytes

        // Read back
        let mut recovered = Vec::new();
        for chunk in blob.chunks(4) {
            let val = f32::from_le_bytes(chunk.try_into().unwrap());
            recovered.push(val);
        }
        assert_eq!(original, recovered);
    }
}
