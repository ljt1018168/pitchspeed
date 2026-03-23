use crate::types::PitchDetection;
use pitch_detection::detector::PitchDetector;
use pitch_detection::detector::mcleod;
use std::collections::HashMap;

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

/// Frequency range for musical notes (C1 to B8)
const MIN_FREQ: f32 = 32.7;  // C1
const MAX_FREQ: f32 = 7902.0; // B8

/// Minimum samples needed for reliable detection
const MIN_SAMPLES: usize = 4096;

/// Analysis window size (in samples) - must be power of 2 for FFT efficiency
const WINDOW_SIZE: usize = 8192;

/// Overlap between windows (50%)
const WINDOW_HOP: usize = 4096;

/// Padding for FFT (typically same as signal length)
const PADDING: usize = 8192;

/// Detect pitch with enhanced accuracy using multi-segment analysis
pub fn detect_pitch(samples: &[f32], sample_rate: u32) -> Option<PitchDetection> {
	if samples.len() < MIN_SAMPLES {
		return None;
	}

	// Preprocess: normalize and remove DC offset
	let processed = preprocess_audio(samples);

	// Analyze multiple windows for robustness
	let pitch_estimates = analyze_windows(&processed, sample_rate);

	if pitch_estimates.is_empty() {
		return None;
	}

	// Find the most consistent pitch
	let best_estimate = find_dominant_pitch(&pitch_estimates)?;

	// Convert to note
	frequency_to_note(best_estimate.frequency, best_estimate.clarity)
}

/// Preprocess audio: normalize and remove DC offset
fn preprocess_audio(samples: &[f32]) -> Vec<f32> {
	if samples.is_empty() {
		return Vec::new();
	}

	// Calculate mean (DC offset)
	let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;

	// Calculate RMS for normalization
	let rms: f32 = (samples.iter().map(|s| (s - mean).powi(2)).sum::<f32>() / samples.len() as f32).sqrt();

	if rms < 1e-6 {
		return Vec::new();
	}

	// Remove DC offset and normalize
	samples.iter().map(|s| (s - mean) / rms).collect()
}

/// Analyze multiple windows and return pitch estimates
fn analyze_windows(samples: &[f32], sample_rate: u32) -> Vec<PitchEstimate> {
	let mut estimates = Vec::new();
	let total_len = samples.len();

	// Analyze multiple overlapping windows
	let mut pos = 0;
	while pos + WINDOW_SIZE <= total_len {
		let window = &samples[pos..pos + WINDOW_SIZE];

		if let Some(estimate) = detect_pitch_in_window(window, sample_rate) {
			estimates.push(estimate);
		}

		pos += WINDOW_HOP;

		// Limit number of windows to analyze (for performance)
		if estimates.len() >= 10 {
			break;
		}
	}

	estimates
}

#[derive(Debug, Clone)]
struct PitchEstimate {
	frequency: f32,
	clarity: f32,
}

/// Detect pitch in a single window
fn detect_pitch_in_window(samples: &[f32], sample_rate: u32) -> Option<PitchEstimate> {
	// Create detector with exact window size
	let mut detector = mcleod::McLeodDetector::new(WINDOW_SIZE, PADDING);

	// Try different threshold combinations for robustness
	let thresholds = [
		(0.001, 0.3),  // Very sensitive
		(0.005, 0.4),  // Sensitive
		(0.01, 0.5),   // Default
		(0.02, 0.6),   // Conservative
	];

	for (power_threshold, clarity_threshold) in thresholds {
		if let Some(pitch) = detector.get_pitch(samples, sample_rate as usize, power_threshold, clarity_threshold) {
			let freq = pitch.frequency;

			// Validate frequency is in musical range
			if freq >= MIN_FREQ && freq <= MAX_FREQ && pitch.clarity > 0.3 {
				return Some(PitchEstimate {
					frequency: freq,
					clarity: pitch.clarity,
				});
			}
		}
	}

	None
}

/// Find the most dominant/consistent pitch from multiple estimates
fn find_dominant_pitch(estimates: &[PitchEstimate]) -> Option<PitchEstimate> {
	if estimates.is_empty() {
		return None;
	}

	if estimates.len() == 1 {
		return Some(estimates[0].clone());
	}

	// Group estimates by nearest semitone
	let mut note_groups: HashMap<i32, Vec<&PitchEstimate>> = HashMap::new();

	for estimate in estimates {
		// Convert frequency to MIDI note number
		let midi_note = 12.0 * (estimate.frequency as f64 / 440.0).log2() + 69.0;
		let note_number = midi_note.round() as i32;

		note_groups.entry(note_number).or_default().push(estimate);
	}

	// Find the group with most estimates and highest average clarity
	let mut best_score: f32 = 0.0;
	let mut best_estimate: Option<PitchEstimate> = None;

	for (_note, group) in &note_groups {
		let count = group.len() as f32;
		let avg_clarity: f32 = group.iter().map(|e| e.clarity).sum::<f32>() / count;
		let avg_freq: f32 = group.iter().map(|e| e.frequency).sum::<f32>() / count;

		// Score based on count and clarity
		let score = count * avg_clarity;

		if score > best_score {
			best_score = score;
			best_estimate = Some(PitchEstimate {
				frequency: avg_freq,
				clarity: avg_clarity,
			});
		}
	}

	best_estimate
}

