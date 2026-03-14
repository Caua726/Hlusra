use std::collections::HashMap;
use gstreamer as gst;
use gstreamer::prelude::*;
use crate::recorder::types::*;

const BACKENDS: &[EncoderBackend] = &[
    EncoderBackend::Vaapi,
    EncoderBackend::Cuda,
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
            if let Some(name) = backend.gst_element_name(codec) {
                if gst::ElementFactory::find(name).is_some() {
                    tracing::debug!("probed {:?}+{:?} -> {} (available)", backend, codec, name);
                    codecs.push(codec);
                } else {
                    tracing::debug!("probed {:?}+{:?} -> {} (not found)", backend, codec, name);
                }
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
    let element_name = backend.gst_element_name(codec)
        .ok_or(format!("{:?} + {:?} is not supported", backend, codec))?;
    tracing::info!("creating encoder: {} ({:?} + {:?})", element_name, backend, codec);

    let encoder = gst::ElementFactory::make(element_name)
        .build()
        .map_err(|e| format!("Failed to create encoder {}: {}", element_name, e))?;

    // Clamp bitrate: use default (2 Mbps) if zero or unreasonably low
    let bitrate = if config.bitrate == 0 {
        tracing::warn!("bitrate is 0, using default 2000000 bps");
        2_000_000
    } else {
        config.bitrate
    };

    // Set bitrate -- property name varies by encoder
    if element_name == "svtav1enc" {
        encoder.set_property("target-bitrate", bitrate / 1000);
    } else {
        encoder.set_property("bitrate", bitrate / 1000);
    }

    Ok(encoder)
}

pub fn create_audio_encoder(config: &AudioConfig) -> Result<gst::Element, String> {
    // Clamp audio bitrate: use default (64 kbps) if zero
    let bitrate = if config.bitrate == 0 {
        tracing::warn!("audio bitrate is 0, using default 64000 bps");
        64_000
    } else {
        config.bitrate
    };

    let encoder = gst::ElementFactory::make("opusenc")
        .property("bitrate", bitrate as i32)
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
    // Probe what is actually available before attempting anything
    let available = probe_available();
    tracing::debug!("available backends: {:?}", available);

    // Try preferred first if it supports the codec
    if available.get(&preferred).map_or(false, |c| c.contains(&codec)) {
        if let Ok(enc) = create_video_encoder(preferred, codec, config) {
            tracing::info!("selected preferred backend: {:?}", preferred);
            return Ok((enc, preferred));
        }
    }

    // Try others in order
    for &backend in BACKENDS {
        if backend == preferred {
            continue;
        }
        if let Some(codecs) = available.get(&backend) {
            if codecs.contains(&codec) {
                if let Ok(enc) = create_video_encoder(backend, codec, config) {
                    tracing::info!("selected fallback backend: {:?}", backend);
                    return Ok((enc, backend));
                }
            }
        }
    }

    Err(format!("No encoder available for {:?}", codec))
}
