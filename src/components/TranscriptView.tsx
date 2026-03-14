import { useState, useEffect, useRef, useCallback } from "react";
import { transcribeMeeting, getTranscriptionStatus } from "../lib/api";
import type { TranscriptResult } from "../lib/api";
import { formatError } from "../lib/format";

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
  const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const clearPoll = useCallback(() => {
    if (pollRef.current) {
      clearInterval(pollRef.current);
      pollRef.current = null;
    }
  }, []);

  useEffect(() => {
    return clearPoll;
  }, [clearPoll]);

  async function handleTranscribe() {
    setError(null);
    setTranscribing(true);
    try {
      await transcribeMeeting(meetingId);

      // Poll for completion every 2 seconds
      pollRef.current = setInterval(async () => {
        try {
          const status = await getTranscriptionStatus(meetingId);
          if (status === "done" || status === "failed") {
            clearPoll();
            setTranscribing(false);
            onStatusChange();
          }
        } catch {
          // polling error is non-fatal
        }
      }, 2000);
    } catch (e) {
      setError(formatError(e));
      setTranscribing(false);
    }
  }

  if (transcriptionStatus === "processing" || transcribing) {
    return (
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Transcricao</h3>
        <div className="flex flex-col items-center gap-3 p-8 text-zinc-500">
          <div className="w-6 h-6 border-[3px] border-zinc-700 border-t-rose-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
          <p>Transcrevendo...</p>
        </div>
      </div>
    );
  }

  if (transcriptionStatus === "failed") {
    return (
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Transcricao</h3>
        <div className="text-center p-8 text-zinc-500 flex flex-col items-center gap-4">
          <p className="text-red-500 text-sm">A transcricao falhou.</p>
          <button className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleTranscribe} disabled={transcribing}>
            Tentar novamente
          </button>
          {error && <p className="text-red-500 text-sm">{error}</p>}
        </div>
      </div>
    );
  }

  if (transcriptionStatus === "pending" || (!transcript && transcriptionStatus !== "done")) {
    return (
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Transcricao</h3>
        <div className="text-center p-8 text-zinc-500 flex flex-col items-center gap-4">
          <p>Nenhuma transcricao disponivel.</p>
          <button className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleTranscribe} disabled={transcribing}>
            Transcrever agora
          </button>
          {error && <p className="text-red-500 text-sm">{error}</p>}
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
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Transcricao</h3>
        <div className="max-h-[400px] overflow-y-auto pr-2">
          {parsed.segments.map((seg, i) => (
            <div key={i} className="flex gap-4 py-2 border-b border-zinc-800 last:border-b-0">
              <button
                className="bg-transparent border-none text-blue-500 text-xs tabular-nums cursor-pointer whitespace-nowrap py-0.5 shrink-0 transition-colors duration-150 hover:text-blue-400"
                onClick={() => onSeek(seg.start)}
                title="Ir para este trecho"
              >
                {formatTime(seg.start)} - {formatTime(seg.end)}
              </button>
              <p className="text-sm leading-relaxed text-zinc-100">{seg.text}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  // Plain text fallback
  return (
    <div className="mb-8">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Transcricao</h3>
      {transcript ? (
        <div className="p-4 bg-zinc-800/50 rounded-lg text-sm leading-7 max-h-[400px] overflow-y-auto whitespace-pre-wrap">
          <p>{transcript}</p>
        </div>
      ) : (
        <div className="text-center p-8 text-zinc-500 flex flex-col items-center gap-4">
          <p>Transcricao concluida, mas sem conteudo disponivel.</p>
        </div>
      )}
    </div>
  );
}
