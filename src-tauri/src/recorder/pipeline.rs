use gstreamer as gst;
use gstreamer::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use crate::recorder::capture::PipeWireSource;
use crate::recorder::encode;
use crate::recorder::types::*;

pub struct RecordingPipeline {
    pipeline: gst::Pipeline,
    start_time: Instant,
    output_path: PathBuf,
    has_video: bool,
    stopped_duration: Option<f64>,
}

impl RecordingPipeline {
    /// Build an audio-only pipeline (PipeWire mic source -> Opus -> MKV)
    /// TODO: Add system audio capture as second track once pipewire-rs device
    /// selection is implemented. For MVP, only mic is captured.
    pub fn build_audio_only(
        output_path: PathBuf,
        audio_config: &AudioConfig,
    ) -> Result<Self, String> {
        let pipeline = gst::Pipeline::builder().name("hlusra-audio").build();

        // Mic source — captures from PipeWire default input device
        let mic_src = gst::ElementFactory::make("pipewiresrc")
            .name("mic_src")
            .build()
            .map_err(|e| format!("pipewiresrc: {}", e))?;

        let mic_queue = gst::ElementFactory::make("queue")
            .name("mic_queue")
            .property("max-size-time", 5_000_000_000u64)
            .property("max-size-buffers", 0u32)
            .property("max-size-bytes", 0u32)
            .build()
            .map_err(|e| e.to_string())?;
        let mic_convert = gst::ElementFactory::make("audioconvert").name("mic_convert").build().map_err(|e| e.to_string())?;
        let mic_resample = gst::ElementFactory::make("audioresample").name("mic_resample").build().map_err(|e| e.to_string())?;
        let mic_enc = encode::create_audio_encoder(audio_config)?;

        // Muxer + sink
        let mux = gst::ElementFactory::make("matroskamux").name("mux").build().map_err(|e| e.to_string())?;
        let filesink = gst::ElementFactory::make("filesink")
            .name("filesink")
            .property("location", output_path.to_str().ok_or("Path contains invalid UTF-8")?)
            .build()
            .map_err(|e| e.to_string())?;

        pipeline.add_many(&[
            &mic_src, &mic_queue, &mic_convert, &mic_resample, &mic_enc,
            &mux, &filesink,
        ]).map_err(|e| e.to_string())?;

        // Link: pipewiresrc -> queue -> audioconvert -> audioresample -> opusenc -> mux -> filesink
        gst::Element::link_many(&[&mic_src, &mic_queue, &mic_convert, &mic_resample, &mic_enc])
            .map_err(|e| format!("Link mic: {}", e))?;
        mic_enc.link(&mux).map_err(|e| format!("Link mic->mux: {}", e))?;
        mux.link(&filesink).map_err(|e| format!("Link mux->sink: {}", e))?;

        Ok(Self {
            pipeline,
            start_time: Instant::now(),
            output_path,
            has_video: false,
            stopped_duration: None,
        })
    }

