use std::fs;
use std::path::{Path, PathBuf};
use crate::library::types::ArtifactKind;

pub struct LibraryFs {
    base_dir: PathBuf,
}

impl LibraryFs {
    pub fn new(base_dir: PathBuf) -> std::io::Result<Self> {
        fs::create_dir_all(&base_dir)?;
        let base_dir = fs::canonicalize(&base_dir)?;
        Ok(LibraryFs { base_dir })
    }

    /// Verify that `path` is contained within `self.base_dir`.
    /// Uses canonical paths to prevent traversal via `..` or symlinks.
    fn ensure_contained(&self, path: &Path) -> std::io::Result<()> {
        let canonical = fs::canonicalize(path)?;
        if !canonical.starts_with(&self.base_dir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!(
                    "path {} escapes library base directory {}",
                    canonical.display(),
                    self.base_dir.display()
                ),
            ));
        }
        Ok(())
    }

    /// Verify containment for a path that may not exist yet by checking its
    /// parent directory.
    fn ensure_parent_contained(&self, path: &Path) -> std::io::Result<()> {
        let parent = path.parent().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no parent")
        })?;
        let canonical_parent = fs::canonicalize(parent)?;
        if !canonical_parent.starts_with(&self.base_dir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!(
                    "path {} escapes library base directory {}",
                    path.display(),
                    self.base_dir.display()
                ),
            ));
        }
        Ok(())
    }

    pub fn create_meeting_dir(&self, dir_name: &str) -> std::io::Result<PathBuf> {
        let path = self.base_dir.join(dir_name);
        // Validate that the joined path doesn't escape base_dir
        // (e.g. dir_name = "../../etc").  We check lexically first since
        // the directory doesn't exist yet, then canonicalize after creation.
        if dir_name.contains("..") || dir_name.contains('/') || dir_name.contains('\\') {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "meeting directory name must not contain path separators or '..'",
            ));
        }
        fs::create_dir_all(&path)?;
        self.ensure_contained(&path)?;
        Ok(path)
    }

    pub fn get_artifact_path(&self, meeting_dir: &Path, kind: &ArtifactKind) -> PathBuf {
        meeting_dir.join(kind.filename())
    }

    pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> std::io::Result<PathBuf> {
        self.ensure_contained(meeting_dir)?;
        let path = self.get_artifact_path(meeting_dir, kind);
        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, data)?;
        fs::rename(&tmp_path, &path)?;
        Ok(path)
    }

    pub fn has_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> bool {
        self.get_artifact_path(meeting_dir, kind).exists()
    }

    pub fn delete_meeting_dir(&self, meeting_dir: &Path) -> std::io::Result<()> {
        // Reject symlinks to prevent traversal attacks
        match fs::symlink_metadata(meeting_dir) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "refusing to delete symlink meeting directory",
                    ));
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e),
        }
        // Path containment check
        self.ensure_contained(meeting_dir)?;
        match fs::remove_dir_all(meeting_dir) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn delete_media_files(&self, meeting_dir: &Path) -> std::io::Result<()> {
        self.ensure_contained(meeting_dir)?;
        let recording = self.get_artifact_path(meeting_dir, &ArtifactKind::Recording);
        match fs::remove_file(&recording) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        let audio_preview = self.get_artifact_path(meeting_dir, &ArtifactKind::AudioPreview);
        match fs::remove_file(&audio_preview) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        let thumbnail = self.get_artifact_path(meeting_dir, &ArtifactKind::Thumbnail);
        match fs::remove_file(&thumbnail) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        Ok(())
    }

    pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> std::io::Result<Vec<u8>> {
        self.ensure_contained(meeting_dir)?;
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
