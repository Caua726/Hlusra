use std::sync::Mutex;

use tauri::{AppHandle, Emitter, State};

use crate::library::api::Library;
use crate::library::types::{ArtifactKind, ChatStatus};
use crate::rag::chunker::chunk_transcript;
use crate::rag::chat::ChatClient;
use crate::rag::embeddings::EmbeddingsClient;
use crate::rag::prompt::build_messages;
use crate::rag::types::RagConfig;
use crate::rag::vector_store::VectorStore;
use crate::settings::config::load_settings;
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

/// Reload the RAG config from the user's saved settings into the managed state.
/// This ensures the RAG commands always use the latest configured API keys/URLs
/// instead of the empty defaults set at startup.
fn refresh_rag_config(rag: &RagState) -> Result<(), RagCommandError> {
    let settings = load_settings()
        .map_err(|e| RagCommandError::Other(format!("Failed to load settings: {e}")))?;
    let new_config = RagConfig::from_settings(&settings.rag);
    let mut config = rag
        .config
        .lock()
        .map_err(|e| RagCommandError::Other(e.to_string()))?;
    *config = new_config;
    Ok(())
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
    // Reload RAG config from user settings before each operation.
    refresh_rag_config(&rag)?;

    // Validate that RAG is configured before starting.
    {
        let config = rag
            .config
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;
        if config.embeddings_url.is_empty() || config.chat_url.is_empty() {
            return Err(RagCommandError::Other(
                "RAG not configured \u{2014} set API keys in Settings".to_string(),
            ));
        }
    }

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
    // Reload RAG config and validate BEFORE deleting existing chunks.
    refresh_rag_config(&rag)?;
    {
        let config = rag
            .config
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;
        if config.embeddings_url.is_empty() || config.chat_url.is_empty() {
            return Err(RagCommandError::Other(
                "RAG not configured \u{2014} set API keys in Settings".to_string(),
            ));
        }
    }

    // Delete existing chunks for this meeting.
    {
        let store = rag
            .store
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;
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
    // Reload RAG config from user settings before each operation.
    refresh_rag_config(&rag)?;

    let (config, relevant_chunks) = {
        let config = rag
            .config
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?
            .clone();

        // Validate that RAG is configured.
        if config.embeddings_url.is_empty() || config.chat_url.is_empty() {
            return Err(RagCommandError::Other(
                "RAG not configured \u{2014} set API keys in Settings".to_string(),
            ));
        }

        // Embed the user's question.
        let emb_client = EmbeddingsClient::new(&config);
        let query_embedding = emb_client.embed_one(&message).await?;

        // Search for relevant chunks in the vector store.
        let store = rag
            .store
            .lock()
            .map_err(|e| RagCommandError::Other(e.to_string()))?;
        let scored_chunks = store.search(&meeting_id, &query_embedding, config.top_k)?;

        // Extract just the chunks for prompt building (distances are
        // available for future scoring/filtering).
        let chunks: Vec<_> = scored_chunks.into_iter().map(|(c, _score)| c).collect();

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
                if let Err(e) = app.emit("chat-stream-chunk", &text) {
                    tracing::error!("failed to emit chat-stream-chunk: {e}");
                }
            }
            Err(e) => {
                if let Err(emit_err) = app.emit("chat-stream-error", e.to_string()) {
                    tracing::error!("failed to emit chat-stream-error: {emit_err}");
                }
                return Err(RagCommandError::Chat(e));
            }
        }
    }

    if let Err(e) = app.emit("chat-stream-done", ()) {
        tracing::error!("failed to emit chat-stream-done: {e}");
    }
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
    let has_transcript = library
        .has_artifact(id, &ArtifactKind::TranscriptJson)
        .map_err(|e| RagCommandError::Library(e.to_string()))?;

    if !has_transcript {
        return Err(RagCommandError::TranscriptNotFound(id.to_string()));
    }

    let transcript_bytes = library
        .read_artifact(id, &ArtifactKind::TranscriptJson)
        .map_err(|e| RagCommandError::Library(e.to_string()))?;

    let transcript: TranscriptResult = serde_json::from_slice(&transcript_bytes)?;

    let config = rag
        .config
        .lock()
        .map_err(|e| RagCommandError::Other(e.to_string()))?
        .clone();

    // Chunk the transcript with ~20% overlap for better retrieval.
    let overlap = config.chunk_size / 5;
    let chunks = chunk_transcript(id, &transcript, config.chunk_size, overlap);
    if chunks.is_empty() {
        // Nothing to index — still mark as ready (empty transcript).
        return Ok(());
    }

    // Embed all chunks in batches of 50 to avoid overloading the API.
    let emb_client = EmbeddingsClient::new(&config);
    let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
    let mut embeddings: Vec<Vec<f32>> = Vec::with_capacity(texts.len());
    const BATCH_SIZE: usize = 50;
    for batch in texts.chunks(BATCH_SIZE) {
        let batch_embeddings = emb_client.embed_batch(batch).await?;
        embeddings.extend(batch_embeddings);
    }

    // Determine dimension from the first embedding and ensure the vector
    // table is set up correctly.
    let dimension = embeddings.first().map(|e| e.len()).unwrap_or(0);

    if dimension == 0 {
        return Err(RagCommandError::Other(
            "Embedding dimension is 0 — cannot initialise vector table".to_string(),
        ));
    }

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
