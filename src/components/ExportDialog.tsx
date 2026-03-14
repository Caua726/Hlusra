import { useState } from "react";
import { save } from "@tauri-apps/plugin-dialog";
import {
  exportAudio,
  exportVideo,
  exportTranscript,
} from "../lib/api";
import type { AudioFormat, VideoFormat, TranscriptFormat, SaveMode } from "../lib/api";

interface Props {
  meetingId: string;
  hasVideo: boolean;
  hasTranscript: boolean;
  onClose: () => void;
}

type ExportTab = "audio" | "video" | "transcript";

const AUDIO_FORMATS: { value: AudioFormat; label: string }[] = [
  { value: "mp3", label: "MP3" },
  { value: "wav", label: "WAV" },
  { value: "opus", label: "Opus" },
  { value: "ogg", label: "OGG" },
];

const VIDEO_FORMATS: { value: VideoFormat; label: string }[] = [
  { value: "mp4_h264", label: "MP4 (H.264)" },
  { value: "mp4_h265", label: "MP4 (H.265)" },
  { value: "mkv_h264", label: "MKV (H.264)" },
  { value: "mkv_h265", label: "MKV (H.265)" },
];

const TRANSCRIPT_FORMATS: { value: TranscriptFormat; label: string }[] = [
  { value: "txt", label: "TXT" },
  { value: "json", label: "JSON" },
  { value: "srt", label: "SRT" },
  { value: "pdf", label: "PDF" },
];

function getExtension(tab: ExportTab, format: string): string {
  if (tab === "audio") return format === "opus" ? "opus" : format;
  if (tab === "video") return format.startsWith("mp4") ? "mp4" : "mkv";
  return format;
}

export default function ExportDialog({ meetingId, hasVideo, hasTranscript, onClose }: Props) {
  const [tab, setTab] = useState<ExportTab>("audio");
  const [audioFormat, setAudioFormat] = useState<AudioFormat>("mp3");
  const [videoFormat, setVideoFormat] = useState<VideoFormat>("mp4_h264");
  const [transcriptFormat, setTranscriptFormat] = useState<TranscriptFormat>("txt");
  const [exporting, setExporting] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  function getCurrentFormat(): string {
    if (tab === "audio") return audioFormat;
    if (tab === "video") return videoFormat;
    return transcriptFormat;
  }

  async function doExport(saveMode: SaveMode) {
    setError(null);
    setResult(null);
    setExporting(true);
    try {
      let path: string;
      if (tab === "audio") {
        path = await exportAudio(meetingId, audioFormat, saveMode);
      } else if (tab === "video") {
        path = await exportVideo(meetingId, videoFormat, saveMode);
      } else {
        path = await exportTranscript(meetingId, transcriptFormat, saveMode);
      }
      setResult(path);
    } catch (e) {
      setError(String(e));
    } finally {
      setExporting(false);
    }
  }

  async function handleSave() {
    await doExport({ mode: "save" });
  }

  async function handleSaveAs() {
    const ext = getExtension(tab, getCurrentFormat());
    const filePath = await save({
      filters: [{ name: ext.toUpperCase(), extensions: [ext] }],
    });
    if (!filePath) return;
    await doExport({ mode: "save_as", path: filePath });
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Exportar</h2>
          <button className="modal-close" onClick={onClose}>&times;</button>
        </div>

        <div className="export-tabs">
          <button
            className={`export-tab ${tab === "audio" ? "active" : ""}`}
            onClick={() => setTab("audio")}
          >
            Áudio
          </button>
          {hasVideo && (
            <button
              className={`export-tab ${tab === "video" ? "active" : ""}`}
              onClick={() => setTab("video")}
            >
              Vídeo
            </button>
          )}
          {hasTranscript && (
            <button
              className={`export-tab ${tab === "transcript" ? "active" : ""}`}
              onClick={() => setTab("transcript")}
            >
              Transcrição
            </button>
          )}
        </div>

        <div className="export-options">
          {tab === "audio" && (
            <div className="format-list">
              {AUDIO_FORMATS.map((f) => (
                <label key={f.value} className="format-option">
                  <input
                    type="radio"
                    name="audio-format"
                    checked={audioFormat === f.value}
                    onChange={() => setAudioFormat(f.value)}
                  />
                  <span>{f.label}</span>
                </label>
              ))}
            </div>
          )}

          {tab === "video" && (
            <div className="format-list">
              {VIDEO_FORMATS.map((f) => (
                <label key={f.value} className="format-option">
                  <input
                    type="radio"
                    name="video-format"
                    checked={videoFormat === f.value}
                    onChange={() => setVideoFormat(f.value)}
                  />
                  <span>{f.label}</span>
                </label>
              ))}
            </div>
          )}

          {tab === "transcript" && (
            <div className="format-list">
              {TRANSCRIPT_FORMATS.map((f) => (
                <label key={f.value} className="format-option">
                  <input
                    type="radio"
                    name="transcript-format"
                    checked={transcriptFormat === f.value}
                    onChange={() => setTranscriptFormat(f.value)}
                  />
                  <span>{f.label}</span>
                </label>
              ))}
            </div>
          )}
        </div>

        <div className="export-actions">
          <button className="btn-primary" onClick={handleSave} disabled={exporting}>
            {exporting ? "Exportando..." : "Salvar"}
          </button>
          <button className="btn-secondary" onClick={handleSaveAs} disabled={exporting}>
            Salvar como...
          </button>
        </div>

        {result && <p className="export-result">Salvo em: {result}</p>}
        {error && <p className="error-text">{error}</p>}
      </div>
    </div>
  );
}
