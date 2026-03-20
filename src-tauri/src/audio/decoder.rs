use anyhow::{Result, Context};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use std::fs::File;
use std::path::Path;

pub struct DecodedAudio {
	pub samples: Vec<f32>,
	pub sample_rate: u32,
	pub channels: u16,
	pub duration_secs: f64,
}

pub fn decode_file(path: &Path) -> Result<DecodedAudio> {
	let file = File::open(path)
		.with_context(|| format!("Failed to open file: {:?}", path))?;

	let mss = MediaSourceStream::new(Box::new(file), Default::default());

	let probe = symphonia::default::get_probe()
		.format(&Default::default(), mss, &FormatOptions::default(), &MetadataOptions::default())?;

	let mut format = probe.format;

	let track = format.default_track()
		.context("No default audio track found")?;

	let mut decoder = symphonia::default::get_codecs()
		.make(&track.codec_params, &DecoderOptions::default())?;

	let sample_rate = track.codec_params.sample_rate
		.context("Unknown sample rate")?;
	let channels = track.codec_params.channels
		.context("Unknown channel count")?
		.count() as u16;

	let mut samples = Vec::new();
	let mut sample_buf = None;

	loop {
		let packet = match format.next_packet() {
			Ok(packet) => packet,
			Err(_) => break,
		};

		let decoded = decoder.decode(&packet)?;

		if sample_buf.is_none() {
			let spec = decoded.spec().clone();
			let duration = decoded.capacity() as u64;
			sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
		}

		if let Some(ref mut buf) = sample_buf {
			buf.copy_interleaved_ref(decoded);
			samples.extend_from_slice(buf.samples());
		}
	}

	let duration_secs = samples.len() as f64 / (sample_rate as f64 * channels as f64);

	Ok(DecodedAudio {
		samples,
		sample_rate,
		channels,
		duration_secs,
	})
}