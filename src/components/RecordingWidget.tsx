import { useState, useEffect, useRef } from "react";
import { getRecordingStatus, stopRecording } from "../lib/api";
import { formatTimer } from "../lib/format";

export default function RecordingWidget() {
  const [elapsed, setElapsed] = useState(0);
  const [stopping, setStopping] = useState(false);
  const mountedRef = useRef(true);

  useEffect(() => {
    mountedRef.current = true;

    const poll = setInterval(async () => {
      try {
        const status = await getRecordingStatus();
        if (!mountedRef.current) return;
        if (status.state === "recording") {
          setElapsed(status.duration_secs);
        }
      } catch {
        // polling error is non-fatal
      }
    }, 1000);

    return () => {
      mountedRef.current = false;
      clearInterval(poll);
    };
  }, []);

  async function handleStop() {
    setStopping(true);
    try {
      await stopRecording();
    } catch (e) {
      console.error("[widget] stop failed:", e);
    } finally {
      if (mountedRef.current) setStopping(false);
    }
  }

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        height: "100vh",
        padding: "0 16px",
        background: "#000",
        color: "#fff",
        fontFamily: "Inter, system-ui, sans-serif",
        userSelect: "none",
      }}
    >
      {/* Pulsing red dot + timer */}
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <div
          style={{
            width: 8,
            height: 8,
            borderRadius: "50%",
            background: "#f43f5e",
            animation: "pulse 1.2s ease-in-out infinite",
          }}
        />
        <span
          style={{
            fontFamily: "'JetBrains Mono', monospace",
            fontSize: 18,
            fontWeight: 300,
            letterSpacing: "0.08em",
            fontVariantNumeric: "tabular-nums",
          }}
        >
          {formatTimer(elapsed)}
        </span>
      </div>

      {/* Stop button */}
      <button
        onClick={handleStop}
        disabled={stopping}
        style={{
          background: "rgba(244, 63, 94, 0.15)",
          border: "1px solid rgba(244, 63, 94, 0.3)",
          color: "#fb7185",
          borderRadius: 8,
          padding: "4px 14px",
          fontSize: 12,
          cursor: stopping ? "not-allowed" : "pointer",
          opacity: stopping ? 0.5 : 1,
          transition: "all 0.2s",
          fontFamily: "Inter, system-ui, sans-serif",
        }}
      >
        {stopping ? "..." : "Parar"}
      </button>

      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; transform: scale(1); }
          50% { opacity: 0.3; transform: scale(1.3); }
        }
      `}</style>
    </div>
  );
}
