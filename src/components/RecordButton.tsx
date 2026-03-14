import { useState, useEffect, useRef, useCallback } from "react";
import { startRecording, stopRecording, getRecordingStatus } from "../lib/api";
import { formatTimer } from "../lib/format";

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
      setError(String(e));
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
      setError(String(e));
    } finally {
      setStopping(false);
    }
  }

  if (recording) {
    return (
      <div className="record-active">
        <div className="record-indicator">
          <span className="record-dot" />
          <span className="record-label">Gravando</span>
        </div>
        <div className="record-timer">{formatTimer(elapsed)}</div>
        <button
          className="btn-stop"
          onClick={handleStop}
          disabled={stopping}
        >
          {stopping ? "Parando..." : "Parar"}
        </button>
        {error && <p className="error-text">{error}</p>}
      </div>
    );
  }

  return (
    <div className="home-actions">
      <button
        className="btn-primary btn-large"
        onClick={handleStart}
        disabled={starting}
      >
        {starting ? "Iniciando..." : "Começar a gravar"}
      </button>
      <label className="toggle">
        <input
          type="checkbox"
          checked={withVideo}
          onChange={(e) => setWithVideo(e.target.checked)}
          disabled={starting}
        />
        <span>Gravar tela</span>
      </label>
      {error && <p className="error-text">{error}</p>}
    </div>
  );
}
