use std::fs;
use std::path::{Path, PathBuf};
use crate::library::types::ArtifactKind;

pub struct LibraryFs {
    base_dir: PathBuf,
}

impl LibraryFs {
    pub fn new(base_dir: PathBuf) -> std::io::Result<Self> {
        fs::create_dir_all(&base_dir)?;
        Ok(LibraryFs { base_dir })
    }

    pub fn create_meeting_dir(&self, dir_name: &str) -> std::io::Result<PathBuf> {
        let path = self.base_dir.join(dir_name);
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn get_artifact_path(&self, meeting_dir: &Path, kind: &ArtifactKind) -> PathBuf {
        meeting_dir.join(kind.filename())
    }

    pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> std::io::Result<PathBuf> {
        let path = self.get_artifact_path(meeting_dir, kind);
        fs::write(&path, data)?;
        Ok(path)
    }

    pub fn has_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> bool {
        self.get_artifact_path(meeting_dir, kind).exists()
    }

    pub fn delete_meeting_dir(&self, meeting_dir: &Path) -> std::io::Result<()> {
        if meeting_dir.exists() {
            fs::remove_dir_all(meeting_dir)?;
        }
        Ok(())
    }

    pub fn delete_media_files(&self, meeting_dir: &Path) -> std::io::Result<()> {
        let recording = self.get_artifact_path(meeting_dir, &ArtifactKind::Recording);
        if recording.exists() {
            fs::remove_file(&recording)?;
        }
        let thumbnail = self.get_artifact_path(meeting_dir, &ArtifactKind::Thumbnail);
        if thumbnail.exists() {
            fs::remove_file(&thumbnail)?;
        }
        Ok(())
    }

    pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> std::io::Result<Vec<u8>> {
        let path = self.get_artifact_path(meeting_dir, kind);
        fs::read(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_meeting_dir() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("2026-03-14_abc123").unwrap();
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    #[test]
    fn test_save_and_read_artifact() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("test").unwrap();

        let data = b"test transcript content";
        lib_fs.save_artifact(&dir, &ArtifactKind::TranscriptTxt, data).unwrap();

        assert!(lib_fs.has_artifact(&dir, &ArtifactKind::TranscriptTxt));
        assert!(!lib_fs.has_artifact(&dir, &ArtifactKind::Recording));

        let read = lib_fs.read_artifact(&dir, &ArtifactKind::TranscriptTxt).unwrap();
        assert_eq!(read, data);
    }

    #[test]
    fn test_delete_media_files() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("test").unwrap();

        lib_fs.save_artifact(&dir, &ArtifactKind::Recording, b"video").unwrap();
        lib_fs.save_artifact(&dir, &ArtifactKind::Thumbnail, b"thumb").unwrap();
        lib_fs.save_artifact(&dir, &ArtifactKind::TranscriptTxt, b"text").unwrap();

        lib_fs.delete_media_files(&dir).unwrap();

        assert!(!lib_fs.has_artifact(&dir, &ArtifactKind::Recording));
        assert!(!lib_fs.has_artifact(&dir, &ArtifactKind::Thumbnail));
        assert!(lib_fs.has_artifact(&dir, &ArtifactKind::TranscriptTxt));
    }

    #[test]
    fn test_delete_meeting_dir() {
        let tmp = TempDir::new().unwrap();
        let lib_fs = LibraryFs::new(tmp.path().to_path_buf()).unwrap();
        let dir = lib_fs.create_meeting_dir("deleteme").unwrap();
        lib_fs.save_artifact(&dir, &ArtifactKind::Recording, b"data").unwrap();

        lib_fs.delete_meeting_dir(&dir).unwrap();
        assert!(!dir.exists());
    }
}
