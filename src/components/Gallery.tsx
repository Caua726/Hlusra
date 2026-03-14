import { useState, useEffect, useCallback } from "react";
import { MeetingSummary, listMeetings } from "../lib/api";
import MeetingCard from "./MeetingCard";

interface Props {
  onSelectMeeting: (id: string) => void;
}

export default function Gallery({ onSelectMeeting }: Props) {
  const [meetings, setMeetings] = useState<MeetingSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadMeetings = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const list = await listMeetings();
      setMeetings(list);
    } catch (err) {
      console.error("Failed to load meetings:", err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadMeetings();
  }, [loadMeetings]);

  if (loading) return <div className="loading">Carregando...</div>;

  return (
    <div className="gallery">
      <h2>Reuniões</h2>
      {error ? (
        <div className="gallery-error">
          <p className="error-text">{error}</p>
          <button className="btn-secondary" onClick={loadMeetings}>
            Tentar novamente
          </button>
        </div>
      ) : meetings.length === 0 ? (
        <p className="empty">Nenhuma reunião gravada ainda.</p>
      ) : (
        <div className="meeting-grid">
          {meetings.map((m) => (
            <MeetingCard key={m.id} meeting={m} onClick={onSelectMeeting} />
          ))}
        </div>
      )}
    </div>
  );
}
