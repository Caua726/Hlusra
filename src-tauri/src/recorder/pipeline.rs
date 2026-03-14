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
}

impl RecordingPipeline {
    /// Build an audio-only pipeline (2 PipeWire sources -> Opus -> MKV)
    pub fn build_audio_only(
        output_path: PathBuf,
        audio_config: &AudioConfig,
    ) -> Result<Self, String> {
        gst::init().map_err(|e| format!("GStreamer init failed: {}", e))?;

        let pipeline = gst::Pipeline::new();

        // Mic source
        let mic_src = gst::ElementFactory::make("pipewiresrc")
            .name("mic_src")
            .build()
            .map_err(|e| format!("pipewiresrc: {}", e))?;

        let mic_queue = gst::ElementFactory::make("queue").name("mic_queue").build().map_err(|e| e.to_string())?;
        let mic_convert = gst::ElementFactory::make("audioconvert").name("mic_convert").build().map_err(|e| e.to_string())?;
        let mic_enc = encode::create_audio_encoder(audio_config)?;
        mic_enc.set_name("mic_enc").ok();

        // System audio source
        let sys_src = gst::ElementFactory::make("pipewiresrc")
            .name("sys_src")
            .build()
            .map_err(|e| format!("pipewiresrc: {}", e))?;

        let sys_queue = gst::ElementFactory::make("queue").name("sys_queue").build().map_err(|e| e.to_string())?;
        let sys_convert = gst::ElementFactory::make("audioconvert").name("sys_convert").build().map_err(|e| e.to_string())?;
        let sys_enc = encode::create_audio_encoder(audio_config)?;
        sys_enc.set_name("sys_enc").ok();

        // Muxer + sink
        let mux = gst::ElementFactory::make("matroskamux").name("mux").build().map_err(|e| e.to_string())?;
        let filesink = gst::ElementFactory::make("filesink")
            .name("filesink")
            .property("location", output_path.to_string_lossy().to_string())
            .build()
            .map_err(|e| e.to_string())?;

        pipeline.add_many(&[
            &mic_src, &mic_queue, &mic_convert, &mic_enc,
            &sys_src, &sys_queue, &sys_convert, &sys_enc,
            &mux, &filesink,
        ]).map_err(|e| e.to_string())?;

        // Link mic path
        gst::Element::link_many(&[&mic_src, &mic_queue, &mic_convert, &mic_enc])
            .map_err(|e| format!("Link mic: {}", e))?;
        mic_enc.link(&mux).map_err(|e| format!("Link mic->mux: {}", e))?;

        // Link system path
        gst::Element::link_many(&[&sys_src, &sys_queue, &sys_convert, &sys_enc])
            .map_err(|e| format!("Link sys: {}", e))?;
        sys_enc.link(&mux).map_err(|e| format!("Link sys->mux: {}", e))?;

        // Link mux -> sink
        mux.link(&filesink).map_err(|e| format!("Link mux->sink: {}", e))?;

        Ok(Self {
            pipeline,
            start_time: Instant::now(),
            output_path,
            has_video: false,
        })
    }

    /// Build a video + audio pipeline
    pub fn build_with_video(
        output_path: PathBuf,
        screen_source: &PipeWireSource,
        video_config: &VideoConfig,
        audio_config: &AudioConfig,
    ) -> Result<Self, String> {
        gst::init().map_err(|e| format!("GStreamer init failed: {}", e))?;

        let pipeline = gst::Pipeline::new();

        // Screen source
        let screen_src = gst::ElementFactory::make("pipewiresrc")
            .name("screen_src")
            .property("fd", screen_source.fd)
            .property("path", screen_source.node_id.to_string())
            .build()
            .map_err(|e| format!("pipewiresrc screen: {}", e))?;

        let video_queue = gst::ElementFactory::make("queue").name("video_queue").build().map_err(|e| e.to_string())?;
        let videoconvert = gst::ElementFactory::make("videoconvert").name("videoconvert").build().map_err(|e| e.to_string())?;

        let (video_enc, _actual_backend) = encode::create_video_encoder_with_fallback(
            video_config.backend, video_config.codec, video_config,
        )?;
        video_enc.set_name("video_enc").ok();

        // Audio sources (same as audio-only)
        let mic_src = gst::ElementFactory::make("pipewiresrc").name("mic_src").build().map_err(|e| e.to_string())?;
        let mic_queue = gst::ElementFactory::make("queue").name("mic_queue").build().map_err(|e| e.to_string())?;
        let mic_convert = gst::ElementFactory::make("audioconvert").name("mic_convert").build().map_err(|e| e.to_string())?;
        let mic_enc = encode::create_audio_encoder(audio_config)?;

        let sys_src = gst::ElementFactory::make("pipewiresrc").name("sys_src").build().map_err(|e| e.to_string())?;
        let sys_queue = gst::ElementFactory::make("queue").name("sys_queue").build().map_err(|e| e.to_string())?;
        let sys_convert = gst::ElementFactory::make("audioconvert").name("sys_convert").build().map_err(|e| e.to_string())?;
        let sys_enc = encode::create_audio_encoder(audio_config)?;

        let mux = gst::ElementFactory::make("matroskamux").name("mux").build().map_err(|e| e.to_string())?;
        let filesink = gst::ElementFactory::make("filesink")
            .name("filesink")
            .property("location", output_path.to_string_lossy().to_string())
            .build()
            .map_err(|e| e.to_string())?;

        pipeline.add_many(&[
            &screen_src, &video_queue, &videoconvert, &video_enc,
            &mic_src, &mic_queue, &mic_convert, &mic_enc,
            &sys_src, &sys_queue, &sys_convert, &sys_enc,
            &mux, &filesink,
        ]).map_err(|e| e.to_string())?;

        // Link video
        gst::Element::link_many(&[&screen_src, &video_queue, &videoconvert, &video_enc])
            .map_err(|e| format!("Link video: {}", e))?;
        video_enc.link(&mux).map_err(|e| format!("Link video->mux: {}", e))?;

        // Link mic
        gst::Element::link_many(&[&mic_src, &mic_queue, &mic_convert, &mic_enc])
            .map_err(|e| format!("Link mic: {}", e))?;
        mic_enc.link(&mux).map_err(|e| format!("Link mic->mux: {}", e))?;

        // Link system
        gst::Element::link_many(&[&sys_src, &sys_queue, &sys_convert, &sys_enc])
            .map_err(|e| format!("Link sys: {}", e))?;
        sys_enc.link(&mux).map_err(|e| format!("Link sys->mux: {}", e))?;

        mux.link(&filesink).map_err(|e| format!("Link mux->sink: {}", e))?;

        Ok(Self {
            pipeline,
            start_time: Instant::now(),
            output_path,
            has_video: true,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.start_time = Instant::now();
        self.pipeline.set_state(gst::State::Playing)
            .map_err(|e| format!("Failed to start pipeline: {:?}", e))?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.pipeline.send_event(gst::event::Eos::new());
        // Wait for EOS to propagate
        let bus = self.pipeline.bus().ok_or("No bus")?;
        let _msg = bus.timed_pop_filtered(
            gst::ClockTime::from_seconds(5),
            &[gst::MessageType::Eos, gst::MessageType::Error],
        );
        self.pipeline.set_state(gst::State::Null)
            .map_err(|e| format!("Failed to stop pipeline: {:?}", e))?;
        Ok(())
    }

    pub fn duration_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
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
