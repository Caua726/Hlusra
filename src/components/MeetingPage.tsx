import { useState, useEffect, useRef, useCallback } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
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

interface Props {
  meetingId: string;
  onBack: () => void;
  onChat: (ctx: { hasVideo: boolean; hasTranscript: boolean; chatStatus: "not_indexed" | "indexing" | "ready" | "failed"; title: string }) => void;
  onExport: (ctx: { hasVideo: boolean; hasTranscript: boolean; chatStatus: "not_indexed" | "indexing" | "ready" | "failed"; title: string }) => void;
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

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(0)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

const TRANSCRIPTION_BADGE: Record<string, { label: string; cls: string }> = {
  pending: { label: "Pendente", cls: "bg-amber-500/10 text-amber-400" },
  processing: { label: "Processando", cls: "bg-blue-500/10 text-blue-400" },
  done: { label: "Transcrita", cls: "bg-emerald-500/10 text-emerald-400" },
  failed: { label: "Falhou", cls: "bg-red-500/10 text-red-400" },
};

const CHAT_BADGE: Record<string, { label: string; cls: string }> = {
  not_indexed: { label: "Não indexado", cls: "bg-white/5 text-white/30" },
  indexing: { label: "Indexando", cls: "bg-blue-500/10 text-blue-400" },
  ready: { label: "Chat pronto", cls: "bg-emerald-500/10 text-emerald-400" },
  failed: { label: "Chat falhou", cls: "bg-red-500/10 text-red-400" },
};

export default function MeetingPage({ meetingId, onBack, onChat, onExport }: Props) {
  const [meeting, setMeeting] = useState<MeetingDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [editingTitle, setEditingTitle] = useState(false);
  const [titleDraft, setTitleDraft] = useState("");
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [actionBusy, setActionBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const [mediaBlobUrl, setMediaBlobUrl] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const savingTitleRef = useRef(false);
  const mediaElRef = useRef<HTMLMediaElement | null>(null);
  const setMediaRef = useCallback((el: HTMLMediaElement | null) => {
    mediaElRef.current = el;
  }, []);

  const loadMeeting = useCallback(async () => {
    setError(null);
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

  // Load media file as blob URL
  useEffect(() => {
    let revoked = false;
    let url: string | null = null;

    async function loadMedia() {
      if (!meeting || meeting.media_status !== "present") return;
      const mediaPath = meeting.dir_path + "/recording.mkv";
      // Use convertFileSrc for streaming — no file loaded into JS memory
      const assetUrl = convertFileSrc(mediaPath);
      if (!revoked) {
        setMediaBlobUrl(assetUrl);
      }
    }

    loadMedia();
    return () => {
      revoked = true;
      setMediaBlobUrl(null);
    };
  }, [meeting?.id, meeting?.media_status]);

  // Escape key handler for delete modal
  useEffect(() => {
    if (!showDeleteConfirm) return;
    function handleEscape(e: KeyboardEvent) {
      if (e.key === "Escape") setShowDeleteConfirm(false);
    }
    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [showDeleteConfirm]);

  function handleSeek(time: number) {
    if (mediaElRef.current) {
      mediaElRef.current.currentTime = time;
      mediaElRef.current.play().catch(() => {});
    }
  }

  async function handleSaveTitle() {
    if (!meeting || savingTitleRef.current) return;
    savingTitleRef.current = true;
    const trimmed = titleDraft.trim();
    if (!trimmed || trimmed === meeting.title) {
      setEditingTitle(false);
      setTitleDraft(meeting.title);
      savingTitleRef.current = false;
      return;
    }
    try {
      await updateMeetingTitle(meetingId, trimmed);
      setMeeting({ ...meeting, title: trimmed });
      setEditingTitle(false);
    } catch (e) {
      setActionError(formatError(e));
    } finally {
      savingTitleRef.current = false;
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
      setActionError(formatError(e));
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
      setActionError(formatError(e));
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
      setActionError(formatError(e));
    } finally {
      setActionBusy(false);
    }
  }

  function getMeetingContext() {
    if (!meeting) return { hasVideo: false, hasTranscript: false, chatStatus: "not_indexed" as const, title: "" };
    return {
      hasVideo: meeting.has_video,
      hasTranscript: meeting.transcription_status === "done",
      chatStatus: meeting.chat_status,
      title: meeting.title,
    };
  }

  if (loading) {
    return (
      <div className="flex-1 flex items-center justify-center">
        <div className="w-6 h-6 border-[3px] border-white/10 border-t-brand-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
      </div>
    );
  }

  if (error || !meeting) {
    return (
      <>
        <header className="glass shrink-0 border-b border-white/5">
          <div className="px-5 h-12 flex items-center gap-3">
            <button onClick={onBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Erro</h1>
          </div>
        </header>
        <div className="flex-1 flex items-center justify-center">
          <p className="text-red-400/80 text-xs">{error || "Reunião não encontrada."}</p>
        </div>
      </>
    );
  }

  const transBadge = TRANSCRIPTION_BADGE[meeting.transcription_status];
  const chatBadge = CHAT_BADGE[meeting.chat_status];

  return (
    <>
      {/* Header */}
      <header className="glass shrink-0 border-b border-white/5">
        <div className="px-5 h-12 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button onClick={onBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            {editingTitle ? (
              <input
                className="text-sm font-semibold text-white/80 bg-transparent border-b border-brand-500 outline-none px-1 py-0.5"
                value={titleDraft}
                onChange={(e) => setTitleDraft(e.target.value)}
                onBlur={handleSaveTitle}
                onKeyDown={handleTitleKeyDown}
                autoFocus
              />
            ) : (
              <h1
                className="text-sm font-semibold text-white/80 group cursor-pointer flex items-center gap-1.5"
                onClick={() => setEditingTitle(true)}
                onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); setEditingTitle(true); } }}
                tabIndex={0}
                role="button"
                title="Clique para editar"
              >
                {meeting.title}
                <svg className="w-3 h-3 text-white/10 opacity-0 group-hover:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                </svg>
              </h1>
            )}
          </div>
          <div className="flex gap-1.5">
            <button
              onClick={() => onExport(getMeetingContext())}
              disabled={actionBusy}
              aria-label="Exportar reunião"
              className="glass-input px-3 py-1.5 text-[10px] rounded-lg text-white/35 hover:text-white/60 transition-all cursor-pointer disabled:opacity-40"
            >
              Exportar
            </button>
            <button
              onClick={() => setShowDeleteConfirm(true)}
              disabled={actionBusy}
              aria-label="Excluir reunião"
              className="px-3 py-1.5 text-[10px] rounded-lg text-red-400/50 hover:text-red-400 hover:bg-red-500/10 border border-white/5 hover:border-red-500/20 transition-all bg-transparent cursor-pointer disabled:opacity-40"
            >
              Excluir
            </button>
          </div>
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-5 space-y-4">
        {/* Info card */}
        <div className="glass-card rounded-2xl p-5">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              {/* Play/Pause button */}
              {meeting.media_status === "present" && mediaBlobUrl && (
                <button
                  onClick={() => { if (mediaElRef.current) mediaElRef.current.paused ? mediaElRef.current.play() : mediaElRef.current.pause(); }}
                  aria-label={isPlaying ? "Pausar" : "Reproduzir"}
                  className="w-12 h-12 rounded-xl glass-heavy flex items-center justify-center hover:scale-105 transition-all active:scale-95 group border-0 cursor-pointer"
                >
                  {isPlaying ? (
                    <svg className="w-5 h-5 text-white/40 group-hover:text-brand-400 transition-colors" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
                    </svg>
                  ) : (
                    <svg className="w-5 h-5 text-white/40 ml-0.5 group-hover:text-brand-400 transition-colors" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M8 5v14l11-7z" />
                    </svg>
                  )}
                </button>
              )}
              <div>
                <div className="flex items-center gap-3 text-[12px] text-white/50">
                  <span className={`text-[9px] font-bold px-2 py-0.5 rounded-md uppercase tracking-wider ${
                    meeting.has_video ? "bg-blue-500/15 text-blue-400" : "bg-white/6 text-white/40"
                  }`}>
                    {meeting.has_video ? "Vídeo" : "Áudio"}
                  </span>
                  <span>{formatDate(meeting.created_at)}</span>
                </div>
                <div className="flex items-center gap-4 mt-1.5 text-[11px] text-white/30">
                  <span>{formatDuration(meeting.duration_secs)}</span>
                  <span>{formatSize(meeting.file_size)}</span>
                  <span>{meeting.tracks.length} faixa{meeting.tracks.length !== 1 ? "s" : ""}</span>
                </div>
              </div>
            </div>
            <div className="flex gap-2">
              {transBadge && (
                <span className={`text-[8px] px-2 py-0.5 rounded-md font-semibold uppercase ${transBadge.cls}`}>
                  {transBadge.label}
                </span>
              )}
              {chatBadge && chatBadge.label && (
                <span className={`text-[8px] px-2 py-0.5 rounded-md font-semibold uppercase ${chatBadge.cls}`}>
                  {chatBadge.label}
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Hidden media player */}
        {meeting.media_status === "present" && mediaBlobUrl && (
          <div className="hidden">
            {meeting.has_video ? (
              <video
                ref={setMediaRef}
                src={mediaBlobUrl}
                onPlay={() => setIsPlaying(true)}
                onPause={() => setIsPlaying(false)}
              />
            ) : (
              <audio
                ref={setMediaRef}
                src={mediaBlobUrl}
                onPlay={() => setIsPlaying(true)}
                onPause={() => setIsPlaying(false)}
              />
            )}
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

        {/* Chat button */}
        <button
          onClick={() => onChat(getMeetingContext())}
          className="w-full glass-card rounded-2xl p-4 flex items-center justify-between group hover:border-white/15 hover:bg-white/[0.05] transition-all duration-300 cursor-pointer bg-transparent"
        >
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-brand-500/10 flex items-center justify-center group-hover:bg-brand-500/15 group-hover:scale-105 transition-all">
              <svg className="w-5 h-5 text-brand-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
            </div>
            <div className="text-left">
              <h3 className="text-[13px] font-medium text-white/70 group-hover:text-white/90 transition-colors">Conversar sobre esta reunião</h3>
              <p className="text-[10px] text-white/25 mt-0.5">Pergunte qualquer coisa sobre o conteúdo</p>
            </div>
          </div>
          <svg className="w-4 h-4 text-white/15 group-hover:text-white/40 group-hover:translate-x-1 transition-all" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>

        {/* Actions */}
        <div className="flex gap-2">
          {meeting.transcription_status === "done" && (
            <button
              onClick={handleRetranscribe}
              disabled={actionBusy}
              className="glass-input px-3 py-2 text-[10px] rounded-xl text-white/30 hover:text-white/60 transition-all flex-1 cursor-pointer bg-transparent disabled:opacity-40"
            >
              {actionBusy ? "Processando..." : "Retranscrever"}
            </button>
          )}
          {meeting.chat_status === "ready" && (
            <button
              onClick={handleReindex}
              disabled={actionBusy}
              className="glass-input px-3 py-2 text-[10px] rounded-xl text-white/30 hover:text-white/60 transition-all flex-1 cursor-pointer bg-transparent disabled:opacity-40"
            >
              {actionBusy ? "Processando..." : "Reindexar"}
            </button>
          )}
        </div>

        {actionError && <p className="text-red-400/80 text-xs">{actionError}</p>}
      </div>

      {/* Delete confirmation overlay */}
      {showDeleteConfirm && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[100] backdrop-blur-sm" onClick={() => setShowDeleteConfirm(false)}>
          <div className="glass-heavy rounded-2xl p-7 max-w-[380px] w-[90%] text-center" onClick={(e) => e.stopPropagation()}>
            <h3 className="mb-3 font-semibold text-white/80">Confirmar exclusão</h3>
            <p className="text-white/30 text-sm mb-5">O que deseja excluir?</p>
            <div className="flex gap-3 justify-center">
              <button
                className="glass-input px-5 py-2 rounded-xl text-[11px] text-white/50 hover:text-white/80 transition-all cursor-pointer bg-transparent disabled:opacity-40"
                onClick={() => setShowDeleteConfirm(false)}
                disabled={actionBusy}
              >
                Cancelar
              </button>
              <button
                className="glass-input px-5 py-2 rounded-xl text-[11px] text-white/50 hover:text-white/80 transition-all cursor-pointer bg-transparent disabled:opacity-40"
                onClick={() => handleDelete("media_only")}
                disabled={actionBusy}
              >
                {actionBusy ? "Excluindo..." : "Apagar só mídia"}
              </button>
              <button
                className="px-5 py-2 rounded-xl text-[11px] text-red-400/70 hover:text-red-400 hover:bg-red-500/10 border border-red-500/20 transition-all cursor-pointer bg-transparent disabled:opacity-40"
                onClick={() => handleDelete("everything")}
                disabled={actionBusy}
              >
                {actionBusy ? "Excluindo..." : "Apagar tudo"}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
