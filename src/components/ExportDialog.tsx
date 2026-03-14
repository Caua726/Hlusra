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
      setError(formatError(e));
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
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[100] backdrop-blur-sm" onClick={onClose}>
      <div className="bg-zinc-900 border border-zinc-700 rounded-xl p-7 max-w-[500px] w-[90%] max-h-[85vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex justify-between items-center mb-5">
          <h2 className="text-xl font-semibold">Exportar</h2>
          <button className="bg-transparent border-none text-zinc-500 text-2xl cursor-pointer p-1 leading-none transition-colors duration-150 hover:text-zinc-100" onClick={onClose}>&times;</button>
        </div>

        <div className="flex gap-1 mb-5">
          <button
            className={`border border-zinc-700 px-5 py-2 text-sm cursor-pointer rounded-lg transition-all duration-150 ${tab === "audio" ? "bg-rose-500 border-rose-500 text-white" : "bg-transparent text-zinc-500 hover:border-zinc-600 hover:text-zinc-100"}`}
            onClick={() => setTab("audio")}
          >
            Audio
          </button>
          {hasVideo && (
            <button
              className={`border border-zinc-700 px-5 py-2 text-sm cursor-pointer rounded-lg transition-all duration-150 ${tab === "video" ? "bg-rose-500 border-rose-500 text-white" : "bg-transparent text-zinc-500 hover:border-zinc-600 hover:text-zinc-100"}`}
              onClick={() => setTab("video")}
            >
              Video
            </button>
          )}
          {hasTranscript && (
            <button
              className={`border border-zinc-700 px-5 py-2 text-sm cursor-pointer rounded-lg transition-all duration-150 ${tab === "transcript" ? "bg-rose-500 border-rose-500 text-white" : "bg-transparent text-zinc-500 hover:border-zinc-600 hover:text-zinc-100"}`}
              onClick={() => setTab("transcript")}
            >
              Transcricao
            </button>
          )}
        </div>

        <div className="mb-5">
          {tab === "audio" && (
            <div className="flex flex-col gap-2">
              {AUDIO_FORMATS.map((f) => (
                <label key={f.value} className="flex items-center gap-2.5 px-3 py-2 rounded-lg cursor-pointer transition-colors duration-150 text-sm hover:bg-zinc-800">
                  <input
                    type="radio"
                    name="audio-format"
                    checked={audioFormat === f.value}
                    onChange={() => setAudioFormat(f.value)}
                    className="accent-rose-500"
                  />
                  <span>{f.label}</span>
                </label>
              ))}
            </div>
          )}

          {tab === "video" && (
            <div className="flex flex-col gap-2">
              {VIDEO_FORMATS.map((f) => (
                <label key={f.value} className="flex items-center gap-2.5 px-3 py-2 rounded-lg cursor-pointer transition-colors duration-150 text-sm hover:bg-zinc-800">
                  <input
                    type="radio"
                    name="video-format"
                    checked={videoFormat === f.value}
                    onChange={() => setVideoFormat(f.value)}
                    className="accent-rose-500"
                  />
                  <span>{f.label}</span>
                </label>
              ))}
            </div>
          )}

          {tab === "transcript" && (
            <div className="flex flex-col gap-2">
              {TRANSCRIPT_FORMATS.map((f) => (
                <label key={f.value} className="flex items-center gap-2.5 px-3 py-2 rounded-lg cursor-pointer transition-colors duration-150 text-sm hover:bg-zinc-800">
                  <input
                    type="radio"
                    name="transcript-format"
                    checked={transcriptFormat === f.value}
                    onChange={() => setTranscriptFormat(f.value)}
                    className="accent-rose-500"
                  />
                  <span>{f.label}</span>
                </label>
              ))}
            </div>
          )}
        </div>

        <div className="flex gap-3">
          <button className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleSave} disabled={exporting}>
            {exporting ? "Exportando..." : "Salvar"}
          </button>
          <button className="bg-transparent text-zinc-100 border border-zinc-700 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:border-zinc-500 hover:bg-zinc-900 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleSaveAs} disabled={exporting}>
            Salvar como...
          </button>
        </div>

        {result && <p className="mt-4 text-sm text-green-500 break-all">Salvo em: {result}</p>}
        {error && <p className="text-red-500 text-sm mt-2">{error}</p>}
      </div>
    </div>
  );
}
