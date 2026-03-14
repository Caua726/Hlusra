import { useState } from "react";
import { save } from "@tauri-apps/plugin-dialog";
import {
  exportAudio,
  exportVideo,
  exportTranscript,
} from "../lib/api";
import type { AudioFormat, VideoFormat, TranscriptFormat, SaveMode } from "../lib/api";
import { formatError } from "../lib/format";

interface Props {
  meetingId: string;
  hasVideo: boolean;
  hasTranscript: boolean;
  meetingTitle: string;
  onBack: () => void;
}

type ExportType = "audio" | "video" | "transcript";

interface FormatOption {
  type: ExportType;
  value: string;
  label: string;
}

const AUDIO_FORMATS: FormatOption[] = [
  { type: "audio", value: "mp3", label: "MP3" },
  { type: "audio", value: "wav", label: "WAV" },
  { type: "audio", value: "opus", label: "Opus" },
  { type: "audio", value: "ogg", label: "OGG" },
];

const VIDEO_FORMATS: FormatOption[] = [
  { type: "video", value: "mp4_h264", label: "MP4 H.264" },
  { type: "video", value: "mp4_h265", label: "MP4 H.265" },
  { type: "video", value: "mkv_h264", label: "MKV H.264" },
  { type: "video", value: "mkv_h265", label: "MKV H.265" },
];

const TRANSCRIPT_FORMATS: FormatOption[] = [
  { type: "transcript", value: "txt", label: "TXT" },
  { type: "transcript", value: "json", label: "JSON" },
  { type: "transcript", value: "srt", label: "SRT" },
  { type: "transcript", value: "pdf", label: "PDF" },
];

function getExtension(type: ExportType, format: string): string {
  if (type === "audio") return format === "opus" ? "opus" : format;
  if (type === "video") return format.startsWith("mp4") ? "mp4" : "mkv";
  return format;
}

