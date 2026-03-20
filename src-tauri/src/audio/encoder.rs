use anyhow::{Context, Result};
use hound::{WavSpec, WavWriter};
use std::path::Path;

pub struct ExportOptions {
	pub format: ExportFormat,
	pub sample_rate: u32,
	pub bit_depth: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
	Wav,
	Mp3,
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

pub fn normalize_samples(
	samples: &[f32],
	target_sample_rate: u32,
	source_sample_rate: u32,
) -> Vec<f32> {
	if target_sample_rate == source_sample_rate {
		return samples.to_vec();
	}

	// Simple linear interpolation resampling
	let ratio = source_sample_rate as f64 / target_sample_rate as f64;
	let output_len = (samples.len() as f64 / ratio) as usize;
	let mut output = Vec::with_capacity(output_len);

	for i in 0..output_len {
		let src_pos = i as f64 * ratio;
		let src_idx = src_pos as usize;
		let frac = src_pos - src_idx as f64;

		let sample = if src_idx + 1 < samples.len() {
			samples[src_idx] * (1.0 - frac as f32) + samples[src_idx + 1] * frac as f32
		} else {
			samples[src_idx.min(samples.len() - 1)]
		};
		output.push(sample);
	}

	output
}