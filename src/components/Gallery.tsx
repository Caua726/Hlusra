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

  if (loading) return <div className="text-center p-12 text-zinc-500">Carregando...</div>;

  return (
    <div className="py-2">
      <h2 className="mb-5 text-2xl font-semibold">Reunioes</h2>
      {error ? (
        <div className="text-center p-8 flex flex-col items-center gap-4">
          <p className="text-red-500 text-sm">{error}</p>
          <button
            className="bg-transparent text-zinc-100 border border-zinc-700 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:border-zinc-500 hover:bg-zinc-900"
            onClick={loadMeetings}
          >
            Tentar novamente
          </button>
        </div>
      ) : meetings.length === 0 ? (
        <p className="text-center p-12 text-zinc-500">Nenhuma reuniao gravada ainda.</p>
      ) : (
        <div className="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-4">
          {meetings.map((m) => (
            <MeetingCard key={m.id} meeting={m} onClick={onSelectMeeting} />
          ))}
        </div>
      )}
    </div>
  );
}
