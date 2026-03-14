import { MeetingSummary } from "../lib/api";

interface Props {
  meeting: MeetingSummary;
  onClick: (id: string) => void;
}

function formatDuration(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m ${s}s`;
  return `${s}s`;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

export default function MeetingCard({ meeting, onClick }: Props) {
  return (
    <div className="meeting-card" onClick={() => onClick(meeting.id)}>
      <div className="meeting-card-header">
        <span className="meeting-type">{meeting.has_video ? "Video" : "Audio"}</span>
        <span className="meeting-duration">{formatDuration(meeting.duration_secs)}</span>
      </div>
      <h3 className="meeting-title">{meeting.title}</h3>
      <div className="meeting-meta">
        <span>{new Date(meeting.created_at).toLocaleDateString()}</span>
        <span>{formatSize(meeting.file_size)}</span>
      </div>
      <div className="meeting-status">
        <span className={`status-badge ${meeting.transcription_status}`}>
          {meeting.transcription_status}
        </span>
      </div>
    </div>
  );
}
