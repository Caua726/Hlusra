import { useState, useEffect, useRef, useCallback } from "react";
import { startRecording, stopRecording, getRecordingStatus } from "../lib/api";
import { formatTimer, formatError } from "../lib/format";

interface Props {
  onRecordingDone: () => void;
}

export default function RecordButton({ onRecordingDone }: Props) {
  const [recording, setRecording] = useState(false);
  const [withVideo, setWithVideo] = useState(false);
  const [elapsed, setElapsed] = useState(0);
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

  async function handleStart() {
    setError(null);
    setStarting(true);
    try {
      await startRecording(withVideo);
      setRecording(true);
      setElapsed(0);

      pollRef.current = setInterval(async () => {
        try {
          const status = await getRecordingStatus();
          if (status.state === "recording") {
            setElapsed(status.duration_secs);
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

  if (recording) {
    return (
      <div className="flex flex-col items-center gap-6">
        <div className="flex items-center gap-2.5">
          <span className="w-3.5 h-3.5 bg-red-500 rounded-full animate-[pulse-dot_1.2s_ease-in-out_infinite]" />
          <span className="text-base font-semibold text-red-500 uppercase tracking-wider">Gravando</span>
        </div>
        <div className="text-4xl font-bold tabular-nums tracking-wider">{formatTimer(elapsed)}</div>
        <button
          className="bg-red-500 text-white border-none px-10 py-3 rounded-lg text-lg font-semibold cursor-pointer transition-colors duration-150 hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={handleStop}
          disabled={stopping}
        >
          {stopping ? "Parando..." : "Parar"}
        </button>
        {error && <p className="text-red-500 text-sm mt-2">{error}</p>}
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center gap-4">
      <button
        className="bg-rose-500 text-white border-none px-12 py-4 rounded-lg text-lg cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed"
        onClick={handleStart}
        disabled={starting}
      >
        {starting ? "Iniciando..." : "Comecar a gravar"}
      </button>
      <label className="flex items-center gap-2 text-zinc-500 text-sm cursor-pointer">
        <input
          type="checkbox"
          checked={withVideo}
          onChange={(e) => setWithVideo(e.target.checked)}
          disabled={starting}
          className="accent-rose-500 w-4 h-4"
        />
        <span>Gravar tela</span>
      </label>
      {error && <p className="text-red-500 text-sm mt-2">{error}</p>}
    </div>
  );
}
