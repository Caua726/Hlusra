import { useState, useEffect, useCallback } from "react";
import { MeetingSummary, listMeetings } from "../lib/api";
import MeetingCard from "./MeetingCard";

interface Props {
  onSelectMeeting: (id: string) => void;
  onBack: () => void;
  onSettings: () => void;
}

function formatTotalSize(meetings: MeetingSummary[]): string {
  const total = meetings.reduce((acc, m) => acc + m.file_size, 0);
  if (total < 1024) return `${total} B`;
  if (total < 1024 * 1024) return `${(total / 1024).toFixed(0)} KB`;
  if (total < 1024 * 1024 * 1024) return `${(total / (1024 * 1024)).toFixed(0)} MB`;
  return `${(total / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

export default function Gallery({ onSelectMeeting, onBack, onSettings }: Props) {
  const [meetings, setMeetings] = useState<MeetingSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");

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

  const filtered = search.trim()
    ? meetings.filter((m) => m.title.toLowerCase().includes(search.toLowerCase()))
    : meetings;

  return (
    <>
      {/* Header */}
      <header className="glass shrink-0 border-b border-white/5">
        <div className="px-5 h-12 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button
              onClick={onBack}
              className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Galeria</h1>
            <span className="text-[10px] text-white/20">{meetings.length} reunioes</span>
          </div>
          <div className="relative">
            <input
              type="text"
              placeholder="Buscar..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="glass-input rounded-lg pl-7 pr-3 py-1.5 text-xs text-white/60 placeholder-white/20 w-40 focus:w-52 focus:outline-none focus:border-white/20 transition-all duration-300"
            />
            <svg className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-5">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <div className="w-6 h-6 border-[3px] border-white/10 border-t-brand-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
          </div>
        ) : error ? (
          <div className="text-center p-8 flex flex-col items-center gap-4">
            <p className="text-red-500 text-sm">{error}</p>
            <button
              className="glass-heavy px-4 py-2 text-[11px] rounded-xl text-white/40 hover:text-white/70 transition-all cursor-pointer border-0"
              onClick={loadMeetings}
            >
              Tentar novamente
            </button>
          </div>
        ) : filtered.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <p className="text-white/20 text-sm">
              {search ? "Nenhuma reuniao encontrada." : "Nenhuma reuniao gravada ainda."}
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-3 gap-4">
            {filtered.map((m) => (
              <MeetingCard key={m.id} meeting={m} onClick={onSelectMeeting} />
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      <footer className="glass shrink-0 border-t border-white/5">
        <div className="px-5 h-10 flex items-center justify-between">
          <span className="text-[10px] text-white/15">
            {meetings.length} reunioes &middot; {formatTotalSize(meetings)}
          </span>
          <button
            onClick={onSettings}
            className="text-white/20 hover:text-white/50 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </button>
        </div>
      </footer>
    </>
  );
}
