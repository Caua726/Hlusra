use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, State};

use crate::library::api::Library;
use crate::library::types::{ArtifactKind, ChatStatus};
use crate::rag::chunker::chunk_transcript;
use crate::rag::chat::ChatClient;
use crate::rag::embeddings::EmbeddingsClient;
use crate::rag::prompt::build_messages;
use crate::rag::types::RagConfig;
use crate::rag::vector_store::VectorStore;
use crate::transcription::types::TranscriptResult;

/// Shared state wrapper for the VectorStore (needs Mutex since rusqlite
/// Connection is not Sync).
pub struct RagState {
    pub store: Mutex<VectorStore>,
    pub config: Mutex<RagConfig>,
}

#[derive(Debug, thiserror::Error)]
pub enum RagCommandError {
    #[error("Library error: {0}")]
    Library(String),
    #[error("Vector store error: {0}")]
    VectorStore(#[from] crate::rag::vector_store::VectorStoreError),
    #[error("Embeddings error: {0}")]
    Embeddings(#[from] crate::rag::embeddings::EmbeddingsError),
    #[error("Chat error: {0}")]
    Chat(#[from] crate::rag::chat::ChatError),
    #[error("Transcript not found for meeting {0}")]
    TranscriptNotFound(String),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}

impl serde::Serialize for RagCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Index a meeting's transcript: chunk it, embed chunks, store in vector DB.
///
/// Updates the meeting's `chat_status` in the Library as it progresses.
#[tauri::command]
pub async fn index_meeting(
    id: String,
    library: State<'_, Library>,
    rag: State<'_, RagState>,
) -> Result<(), RagCommandError> {
    // Mark as indexing.
    library
        .update_chat_status(&id, ChatStatus::Indexing)
        .map_err(|e| RagCommandError::Library(e.to_string()))?;

    match do_index_meeting(&id, &library, &rag).await {
        Ok(()) => {
            library
                .update_chat_status(&id, ChatStatus::Ready)
                .map_err(|e| RagCommandError::Library(e.to_string()))?;
            Ok(())
        }
        Err(e) => {
            library
                .update_chat_status(&id, ChatStatus::Failed)
                .map_err(|e2| RagCommandError::Library(e2.to_string()))?;
            Err(e)
        }
    }
}

/// Reindex a meeting: delete existing chunks and re-run the indexing pipeline.
#[tauri::command]
pub async fn reindex_meeting(
    id: String,
    library: State<'_, Library>,
    rag: State<'_, RagState>,
) -> Result<(), RagCommandError> {
    // Delete existing chunks for this meeting.
    {
        let store = rag.store.lock().map_err(|e| RagCommandError::Other(e.to_string()))?;
        store.delete_meeting_chunks(&id)?;
    }

    // Re-run indexing.
    index_meeting(id, library, rag).await
}

/// Send a chat message about a meeting and stream the response.
///
/// Emits `chat-stream-chunk` events to the frontend with each text token,
/// and a final `chat-stream-done` event when complete.
#[tauri::command]
pub async fn chat_message(
    app: AppHandle,
    meeting_id: String,
    message: String,
    rag: State<'_, RagState>,
) -> Result<(), RagCommandError> {
    let (config, relevant_chunks) = {
        let config = rag
            .config
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?
            .clone();

        // Embed the user's question.
        let emb_client = EmbeddingsClient::new(&config);
        let query_embedding = emb_client.embed_one(&message).await?;

        // Search for relevant chunks in the vector store.
        let store = rag
            .store
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;
        let chunks = store.search(&meeting_id, &query_embedding, config.top_k)?;

        (config, chunks)
    };

    // Build prompt messages.
    let messages = build_messages(&relevant_chunks, &message);

    // Create chat client and stream the response.
    let chat_client = ChatClient::new(&config);
    let mut rx = chat_client.chat_stream(messages).await?;

    // Forward stream chunks to the frontend via Tauri events.
    while let Some(chunk_result) = rx.recv().await {
        match chunk_result {
            Ok(text) => {
                let _ = app.emit("chat-stream-chunk", &text);
            }
            Err(e) => {
                let _ = app.emit("chat-stream-error", e.to_string());
                return Err(RagCommandError::Chat(e));
            }
        }
    }

    let _ = app.emit("chat-stream-done", ());
    Ok(())
}

/// Get the current chat/indexing status for a meeting.
#[tauri::command]
pub fn get_chat_status(
    id: String,
    library: State<'_, Library>,
    rag: State<'_, RagState>,
) -> Result<ChatStatus, RagCommandError> {
    // First check the Library's persisted status.
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| RagCommandError::Library(e.to_string()))?;

    // Cross-check with vector store — if Library says Ready but no chunks
    // exist, correct to NotIndexed.
    if meeting.chat_status == ChatStatus::Ready {
        let store = rag
            .store
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;
        if !store.is_meeting_indexed(&id)? {
            // Status is stale; correct it.
            let _ = library.update_chat_status(&id, ChatStatus::NotIndexed);
            return Ok(ChatStatus::NotIndexed);
        }
    }

    Ok(meeting.chat_status)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Core indexing logic: read transcript, chunk, embed, store.
async fn do_index_meeting(
    id: &str,
    library: &Library,
    rag: &RagState,
) -> Result<(), RagCommandError> {
    // Read the transcript JSON from the library's filesystem.
    let meeting = library
        .get_meeting(id)
        .map_err(|e| RagCommandError::Library(e.to_string()))?;

    if !library.has_artifact(&meeting.dir_path, &ArtifactKind::TranscriptJson) {
        return Err(RagCommandError::TranscriptNotFound(id.to_string()));
    }

    let transcript_bytes = library
        .read_artifact(&meeting.dir_path, &ArtifactKind::TranscriptJson)
        .map_err(|e| RagCommandError::Library(e.to_string()))?;

    let transcript: TranscriptResult = serde_json::from_slice(&transcript_bytes)?;

    let config = rag
        .config
        .lock()
        .map_err(|e| RagCommandError::Other(e.to_string()))?
        .clone();

    // Chunk the transcript.
    let chunks = chunk_transcript(id, &transcript, config.chunk_size);
    if chunks.is_empty() {
        // Nothing to index — still mark as ready (empty transcript).
        return Ok(());
    }

    // Embed all chunks.
    let emb_client = EmbeddingsClient::new(&config);
    let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
    let embeddings = emb_client.embed_batch(&texts).await?;

    // Determine dimension from the first embedding and ensure the vector
    // table is set up correctly.
    let dimension = embeddings
        .first()
        .map(|e| e.len())
        .unwrap_or(0);

    {
        let store = rag
            .store
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;

        store.ensure_ready(&config.embeddings_model, dimension)?;

        // Delete any pre-existing chunks for this meeting (idempotent).
        store.delete_meeting_chunks(id)?;

        // Insert new chunks + embeddings.
        store.insert_chunks(&chunks, &embeddings)?;
    }

    Ok(())
}
