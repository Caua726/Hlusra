use tauri::State;
use crate::library::api::{Library, LibraryError};
use crate::library::types::*;

type Result<T> = std::result::Result<T, LibraryError>;

#[tauri::command]
pub fn list_meetings(library: State<'_, Library>) -> Result<Vec<MeetingSummary>> {
    library.list_meetings()
}

#[tauri::command]
pub fn get_meeting(library: State<'_, Library>, id: String) -> Result<Meeting> {
    library.get_meeting(&id)
}

#[tauri::command]
pub fn update_meeting_title(library: State<'_, Library>, id: String, title: String) -> Result<()> {
    library.update_title(&id, &title)
}

#[tauri::command]
pub fn delete_meeting(library: State<'_, Library>, id: String, mode: DeleteMode) -> Result<()> {
    library.delete_meeting(&id, mode)
}

#[tauri::command]
pub fn get_thumbnail(library: State<'_, Library>, id: String) -> Result<Option<Vec<u8>>> {
    let meeting = library.get_meeting(&id)?;
    if library.has_artifact(&meeting.dir_path, &ArtifactKind::Thumbnail) {
        let data = library.read_artifact(&meeting.dir_path, &ArtifactKind::Thumbnail)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}
