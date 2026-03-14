use crate::rag::chat::ChatMessage;
use crate::rag::types::Chunk;

/// Format seconds as MM:SS for display in prompts.
fn format_timestamp(seconds: f64) -> String {
    let total_secs = seconds as u64;
    let minutes = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", minutes, secs)
}

/// The system prompt that instructs the LLM how to behave.
const SYSTEM_PROMPT: &str = "\
You are an AI assistant that helps users understand their meeting recordings. \
You answer questions based on the provided transcript excerpts. \

Guidelines:
- Answer questions accurately based on the transcript context provided below.
- If the answer is not in the provided context, say so honestly.
- When referencing specific parts of the meeting, mention the timestamps.
- Be concise and helpful.
- Respond in the same language the user asks their question in.";

/// Build the full list of messages for a chat completion request.
///
/// The message list consists of:
/// 1. A system prompt with instructions
/// 2. A system message with the relevant transcript chunks (with timestamps)
/// 3. The user's question
pub fn build_messages(chunks: &[Chunk], user_question: &str) -> Vec<ChatMessage> {
    let mut messages = Vec::with_capacity(3);

    // System prompt.
    messages.push(ChatMessage {
        role: "system".to_string(),
        content: SYSTEM_PROMPT.to_string(),
    });

    // Context from transcript chunks.
    if !chunks.is_empty() {
        let context = build_context(chunks);
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: context,
        });
    }

    // User question.
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: user_question.to_string(),
    });

    messages
}

/// Build the context block from relevant chunks.
///
/// Each chunk is presented with its timestamp range so the LLM can reference
/// specific moments in the meeting.
fn build_context(chunks: &[Chunk]) -> String {
    let mut context = String::from(
        "Here are the most relevant excerpts from the meeting transcript:\n\n",
    );

    for (i, chunk) in chunks.iter().enumerate() {
        let start = format_timestamp(chunk.start_time);
        let end = format_timestamp(chunk.end_time);
        context.push_str(&format!(
            "[Excerpt {} | {} - {}]\n{}\n\n",
            i + 1,
            start,
            end,
            chunk.text.trim()
        ));
    }

    context
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_chunk(index: usize, start: f64, end: f64, text: &str) -> Chunk {
        Chunk {
            id: format!("c{}", index),
            meeting_id: "m1".into(),
            text: text.into(),
            start_time: start,
            end_time: end,
            chunk_index: index,
        }
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(0.0), "00:00");
        assert_eq!(format_timestamp(65.0), "01:05");
        assert_eq!(format_timestamp(3661.0), "61:01");
    }

    #[test]
    fn test_build_messages_with_chunks() {
        let chunks = vec![
            make_chunk(0, 0.0, 30.0, "Welcome everyone to the meeting."),
            make_chunk(1, 30.0, 60.0, "Let's discuss the roadmap."),
        ];
        let messages = build_messages(&chunks, "What was discussed?");

        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].role, "system");
        assert!(messages[0].content.contains("AI assistant"));
        assert_eq!(messages[1].role, "system");
        assert!(messages[1].content.contains("Excerpt 1"));
        assert!(messages[1].content.contains("00:00 - 00:30"));
        assert!(messages[1].content.contains("Excerpt 2"));
        assert!(messages[1].content.contains("00:30 - 01:00"));
        assert_eq!(messages[2].role, "user");
        assert_eq!(messages[2].content, "What was discussed?");
    }

    #[test]
    fn test_build_messages_without_chunks() {
        let messages = build_messages(&[], "What was discussed?");
        // System + user, no context message.
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
    }

    #[test]
    fn test_context_includes_timestamps() {
        let chunks = vec![make_chunk(0, 125.0, 185.0, "Some discussion")];
        let context = build_context(&chunks);
        assert!(context.contains("02:05 - 03:05"));
        assert!(context.contains("Some discussion"));
    }
}
