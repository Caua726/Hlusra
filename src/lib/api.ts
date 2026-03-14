import { invoke } from "@tauri-apps/api/core";

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

export interface Meeting extends MeetingSummary {
  dir_path: string;
  tracks: TrackInfo[];
  transcript: string | null;
}

export interface TrackInfo {
  index: number;
  label: string;
  codec: string;
}

export async function listMeetings(): Promise<MeetingSummary[]> {
  return invoke("list_meetings");
}

export async function getMeeting(id: string): Promise<Meeting> {
  return invoke("get_meeting", { id });
}

export async function updateMeetingTitle(id: string, title: string): Promise<void> {
  return invoke("update_meeting_title", { id, title });
}

export async function deleteMeeting(id: string, mode: "everything" | "media_only"): Promise<void> {
  return invoke("delete_meeting", { id, mode });
}
