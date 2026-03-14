use ashpd::desktop::screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType, StartCastOptions};
use ashpd::desktop::{CreateSessionOptions, PersistMode};

/// Holds a borrowed PipeWire file descriptor and node ID for screen capture.
/// The fd is borrowed from ScreenCapture's OwnedFd — ScreenCapture must outlive
/// any pipeline that uses this source, otherwise the fd becomes invalid.
#[derive(Debug)]
pub struct PipeWireSource {
    pub node_id: u32,
    pub fd: std::os::fd::RawFd,
}

pub struct ScreenCapture {
    node_id: Option<u32>,
    fd: Option<std::os::unix::io::OwnedFd>,
}

impl ScreenCapture {
    pub fn new() -> Self {
        Self { node_id: None, fd: None }
    }

    /// Opens the XDG Desktop Portal screen picker and returns a PipeWire source.
    pub async fn request_screen(&mut self) -> Result<PipeWireSource, String> {
        let proxy = Screencast::new()
            .await
            .map_err(|e| format!("Failed to create screencast proxy: {}", e))?;

        let session = proxy.create_session(CreateSessionOptions::default())
            .await
            .map_err(|e| format!("Failed to create session: {}", e))?;

        let select_sources_options = SelectSourcesOptions::default()
            .set_cursor_mode(CursorMode::Embedded)
            .set_sources(SourceType::Monitor | SourceType::Window)
            .set_multiple(false)
            .set_persist_mode(PersistMode::DoNot);

        proxy.select_sources(&session, select_sources_options)
            .await
            .map_err(|e| format!("Failed to select sources: {}", e))?;

        let response = proxy.start(&session, None, StartCastOptions::default())
            .await
            .map_err(|e| format!("Failed to start: {}", e))?;

        let streams_response = response.response()
            .map_err(|e| format!("Failed to get response: {}", e))?;

        let stream = streams_response.streams().first()
            .ok_or("No stream returned from portal")?;

        let fd = proxy.open_pipe_wire_remote(
            &session,
            Default::default(),
        )
        .await
        .map_err(|e| format!("Failed to get fd: {}", e))?;

        let node_id = stream.pipe_wire_node_id();

        use std::os::fd::AsRawFd;
        let raw_fd = fd.as_raw_fd();

        self.node_id = Some(node_id);
        self.fd = Some(fd);

        Ok(PipeWireSource { node_id, fd: raw_fd })
    }
}
