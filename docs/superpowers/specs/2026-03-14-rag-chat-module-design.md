# RAG/Chat Module — Design Spec

## Overview

The RAG/Chat module enables conversational interaction with meeting content. After a meeting is transcribed, the text is chunked, embedded, and stored in SQLite with `sqlite-vec`. Users can then ask questions about a specific meeting and get answers grounded in the transcript.

## Architecture

### Two Pipelines

**Indexing Pipeline** (runs after transcription):
1. Get `transcript.json` from Library
2. Chunker splits transcript into chunks (based on segments, configurable size)
3. Send chunks to embeddings API (OpenRouter)
4. Store chunks + vectors in SQLite with `sqlite-vec`
5. Update `chat_status → Ready` via Library

**Query Pipeline** (when user asks a question):
1. Embed user's question via same embeddings API
2. Vector search top-k in SQLite, scoped to that meeting
3. Build prompt: system prompt + relevant chunks + question
4. Send to LLM via Chat API (OpenRouter)
5. Stream response back to frontend

### Components

1. **Chunker** — splits transcript into chunks, preserves timestamps
2. **EmbeddingsClient** — HTTP client, OpenAI-compatible, focused on OpenRouter
3. **VectorStore** — SQLite + sqlite-vec, scoped search per meeting
4. **ChatClient** — HTTP client, OpenAI-compatible, focused on OpenRouter, streaming

## Data Storage

### Separate SQLite Database

The RAG module uses its own SQLite database file (`~/.local/share/hlusra/rag.db`), separate from the Library's database. This preserves the Library's exclusive ownership of the main database while giving the RAG module direct control over vector operations.

The RAG database references meetings by ID but has no foreign key constraint to the Library database.

### Schema

```sql
CREATE TABLE chunks (
    id TEXT PRIMARY KEY,
    meeting_id TEXT NOT NULL,       -- matches meeting ID from Library
    text TEXT NOT NULL,
    start_time REAL NOT NULL,       -- timestamp of first segment in chunk
    end_time REAL NOT NULL,         -- timestamp of last segment in chunk
    chunk_index INTEGER NOT NULL,   -- order in transcript
    embedding BLOB NOT NULL         -- vector via sqlite-vec
);

-- sqlite-vec virtual table for vector search
-- Dimension is set at table creation based on the configured embedding model
CREATE VIRTUAL TABLE chunks_vec USING vec0(
    chunk_id TEXT PRIMARY KEY,
    embedding float[1536]           -- default for common models; recreated if model changes
);

CREATE TABLE rag_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
-- Stores: embedding_model, embedding_dimension
```

### Embedding Dimension Strategy

- The embedding dimension is determined by the configured model and stored in `rag_meta`
- If the user changes the embedding model to one with a different dimension, all existing embeddings become incompatible
- On model change: the app warns the user and offers to reindex all meetings
- The `chunks_vec` virtual table is dropped and recreated with the new dimension

### Chunk

```rust
Chunk {
    id: String,
    meeting_id: String,
    text: String,
    start_time: f64,
    end_time: f64,
    chunk_index: usize,
}
```

### Chunking Strategy

- Groups consecutive transcript segments until reaching configurable size (default: ~500 tokens)
- Preserves timestamps from first and last segment in each chunk
- Respects segment boundaries — never splits mid-segment

## API Configuration

Both embeddings and chat use OpenAI-compatible endpoints, with separate configuration:

```rust
RagConfig {
    // Embeddings
    embeddings_url: String,        // default: OpenRouter endpoint
    embeddings_api_key: String,
    embeddings_model: String,

    // Chat LLM
    chat_url: String,              // default: OpenRouter endpoint
    chat_api_key: String,
    chat_model: String,

    // RAG parameters
    chunk_size: usize,             // default: 500 tokens
    top_k: usize,                  // default: 5
}
```

## Chat Behavior

- **Scoped per meeting** — each chat searches only the chunks of that specific meeting
- **Ephemeral history** — no chat history is persisted. Opening a meeting starts a fresh chat every time
- **Streaming** — LLM responses are streamed to the frontend for responsive UX

## Tauri Commands

```rust
index_meeting(id: String) -> Result<()>
reindex_meeting(id: String) -> Result<()>
chat_message(meeting_id: String, message: String) -> Stream<String>
get_chat_status(id: String) -> ChatStatus  // NotIndexed | Indexing | Ready | Failed
```

## Dependencies

### Rust crates

| Crate | Purpose |
|-------|---------|
| `sqlite-vec` | Vector search extension for SQLite |
| `reqwest` | HTTP client for embeddings + chat APIs |
| `serde` / `serde_json` | Serialization for API requests/responses |
| `tokio` | Async runtime for streaming |

## File Structure

```
src-tauri/src/rag/
    mod.rs              — public exports
    chunker.rs          — splits transcript into chunks
    embeddings.rs       — EmbeddingsClient (HTTP, OpenAI-compatible)
    vector_store.rs     — SQLite + sqlite-vec (store + search)
    chat.rs             — ChatClient (HTTP, streaming, OpenAI-compatible)
    prompt.rs           — prompt assembly (system + context + question)
    types.rs            — Chunk, RagConfig, ChatStatus, enums
```

## Decisions Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Vector storage | Separate SQLite DB + sqlite-vec | Own DB preserves Library exclusivity, no extra service |
| Embedding dimension | Fixed per model, reindex on change | Simple, explicit, no mixed-dimension vectors |
| Embeddings provider | OpenAI-compatible endpoint, focused on OpenRouter | Flexible, covers free/paid options |
| Chat provider | Same format, separate config | May use different model/provider than embeddings |
| Chat scope | Per meeting only | Simpler, clear context, no cross-meeting confusion |
| Chat history | Ephemeral | No state to manage, fresh context every time |
| Chunking | Segment-based with size limit | Preserves natural speech boundaries |
