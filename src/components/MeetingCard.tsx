import { MeetingSummary } from "../lib/api";
import { formatDuration } from "../lib/format";

interface Props {
  meeting: MeetingSummary;
  onClick: (id: string) => void;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

const TRANSCRIPTION_STATUS_LABEL: Record<string, string> = {
  pending: "Pendente",
  processing: "Processando",
  done: "Concluida",
  failed: "Falhou",
};

const CHAT_STATUS_LABEL: Record<string, string> = {
  not_indexed: "Nao indexado",
  indexing: "Indexando",
  ready: "Pronto",
  failed: "Falhou",
};

const STATUS_BADGE_CLASSES: Record<string, string> = {
  pending: "bg-amber-500/15 text-amber-500",
  processing: "bg-blue-500/15 text-blue-500",
  done: "bg-green-500/15 text-green-500",
  failed: "bg-red-500/15 text-red-500",
  not_indexed: "bg-zinc-500/20 text-zinc-500",
  ready: "bg-green-500/15 text-green-500",
  indexing: "bg-blue-500/15 text-blue-500",
};

export default function MeetingCard({ meeting, onClick }: Props) {
  return (
    <div
      className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-5 cursor-pointer transition-all duration-150 hover:border-rose-500 hover:-translate-y-0.5 hover:shadow-lg hover:shadow-black/30"
      onClick={() => onClick(meeting.id)}
      onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); onClick(meeting.id); } }}
      role="button"
      tabIndex={0}
    >
      <div className="flex justify-between items-center mb-2.5">
        <span className="text-[0.7rem] uppercase text-rose-500 font-semibold tracking-wider">{meeting.has_video ? "Video" : "Audio"}</span>
        <span className="text-xs text-zinc-500">{formatDuration(meeting.duration_secs)}</span>
      </div>
      <h3 className="text-base mb-2.5 font-medium">{meeting.title}</h3>
      <div className="flex justify-between text-xs text-zinc-500 mb-2.5">
        <span>{new Date(meeting.created_at).toLocaleDateString("pt-BR")}</span>
        <span>{formatSize(meeting.file_size)}</span>
      </div>
      <div className="flex gap-2">
        <span className={`text-[0.65rem] px-2 py-0.5 rounded uppercase font-semibold tracking-wider ${STATUS_BADGE_CLASSES[meeting.transcription_status] ?? ""}`}>
          {TRANSCRIPTION_STATUS_LABEL[meeting.transcription_status] ?? meeting.transcription_status}
        </span>
        <span className={`text-[0.65rem] px-2 py-0.5 rounded uppercase font-semibold tracking-wider ${STATUS_BADGE_CLASSES[meeting.chat_status] ?? ""}`}>
          {CHAT_STATUS_LABEL[meeting.chat_status] ?? meeting.chat_status}
        </span>
      </div>
    </div>
  );
}
