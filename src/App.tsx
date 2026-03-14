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
    <div className="app">
      {/* Nav bar */}
      <nav className="nav-bar">
        <button
          className={`nav-link ${view.kind === "home" ? "active" : ""}`}
          onClick={() => setView({ kind: "home" })}
        >
          Início
        </button>
        <button
          className={`nav-link ${view.kind === "gallery" ? "active" : ""}`}
          onClick={() => setView({ kind: "gallery" })}
        >
          Galeria
        </button>
        <button
          className={`nav-link ${view.kind === "settings" ? "active" : ""}`}
          onClick={() => setView({ kind: "settings" })}
        >
          Configurações
        </button>
      </nav>

      {/* Views */}
      {view.kind === "home" && (
        <div className="home">
          <h1>Hlusra</h1>
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
