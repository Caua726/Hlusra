import { useState, useRef } from "react";
import Gallery from "./components/Gallery";
import RecordButton from "./components/RecordButton";
import MeetingPage from "./components/MeetingPage";
import SettingsPage from "./components/SettingsPage";
import "./styles/app.css";

type View =
  | { kind: "home" }
  | { kind: "gallery" }
  | { kind: "meeting"; id: string }
  | { kind: "settings" };

function App() {
  const [view, setView] = useState<View>({ kind: "home" });
  const galleryRefreshRef = useRef(0);

  return (
    <div className="max-w-[960px] mx-auto px-6 pb-12">
      {/* Nav bar */}
      <nav className="flex gap-1 py-3 border-b border-zinc-800 mb-6">
        <button
          className={`bg-transparent border-none text-zinc-500 px-4 py-2 text-sm cursor-pointer rounded-lg transition-colors duration-150 hover:text-zinc-100 hover:bg-zinc-900 ${view.kind === "home" ? "text-zinc-100 bg-zinc-800" : ""}`}
          onClick={() => setView({ kind: "home" })}
        >
          Inicio
        </button>
        <button
          className={`bg-transparent border-none text-zinc-500 px-4 py-2 text-sm cursor-pointer rounded-lg transition-colors duration-150 hover:text-zinc-100 hover:bg-zinc-900 ${view.kind === "gallery" ? "text-zinc-100 bg-zinc-800" : ""}`}
          onClick={() => setView({ kind: "gallery" })}
        >
          Galeria
        </button>
        <button
          className={`bg-transparent border-none text-zinc-500 px-4 py-2 text-sm cursor-pointer rounded-lg transition-colors duration-150 hover:text-zinc-100 hover:bg-zinc-900 ${view.kind === "settings" ? "text-zinc-100 bg-zinc-800" : ""}`}
          onClick={() => setView({ kind: "settings" })}
        >
          Configuracoes
        </button>
      </nav>

      {/* Views */}
      {view.kind === "home" && (
        <div className="flex flex-col items-center justify-center min-h-[70vh] gap-8">
          <h1 className="text-5xl font-bold text-rose-500 tracking-tight">Hlusra</h1>
          <RecordButton
            onRecordingDone={() => {
              galleryRefreshRef.current += 1;
              setView({ kind: "gallery" });
            }}
          />
        </div>
      )}

      {view.kind === "gallery" && (
        <Gallery
          key={galleryRefreshRef.current}
          onSelectMeeting={(id) => setView({ kind: "meeting", id })}
        />
      )}

      {view.kind === "meeting" && (
        <MeetingPage
          meetingId={view.id}
          onBack={() => setView({ kind: "gallery" })}
        />
      )}

      {view.kind === "settings" && (
        <SettingsPage onBack={() => setView({ kind: "home" })} />
      )}
    </div>
  );
}

export default App;
