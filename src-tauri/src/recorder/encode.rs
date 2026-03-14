use std::collections::HashMap;
use gstreamer as gst;
use gstreamer::prelude::*;
use crate::recorder::types::*;

const BACKENDS: &[EncoderBackend] = &[
    EncoderBackend::Vaapi,
    EncoderBackend::Cuda,
    EncoderBackend::Vulkan,
    EncoderBackend::Software,
];

const CODECS: &[VideoCodec] = &[
    VideoCodec::H264,
    VideoCodec::H265,
    VideoCodec::Av1,
];

pub fn probe_available() -> HashMap<EncoderBackend, Vec<VideoCodec>> {
    let mut result = HashMap::new();
    for &backend in BACKENDS {
        let mut codecs = Vec::new();
        for &codec in CODECS {
            let name = backend.gst_element_name(&codec);
            if gst::ElementFactory::find(name).is_some() {
                codecs.push(codec);
            }
        }
        if !codecs.is_empty() {
            result.insert(backend, codecs);
        }
    }
    result
}

pub fn create_video_encoder(
    backend: EncoderBackend,
    codec: VideoCodec,
    config: &VideoConfig,
) -> Result<gst::Element, String> {
    let element_name = backend.gst_element_name(&codec);
    let encoder = gst::ElementFactory::make(element_name)
        .build()
        .map_err(|e| format!("Failed to create encoder {}: {}", element_name, e))?;

    // Set bitrate -- property name varies by encoder
    match backend {
        EncoderBackend::Vaapi => {
            encoder.set_property("bitrate", config.bitrate / 1000); // kbps
        }
        EncoderBackend::Cuda => {
            encoder.set_property("bitrate", config.bitrate / 1000); // kbps
        }
        EncoderBackend::Software => {
            encoder.set_property("bitrate", config.bitrate / 1000); // kbps
        }
        EncoderBackend::Vulkan => {
            // Vulkan encoder properties vary, set if available
        }
    }

    Ok(encoder)
}

pub fn create_audio_encoder(config: &AudioConfig) -> Result<gst::Element, String> {
    let encoder = gst::ElementFactory::make("opusenc")
        .property("bitrate", config.bitrate as i32)
        .build()
        .map_err(|e| format!("Failed to create opusenc: {}", e))?;
    Ok(encoder)
}

/// Try to create a video encoder with fallback chain.
/// Order: requested backend -> next available -> software.
pub fn create_video_encoder_with_fallback(
    preferred: EncoderBackend,
    codec: VideoCodec,
    config: &VideoConfig,
) -> Result<(gst::Element, EncoderBackend), String> {
    // Try preferred first
    if let Ok(enc) = create_video_encoder(preferred, codec, config) {
        return Ok((enc, preferred));
    }

    // Try others in order
    let available = probe_available();
    for &backend in BACKENDS {
        if backend == preferred {
            continue;
        }
        if let Some(codecs) = available.get(&backend) {
            if codecs.contains(&codec) {
                if let Ok(enc) = create_video_encoder(backend, codec, config) {
                    return Ok((enc, backend));
                }
            }
        }
    }

    Err(format!("No encoder available for {:?}", codec))
}
