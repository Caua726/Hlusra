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
      <div className="glass-card rounded-2xl overflow-hidden">
        <div className="px-5 py-3 flex items-center justify-between border-b border-white/5">
          <h2 className="text-[11px] font-semibold text-white/50 uppercase tracking-wider">Transcrição</h2>
        </div>
        <div className="flex flex-col items-center gap-3 p-8 text-white/30">
          <div className="w-6 h-6 border-[3px] border-white/10 border-t-brand-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
          <p className="text-[12px]">Transcrevendo...</p>
        </div>
      </div>
    );
  }

  if (transcriptionStatus === "failed") {
    return (
      <div className="glass-card rounded-2xl overflow-hidden">
        <div className="px-5 py-3 flex items-center justify-between border-b border-white/5">
          <h2 className="text-[11px] font-semibold text-white/50 uppercase tracking-wider">Transcrição</h2>
        </div>
        <div className="flex flex-col items-center gap-3 p-8">
          <p className="text-red-400/80 text-xs" role="alert">A transcrição falhou.</p>
          <button
            className="glass-heavy px-4 py-2 text-[11px] rounded-xl text-white/40 hover:text-white/70 transition-all cursor-pointer border-0 bg-transparent"
            onClick={handleTranscribe}
            disabled={transcribing}
          >
            Tentar novamente
          </button>
          {error && <p className="text-red-400/80 text-xs" role="alert">{error}</p>}
        </div>
      </div>
    );
  }

  if (transcriptionStatus === "pending" || (!transcript && transcriptionStatus !== "done")) {
    return (
      <div className="glass-card rounded-2xl overflow-hidden">
        <div className="px-5 py-3 flex items-center justify-between border-b border-white/5">
          <h2 className="text-[11px] font-semibold text-white/50 uppercase tracking-wider">Transcrição</h2>
        </div>
        <div className="flex flex-col items-center gap-3 p-8">
          <p className="text-white/30 text-[12px]">Nenhuma transcrição disponível.</p>
          <button
            className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm cursor-pointer border-0 disabled:opacity-40"
            onClick={handleTranscribe}
            disabled={transcribing}
          >
            Transcrever agora
          </button>
          {error && <p className="text-red-400/80 text-xs" role="alert">{error}</p>}
        </div>
      </div>
    );
  }

  // Try to parse transcript as JSON
  let parsed: TranscriptResult | null = null;
  if (transcript) {
    try {
      parsed = JSON.parse(transcript) as TranscriptResult;
    } catch {
      // Not JSON
    }
  }

  if (parsed && parsed.segments && parsed.segments.length > 0) {
    return (
      <div className="glass-card rounded-2xl overflow-hidden">
        <div className="px-5 py-3 flex items-center justify-between border-b border-white/5">
          <h2 className="text-[11px] font-semibold text-white/50 uppercase tracking-wider">Transcrição</h2>
          <button
            className="text-[10px] text-white/20 hover:text-white/50 transition-colors bg-transparent border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
            onClick={handleTranscribe}
            disabled={transcribing}
          >
            Retranscrever
          </button>
        </div>
        <div className="max-h-[180px] overflow-y-auto p-5 space-y-3">
          {parsed.segments.map((seg, i) => (
            <div key={i} className="group cursor-pointer">
              <button
                className="text-[10px] font-mono text-brand-400/60 hover:text-brand-400 tabular-nums transition-colors bg-transparent border-0 cursor-pointer"
                onClick={() => onSeek(seg.start)}
                title="Ir para este trecho"
              >
                {formatTime(seg.start)} — {formatTime(seg.end)}
              </button>
              <p className="text-[12px] text-white/40 mt-1 leading-relaxed group-hover:text-white/60 transition-colors">
                {seg.text}
              </p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  // Plain text fallback
  return (
    <div className="glass-card rounded-2xl overflow-hidden">
      <div className="px-5 py-3 flex items-center justify-between border-b border-white/5">
        <h2 className="text-[11px] font-semibold text-white/50 uppercase tracking-wider">Transcrição</h2>
      </div>
      {transcript ? (
        <div className="max-h-[180px] overflow-y-auto p-5">
          <p className="text-[12px] text-white/40 leading-relaxed whitespace-pre-wrap">{transcript}</p>
        </div>
      ) : (
        <div className="flex items-center justify-center p-8">
          <p className="text-white/30 text-[12px]">Transcrição concluída, mas sem conteúdo disponível.</p>
        </div>
      )}
    </div>
  );
}
