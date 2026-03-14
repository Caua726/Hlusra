import { useState, useEffect, useRef, useCallback } from "react";
import { readFile } from "@tauri-apps/plugin-fs";
import {
  getMeeting,
  updateMeetingTitle,
  deleteMeeting,
  retranscribeMeeting,
  reindexMeeting,
} from "../lib/api";
import type { MeetingDetail } from "../lib/api";
import { formatDuration, formatError } from "../lib/format";
import TranscriptView from "./TranscriptView";
import ChatPanel from "./ChatPanel";
import ExportDialog from "./ExportDialog";

interface Props {
  meetingId: string;
  onBack: () => void;
}

function formatDate(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleDateString("pt-BR", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export default function MeetingPage({ meetingId, onBack }: Props) {
  const [meeting, setMeeting] = useState<MeetingDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [editingTitle, setEditingTitle] = useState(false);
  const [titleDraft, setTitleDraft] = useState("");
  const [showExport, setShowExport] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [actionBusy, setActionBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const [mediaBlobUrl, setMediaBlobUrl] = useState<string | null>(null);
  const mediaElRef = useRef<HTMLMediaElement | null>(null);
  const setMediaRef = useCallback((el: HTMLMediaElement | null) => {
    mediaElRef.current = el;
  }, []);

  const loadMeeting = useCallback(async () => {
    try {
      const data = await getMeeting(meetingId);
      setMeeting(data);
      setTitleDraft(data.title);
    } catch (e) {
      setError(formatError(e));
    } finally {
      setLoading(false);
    }
  }, [meetingId]);

  useEffect(() => {
    loadMeeting();
  }, [loadMeeting]);

  // Load media file as blob URL to avoid asset protocol issues on Linux/WebKitGTK
  useEffect(() => {
    let revoked = false;
    let url: string | null = null;

    async function loadMedia() {
      if (!meeting || meeting.media_status !== "present") return;
      const mediaPath = meeting.dir_path + "/recording.mkv";
      try {
        const bytes = await readFile(mediaPath);
        if (revoked) return;
        const mimeType = meeting.has_video ? "video/x-matroska" : "audio/x-matroska";
        const blob = new Blob([bytes], { type: mimeType });
        url = URL.createObjectURL(blob);
        setMediaBlobUrl(url);
      } catch (e) {
        console.error("[MeetingPage] failed to load media file:", e);
      }
    }

    loadMedia();
    return () => {
      revoked = true;
      if (url) URL.revokeObjectURL(url);
    };
  }, [meeting?.id, meeting?.media_status]);

  function handleSeek(time: number) {
    if (mediaElRef.current) {
      mediaElRef.current.currentTime = time;
      mediaElRef.current.play().catch(() => {});
    }
  }

  async function handleSaveTitle() {
    if (!meeting) return;
    const trimmed = titleDraft.trim();
    if (!trimmed || trimmed === meeting.title) {
      setEditingTitle(false);
      setTitleDraft(meeting.title);
      return;
    }
    try {
      await updateMeetingTitle(meetingId, trimmed);
      setMeeting({ ...meeting, title: trimmed });
      setEditingTitle(false);
    } catch (e) {
      setActionError(String(e));
    }
  }

  function handleTitleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      handleSaveTitle();
    }
    if (e.key === "Escape") {
      setEditingTitle(false);
      if (meeting) setTitleDraft(meeting.title);
    }
  }

  async function handleDelete(mode: "everything" | "media_only") {
    setActionError(null);
    setActionBusy(true);
    try {
      await deleteMeeting(meetingId, mode);
      if (mode === "everything") {
        onBack();
      } else {
        await loadMeeting();
        setShowDeleteConfirm(false);
      }
    } catch (e) {
      setActionError(String(e));
    } finally {
      setActionBusy(false);
    }
  }

  async function handleRetranscribe() {
    setActionError(null);
    setActionBusy(true);
    try {
      await retranscribeMeeting(meetingId);
      await loadMeeting();
    } catch (e) {
      setActionError(String(e));
    } finally {
      setActionBusy(false);
    }
  }

  async function handleReindex() {
    setActionError(null);
    setActionBusy(true);
    try {
      await reindexMeeting(meetingId);
      await loadMeeting();
    } catch (e) {
      setActionError(String(e));
    } finally {
      setActionBusy(false);
    }
  }

  if (loading) {
    return <div className="text-center p-12 text-zinc-500">Carregando...</div>;
  }

  if (error || !meeting) {
    return (
      <div className="pb-8">
        <button className="bg-transparent text-zinc-500 border-none py-1.5 text-sm cursor-pointer mb-4 transition-colors duration-150 hover:text-zinc-100" onClick={onBack}>&larr; Voltar</button>
        <p className="text-red-500 text-sm mt-2">{error || "Reuniao nao encontrada."}</p>
      </div>
    );
  }

  return (
    <div className="pb-8">
      <button className="bg-transparent text-zinc-500 border-none py-1.5 text-sm cursor-pointer mb-4 transition-colors duration-150 hover:text-zinc-100" onClick={onBack}>&larr; Voltar</button>

      {/* Header */}
      <div className="mb-6">
        <div className="mb-2">
          {editingTitle ? (
            <input
              className="text-3xl font-semibold bg-zinc-800 text-zinc-100 border border-rose-500 rounded-lg px-2 py-1 w-full outline-none"
              value={titleDraft}
              onChange={(e) => setTitleDraft(e.target.value)}
              onBlur={handleSaveTitle}
              onKeyDown={handleTitleKeyDown}
              autoFocus
            />
          ) : (
            <h1
              className="text-3xl font-semibold cursor-pointer py-0.5 border-b border-dashed border-transparent transition-colors duration-150 hover:border-zinc-600"
              onClick={() => setEditingTitle(true)}
              title="Clique para editar"
            >
              {meeting.title}
            </h1>
          )}
        </div>
        <div className="flex gap-6 text-zinc-500 text-sm">
          <span>{formatDate(meeting.created_at)}</span>
          <span>{formatDuration(meeting.duration_secs)}</span>
          <span>{meeting.has_video ? "Video" : "Audio"}</span>
        </div>
      </div>

      {/* Media Player */}
      {meeting.media_status === "present" ? (
        <div className="mb-8">
          {mediaBlobUrl ? (
            meeting.has_video ? (
              <video
                ref={setMediaRef}
                src={mediaBlobUrl}
                controls
                className="w-full max-h-[480px] rounded-xl bg-black"
              />
            ) : (
              <audio
                ref={setMediaRef}
                src={mediaBlobUrl}
                controls
                className="w-full rounded-lg bg-zinc-800"
              />
            )
          ) : (
            <p className="text-center p-12 text-zinc-500">Carregando midia...</p>
          )}
        </div>
      ) : (
        <div className="mb-8">
          <p className="text-center p-12 text-zinc-500">Midia excluida.</p>
        </div>
      )}

      {/* Transcript */}
      <TranscriptView
        meetingId={meetingId}
        transcript={meeting.transcript}
        transcriptionStatus={meeting.transcription_status}
        onSeek={handleSeek}
        onStatusChange={loadMeeting}
      />

      {/* Chat */}
      <ChatPanel
        meetingId={meetingId}
        chatStatus={meeting.chat_status}
        onStatusChange={loadMeeting}
      />

      {/* Actions bar */}
      <div className="flex gap-3 flex-wrap pt-4 border-t border-zinc-700">
        <button className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={() => setShowExport(true)} disabled={actionBusy}>
          Exportar
        </button>
        {meeting.transcription_status === "done" && (
          <button className="bg-transparent text-zinc-100 border border-zinc-700 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:border-zinc-500 hover:bg-zinc-900 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleRetranscribe} disabled={actionBusy}>
            {actionBusy ? "Processando..." : "Retranscrever"}
          </button>
        )}
        {meeting.chat_status === "ready" && (
          <button className="bg-transparent text-zinc-100 border border-zinc-700 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:border-zinc-500 hover:bg-zinc-900 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleReindex} disabled={actionBusy}>
            {actionBusy ? "Processando..." : "Reindexar"}
          </button>
        )}
        <button
          className="bg-transparent text-red-500 border border-red-500 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:bg-red-500/10"
          onClick={() => setShowDeleteConfirm(true)}
          disabled={actionBusy}
        >
          Excluir
        </button>
      </div>

      {actionError && <p className="text-red-500 text-sm mt-2">{actionError}</p>}

      {/* Delete confirmation */}
      {showDeleteConfirm && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[100] backdrop-blur-sm" onClick={() => setShowDeleteConfirm(false)}>
          <div className="bg-zinc-900 border border-zinc-700 rounded-xl p-7 max-w-[380px] w-[90%] text-center" onClick={(e) => e.stopPropagation()}>
            <h3 className="mb-3 font-semibold">Confirmar exclusao</h3>
            <p className="text-zinc-500 text-sm mb-5">O que deseja excluir?</p>
            <div className="flex gap-3 justify-center">
              <button className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={() => setShowDeleteConfirm(false)} disabled={actionBusy}>
                Cancelar
              </button>
              <button className="bg-transparent text-zinc-100 border border-zinc-700 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:border-zinc-500 hover:bg-zinc-900 disabled:opacity-40 disabled:cursor-not-allowed" onClick={() => handleDelete("media_only")} disabled={actionBusy}>
                {actionBusy ? "Excluindo..." : "Apagar so midia"}
              </button>
              <button className="bg-transparent text-red-500 border border-red-500 px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:bg-red-500/10" onClick={() => handleDelete("everything")} disabled={actionBusy}>
                {actionBusy ? "Excluindo..." : "Apagar tudo"}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Export dialog */}
      {showExport && (
        <ExportDialog
          meetingId={meetingId}
          hasVideo={meeting.has_video}
          hasTranscript={meeting.transcription_status === "done"}
          onClose={() => setShowExport(false)}
        />
      )}
    </div>
  );
}
