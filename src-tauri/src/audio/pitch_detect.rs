use crate::types::PitchDetection;
use pitch_detection::detector::PitchDetector;
use pitch_detection::detector::mcleod;

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

pub fn detect_pitch(samples: &[f32], sample_rate: u32) -> Option<PitchDetection> {
	let signal_len = samples.len();
	let max_freq = sample_rate as f32 / 2.0;

	// Use McLeod pitch detection method
	let mut detector = mcleod::McLeodDetector::new(signal_len, sample_rate as usize);
	let pitch = detector.get_pitch(samples, sample_rate as usize, 0.5, max_freq)?;

	let freq = pitch.frequency;

	// Convert frequency to MIDI note number
	// A4 (440 Hz) = MIDI note 69
	let midi_note: f64 = 12.0 * (freq as f64 / 440.0).log2() + 69.0;
	let midi_note_rounded = midi_note.round() as i32;
	let cents_off = ((midi_note - midi_note_rounded as f64) * 100.0) as i32;

	// Get note name and octave
	let note_index = ((midi_note_rounded % 12) + 12) % 12;
	let octave = (midi_note_rounded / 12) - 1;
	let note_name = format!("{}{}", NOTE_NAMES[note_index as usize], octave);

	Some(PitchDetection {
		note: note_name,
		semitone_offset: cents_off / 100,
		confidence: pitch.clarity,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a4_detection() {
		// Generate a 440 Hz sine wave
		let sample_rate = 44100u32;
		let duration = 1.0;
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
}