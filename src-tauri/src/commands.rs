use crate::audio::{decode_file, detect_pitch, encode_wav, ExportOptions, ExportFormat};
use crate::types::{AudioInfo, PitchDetection};
use std::path::Path;

#[tauri::command]
pub fn analyze_audio(path: String) -> Result<(AudioInfo, Option<PitchDetection>), String> {
	let path = Path::new(&path);

	let decoded = decode_file(path)
		.map_err(|e| format!("Failed to decode audio: {}", e))?;

	let format = path.extension()
		.and_then(|e| e.to_str())
		.unwrap_or("unknown")
		.to_uppercase();

	let audio_info = AudioInfo {
		duration_secs: decoded.duration_secs,
		sample_rate: decoded.sample_rate,
		channels: decoded.channels,
		bit_depth: Some(16),
		format,
	};

	// Detect pitch from mono mix
	let mono_samples: Vec<f32> = if decoded.channels > 1 {
		decoded.samples
			.chunks(decoded.channels as usize)
			.map(|chunk| chunk.iter().sum::<f32>() / decoded.channels as f32)
			.collect()
	} else {
		decoded.samples
	};

	let pitch = detect_pitch(&mono_samples, decoded.sample_rate);

	Ok((audio_info, pitch))
}

#[tauri::command]
pub fn process_audio(
	input_path: String,
	output_path: String,
	pitch_semitones: f32,
	tempo_ratio: f32,
	format: String,
) -> Result<AudioInfo, String> {
	let input = Path::new(&input_path);
	let output = Path::new(&output_path);

	let decoded = decode_file(input)
		.map_err(|e| format!("Failed to decode: {}", e))?;

	// For now, skip pitch/tempo processing (signalsmith-stretch needs integration)
	// Just re-encode the audio
	let _ = (pitch_semitones, tempo_ratio); // Suppress unused warnings

	let export_format = match format.to_lowercase().as_str() {
		"wav" => ExportFormat::Wav,
		_ => ExportFormat::Wav,
	};

	let export_options = ExportOptions {
		format: export_format,
		sample_rate: decoded.sample_rate,
		bit_depth: 24,
	};

	encode_wav(&decoded.samples, decoded.channels, &export_options, output)
		.map_err(|e| format!("Failed to encode: {}", e))?;

	Ok(AudioInfo {
		duration_secs: decoded.duration_secs,
		sample_rate: decoded.sample_rate,
		channels: decoded.channels,
		bit_depth: Some(24),
		format,
	})
}

#[tauri::command]
pub fn preview_audio(
	_input_path: String,
	_pitch_semitones: f32,
	_tempo_ratio: f32,
) -> Result<(), String> {
	// Placeholder for preview functionality
	Ok(())
}