use anyhow::{Context, Result};
use hound::{WavSpec, WavWriter};
use mp3lame_encoder::{Builder, FlushGap, InterleavedPcm, MonoPcm, Bitrate, Quality};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
	Wav,
	Mp3,
}

pub struct ExportOptions {
	pub format: ExportFormat,
	pub sample_rate: u32,
	pub bit_depth: u16,
}

pub fn encode_wav(
	samples: &[f32],
	channels: u16,
	options: &ExportOptions,
	output_path: &Path,
) -> Result<()> {
	let spec = WavSpec {
		channels: channels as u16,
		sample_rate: options.sample_rate,
		bits_per_sample: options.bit_depth,
		sample_format: hound::SampleFormat::Int,
	};

	let mut writer = WavWriter::create(output_path, spec)
		.with_context(|| "Failed to create WAV file")?;

	let max_val = (1 << (options.bit_depth - 1)) - 1;

	for &sample in samples {
		let sample_int = (sample * max_val as f32).clamp(-max_val as f32, max_val as f32) as i32;
		writer
			.write_sample(sample_int)
			.with_context(|| "Failed to write sample")?;
	}

	writer
		.finalize()
		.with_context(|| "Failed to finalize WAV file")?;

	Ok(())
}

pub fn encode_mp3(
	samples: &[f32],
	channels: u16,
	sample_rate: u32,
	output_path: &Path,
) -> Result<()> {
	let file = File::create(output_path)
		.with_context(|| "Failed to create MP3 file")?;
	let mut writer = BufWriter::new(file);

	// Build MP3 encoder
	let mut builder = Builder::new()
		.ok_or_else(|| anyhow::anyhow!("Failed to create LAME builder"))?;

	builder.set_num_channels(channels as u8)
		.map_err(|e| anyhow::anyhow!("Failed to set channels: {:?}", e))?;
	builder.set_sample_rate(sample_rate)
		.map_err(|e| anyhow::anyhow!("Failed to set sample rate: {:?}", e))?;
	builder.set_brate(Bitrate::Kbps320)
		.map_err(|e| anyhow::anyhow!("Failed to set bitrate: {:?}", e))?;
	builder.set_quality(Quality::Best)
		.map_err(|e| anyhow::anyhow!("Failed to set quality: {:?}", e))?;

	let mut mp3_encoder = builder.build()
		.map_err(|e| anyhow::anyhow!("Failed to build MP3 encoder: {:?}", e))?;

	// Prepare output buffer
	let mut mp3_output = Vec::with_capacity(mp3lame_encoder::max_required_buffer_size(samples.len()));

	// Encode based on channel count
	if channels == 1 {
		// Mono: use f32 samples directly
		let input = MonoPcm(samples);
		mp3_encoder.encode_to_vec(input, &mut mp3_output)
			.map_err(|e| anyhow::anyhow!("MP3 encoding error: {:?}", e))?;
	} else {
		// Stereo or more: convert to interleaved i16
		let samples_i16: Vec<i16> = samples
			.iter()
			.map(|&s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16)
			.collect();
		let input = InterleavedPcm(&samples_i16);
		mp3_encoder.encode_to_vec(input, &mut mp3_output)
			.map_err(|e| anyhow::anyhow!("MP3 encoding error: {:?}", e))?;
	}

	// Flush remaining data
	mp3_encoder.flush_to_vec::<FlushGap>(&mut mp3_output)
		.map_err(|e| anyhow::anyhow!("MP3 flush error: {:?}", e))?;

	// Write to file
	use std::io::Write;
	writer.write_all(&mp3_output)
		.with_context(|| "Failed to write MP3 data")?;

	writer.flush()
		.with_context(|| "Failed to flush MP3 file")?;

	Ok(())
}

pub fn encode_audio(
	samples: &[f32],
	channels: u16,
	options: &ExportOptions,
	output_path: &Path,
) -> Result<()> {
	match options.format {
		ExportFormat::Wav => encode_wav(samples, channels, options, output_path),
		ExportFormat::Mp3 => encode_mp3(samples, channels, options.sample_rate, output_path),
	}
}