use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncoderBackend {
    Vaapi,
    Cuda,
    Vulkan,
    Software,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoCodec {
    H264,
    H265,
    Av1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingState {
    Idle,
    Recording,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingStatus {
    pub state: RecordingState,
    pub duration_secs: f64,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub codec: VideoCodec,
    pub backend: EncoderBackend,
    pub bitrate: u32,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub bitrate: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            codec: VideoCodec::H265,
            backend: EncoderBackend::Vaapi,
            bitrate: 2_000_000,
            fps: 15,
            width: 1280,
            height: 720,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self { bitrate: 64_000 }
    }
}

impl EncoderBackend {
    pub fn gst_element_name(&self, codec: &VideoCodec) -> &'static str {
        match (self, codec) {
            (Self::Vaapi, VideoCodec::H264) => "vaapih264enc",
            (Self::Vaapi, VideoCodec::H265) => "vaapih265enc",
            (Self::Vaapi, VideoCodec::Av1) => "vaapiav1enc",
            (Self::Cuda, VideoCodec::H264) => "nvh264enc",
            (Self::Cuda, VideoCodec::H265) => "nvh265enc",
            (Self::Cuda, VideoCodec::Av1) => "nvav1enc",
            (Self::Vulkan, VideoCodec::H264) => "vulkanh264enc",
            (Self::Vulkan, VideoCodec::H265) => "vulkanh265enc",
            (Self::Vulkan, VideoCodec::Av1) => "vulkanav1enc",
            (Self::Software, VideoCodec::H264) => "x264enc",
            (Self::Software, VideoCodec::H265) => "x265enc",
            (Self::Software, VideoCodec::Av1) => "svtav1enc",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_element_names() {
        assert_eq!(EncoderBackend::Vaapi.gst_element_name(&VideoCodec::H265), "vaapih265enc");
        assert_eq!(EncoderBackend::Cuda.gst_element_name(&VideoCodec::H264), "nvh264enc");
        assert_eq!(EncoderBackend::Software.gst_element_name(&VideoCodec::Av1), "svtav1enc");
        assert_eq!(EncoderBackend::Vulkan.gst_element_name(&VideoCodec::Av1), "vulkanav1enc");
    }
}
