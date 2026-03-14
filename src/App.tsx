import { useState } from "react";
import Gallery from "./components/Gallery";
import "./styles/app.css";

type View = { kind: "home" } | { kind: "gallery" } | { kind: "meeting"; id: string };

function App() {
  const [view, setView] = useState<View>({ kind: "home" });

  return (
    <div className="app">
      {view.kind === "home" && (
        <div className="home">
          <h1>Hlusra</h1>
          <div className="home-actions">
            <button className="btn-primary btn-large" disabled>
              Comecar a gravar
            </button>
            <label className="toggle">
              <input type="checkbox" disabled />
              <span>Gravar tela</span>
            </label>
          </div>
          <button className="btn-secondary" onClick={() => setView({ kind: "gallery" })}>
            Galeria
          </button>
        </div>
      )}

      {view.kind === "gallery" && (
        <div>
          <button className="btn-back" onClick={() => setView({ kind: "home" })}>
            &larr; Voltar
          </button>
          <Gallery onSelectMeeting={(id) => setView({ kind: "meeting", id })} />
        </div>
      )}

      {view.kind === "meeting" && (
        <div>
          <button className="btn-back" onClick={() => setView({ kind: "gallery" })}>
            &larr; Voltar
          </button>
          <p>Meeting: {view.id}</p>
        </div>
      )}
    </div>
  );
}

export default App;
