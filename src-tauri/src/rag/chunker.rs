use uuid::Uuid;

use crate::rag::types::Chunk;
use crate::transcription::types::TranscriptResult;

/// Approximate token count for a string.
///
/// Uses a simple heuristic: for CJK-heavy text (where whitespace splitting
/// drastically undercounts), we use `chars().count() / 2`.  For Latin/other
/// scripts we split on whitespace.  This is intentionally cheap — the goal is
/// "roughly 500 tokens", not exact BPE tokenisation.
fn estimate_tokens(text: &str) -> usize {
    if has_significant_cjk(text) {
        // CJK characters map to ~1-2 tokens each in most tokenisers.
        // chars/2 is a reasonable middle-ground.
        (text.chars().count() + 1) / 2
    } else {
        text.split_whitespace().count()
    }
}

/// Returns true if more than 30% of the non-whitespace characters in `text`
/// fall in CJK Unified Ideographs ranges (U+4E00..U+9FFF, U+3400..U+4DBF,
/// U+F900..U+FAFF, plus Hiragana U+3040..U+309F and Katakana U+30A0..U+30FF).
fn has_significant_cjk(text: &str) -> bool {
    let mut total: usize = 0;
    let mut cjk: usize = 0;
    for ch in text.chars() {
        if ch.is_whitespace() {
            continue;
        }
        total += 1;
        if is_cjk_char(ch) {
            cjk += 1;
        }
    }
    total > 0 && (cjk * 100 / total) > 30
}

/// Check if a character belongs to a CJK / Kana Unicode block.
fn is_cjk_char(ch: char) -> bool {
    matches!(ch,
        '\u{4E00}'..='\u{9FFF}'   // CJK Unified Ideographs
        | '\u{3400}'..='\u{4DBF}' // CJK Unified Ideographs Extension A
        | '\u{F900}'..='\u{FAFF}' // CJK Compatibility Ideographs
        | '\u{3040}'..='\u{309F}' // Hiragana
        | '\u{30A0}'..='\u{30FF}' // Katakana
        | '\u{AC00}'..='\u{D7AF}' // Hangul Syllables
    )
}

