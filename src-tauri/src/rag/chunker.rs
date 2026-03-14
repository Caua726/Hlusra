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

/// Split a `TranscriptResult` into chunks of approximately `chunk_size` tokens,
/// with optional overlap between consecutive chunks.
///
/// The chunker groups consecutive transcript segments until the running token
/// count reaches or exceeds `chunk_size`, then starts a new chunk.  Segment
/// boundaries are always respected — a segment is never split mid-text.
///
/// When `overlap` is non-zero, the last segments whose combined token count
/// fits within `overlap` are retained in the buffer for the next chunk.  This
/// produces overlapping context windows that improve retrieval quality.
///
/// Each chunk preserves the `start_time` of its first segment and the
/// `end_time` of its last segment.
pub fn chunk_transcript(
    meeting_id: &str,
    transcript: &TranscriptResult,
    chunk_size: usize,
    overlap: usize,
) -> Vec<Chunk> {
    if chunk_size == 0 {
        return Vec::new();
    }

    if transcript.segments.is_empty() {
        return Vec::new();
    }

    /// A buffered segment with its precomputed token count.
    struct BufSeg {
        text: String,
        tokens: usize,
        start: f64,
        end: f64,
    }

    let mut chunks: Vec<Chunk> = Vec::new();
    let mut buffer: Vec<BufSeg> = Vec::new();
    let mut current_tokens: usize = 0;
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
            let text = buffer
                .iter()
                .map(|s| s.text.as_str())
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            if !text.is_empty() {
                let start_time = buffer.first().map(|s| s.start).unwrap_or(0.0);
                let end_time = buffer.last().map(|s| s.end).unwrap_or(0.0);
                chunks.push(Chunk {
                    id: Uuid::new_v4().to_string(),
                    meeting_id: meeting_id.to_string(),
                    text,
                    start_time,
                    end_time,
                    chunk_index,
                });
                chunk_index += 1;
            }

            // Retain the last segments that fit within `overlap` tokens.
            if overlap > 0 {
                let mut keep_tokens: usize = 0;
                let mut keep_from = buffer.len();
                for (i, seg) in buffer.iter().enumerate().rev() {
                    if keep_tokens + seg.tokens > overlap {
                        break;
                    }
                    keep_tokens += seg.tokens;
                    keep_from = i;
                }
                buffer.drain(..keep_from);
                current_tokens = keep_tokens;
            } else {
                buffer.clear();
                current_tokens = 0;
            }
        }

        buffer.push(BufSeg {
            text: segment.text.clone(),
            tokens: seg_tokens,
            start: segment.start,
            end: segment.end,
        });
        current_tokens += seg_tokens;
    }

    // Flush remaining content.
    if !buffer.is_empty() {
        let text = buffer
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            let start_time = buffer.first().map(|s| s.start).unwrap_or(0.0);
            let end_time = buffer.last().map(|s| s.end).unwrap_or(0.0);
            chunks.push(Chunk {
                id: Uuid::new_v4().to_string(),
                meeting_id: meeting_id.to_string(),
                text,
                start_time,
                end_time,
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
        let chunks = chunk_transcript("m1", &tr, 500, 0);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_single_segment_produces_one_chunk() {
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![make_segment(0.0, 5.0, "Hello world")],
            full_text: "Hello world".into(),
        };
        let chunks = chunk_transcript("m1", &tr, 500, 0);
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
        let chunks = chunk_transcript("m1", &tr, 5, 0);
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
        let chunks = chunk_transcript("m1", &tr, 500, 0);
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
        let chunks = chunk_transcript("m1", &tr, 1, 0);
        let ids: std::collections::HashSet<&str> =
            chunks.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids.len(), chunks.len());
    }

    #[test]
    fn test_overlap_retains_trailing_segments() {
        // 4 segments, each 3 tokens. chunk_size=5 flushes after each segment.
        // overlap=3 means after flush the last segment (3 tokens) is retained.
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![
                make_segment(0.0, 2.0, "one two three"),
                make_segment(2.0, 4.0, "four five six"),
                make_segment(4.0, 6.0, "seven eight nine"),
            ],
            full_text: String::new(),
        };
        let chunks = chunk_transcript("m1", &tr, 5, 3);
        // With overlap, chunk 0 = seg0, then seg0 retained + seg1 triggers flush
        // -> chunk 1 = seg0+seg1, then seg1 retained + seg2 triggers flush
        // -> chunk 2 = seg1+seg2 (final flush).
        // Actually: first iteration, seg0(3 tokens) fits. seg1: 3+3=6>5, flush
        // seg0 as chunk0. Retain seg0(3<=3). Buffer=[seg0(3)]. Add seg1: 3+3=6>5,
        // flush [seg0,seg1] as chunk1. Retain seg1(3<=3). Buffer=[seg1(3)].
        // Add seg2: 3+3=6>5, flush [seg1,seg2] as chunk2. Retain seg2. Final
        // flush: chunk3=[seg2].
        // Wait, the flush happens BEFORE adding the new segment.
        // Let me re-trace:
        // Start: buffer=[], tokens=0
        // seg0(3): 0+3<=5, push. buffer=[seg0], tokens=3
        // seg1(3): 3+3=6>5, flush buffer=[seg0]->chunk0. Retain: seg0(3<=3).
        //   buffer=[seg0], tokens=3. Push seg1: buffer=[seg0,seg1], tokens=6
        // seg2(3): 6+3=9>5, flush buffer=[seg0,seg1]->chunk1. Retain: seg1(3<=3).
        //   buffer=[seg1], tokens=3. Push seg2: buffer=[seg1,seg2], tokens=6
        // End: flush buffer=[seg1,seg2]->chunk2.
        assert_eq!(chunks.len(), 3);
        // chunk1 should contain text from seg0 and seg1 (overlap carried seg0)
        assert!(chunks[1].text.contains("one"));
        assert!(chunks[1].text.contains("four"));
        // chunk2 should contain text from seg1 and seg2 (overlap carried seg1)
        assert!(chunks[2].text.contains("four"));
        assert!(chunks[2].text.contains("seven"));
    }

    #[test]
    fn test_zero_overlap_is_same_as_no_overlap() {
        let tr = TranscriptResult {
            language: "en".into(),
            segments: vec![
                make_segment(0.0, 2.0, "one two three"),
                make_segment(2.0, 4.0, "four five six"),
            ],
            full_text: String::new(),
        };
        let no_overlap = chunk_transcript("m1", &tr, 5, 0);
        assert_eq!(no_overlap.len(), 2);
        assert!(!no_overlap[1].text.contains("one"));
    }
}