/// Convert frequency to musical note
fn frequency_to_note(frequency: f32, clarity: f32) -> Option<PitchDetection> {
	if frequency <= 0.0 || frequency.is_nan() || frequency.is_infinite() {
		return None;
	}

	// Convert frequency to MIDI note number
	// A4 (440 Hz) = MIDI note 69
	let midi_note: f64 = 12.0 * (frequency as f64 / 440.0).log2() + 69.0;
	let midi_note_rounded = midi_note.round() as i32;

	// Calculate cents off from nearest note
	let cents_off = ((midi_note - midi_note_rounded as f64) * 100.0) as i32;

	// Get note name and octave
	let note_index = ((midi_note_rounded % 12) + 12) % 12;
	let octave = (midi_note_rounded / 12) - 1;
	let note_name = format!("{}{}", NOTE_NAMES[note_index as usize], octave);

	Some(PitchDetection {
		note: note_name,
		semitone_offset: cents_off / 100,
		confidence: clarity,
	})
}

/// Detect the musical key of a song (for chordal/harmonic content)
#[allow(dead_code)]
pub fn detect_key(samples: &[f32], sample_rate: u32) -> Option<String> {
	if samples.len() < MIN_SAMPLES * 4 {
		return None;
	}

	// Analyze pitch distribution over the entire audio
	let processed = preprocess_audio(samples);

	// Collect pitch estimates
	let mut pitch_histogram: HashMap<i32, f32> = HashMap::new();

	let mut pos = 0;
	let hop = WINDOW_HOP * 2; // Larger hop for key detection
	while pos + WINDOW_SIZE <= processed.len() {
		let window = &processed[pos..pos + WINDOW_SIZE];
		let mut detector = mcleod::McLeodDetector::new(WINDOW_SIZE, PADDING);

		if let Some(pitch) = detector.get_pitch(window, sample_rate as usize, 0.01, 0.5) {
			if pitch.frequency >= MIN_FREQ && pitch.frequency <= MAX_FREQ {
				let midi_note = 12.0 * (pitch.frequency as f64 / 440.0).log2() + 69.0;
				let note_class = ((midi_note.round() as i32) % 12 + 12) % 12;
				*pitch_histogram.entry(note_class).or_default() += pitch.clarity;
			}
		}

		pos += hop;
		if pitch_histogram.len() >= 50 {
			break;
		}
	}

	// Simple key detection: find most common note
	// (A more sophisticated version would use Krumhansl-Schmuckler algorithm)
	pitch_histogram
		.into_iter()
		.max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
		.map(|(note_class, _)| {
			format!("{} major", NOTE_NAMES[note_class as usize])
		})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a4_detection() {
		let sample_rate = 44100u32;
		let duration = 2.0;
		let freq = 440.0;
		let samples: Vec<f32> = (0..(sample_rate as f64 * duration) as usize)
			.map(|i| (2.0 * std::f64::consts::PI * freq * i as f64 / sample_rate as f64).sin() as f32)
			.collect();

		let result = detect_pitch(&samples, sample_rate);
		assert!(result.is_some());

		let detection = result.unwrap();
		assert_eq!(detection.note, "A4");
		assert!(detection.confidence > 0.8);
	}

	#[test]
	fn test_c4_detection() {
		let sample_rate = 44100u32;
		let duration = 2.0;
		let freq = 261.63; // C4 (Middle C)
		let samples: Vec<f32> = (0..(sample_rate as f64 * duration) as usize)
			.map(|i| (2.0 * std::f64::consts::PI * freq * i as f64 / sample_rate as f64).sin() as f32)
			.collect();

		let result = detect_pitch(&samples, sample_rate);
		assert!(result.is_some());

		let detection = result.unwrap();
		assert_eq!(detection.note, "C4");
	}

	#[test]
	fn test_low_note_detection() {
		let sample_rate = 44100u32;
		let duration = 3.0;
		let freq = 82.41; // E2 (low E on guitar)
		let samples: Vec<f32> = (0..(sample_rate as f64 * duration) as usize)
			.map(|i| (2.0 * std::f64::consts::PI * freq * i as f64 / sample_rate as f64).sin() as f32)
			.collect();

		let result = detect_pitch(&samples, sample_rate);
		assert!(result.is_some());

		let detection = result.unwrap();
		// Should detect E2 or very close
		assert!(detection.note.starts_with('E'));
	}

	#[test]
	fn test_preprocess_audio() {
		// Test with DC offset
		let samples: Vec<f32> = vec![0.5, 0.6, 0.4, 0.5, 0.6, 0.4];
		let processed = preprocess_audio(&samples);

		// Should have zero mean
		let mean: f32 = processed.iter().sum::<f32>() / processed.len() as f32;
		assert!(mean.abs() < 0.001);
	}

	#[test]
	fn test_noise_handling() {
		let sample_rate = 44100u32;
		let duration = 2.0;

		// Pure noise should not produce false detection with high confidence
		let noise: Vec<f32> = (0..(sample_rate as f64 * duration) as usize)
			.map(|i| ((i as f64 * 0.001).sin() * 0.1 + (i as f64 * 0.0017).sin() * 0.15) as f32)
			.collect();

		// Should either return None or have low confidence
		if let Some(detection) = detect_pitch(&noise, sample_rate) {
			assert!(detection.confidence < 0.7);
		}
	}
}