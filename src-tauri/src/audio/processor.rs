use crate::audio::decoder::DecodedAudio;

/// Apply pitch shift and time stretch to audio using signalsmith-stretch
///
/// # Arguments
/// * `audio` - Decoded audio to process
/// * `pitch_semitones` - Pitch shift in semitones (negative = lower, positive = higher)
/// * `tempo_ratio` - Tempo multiplier (1.0 = original speed, 2.0 = twice as fast)
///
/// # Returns
/// Processed audio samples
pub fn process_audio(audio: &DecodedAudio, pitch_semitones: f32, tempo_ratio: f32) -> Vec<f32> {
	// Skip processing if no changes needed
	if pitch_semitones == 0.0 && tempo_ratio == 1.0 {
		return audio.samples.clone();
	}

	let num_channels = audio.channels as usize;

	// signalsmith-stretch parameters
	// block_length and interval are typically 1024-4096 for good quality
	let block_length = 2048;
	let interval = 512;

	// Pitch shift factor: 2^(semitones/12)
	let pitch_factor = 2.0_f32.powf(pitch_semitones / 12.0);

	// Process each channel separately
	let mut output_channels: Vec<Vec<f32>> = Vec::with_capacity(num_channels);

	for ch in 0..num_channels {
		// Extract channel samples (deinterleave)
		let channel_samples: Vec<f32> = audio.samples
			.iter()
			.skip(ch)
			.step_by(num_channels)
			.copied()
			.collect();

		// Create stretch processor for this channel
		let mut stretch = signalsmith_stretch::Stretch::new(1, block_length, interval);
		stretch.set_transpose_factor(pitch_factor, None);

		// For time stretching, we need to process with different input/output sizes
		// tempo_ratio > 1.0 means faster = shorter output
		let output_size = ((channel_samples.len() as f32) / tempo_ratio).ceil() as usize;
		let mut output = vec![0.0f32; output_size];

		// Process the audio
		stretch.process(&channel_samples, &mut output);

		output_channels.push(output);
	}

	// Interleave output channels
	let output_len = output_channels[0].len();
	let mut output = Vec::with_capacity(output_len * num_channels);

	for i in 0..output_len {
		for ch in 0..num_channels {
			output.push(output_channels[ch][i]);
		}
	}

	output
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_no_op() {
		let audio = DecodedAudio {
			samples: vec![0.5, 0.5, 0.3, 0.3],
			sample_rate: 44100,
			channels: 2,
			duration_secs: 1.0,
		};

		let result = process_audio(&audio, 0.0, 1.0);
		assert_eq!(result, audio.samples);
	}

	#[test]
	fn test_pitch_shift() {
		let audio = DecodedAudio {
			samples: vec![0.5; 8192],
			sample_rate: 44100,
			channels: 1,
			duration_secs: 0.186,
		};

		let result = process_audio(&audio, 2.0, 1.0);
		// Should have roughly the same length for pitch shift only
		assert!((result.len() as f32 - audio.samples.len() as f32).abs() < 100.0);
	}

	#[test]
	fn test_tempo_change() {
		let audio = DecodedAudio {
			samples: vec![0.5; 8192],
			sample_rate: 44100,
			channels: 1,
			duration_secs: 0.186,
		};

		let result = process_audio(&audio, 0.0, 2.0);
		// Should be roughly half the length for 2x speed
		assert!(result.len() < audio.samples.len());
	}
}