use ashpd::desktop::screencast::{CursorMode, PersistMode, Screencast, SourceType};
use ashpd::WindowIdentifier;

#[derive(Debug, Clone)]
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

        let session = proxy.create_session()
            .await
            .map_err(|e| format!("Failed to create session: {}", e))?;

        proxy.select_sources(
            &session,
            CursorMode::Embedded,
            SourceType::Monitor | SourceType::Window,
            false,
            None,
            PersistMode::DoNot,
        )
        .await
        .map_err(|e| format!("Failed to select sources: {}", e))?;

        let response = proxy.start(&session, &WindowIdentifier::default())
            .await
            .map_err(|e| format!("Failed to start: {}", e))?;

        let stream = response.streams().first()
            .ok_or("No stream returned from portal")?;

        let fd = response.fd()
            .map_err(|e| format!("Failed to get fd: {}", e))?;

        let node_id = stream.pipe_wire_node_id();

        use std::os::fd::AsRawFd;
        let raw_fd = fd.as_raw_fd();

        self.node_id = Some(node_id);
        self.fd = Some(fd);

        Ok(PipeWireSource { node_id, fd: raw_fd })
    }
}
