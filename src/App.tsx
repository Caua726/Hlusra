import { useState, useRef, useEffect, useCallback } from "react";
import type { ChatStatus } from "./lib/api";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { motion, AnimatePresence } from "motion/react";
import Gallery from "./components/Gallery";
import RecordButton from "./components/RecordButton";
import MeetingPage from "./components/MeetingPage";
import SettingsPage from "./components/SettingsPage";
import ChatPanel from "./components/ChatPanel";
import ExportDialog from "./components/ExportDialog";
import "./styles/app.css";

type ViewKind = "home" | "recording" | "gallery" | "meeting" | "chat" | "export" | "settings";

interface MeetingCtx {
  hasVideo: boolean;
  hasTranscript: boolean;
  chatStatus: ChatStatus;
  title: string;
}

const HOME_VIEWS: ViewKind[] = ["home", "recording"];

function App() {
  const [view, setView] = useState<ViewKind>("home");
  const galleryKey = useRef(0);
  const meetingId = useRef("");
  const [meetingCtx, setMeetingCtx] = useState<MeetingCtx>({
    hasVideo: false, hasTranscript: false, chatStatus: "not_indexed", title: "",
  });

  const go = useCallback((v: ViewKind, id?: string) => {
    if (id) meetingId.current = id;
    setView(v);
  }, []);

  // Resize window
  useEffect(() => {
    (async () => {
      try {
        const win = getCurrentWindow();
        const [w, h] = HOME_VIEWS.includes(view) ? [800, 400] : [800, 600];
        await win.setSize(new LogicalSize(w, h));
      } catch (e) {
        console.error("[hlusra] Failed to resize window:", e);
      }
    })();
  }, [view]);

  function renderView() {
    switch (view) {
      case "home":
        return (
          <div className="w-full h-full flex">
            {/* Ambient glow */}
            <div className="absolute top-1/2 left-[35%] -translate-x-1/2 -translate-y-1/2 w-[300px] h-[300px] rounded-full bg-brand-500/6 blur-[100px] pointer-events-none" />

            <div className="flex-1 flex flex-col items-center justify-center relative">
              <h1 className="text-xl font-extralight tracking-[0.25em] text-white/40 uppercase mb-10">
                Hlusra
              </h1>
              <RecordButton
                onRecordingStart={() => go("recording")}
                onRecordingDone={() => { galleryKey.current++; go("gallery"); }}
              />
            </div>

            {/* Gallery sidebar */}
            <button
              onClick={() => go("gallery")}
              className="w-48 h-full glass-heavy flex flex-col items-center justify-center gap-3 cursor-pointer group hover:bg-white/[0.06] transition-all duration-300 shrink-0 border-0"
            >
              <div className="w-10 h-10 rounded-xl glass flex items-center justify-center group-hover:scale-110 transition-transform duration-300">
                <svg className="w-5 h-5 text-white/25 group-hover:text-brand-400 transition-colors duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                </svg>
              </div>
              <span className="text-xs text-white/20 group-hover:text-white/50 transition-colors">Galeria</span>
              <span className="text-[10px] text-white/8 group-hover:text-white/15 transition-colors">reuniões</span>
            </button>
          </div>
        );

      case "recording":
        return (
          <div className="w-full h-full flex flex-col items-center justify-center relative">
            <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[250px] h-[250px] rounded-full bg-brand-500/8 blur-[80px] pointer-events-none" />
            <RecordButton
              isRecordingView
              onRecordingStart={() => {}}
              onRecordingDone={() => { galleryKey.current++; go("gallery"); }}
              onCancel={() => go("home")}
            />
          </div>
        );

      case "gallery":
        return (
          <Gallery
            key={galleryKey.current}
            onSelectMeeting={(id) => go("meeting", id)}
            onBack={() => go("home")}
            onSettings={() => go("settings")}
          />
        );

      case "meeting":
        return (
          <MeetingPage
            meetingId={meetingId.current}
            onBack={() => go("gallery")}
            onChat={(ctx) => { setMeetingCtx(ctx); go("chat"); }}
            onExport={(ctx) => { setMeetingCtx(ctx); go("export"); }}
          />
        );

      case "chat":
        return (
          <ChatPanel
            meetingId={meetingId.current}
            chatStatus={meetingCtx.chatStatus}
            meetingTitle={meetingCtx.title}
            onBack={() => go("meeting")}
            onStatusChange={(status) => setMeetingCtx((prev) => ({ ...prev, chatStatus: status }))}
          />
        );

      case "export":
        return (
          <ExportDialog
            meetingId={meetingId.current}
            hasVideo={meetingCtx.hasVideo}
            hasTranscript={meetingCtx.hasTranscript}
            meetingTitle={meetingCtx.title}
            onBack={() => go("meeting")}
          />
        );

      case "settings":
        return <SettingsPage onBack={() => go("gallery")} />;

      default: {
        const _never: never = view;
        throw new Error(`Unknown view: ${_never}`);
      }
    }
  }

  return (
    <div className="w-full h-full relative overflow-hidden">
      <AnimatePresence mode="wait">
        <motion.div
          key={view}
          className="w-full h-full flex flex-col"
          initial={{ opacity: 0, scale: 0.97, y: 8 }}
          animate={{ opacity: 1, scale: 1, y: 0 }}
          exit={{ opacity: 0, scale: 0.97, y: -8 }}
          transition={{ duration: 0.2, ease: [0.16, 1, 0.3, 1] }}
        >
          {renderView()}
        </motion.div>
      </AnimatePresence>
    </div>
  );
}

export default App;
