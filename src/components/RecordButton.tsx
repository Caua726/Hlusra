import { useState, useEffect, useRef, useCallback } from "react";
import { startRecording, stopRecording, getRecordingStatus } from "../lib/api";
import { formatTimer, formatError } from "../lib/format";

interface Props {
  onRecordingStart: () => void;
  onRecordingDone: () => void;
  isRecordingView?: boolean;
}

export default function RecordButton({ onRecordingStart, onRecordingDone, isRecordingView }: Props) {
  const [recording, setRecording] = useState(false);
  const [withVideo, setWithVideo] = useState(false);
  const [elapsed, setElapsed] = useState(0);
  const [fileSize, setFileSize] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [starting, setStarting] = useState(false);
  const [stopping, setStopping] = useState(false);
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

  // If mounted in recording view, start recording immediately
  useEffect(() => {
    if (isRecordingView && !recording && !starting) {
      handleStart();
    }
  }, [isRecordingView]);

  async function handleStart() {
    setError(null);
    setStarting(true);
    try {
      await startRecording(withVideo);
      setRecording(true);
      setElapsed(0);
      setFileSize(0);

      // Signal parent to switch to recording view
      onRecordingStart();

      pollRef.current = setInterval(async () => {
        try {
          const status = await getRecordingStatus();
          if (status.state === "recording") {
            setElapsed(status.duration_secs);
            setFileSize(status.file_size);
          }
        } catch {
          // polling error is non-fatal
        }
      }, 1000);
    } catch (e) {
      setError(formatError(e));
    } finally {
      setStarting(false);
    }
  }

  async function handleStop() {
    setError(null);
    setStopping(true);
    clearPoll();
    try {
      await stopRecording();
      setRecording(false);
      setElapsed(0);
      onRecordingDone();
    } catch (e) {
      setError(formatError(e));
    } finally {
      setStopping(false);
    }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  // Recording view (shown inside the recording view container)
  if (isRecordingView || recording) {
    return (
      <>
        <div className="flex items-center gap-2.5 mb-8 stagger">
          <div className="relative">
            <div className="w-2.5 h-2.5 rounded-full bg-brand-500 animate-pulse-rec" />
            <div className="absolute inset-0 w-2.5 h-2.5 rounded-full bg-brand-500 animate-pulse-ring" />
          </div>
          <span className="text-[10px] font-semibold text-brand-400 uppercase tracking-[0.3em]">Gravando</span>
        </div>

        <div className="text-6xl font-mono font-extralight text-white tabular-nums tracking-widest mb-3 stagger" style={{ fontVariantNumeric: "tabular-nums" }}>
          {formatTimer(elapsed)}
        </div>
        <div className="text-[11px] text-white/15 mb-10 stagger">
          {withVideo ? "Video" : "Audio"} &middot; {formatSize(fileSize)}
        </div>

        <button
          onClick={handleStop}
          disabled={stopping}
          className="rec-btn glass-heavy px-8 py-3 rounded-2xl text-sm text-white/60 hover:text-brand-400 hover:border-brand-500/30 transition-all duration-300 active:scale-95 stagger cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
        >
          {stopping ? "Parando..." : "Parar"}
        </button>

        {error && <p className="text-red-500 text-sm mt-4">{error}</p>}
      </>
    );
  }

  // Home view - record button
  return (
    <>
      <div className="relative mb-8">
        {/* Pulse rings */}
        <div className="absolute inset-0 rounded-full border border-brand-500/15 animate-pulse-ring" />
        <div className="absolute inset-0 rounded-full border border-brand-500/10 animate-pulse-ring" style={{ animationDelay: "0.6s" }} />

        <button
          onClick={handleStart}
          disabled={starting}
          className="rec-btn relative w-20 h-20 rounded-full bg-gradient-to-br from-brand-500 to-brand-700 flex items-center justify-center glow-brand cursor-pointer group border-0 disabled:opacity-40 disabled:cursor-not-allowed"
        >
          {/* Mic icon */}
          <svg className="w-7 h-7 text-white group-hover:scale-110 transition-transform duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
        </button>
      </div>

      <button
        onClick={handleStart}
        disabled={starting}
        className="text-sm text-white/40 hover:text-white/70 transition-colors mb-6 cursor-pointer bg-transparent border-0 disabled:cursor-not-allowed"
      >
        {starting ? "Iniciando..." : "Gravar reuniao"}
      </button>

      {/* Screen toggle */}
      <label className="flex items-center gap-2.5 cursor-pointer select-none">
        <div className="relative">
          <input
            type="checkbox"
            className="sr-only peer"
            checked={withVideo}
            onChange={(e) => setWithVideo(e.target.checked)}
            disabled={starting}
          />
          <div className="w-10 h-[22px] rounded-full bg-white/5 border border-white/10 peer-checked:bg-brand-500/15 peer-checked:border-brand-500/40 transition-all duration-300 cursor-pointer" />
          <div className="absolute left-[3px] top-[3px] w-4 h-4 rounded-full bg-white/20 peer-checked:bg-brand-500 peer-checked:translate-x-[18px] transition-all duration-300 pointer-events-none peer-checked:shadow-[0_0_12px_rgba(244,63,94,0.6)]" />
        </div>
        <span className="text-xs text-white/25">Gravar tela</span>
      </label>

      {error && <p className="text-red-400/80 text-xs mt-4">{error}</p>}
    </>
  );
}
