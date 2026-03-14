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
        <span className={`status-badge ${meeting.chat_status}`}>
          {meeting.chat_status}
        </span>
      </div>
    </div>
  );
}
