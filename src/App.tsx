import { useState, useRef, useEffect, useCallback } from "react";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import Gallery from "./components/Gallery";
import RecordButton from "./components/RecordButton";
import MeetingPage from "./components/MeetingPage";
import SettingsPage from "./components/SettingsPage";
import ChatPanel from "./components/ChatPanel";
import ExportDialog from "./components/ExportDialog";
import "./styles/app.css";

type ViewKind = "home" | "recording" | "gallery" | "meeting" | "chat" | "export" | "settings";
type MorphDirection = "left" | "right" | "center" | "bottom";

interface ViewState {
  kind: ViewKind;
  meetingId?: string;
  direction: MorphDirection;
}

const HOME_VIEWS: ViewKind[] = ["home", "recording"];

function App() {
  const [view, setView] = useState<ViewState>({ kind: "home", direction: "center" });
  const galleryRefreshRef = useRef(0);
  const meetingIdRef = useRef<string>("");
  // Store meeting context for chat/export
  const [meetingContext, setMeetingContext] = useState<{
    hasVideo: boolean;
    hasTranscript: boolean;
    chatStatus: "not_indexed" | "indexing" | "ready" | "failed";
    title: string;
  }>({ hasVideo: false, hasTranscript: false, chatStatus: "not_indexed", title: "" });

  const go = useCallback((kind: ViewKind, direction: MorphDirection, meetingId?: string) => {
    if (meetingId) meetingIdRef.current = meetingId;
    setView({ kind, direction, meetingId: meetingId || meetingIdRef.current });
  }, []);

  // Resize window when switching between home-sized and expanded views
  useEffect(() => {
    async function resize() {
      try {
        const win = getCurrentWindow();
        const isHome = HOME_VIEWS.includes(view.kind);
        const size = isHome ? new LogicalSize(800, 400) : new LogicalSize(800, 600);
        await win.setSize(size);
      } catch {
        // Silently fail in dev/browser mode
      }
    }
    resize();
  }, [view.kind]);

  return (
    <div className="view-container">
      {/* HOME */}
      <div className={`view ${view.kind === "home" ? "active" : ""} morph-from-${view.kind === "home" ? view.direction : "center"}`}
        style={{ display: "flex" }}
      >
        {/* Ambient glow */}
        <div className="absolute top-1/2 left-[35%] -translate-x-1/2 -translate-y-1/2 w-[300px] h-[300px] rounded-full bg-brand-500/6 blur-[100px] pointer-events-none" />

        <div className="flex-1 flex flex-col items-center justify-center relative">
          <div className="mb-10 stagger">
            <h1 className="text-xl font-extralight tracking-[0.25em] text-white/40 uppercase">Hlusra</h1>
          </div>

          <RecordButton
            onRecordingStart={() => go("recording", "center")}
            onRecordingDone={() => {
              galleryRefreshRef.current += 1;
              go("gallery", "right");
            }}
          />
        </div>

        {/* Gallery sidebar button */}
        <button
          onClick={() => go("gallery", "right")}
          className="w-48 h-full glass-heavy flex flex-col items-center justify-center gap-3 cursor-pointer group hover:bg-white/[0.06] transition-all duration-300 shrink-0 stagger border-0"
        >
          <div className="w-10 h-10 rounded-xl glass flex items-center justify-center group-hover:scale-110 transition-transform duration-300">
            <svg className="w-5 h-5 text-white/25 group-hover:text-brand-400 transition-colors duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
          </div>
          <span className="text-xs text-white/20 group-hover:text-white/50 transition-colors">Galeria</span>
        </button>
      </div>

      {/* RECORDING */}
      <div className={`view ${view.kind === "recording" ? "active" : ""} morph-from-center`}
        style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center" }}
      >
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[250px] h-[250px] rounded-full bg-brand-500/8 blur-[80px] pointer-events-none" />
        {/* RecordButton handles the recording UI internally */}
        {view.kind === "recording" && (
          <RecordButton
            isRecordingView
            onRecordingStart={() => {}}
            onRecordingDone={() => {
              galleryRefreshRef.current += 1;
              go("gallery", "right");
            }}
          />
        )}
      </div>

      {/* GALLERY */}
      <div className={`view ${view.kind === "gallery" ? "active" : ""} morph-from-${view.kind === "gallery" ? view.direction : "right"}`}
        style={{ display: "flex", flexDirection: "column" }}
      >
        <Gallery
          key={galleryRefreshRef.current}
          onSelectMeeting={(id) => go("meeting", "center", id)}
          onBack={() => go("home", "left")}
          onSettings={() => go("settings", "bottom")}
        />
      </div>

      {/* MEETING */}
      <div className={`view ${view.kind === "meeting" ? "active" : ""} morph-from-${view.kind === "meeting" ? view.direction : "center"}`}
        style={{ display: "flex", flexDirection: "column" }}
      >
        {(view.kind === "meeting" || view.kind === "chat" || view.kind === "export") && meetingIdRef.current && (
          <MeetingPage
            meetingId={meetingIdRef.current}
            onBack={() => go("gallery", "left")}
            onChat={(ctx) => {
              setMeetingContext(ctx);
              go("chat", "bottom");
            }}
            onExport={(ctx) => {
              setMeetingContext(ctx);
              go("export", "bottom");
            }}
          />
        )}
      </div>

      {/* CHAT */}
      <div className={`view ${view.kind === "chat" ? "active" : ""} morph-from-bottom`}
        style={{ display: "flex", flexDirection: "column" }}
      >
        {view.kind === "chat" && (
          <ChatPanel
            meetingId={meetingIdRef.current}
            chatStatus={meetingContext.chatStatus}
            meetingTitle={meetingContext.title}
            onBack={() => go("meeting", "left")}
            onStatusChange={() => {}}
          />
        )}
      </div>

      {/* EXPORT */}
      <div className={`view ${view.kind === "export" ? "active" : ""} morph-from-bottom`}
        style={{ display: "flex", flexDirection: "column" }}
      >
        {view.kind === "export" && (
          <ExportDialog
            meetingId={meetingIdRef.current}
            hasVideo={meetingContext.hasVideo}
            hasTranscript={meetingContext.hasTranscript}
            meetingTitle={meetingContext.title}
            onBack={() => go("meeting", "left")}
          />
        )}
      </div>

      {/* SETTINGS */}
      <div className={`view ${view.kind === "settings" ? "active" : ""} morph-from-bottom`}
        style={{ display: "flex", flexDirection: "column" }}
      >
        {view.kind === "settings" && (
          <SettingsPage onBack={() => go("gallery", "left")} />
        )}
      </div>
    </div>
  );
}

export default App;
