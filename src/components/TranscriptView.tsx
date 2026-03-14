import { useState } from "react";
import { transcribeMeeting } from "../lib/api";
import type { TranscriptResult } from "../lib/api";

interface Props {
  meetingId: string;
  transcript: string | null;
  transcriptionStatus: "pending" | "processing" | "done" | "failed";
  onSeek: (time: number) => void;
  onStatusChange: () => void;
}

function formatTime(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = Math.floor(secs % 60);
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export default function TranscriptView({
  meetingId,
  transcript,
  transcriptionStatus,
  onSeek,
  onStatusChange,
}: Props) {
  const [transcribing, setTranscribing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleTranscribe() {
    setError(null);
    setTranscribing(true);
    try {
      await transcribeMeeting(meetingId);
      onStatusChange();
    } catch (e) {
      setError(String(e));
    } finally {
      setTranscribing(false);
    }
  }

  if (transcriptionStatus === "processing" || transcribing) {
    return (
      <div className="transcript-section">
        <h3>Transcrição</h3>
        <div className="transcript-loading">
          <div className="spinner" />
          <p>Transcrevendo...</p>
        </div>
      </div>
    );
  }

  if (transcriptionStatus === "pending" || (!transcript && transcriptionStatus !== "done")) {
    return (
      <div className="transcript-section">
        <h3>Transcrição</h3>
        <div className="transcript-empty">
          <p>Nenhuma transcrição disponível.</p>
          <button className="btn-primary" onClick={handleTranscribe}>
            Transcrever agora
          </button>
          {error && <p className="error-text">{error}</p>}
        </div>
      </div>
    );
  }

  if (transcriptionStatus === "failed") {
    return (
      <div className="transcript-section">
        <h3>Transcrição</h3>
        <div className="transcript-empty">
          <p className="error-text">A transcrição falhou.</p>
          <button className="btn-primary" onClick={handleTranscribe}>
            Tentar novamente
          </button>
          {error && <p className="error-text">{error}</p>}
        </div>
      </div>
    );
  }

  // Try to parse transcript as JSON (TranscriptResult with segments)
  let parsed: TranscriptResult | null = null;
  if (transcript) {
    try {
      parsed = JSON.parse(transcript) as TranscriptResult;
    } catch {
      // Not JSON — show as plain text
    }
  }

  if (parsed && parsed.segments && parsed.segments.length > 0) {
    return (
      <div className="transcript-section">
        <h3>Transcrição</h3>
        <div className="transcript-segments">
          {parsed.segments.map((seg, i) => (
            <div key={i} className="transcript-segment">
              <button
                className="segment-time"
                onClick={() => onSeek(seg.start)}
                title="Ir para este trecho"
              >
                {formatTime(seg.start)} - {formatTime(seg.end)}
              </button>
              <p className="segment-text">{seg.text}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  // Plain text fallback
  return (
    <div className="transcript-section">
      <h3>Transcrição</h3>
      <div className="transcript-plain">
        <p>{transcript}</p>
      </div>
    </div>
  );
}
