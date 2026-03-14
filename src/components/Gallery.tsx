import { useState, useEffect } from "react";
import { MeetingSummary, listMeetings } from "../lib/api";
import MeetingCard from "./MeetingCard";

interface Props {
  onSelectMeeting: (id: string) => void;
}

export default function Gallery({ onSelectMeeting }: Props) {
  const [meetings, setMeetings] = useState<MeetingSummary[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadMeetings();
  }, []);

  async function loadMeetings() {
    try {
      const list = await listMeetings();
      setMeetings(list);
    } catch (err) {
      console.error("Failed to load meetings:", err);
    } finally {
      setLoading(false);
    }
  }

  if (loading) return <div className="loading">Loading...</div>;

  return (
    <div className="gallery">
      <h2>Reunioes</h2>
      {meetings.length === 0 ? (
        <p className="empty">Nenhuma reuniao gravada ainda.</p>
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
