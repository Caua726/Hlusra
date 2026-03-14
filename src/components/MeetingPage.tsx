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
import TranscriptView from "./TranscriptView";
import ChatPanel from "./ChatPanel";
import ExportDialog from "./ExportDialog";

interface Props {
  meetingId: string;
  onBack: () => void;
}

function formatDuration(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  if (h > 0) return `${h}h ${m}m ${s}s`;
  if (m > 0) return `${m}m ${s}s`;
  return `${s}s`;
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
  const [actionError, setActionError] = useState<string | null>(null);
  const mediaElRef = useRef<HTMLMediaElement | null>(null);
  const setMediaRef = useCallback((el: HTMLVideoElement | HTMLAudioElement | null) => {
    mediaElRef.current = el;
  }, []);

  const loadMeeting = useCallback(async () => {
    try {
      const data = await getMeeting(meetingId);
      setMeeting(data);
      setTitleDraft(data.title);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [meetingId]);

  useEffect(() => {
    loadMeeting();
  }, [loadMeeting]);

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

  async function handleDelete() {
    setActionError(null);
    try {
      await deleteMeeting(meetingId, "everything");
      onBack();
    } catch (e) {
      setActionError(String(e));
    }
  }

  async function handleRetranscribe() {
    setActionError(null);
    try {
      await retranscribeMeeting(meetingId);
      await loadMeeting();
    } catch (e) {
      setActionError(String(e));
    }
  }

  async function handleReindex() {
    setActionError(null);
    try {
      await reindexMeeting(meetingId);
      await loadMeeting();
    } catch (e) {
      setActionError(String(e));
    }
  }

  if (loading) {
    return <div className="loading">Carregando...</div>;
  }

  if (error || !meeting) {
    return (
      <div className="meeting-page">
        <button className="btn-back" onClick={onBack}>&larr; Voltar</button>
        <p className="error-text">{error || "Reunião não encontrada."}</p>
      </div>
    );
  }

  const mediaPath = meeting.dir_path + "/recording.mkv";
  const mediaSrc = convertFileSrc(mediaPath);

  return (
    <div className="meeting-page">
      <button className="btn-back" onClick={onBack}>&larr; Voltar</button>

      {/* Header */}
      <div className="meeting-header">
        <div className="meeting-title-row">
          {editingTitle ? (
            <input
              className="title-input"
              value={titleDraft}
              onChange={(e) => setTitleDraft(e.target.value)}
              onBlur={handleSaveTitle}
              onKeyDown={handleTitleKeyDown}
              autoFocus
            />
          ) : (
            <h1
              className="meeting-page-title"
              onClick={() => setEditingTitle(true)}
              title="Clique para editar"
            >
              {meeting.title}
            </h1>
          )}
        </div>
        <div className="meeting-info">
          <span>{formatDate(meeting.created_at)}</span>
          <span>{formatDuration(meeting.duration_secs)}</span>
          <span>{meeting.has_video ? "Vídeo" : "Áudio"}</span>
        </div>
      </div>

      {/* Media Player */}
      <div className="media-player">
        {meeting.has_video ? (
          <video
            ref={setMediaRef}
            src={mediaSrc}
            controls
            className="player-video"
          />
        ) : (
          <audio
            ref={setMediaRef}
            src={mediaSrc}
            controls
            className="player-audio"
          />
        )}
      </div>

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
      <div className="meeting-actions">
        <button className="btn-primary" onClick={() => setShowExport(true)}>
          Exportar
        </button>
        {meeting.transcription_status === "done" && (
          <button className="btn-secondary" onClick={handleRetranscribe}>
            Retranscrever
          </button>
        )}
        {meeting.chat_status === "ready" && (
          <button className="btn-secondary" onClick={handleReindex}>
            Reindexar
          </button>
        )}
        <button
          className="btn-danger"
          onClick={() => setShowDeleteConfirm(true)}
        >
          Excluir
        </button>
      </div>

      {actionError && <p className="error-text">{actionError}</p>}

      {/* Delete confirmation */}
      {showDeleteConfirm && (
        <div className="modal-overlay" onClick={() => setShowDeleteConfirm(false)}>
          <div className="modal-content modal-small" onClick={(e) => e.stopPropagation()}>
            <h3>Confirmar exclusão</h3>
            <p>Tem certeza que deseja excluir esta reunião? Esta ação não pode ser desfeita.</p>
            <div className="modal-actions">
              <button className="btn-danger" onClick={handleDelete}>
                Excluir
              </button>
              <button className="btn-secondary" onClick={() => setShowDeleteConfirm(false)}>
                Cancelar
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