export default function ExportDialog({ meetingId, hasVideo, hasTranscript, meetingTitle, onBack }: Props) {
  const [selectedType, setSelectedType] = useState<ExportType | null>(null);
  const [selectedFormat, setSelectedFormat] = useState<string | null>(null);
  const [exporting, setExporting] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  function selectFormat(type: ExportType, format: string) {
    setSelectedType(type);
    setSelectedFormat(format);
    setResult(null);
    setError(null);
  }

  async function doExport(saveMode: SaveMode) {
    if (!selectedType || !selectedFormat) return;
    setError(null);
    setResult(null);
    setExporting(true);
    try {
      let path: string;
      if (selectedType === "audio") {
        path = await exportAudio(meetingId, selectedFormat as AudioFormat, saveMode);
      } else if (selectedType === "video") {
        path = await exportVideo(meetingId, selectedFormat as VideoFormat, saveMode);
      } else {
        path = await exportTranscript(meetingId, selectedFormat as TranscriptFormat, saveMode);
      }
      setResult(path);
    } catch (e) {
      setError(formatError(e));
    } finally {
      setExporting(false);
    }
  }

  async function handleSave() {
    await doExport({ mode: "save" });
  }

  async function handleSaveAs() {
    if (!selectedType || !selectedFormat) return;
    const ext = getExtension(selectedType, selectedFormat);
    const filePath = await save({
      filters: [{ name: ext.toUpperCase(), extensions: [ext] }],
    });
    if (!filePath) return;
    await doExport({ mode: "save_as", path: filePath });
  }

  function isSelected(type: ExportType, format: string) {
    return selectedType === type && selectedFormat === format;
  }

  return (
    <>
      {/* Header */}
      <header className="glass shrink-0 border-b border-white/5">
        <div className="px-5 h-12 flex items-center gap-3">
          <button onClick={onBack} className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <h1 className="text-sm font-semibold text-white/80">Exportar</h1>
          <span className="text-[10px] text-white/20">{meetingTitle}</span>
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-5 space-y-5">
        {/* Audio section */}
        <div className="glass-card rounded-2xl p-5 stagger">
          <div className="flex items-center gap-2.5 mb-4">
            <svg className="w-4 h-4 text-white/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z" />
            </svg>
            <h2 className="text-[12px] font-semibold text-white/60 uppercase tracking-wider">Audio</h2>
          </div>
          <div className="grid grid-cols-4 gap-2">
            {AUDIO_FORMATS.map((f) => (
              <button
                key={f.value}
                onClick={() => selectFormat("audio", f.value)}
                className={`glass-input rounded-xl py-2.5 text-[11px] font-medium transition-all active:scale-95 cursor-pointer bg-transparent ${
                  isSelected("audio", f.value)
                    ? "border-brand-500/40 bg-brand-500/10 text-brand-400"
                    : "text-white/50 hover:text-white/80 hover:border-brand-500/30 hover:bg-brand-500/5"
                }`}
              >
                {f.label}
              </button>
            ))}
          </div>
        </div>

        {/* Video section */}
        {hasVideo && (
          <div className="glass-card rounded-2xl p-5 stagger">
            <div className="flex items-center gap-2.5 mb-4">
              <svg className="w-4 h-4 text-white/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              <h2 className="text-[12px] font-semibold text-white/60 uppercase tracking-wider">Video</h2>
            </div>
            <div className="grid grid-cols-4 gap-2">
              {VIDEO_FORMATS.map((f) => (
                <button
                  key={f.value}
                  onClick={() => selectFormat("video", f.value)}
                  className={`glass-input rounded-xl py-2.5 text-[11px] font-medium transition-all active:scale-95 cursor-pointer bg-transparent ${
                    isSelected("video", f.value)
                      ? "border-brand-500/40 bg-brand-500/10 text-brand-400"
                      : "text-white/50 hover:text-white/80 hover:border-brand-500/30 hover:bg-brand-500/5"
                  }`}
                >
                  {f.label}
                </button>
              ))}
            </div>
          </div>
        )}

        {/* Transcript section */}
        {hasTranscript && (
          <div className="glass-card rounded-2xl p-5 stagger">
            <div className="flex items-center gap-2.5 mb-4">
              <svg className="w-4 h-4 text-white/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              <h2 className="text-[12px] font-semibold text-white/60 uppercase tracking-wider">Transcrição</h2>
            </div>
            <div className="grid grid-cols-4 gap-2">
              {TRANSCRIPT_FORMATS.map((f) => (
                <button
                  key={f.value}
                  onClick={() => selectFormat("transcript", f.value)}
                  className={`glass-input rounded-xl py-2.5 text-[11px] font-medium transition-all active:scale-95 cursor-pointer bg-transparent ${
                    isSelected("transcript", f.value)
                      ? "border-brand-500/40 bg-brand-500/10 text-brand-400"
                      : "text-white/50 hover:text-white/80 hover:border-brand-500/30 hover:bg-brand-500/5"
                  }`}
                >
                  {f.label}
                </button>
              ))}
            </div>
          </div>
        )}

        {result && <p className="text-emerald-400 text-[11px] break-all">Salvo em: {result}</p>}
        {error && <p className="text-red-500 text-[11px]">{error}</p>}
      </div>

      {/* Save buttons */}
      <div className="p-5 border-t border-white/5 shrink-0 flex gap-3">
        <button
          onClick={handleSave}
          disabled={exporting || !selectedFormat}
          className="flex-1 py-3 bg-gradient-to-r from-brand-500 to-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm hover:shadow-lg hover:shadow-brand-500/20 border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
        >
          {exporting ? "Exportando..." : "Salvar"}
        </button>
        <button
          onClick={handleSaveAs}
          disabled={exporting || !selectedFormat}
          className="flex-1 py-3 glass-heavy rounded-xl text-[12px] text-white/50 font-medium hover:text-white/80 transition-all active:scale-[0.98] border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
        >
          Salvar como...
        </button>
      </div>
    </>
  );
}
