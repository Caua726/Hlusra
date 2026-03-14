import { MeetingSummary } from "../lib/api";
import { formatDuration } from "../lib/format";

interface Props {
  meeting: MeetingSummary;
  onClick: (id: string) => void;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(0)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

const TRANSCRIPTION_BADGE: Record<string, { label: string; cls: string }> = {
  pending: { label: "Pendente", cls: "bg-amber-500/10 text-amber-400" },
  processing: { label: "Processando", cls: "bg-blue-500/10 text-blue-400" },
  done: { label: "Transcrita", cls: "bg-emerald-500/10 text-emerald-400" },
  failed: { label: "Falhou", cls: "bg-red-500/10 text-red-400" },
};

const CHAT_BADGE: Record<string, { label: string; cls: string }> = {
  not_indexed: { label: "", cls: "" },
  indexing: { label: "Indexando", cls: "bg-blue-500/10 text-blue-400" },
  ready: { label: "Chat", cls: "bg-emerald-500/10 text-emerald-400" },
  failed: { label: "Chat falhou", cls: "bg-red-500/10 text-red-400" },
};

// Simple waveform visualization for audio cards
function AudioWaveform() {
  const heights = [30, 60, 45, 80, 55, 90, 40, 70, 50, 85, 35, 65];
  return (
    <div className="flex items-end gap-[3px] h-8 opacity-15 group-hover:opacity-30 transition-opacity">
      {heights.map((h, i) => (
        <div key={i} className="w-1 bg-white rounded-full" style={{ height: `${h}%` }} />
      ))}
    </div>
  );
}

export default function MeetingCard({ meeting, onClick }: Props) {
  const trans = TRANSCRIPTION_BADGE[meeting.transcription_status];
  const chat = CHAT_BADGE[meeting.chat_status];

  return (
    <div
      className="glass-card rounded-2xl overflow-hidden cursor-pointer group hover:border-white/15 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-black/30 stagger"
      onClick={() => onClick(meeting.id)}
      onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); onClick(meeting.id); } }}
      role="button"
      tabIndex={0}
    >
      {/* Preview area */}
      <div className={`aspect-[16/10] relative flex items-center justify-center ${
        meeting.has_video
          ? "bg-gradient-to-br from-white/[0.03] to-transparent"
          : "bg-gradient-to-r from-white/[0.01] via-white/[0.04] to-white/[0.01]"
      }`}>
        {meeting.has_video ? (
          <svg className="w-8 h-8 text-white/8 group-hover:text-white/20 group-hover:scale-110 transition-all duration-300" fill="currentColor" viewBox="0 0 24 24">
            <path d="M8 5v14l11-7z" />
          </svg>
        ) : (
          <AudioWaveform />
        )}

        {/* Type badge */}
        <div className="absolute top-2.5 left-2.5">
          <span className={`text-[8px] font-bold px-1.5 py-0.5 rounded-md backdrop-blur-sm uppercase tracking-wider ${
            meeting.has_video
              ? "bg-blue-500/15 text-blue-400"
              : "bg-white/6 text-white/40"
          }`}>
            {meeting.has_video ? "Vídeo" : "Áudio"}
          </span>
        </div>

        {/* Duration badge */}
        <div className="absolute bottom-2.5 right-2.5">
          <span className="text-[9px] font-mono text-white/30 bg-black/40 backdrop-blur-sm px-1.5 py-0.5 rounded-md">
            {formatDuration(meeting.duration_secs)}
          </span>
        </div>
      </div>

      {/* Info */}
      <div className="p-3.5">
        <h3 className="text-[13px] font-medium text-white/80 truncate group-hover:text-white transition-colors">
          {meeting.title}
        </h3>
        <div className="flex justify-between mt-1.5 text-[10px] text-white/30">
          <span>{new Date(meeting.created_at).toLocaleDateString("pt-BR")}</span>
          <span>{formatSize(meeting.file_size)}</span>
        </div>
        <div className="flex gap-1.5 mt-2.5">
          {trans && trans.label && (
            <span className={`text-[8px] px-2 py-0.5 rounded-md font-semibold ${trans.cls}`}>
              {trans.label}
            </span>
          )}
          {chat && chat.label && (
            <span className={`text-[8px] px-2 py-0.5 rounded-md font-semibold ${chat.cls}`}>
              {chat.label}
            </span>
          )}
        </div>
      </div>
    </div>
  );
}
