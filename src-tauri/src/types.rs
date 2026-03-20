use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
	pub duration_secs: f64,
	pub sample_rate: u32,
	pub channels: u16,
	pub bit_depth: Option<u16>,
	pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitchDetection {
	pub note: String,
	pub semitone_offset: i32,
	pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingParams {
	pub pitch_semitones: f32,
	pub tempo_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingProgress {
	pub stage: String,
	pub progress: f32,
}