    /// Build a video + audio pipeline
    pub fn build_with_video(
        output_path: PathBuf,
        screen_source: &PipeWireSource,
        video_config: &VideoConfig,
        audio_config: &AudioConfig,
    ) -> Result<Self, String> {
        let pipeline = gst::Pipeline::builder().name("hlusra-video").build();

        // Screen source
        let screen_src = gst::ElementFactory::make("pipewiresrc")
            .name("screen_src")
            .property("fd", screen_source.fd)
            .property("path", screen_source.node_id.to_string())
            .build()
            .map_err(|e| format!("pipewiresrc screen: {}", e))?;

        let video_queue = gst::ElementFactory::make("queue")
            .name("video_queue")
            .property("max-size-time", 5_000_000_000u64)
            .property("max-size-buffers", 0u32)
            .property("max-size-bytes", 0u32)
            .build()
            .map_err(|e| e.to_string())?;
        let videoconvert = gst::ElementFactory::make("videoconvert").name("videoconvert").build().map_err(|e| e.to_string())?;

        // Scale and constrain video to configured width/height/fps
        let videoscale = gst::ElementFactory::make("videoscale").name("videoscale").build().map_err(|e| e.to_string())?;
        let videorate = gst::ElementFactory::make("videorate").name("videorate").build().map_err(|e| e.to_string())?;
        let video_caps = gst::Caps::builder("video/x-raw")
            .field("width", video_config.width as i32)
            .field("height", video_config.height as i32)
            .field("framerate", gst::Fraction::new(video_config.fps as i32, 1))
            .build();
        let capsfilter = gst::ElementFactory::make("capsfilter")
            .name("video_capsfilter")
            .property("caps", &video_caps)
            .build()
            .map_err(|e| e.to_string())?;

        let (video_enc, _actual_backend) = encode::create_video_encoder_with_fallback(
            video_config.backend, video_config.codec, video_config,
        )?;

        // Audio sources
        // TODO: pipewire-rs integration for explicit device selection is deferred.
        // For now we rely on PipeWire routing defaults automatically.
        let mic_props = gst::Structure::builder("props")
            .field("media.class", "Audio/Source")
            .build();
        let mic_src = gst::ElementFactory::make("pipewiresrc")
            .name("mic_src")
            .property("stream-properties", &mic_props)
            .build()
            .map_err(|e| e.to_string())?;
        let mic_queue = gst::ElementFactory::make("queue")
            .name("mic_queue")
            .property("max-size-time", 5_000_000_000u64)
            .property("max-size-buffers", 0u32)
            .property("max-size-bytes", 0u32)
            .build()
            .map_err(|e| e.to_string())?;
        let mic_convert = gst::ElementFactory::make("audioconvert").name("mic_convert").build().map_err(|e| e.to_string())?;
        let mic_resample = gst::ElementFactory::make("audioresample").name("mic_resample").build().map_err(|e| e.to_string())?;
        let mic_enc = encode::create_audio_encoder(audio_config)?;

        let sys_props = gst::Structure::builder("props")
            .field("media.class", "Audio/Sink")
            .build();
        let sys_src = gst::ElementFactory::make("pipewiresrc")
            .name("sys_src")
            .property("stream-properties", &sys_props)
            .build()
            .map_err(|e| e.to_string())?;
        let sys_queue = gst::ElementFactory::make("queue")
            .name("sys_queue")
            .property("max-size-time", 5_000_000_000u64)
            .property("max-size-buffers", 0u32)
            .property("max-size-bytes", 0u32)
            .build()
            .map_err(|e| e.to_string())?;
        let sys_convert = gst::ElementFactory::make("audioconvert").name("sys_convert").build().map_err(|e| e.to_string())?;
        let sys_resample = gst::ElementFactory::make("audioresample").name("sys_resample").build().map_err(|e| e.to_string())?;
        let sys_enc = encode::create_audio_encoder(audio_config)?;

        let mux = gst::ElementFactory::make("matroskamux").name("mux").build().map_err(|e| e.to_string())?;
        let filesink = gst::ElementFactory::make("filesink")
            .name("filesink")
            .property("location", output_path.to_str().ok_or("Path contains invalid UTF-8")?)
            .build()
            .map_err(|e| e.to_string())?;

        pipeline.add_many(&[
            &screen_src, &video_queue, &videoconvert, &videoscale, &videorate, &capsfilter, &video_enc,
            &mic_src, &mic_queue, &mic_convert, &mic_resample, &mic_enc,
            &sys_src, &sys_queue, &sys_convert, &sys_resample, &sys_enc,
            &mux, &filesink,
        ]).map_err(|e| e.to_string())?;

        // Link video: pipewiresrc -> queue -> videoconvert -> videoscale -> videorate -> capsfilter -> encoder -> mux
        gst::Element::link_many(&[&screen_src, &video_queue, &videoconvert, &videoscale, &videorate, &capsfilter, &video_enc])
            .map_err(|e| format!("Link video: {}", e))?;
        video_enc.link(&mux).map_err(|e| format!("Link video->mux: {}", e))?;

        // Link mic: pipewiresrc -> queue -> audioconvert -> audioresample -> opusenc -> mux
        gst::Element::link_many(&[&mic_src, &mic_queue, &mic_convert, &mic_resample, &mic_enc])
            .map_err(|e| format!("Link mic: {}", e))?;
        mic_enc.link(&mux).map_err(|e| format!("Link mic->mux: {}", e))?;

        // Link system: pipewiresrc -> queue -> audioconvert -> audioresample -> opusenc -> mux
        gst::Element::link_many(&[&sys_src, &sys_queue, &sys_convert, &sys_resample, &sys_enc])
            .map_err(|e| format!("Link sys: {}", e))?;
        sys_enc.link(&mux).map_err(|e| format!("Link sys->mux: {}", e))?;

        mux.link(&filesink).map_err(|e| format!("Link mux->sink: {}", e))?;

        Ok(Self {
            pipeline,
            start_time: Instant::now(),
            output_path,
            has_video: true,
            stopped_duration: None,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.start_time = Instant::now();
        self.pipeline.set_state(gst::State::Playing)
            .map_err(|e| format!("Failed to start pipeline: {:?}", e))?;

        // Install a bus watch to log GStreamer errors during recording
        if let Some(bus) = self.pipeline.bus() {
            let _ = bus.add_watch(|_bus, msg| {
                if let gst::MessageView::Error(e) = msg.view() {
                    eprintln!(
                        "[hlusra-pipeline] GStreamer error: {} (debug: {})",
                        e.error(),
                        e.debug().unwrap_or_default()
                    );
                }
                gst::glib::ControlFlow::Continue
            });
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        self.stopped_duration = Some(self.start_time.elapsed().as_secs_f64());
        self.pipeline.send_event(gst::event::Eos::new());
        // Wait for EOS to propagate
        let bus = self.pipeline.bus().ok_or("No bus")?;
        match bus.timed_pop_filtered(
            gst::ClockTime::from_seconds(5),
            &[gst::MessageType::Eos, gst::MessageType::Error],
        ) {
            Some(msg) => {
                if let gst::MessageView::Error(e) = msg.view() {
                    let err = format!(
                        "Pipeline error: {} ({})",
                        e.error(),
                        e.debug().unwrap_or_default()
                    );
                    self.pipeline.set_state(gst::State::Null).ok();
                    return Err(err);
                }
                // EOS received successfully
            }
            None => {
                self.pipeline.set_state(gst::State::Null).ok();
                return Err("Timed out waiting for EOS (5s)".to_string());
            }
        }
        self.pipeline.set_state(gst::State::Null)
            .map_err(|e| format!("Failed to stop pipeline: {:?}", e))?;
        Ok(())
    }

    pub fn duration_secs(&self) -> f64 {
        self.stopped_duration.unwrap_or_else(|| self.start_time.elapsed().as_secs_f64())
    }

    pub fn output_path(&self) -> &PathBuf {
        &self.output_path
    }

    pub fn has_video(&self) -> bool {
        self.has_video
    }

    pub fn file_size(&self) -> u64 {
        std::fs::metadata(&self.output_path)
            .map(|m| m.len())
            .unwrap_or(0)
    }
}

impl Drop for RecordingPipeline {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
