import { invoke } from "@tauri-apps/api/core";

// ---------------------------------------------------------------------------
// Library types
// ---------------------------------------------------------------------------

export interface TrackInfo {
  index: number;
  label: string;
  codec: string;
}

export interface MeetingSummary {
  id: string;
  title: string;
  created_at: string;
  duration_secs: number;
  has_video: boolean;
  file_size: number;
  media_status: "present" | "deleted";
  transcription_status: "pending" | "processing" | "done" | "failed";
  chat_status: "not_indexed" | "indexing" | "ready" | "failed";
}

export interface MeetingDetail {
  id: string;
  title: string;
  created_at: string;
  duration_secs: number;
  has_video: boolean;
  file_size: number;
  dir_path: string;
  media_status: "present" | "deleted";
  transcription_status: "pending" | "processing" | "done" | "failed";
  chat_status: "not_indexed" | "indexing" | "ready" | "failed";
  tracks: TrackInfo[];
  transcript: string | null;
}

/** Returned by stop_recording */
export interface Meeting {
  id: string;
  title: string;
  created_at: string;
  duration_secs: number;
  has_video: boolean;
  file_size: number;
  dir_path: string;
  tracks: TrackInfo[];
  media_status: "present" | "deleted";
  transcription_status: "pending" | "processing" | "done" | "failed";
  chat_status: "not_indexed" | "indexing" | "ready" | "failed";
}

// ---------------------------------------------------------------------------
// Recording types
// ---------------------------------------------------------------------------

export type RecordingState = "idle" | "recording" | "stopped";

export interface RecordingStatus {
  state: RecordingState;
  duration_secs: number;
  file_size: number;
}

// ---------------------------------------------------------------------------
// Transcription types
// ---------------------------------------------------------------------------

export interface WhisperModel {
  name: string;
  size_bytes: number;
  downloaded: boolean;
}

export interface TranscriptResult {
  language: string;
  segments: Segment[];
  full_text: string;
}

export interface Segment {
  start: number;
  end: number;
  text: string;
  words: Word[];
}

export interface Word {
  start: number;
  end: number;
  text: string;
  confidence: number;
}

// ---------------------------------------------------------------------------
// Settings types
// ---------------------------------------------------------------------------

export interface GeneralSettings {
  recordings_dir: string;
  auto_meeting_name: string;
  start_minimized: boolean;
}

export interface AudioSettings {
  codec: string;
  bitrate: number;
}

export interface VideoSettings {
  codec: string;
  backend: string;
  container: string;
  bitrate: number;
  fps: number;
  resolution: string;
}

export interface TranscriptionSettings {
  provider: string;
  api_url: string;
  api_key: string;
  model: string;
}

export interface RagSettings {
  embeddings_url: string;
  embeddings_api_key: string;
  embeddings_model: string;
  chat_url: string;
  chat_api_key: string;
  chat_model: string;
  chunk_size: number;
  top_k: number;
}

export interface AppSettings {
  general: GeneralSettings;
  audio: AudioSettings;
  video: VideoSettings;
  transcription: TranscriptionSettings;
  rag: RagSettings;
}

// ---------------------------------------------------------------------------
// Export types
// ---------------------------------------------------------------------------

export type AudioFormat = "mp3" | "wav" | "opus" | "ogg";
export type VideoFormat = "mp4_h264" | "mp4_h265" | "mkv_h264" | "mkv_h265";
export type TranscriptFormat = "txt" | "json" | "srt" | "pdf";

export type SaveMode =
  | { mode: "save" }
  | { mode: "save_as"; path: string };

// ---------------------------------------------------------------------------
// Library commands
// ---------------------------------------------------------------------------

export async function listMeetings(): Promise<MeetingSummary[]> {
  return invoke("list_meetings");
}

export async function getMeeting(id: string): Promise<MeetingDetail> {
  return invoke("get_meeting", { id });
}

export async function updateMeetingTitle(id: string, title: string): Promise<void> {
  return invoke("update_meeting_title", { id, title });
}

export async function deleteMeeting(id: string, mode: "everything" | "media_only"): Promise<void> {
  return invoke("delete_meeting", { id, mode });
}

// ---------------------------------------------------------------------------
// Recording commands
// ---------------------------------------------------------------------------

export async function startRecording(withVideo: boolean): Promise<string> {
  return invoke("start_recording", { with_video: withVideo });
}

export async function stopRecording(): Promise<Meeting> {
  return invoke("stop_recording");
}

export async function getRecordingStatus(): Promise<RecordingStatus> {
  return invoke("get_recording_status");
}

export async function probeEncoders(): Promise<Record<string, string[]>> {
  return invoke("probe_encoders");
}

// ---------------------------------------------------------------------------
// Transcription commands
// ---------------------------------------------------------------------------

export async function transcribeMeeting(id: string): Promise<void> {
  return invoke("transcribe_meeting", { id });
}

export async function retranscribeMeeting(id: string): Promise<void> {
  return invoke("retranscribe_meeting", { id });
}

export async function getTranscriptionStatus(id: string): Promise<string> {
  return invoke("get_transcription_status", { id });
}

export async function listAvailableModels(): Promise<WhisperModel[]> {
  return invoke("list_available_models");
}

export async function getDownloadedModels(): Promise<WhisperModel[]> {
  return invoke("get_downloaded_models");
}

export async function downloadModel(model: string): Promise<void> {
  return invoke("download_model", { model });
}

export async function getActiveModel(): Promise<WhisperModel> {
  return invoke("get_active_model");
}

export async function setActiveModel(model: string): Promise<void> {
  return invoke("set_active_model", { model });
}

// ---------------------------------------------------------------------------
// RAG / Chat commands
// ---------------------------------------------------------------------------

export async function indexMeeting(id: string): Promise<void> {
  return invoke("index_meeting", { id });
}

export async function reindexMeeting(id: string): Promise<void> {
  return invoke("reindex_meeting", { id });
}

export async function chatMessage(meetingId: string, message: string): Promise<void> {
  return invoke("chat_message", { meeting_id: meetingId, message });
}

export async function getChatStatus(id: string): Promise<string> {
  return invoke("get_chat_status", { id });
}

// ---------------------------------------------------------------------------
// Settings commands
// ---------------------------------------------------------------------------

export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function updateSettings(settings: AppSettings): Promise<void> {
  return invoke("update_settings", { settings });
}

// ---------------------------------------------------------------------------
// Export commands
// ---------------------------------------------------------------------------

export async function exportAudio(id: string, format: AudioFormat, saveMode: SaveMode): Promise<string> {
  return invoke("export_audio", { id, format, save_mode: saveMode });
}

export async function exportVideo(id: string, format: VideoFormat, saveMode: SaveMode): Promise<string> {
  return invoke("export_video", { id, format, save_mode: saveMode });
}

export async function exportTranscript(id: string, format: TranscriptFormat, saveMode: SaveMode): Promise<string> {
  return invoke("export_transcript", { id, format, save_mode: saveMode });
}