/// Split a `TranscriptResult` into chunks of approximately `chunk_size` tokens.
///
/// The chunker groups consecutive transcript segments until the running token
/// count reaches or exceeds `chunk_size`, then starts a new chunk.  Segment
/// boundaries are always respected — a segment is never split mid-text.
///
/// Each chunk preserves the `start_time` of its first segment and the
/// `end_time` of its last segment.
pub fn chunk_transcript(
    meeting_id: &str,
    transcript: &TranscriptResult,
    chunk_size: usize,
) -> Vec<Chunk> {
    if chunk_size == 0 {
        return Vec::new();
    }

    if transcript.segments.is_empty() {
        return Vec::new();
    }

    let mut chunks: Vec<Chunk> = Vec::new();
    let mut current_texts: Vec<String> = Vec::new();
    let mut current_tokens: usize = 0;
    let mut current_start: f64 = transcript.segments[0].start;
    let mut current_end: f64 = transcript.segments[0].end;
    let mut chunk_index: usize = 0;

    for segment in &transcript.segments {
        // Skip segments with empty/whitespace-only text.
        if segment.text.trim().is_empty() {
            continue;
        }

        let seg_tokens = estimate_tokens(&segment.text);

        // If adding this segment would exceed the limit and we already have
        // content, flush the current chunk first.
        if current_tokens > 0 && current_tokens + seg_tokens > chunk_size {
            let text = current_texts.join(" ").trim().to_string();
            if !text.is_empty() {
                chunks.push(Chunk {
                    id: Uuid::new_v4().to_string(),
                    meeting_id: meeting_id.to_string(),
                    text,
                    start_time: current_start,
                    end_time: current_end,
                    chunk_index,
                });
                chunk_index += 1;
            }

            current_texts.clear();
            current_tokens = 0;
            current_start = segment.start;
        }

        // If the buffer is empty, set start from this segment.
        if current_texts.is_empty() {
            current_start = segment.start;
        }

        current_texts.push(segment.text.clone());
        current_tokens += seg_tokens;
        current_end = segment.end;
    }

    // Flush remaining content.
    if !current_texts.is_empty() {
        let text = current_texts.join(" ").trim().to_string();
        if !text.is_empty() {
            chunks.push(Chunk {
                id: Uuid::new_v4().to_string(),
                meeting_id: meeting_id.to_string(),
                text,
                start_time: current_start,
                end_time: current_end,
                chunk_index,
            });
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcription::types::{Segment, TranscriptResult};

    fn make_segment(start: f64, end: f64, text: &str) -> Segment {
        Segment {
            start,
            end,
            text: text.to_string(),
            words: Vec::new(),
        }
    }

    #[test]
    fn test_empty_transcript_produces_no_chunks() {
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![],
            full_text: String::new(),
        };
        let chunks = chunk_transcript("m1", &tr, 500);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_single_segment_produces_one_chunk() {
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![make_segment(0.0, 5.0, "Hello world")],
            full_text: "Hello world".into(),
        };
        let chunks = chunk_transcript("m1", &tr, 500);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].meeting_id, "m1");
        assert_eq!(chunks[0].chunk_index, 0);
        assert!((chunks[0].start_time - 0.0).abs() < f64::EPSILON);
        assert!((chunks[0].end_time - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiple_segments_grouped_by_chunk_size() {
        // Each segment has ~3 words. With chunk_size=5, first segment (3 tokens)
        // fits, but adding the second (3 more = 6 > 5) triggers a flush.
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![
                make_segment(0.0, 2.0, "one two three"),
                make_segment(2.0, 4.0, "four five six"),
                make_segment(4.0, 6.0, "seven eight nine"),
                make_segment(6.0, 8.0, "ten eleven twelve"),
            ],
            full_text: String::new(),
        };
        let chunks = chunk_transcript("m1", &tr, 5);
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].chunk_index, 0);
        assert_eq!(chunks[3].chunk_index, 3);
    }

    #[test]
    fn test_timestamps_preserved() {
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![
                make_segment(10.0, 15.0, "hello"),
                make_segment(15.0, 20.0, "world"),
            ],
            full_text: String::new(),
        };
        // chunk_size big enough to keep both in one chunk
        let chunks = chunk_transcript("m1", &tr, 500);
        assert_eq!(chunks.len(), 1);
        assert!((chunks[0].start_time - 10.0).abs() < f64::EPSILON);
        assert!((chunks[0].end_time - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_estimate_tokens_latin() {
        assert_eq!(estimate_tokens("hello world foo bar"), 4);
        assert_eq!(estimate_tokens("one"), 1);
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn test_estimate_tokens_cjk() {
        // 6 CJK chars -> chars/2 = 3 tokens (rounded up)
        let cjk = "\u{4F60}\u{597D}\u{4E16}\u{754C}\u{65E9}\u{5B89}";
        assert_eq!(estimate_tokens(cjk), 3);
    }

    #[test]
    fn test_has_significant_cjk() {
        assert!(has_significant_cjk("\u{4F60}\u{597D}\u{4E16}\u{754C}"));
        assert!(!has_significant_cjk("hello world"));
        // Mixed: 2 CJK + 10 latin chars (non-ws) -> 2/12 = 16% < 30%
        assert!(!has_significant_cjk("hello world \u{4F60}\u{597D}"));
    }

    #[test]
    fn test_chunk_ids_are_unique() {
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![
                make_segment(0.0, 1.0, "a"),
                make_segment(1.0, 2.0, "b"),
                make_segment(2.0, 3.0, "c"),
            ],
            full_text: String::new(),
        };
        let chunks = chunk_transcript("m1", &tr, 1);
        let ids: std::collections::HashSet<&str> =
            chunks.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids.len(), chunks.len());
    }
}
