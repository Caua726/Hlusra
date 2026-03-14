use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::rag::types::RagConfig;

/// A single message in the OpenAI chat format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Request body for the OpenAI-compatible chat completions endpoint.
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

/// A single choice delta in a streaming response chunk.
#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

/// A single choice in a streaming response chunk.
#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

/// A streaming response chunk from the chat endpoint (SSE `data:` line).
#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

/// Error response body from the API.
#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    error: Option<ApiErrorDetail>,
}

#[derive(Debug, Deserialize)]
struct ApiErrorDetail {
    message: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },
    #[error("Stream error: {0}")]
    Stream(String),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

impl serde::Serialize for ChatError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// HTTP client for OpenAI-compatible chat completions (focused on OpenRouter).
///
/// Supports streaming responses via Server-Sent Events (SSE).
pub struct ChatClient {
    client: Client,
    url: String,
    api_key: String,
    model: String,
}

impl ChatClient {
    /// Create a new client from the RAG configuration.
    pub fn new(config: &RagConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .unwrap_or_else(|_| Client::new()),
            url: config.chat_url.clone(),
            api_key: config.chat_api_key.clone(),
            model: config.chat_model.clone(),
        }
    }

    /// Send a chat completion request with streaming.
    ///
    /// Returns an `mpsc::Receiver<String>` that yields text chunks as they
    /// arrive from the API. The channel closes when the stream ends.
    pub async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<mpsc::Receiver<Result<String, ChatError>>, ChatError> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages,
            stream: true,
        };

        let url = format!("{}/chat/completions", self.url.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let status_code = status.as_u16();
            let body_text = response.text().await.unwrap_or_default();
            let message = match serde_json::from_str::<ApiErrorBody>(&body_text) {
                Ok(err_body) => err_body
                    .error
                    .and_then(|e| e.message)
                    .unwrap_or_else(|| format!("HTTP {}", status_code)),
                Err(_) => format!("HTTP {}", status_code),
            };
            return Err(ChatError::Api {
                status: status_code,
                message,
            });
        }

        let (tx, rx) = mpsc::channel::<Result<String, ChatError>>(64);

        // Spawn a task to read the SSE stream and forward chunks.
        let mut byte_stream = response.bytes_stream();
        tokio::spawn(async move {
            use futures_util::StreamExt;

            let mut buf: Vec<u8> = Vec::new();

            /// Process all complete lines in `buf`, returning any incomplete
            /// trailing bytes.  Returns `true` if the stream should stop
            /// (either [DONE] was received or the receiver was dropped).
            async fn process_lines(
                buf: &mut Vec<u8>,
                tx: &mpsc::Sender<Result<String, ChatError>>,
            ) -> bool {
                loop {
                    let newline_pos = match buf.iter().position(|&b| b == b'\n') {
                        Some(p) => p,
                        None => return false,
                    };

                    let line_bytes = buf[..newline_pos].to_vec();
                    *buf = buf[newline_pos + 1..].to_vec();

                    let line = match String::from_utf8(line_bytes) {
                        Ok(s) => s,
                        Err(_) => continue, // skip invalid UTF-8 lines
                    };
                    let line = line.trim();

                    if line.is_empty() || line.starts_with(':') {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        let data = data.trim();

                        if data == "[DONE]" {
                            return true;
                        }

                        match serde_json::from_str::<StreamChunk>(data) {
                            Ok(chunk) => {
                                for choice in &chunk.choices {
                                    if let Some(ref content) = choice.delta.content {
                                        if !content.is_empty() {
                                            if tx.send(Ok(content.clone())).await.is_err() {
                                                return true;
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = tx
                                    .send(Err(ChatError::Stream(format!(
                                        "Failed to parse SSE chunk: {}",
                                        e
                                    ))))
                                    .await;
                                return true;
                            }
                        }
                    }
                }
            }

            while let Some(chunk_result) = byte_stream.next().await {
                match chunk_result {
                    Ok(bytes) => {
                        buf.extend_from_slice(&bytes);

                        if process_lines(&mut buf, &tx).await {
                            return;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(ChatError::Http(e))).await;
                        return;
                    }
                }
            }

            // Process any remaining content in the buffer after the stream ends.
            if !buf.is_empty() {
                // Add a trailing newline so the line-based parser can process it.
                buf.push(b'\n');
                let _ = process_lines(&mut buf, &tx).await;
            }
        });

        Ok(rx)
    }

    /// Send a non-streaming chat completion request.
    ///
    /// Returns the full response content as a single string.
    pub async fn chat_once(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<String, ChatError> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages,
            stream: false,
        };

        let url = format!("{}/chat/completions", self.url.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let status_code = status.as_u16();
            let body_text = response.text().await.unwrap_or_default();
            let message = match serde_json::from_str::<ApiErrorBody>(&body_text) {
                Ok(err_body) => err_body
                    .error
                    .and_then(|e| e.message)
                    .unwrap_or_else(|| format!("HTTP {}", status_code)),
                Err(_) => format!("HTTP {}", status_code),
            };
            return Err(ChatError::Api {
                status: status_code,
                message,
            });
        }

        #[derive(Deserialize)]
        struct NonStreamChoice {
            message: ChatMessage,
        }
        #[derive(Deserialize)]
        struct NonStreamResponse {
            choices: Vec<NonStreamChoice>,
        }

        let resp: NonStreamResponse = response.json().await?;
        if resp.choices.is_empty() {
            return Err(ChatError::Stream("API returned empty choices".to_string()));
        }
        Ok(resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".into(),
            content: "What was discussed?".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("What was discussed?"));
    }

    #[test]
    fn test_stream_chunk_deserialization() {
        let json = r#"{
            "choices": [{
                "delta": {"content": "Hello"},
                "finish_reason": null
            }]
        }"#;
        let chunk: StreamChunk = serde_json::from_str(json).unwrap();
        assert_eq!(chunk.choices.len(), 1);
        assert_eq!(
            chunk.choices[0].delta.content.as_deref(),
            Some("Hello")
        );
    }

    #[test]
    fn test_stream_chunk_done_marker() {
        // The "[DONE]" marker is not JSON — it's handled as a special case
        // in the stream parser, not via deserialization.
        let result = serde_json::from_str::<StreamChunk>("[DONE]");
        assert!(result.is_err());
    }
}
