# PitchSpeed Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a cross-platform desktop app for audio pitch shifting and tempo adjustment with glassmorphism UI.

**Architecture:** Tauri 2 app with SvelteKit frontend for UI and Rust backend for audio processing. Rubber Band library for high-quality pitch/tempo independent processing.

**Tech Stack:** Tauri 2, SvelteKit, TypeScript, Rust, rubberband-rs, pitch-detection, symphonia, hound

---

## File Structure

```
pitchspeed/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── analyze.rs
│       │   ├── process.rs
│       │   └── preview.rs
│       ├── audio/
│       │   ├── mod.rs
│       │   ├── decoder.rs
│       │   ├── encoder.rs
│       │   └── pitch_detect.rs
│       └── types.rs
├── src/
│   ├── app.css
│   ├── app.html
│   ├── lib/
│   │   ├── commands.ts
│   │   ├── stores/
│   │   │   └── audio.ts
│   │   └── components/
│   │       ├── DropZone.svelte
│   │       ├── KeyDisplay.svelte
│   │       ├── ParameterSlider.svelte
│   │       ├── ActionButtons.svelte
│   │       └── LoadingSpinner.svelte
│   └── routes/
│       └── +page.svelte
├── static/
│   └── favicon.png
├── package.json
├── svelte.config.js
├── vite.config.ts
└── tsconfig.json
```

---

## Task 1: Project Scaffolding

**Files:**
- Create: `package.json`
- Create: `svelte.config.js`
- Create: `vite.config.ts`
- Create: `tsconfig.json`
- Create: `src/app.html`
- Create: `src/app.css`

- [ ] **Step 1: Initialize npm project with SvelteKit**

```bash
cd /Users/tonyliu/Codes/audio
npm create svelte@latest . -- --template skeleton --types typescript --no-eslint --no-prettier --no-playwright --no-vitest
```

Expected: SvelteKit project scaffolded

- [ ] **Step 2: Install frontend dependencies**

```bash
npm install @tauri-apps/api
```

Expected: Dependencies installed

- [ ] **Step 3: Create app.html**

Create `src/app.html`:

```html
<!doctype html>
<html lang="en">
	<head>
		<meta charset="utf-8" />
		<link rel="icon" href="%sveltekit.assets%/favicon.png" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />
		<title>PitchSpeed</title>
		%sveltekit.head%
	</head>
	<body data-sveltekit-preload-data="hover">
		<div style="display: contents">%sveltekit.body%</div>
	</body>
</html>
```

- [ ] **Step 4: Create app.css with glassmorphism theme**

Create `src/app.css`:

```css
:root {
	--bg-primary: #0f0f1a;
	--bg-secondary: #1a1a2e;
	--bg-card: rgba(255, 255, 255, 0.05);
	--border-color: rgba(255, 255, 255, 0.1);
	--text-primary: #ffffff;
	--text-secondary: rgba(255, 255, 255, 0.6);
	--text-muted: rgba(255, 255, 255, 0.4);
	--accent-primary: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
	--accent-secondary: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
	--font-display: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
}

* {
	margin: 0;
	padding: 0;
	box-sizing: border-box;
}

html, body {
	height: 100%;
	overflow: hidden;
}

body {
	font-family: var(--font-display);
	background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
	color: var(--text-primary);
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
}

.glass-card {
	background: var(--bg-card);
	backdrop-filter: blur(20px);
	-webkit-backdrop-filter: blur(20px);
	border: 1px solid var(--border-color);
	border-radius: 16px;
}

.gradient-text-primary {
	background: var(--accent-primary);
	-webkit-background-clip: text;
	-webkit-text-fill-color: transparent;
	background-clip: text;
}

.gradient-text-secondary {
	background: var(--accent-secondary);
	-webkit-background-clip: text;
	-webkit-text-fill-color: transparent;
	background-clip: text;
}

.gradient-btn-primary {
	background: var(--accent-primary);
	border: none;
	border-radius: 10px;
	color: white;
	font-weight: 600;
	cursor: pointer;
	transition: transform 0.2s, box-shadow 0.2s;
}

.gradient-btn-primary:hover {
	transform: translateY(-1px);
	box-shadow: 0 4px 20px rgba(102, 126, 234, 0.4);
}

.gradient-btn-secondary {
	background: rgba(255, 255, 255, 0.1);
	border: 1px solid var(--border-color);
	border-radius: 10px;
	color: white;
	cursor: pointer;
	transition: background 0.2s;
}

.gradient-btn-secondary:hover {
	background: rgba(255, 255, 255, 0.15);
}

@keyframes pulse {
	0%, 100% { opacity: 1; }
	50% { opacity: 0.5; }
}

@keyframes spin {
	from { transform: rotate(0deg); }
	to { transform: rotate(360deg); }
}
```

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "chore: scaffold SvelteKit project with glassmorphism theme"
```

---

## Task 2: Tauri Backend Setup

**Files:**
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/capabilities/default.json`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`
- Create: `src-tauri/src/types.rs`

- [ ] **Step 1: Initialize Tauri**

```bash
npm install -D @tauri-apps/cli
npx tauri init
```

Select: CI mode, app name "PitchSpeed", window settings default.

- [ ] **Step 2: Update Cargo.toml with audio dependencies**

Edit `src-tauri/Cargo.toml`:

```toml
[package]
name = "pitchspeed"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["dialog"] }
tauri-plugin-dialog = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
symphonia = { version = "0.5", features = ["all"] }
hound = "3.5"
rubberband = "0.4"
pitch-detection = "0.4"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

- [ ] **Step 3: Configure tauri.conf.json**

Edit `src-tauri/tauri.conf.json`:

```json
{
	"productName": "PitchSpeed",
	"version": "0.1.0",
	"identifier": "com.pitchspeed.app",
	"build": {
		"beforeBuildCommand": "npm run build",
		"beforeDevCommand": "npm run dev",
		"frontendDist": "../build",
		"devUrl": "http://localhost:5173"
	},
	"app": {
		"windows": [
			{
				"title": "PitchSpeed",
				"width": 600,
				"height": 700,
				"resizable": true,
				"minWidth": 500,
				"minHeight": 600,
				"decorations": true,
				"transparent": false
			}
		],
		"security": {
			"csp": null
		}
	},
	"bundle": {
		"active": true,
		"targets": "all",
		"icon": [
			"icons/32x32.png",
			"icons/128x128.png",
			"icons/128x128@2x.png",
			"icons/icon.icns",
			"icons/icon.ico"
		],
		"windows": {
			"wix": {
				"language": "en-US"
			}
		},
		"macOS": {
			"minimumSystemVersion": "10.13"
		}
	}
}
```

- [ ] **Step 4: Create capabilities for file dialog**

Create `src-tauri/capabilities/default.json`:

```json
{
	"identifier": "default",
	"description": "Default capabilities for PitchSpeed",
	"windows": ["main"],
	"permissions": [
		"core:default",
		"dialog:default",
		"dialog:allow-open",
		"dialog:allow-save"
	]
}
```

- [ ] **Step 5: Create types.rs**

Create `src-tauri/src/types.rs`:

```rust
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
```

- [ ] **Step 6: Create lib.rs with module declarations**

Edit `src-tauri/src/lib.rs`:

```rust
mod commands;
mod audio;
mod types;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.invoke_handler(tauri::generate_handler![
			commands::analyze_audio,
			commands::process_audio,
			commands::preview_audio,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
```

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat: setup Tauri backend with audio dependencies"
```

---

## Task 3: Audio Decoder Module (Rust)

**Files:**
- Create: `src-tauri/src/audio/mod.rs`
- Create: `src-tauri/src/audio/decoder.rs`

- [ ] **Step 1: Create audio module**

Create `src-tauri/src/audio/mod.rs`:

```rust
pub mod decoder;
pub mod encoder;
pub mod pitch_detect;

pub use decoder::*;
pub use encoder::*;
pub use pitch_detect::*;
```

- [ ] **Step 2: Create decoder.rs**

Create `src-tauri/src/audio/decoder.rs`:

```rust
use anyhow::{Result, Context};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Probe;
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

	let probe = Probe::open(mss)?
		.format(FormatOptions::default(), MetadataOptions::default())?;

	let format = probe.format;

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
			let spec = decoded.spec();
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
```

- [ ] **Step 3: Commit**

```bash
git add .
git commit -m "feat: add audio decoder module using symphonia"
```

---

## Task 4: Pitch Detection Module (Rust)

**Files:**
- Create: `src-tauri/src/audio/pitch_detect.rs`

- [ ] **Step 1: Create pitch detection module**

Create `src-tauri/src/audio/pitch_detect.rs`:

```rust
use crate::types::PitchDetection;
use pitch_detection::{AutoCorrelation, McLeod};

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

pub fn detect_pitch(samples: &[f32], sample_rate: u32) -> Option<PitchDetection> {
	let signal_len = samples.len();
	let max_freq = sample_rate as f64 / 2.0;

	// Use McLeod pitch detection method
	let pitch = McLeod::new(signal_len, sample_rate as usize)
		.get_pitch(samples, signal_len, sample_rate as usize, 0.5, max_freq)?;

	let freq = pitch.frequency;

	// Convert frequency to MIDI note number
	// A4 (440 Hz) = MIDI note 69
	let midi_note = 12.0 * (freq / 440.0).log2() + 69.0;
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
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add pitch detection module with McLeod algorithm"
```

---

## Task 5: Audio Encoder Module (Rust)

**Files:**
- Create: `src-tauri/src/audio/encoder.rs`

- [ ] **Step 1: Create encoder module**

Create `src-tauri/src/audio/encoder.rs`:

```rust
use anyhow::{Result, Context};
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
		writer.write_sample(sample_int)
			.with_context(|| "Failed to write sample")?;
	}

	writer.finalize()
		.with_context(|| "Failed to finalize WAV file")?;

	Ok(())
}

pub fn normalize_samples(samples: &[f32], target_sample_rate: u32, source_sample_rate: u32) -> Vec<f32> {
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
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add audio encoder module for WAV export"
```

---

## Task 6: Tauri Commands (Rust)

**Files:**
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/analyze.rs`
- Create: `src-tauri/src/commands/process.rs`
- Create: `src-tauri/src/commands/preview.rs`

- [ ] **Step 1: Create commands module**

Create `src-tauri/src/commands/mod.rs`:

```rust
mod analyze;
mod process;
mod preview;

pub use analyze::*;
pub use process::*;
pub use preview::*;
```

- [ ] **Step 2: Create analyze command**

Create `src-tauri/src/commands/analyze.rs`:

```rust
use crate::audio::{decode_file, detect_pitch};
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
		bit_depth: Some(16), // Default for decoded audio
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
```

- [ ] **Step 3: Create process command**

Create `src-tauri/src/commands/process.rs`:

```rust
use crate::audio::{decode_file, encode_wav, normalize_samples, ExportFormat, ExportOptions};
use crate::types::AudioInfo;
use std::path::Path;

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

	// Decode input
	let mut decoded = decode_file(input)
		.map_err(|e| format!("Failed to decode: {}", e))?;

	// Apply pitch shift and time stretch using rubberband
	// Note: rubberband processes in-place for efficiency
	if pitch_semitones != 0.0 || tempo_ratio != 1.0 {
		apply_rubberband(&mut decoded.samples, decoded.channels, decoded.sample_rate, pitch_semitones, tempo_ratio)
			.map_err(|e| format!("Processing failed: {}", e))?;
	}

	// Determine export format
	let export_format = match format.to_lowercase().as_str() {
		"wav" => ExportFormat::Wav,
		"mp3" => ExportFormat::Mp3,
		_ => ExportFormat::Wav,
	};

	let export_options = ExportOptions {
		format: export_format,
		sample_rate: decoded.sample_rate,
		bit_depth: 24,
	};

	// Encode output
	encode_wav(&decoded.samples, decoded.channels, &export_options, output)
		.map_err(|e| format!("Failed to encode: {}", e))?;

	// Update duration after tempo change
	decoded.duration_secs /= tempo_ratio as f64;

	Ok(AudioInfo {
		duration_secs: decoded.duration_secs,
		sample_rate: decoded.sample_rate,
		channels: decoded.channels,
		bit_depth: Some(24),
		format,
	})
}

fn apply_rubberband(
	samples: &mut Vec<f32>,
	channels: u16,
	sample_rate: u32,
	pitch_semitones: f32,
	tempo_ratio: f32,
) -> anyhow::Result<()> {
	use rubberband::{RubberBand, RubberBandOptions};

	let pitch_scale = 2.0_f32.powf(pitch_semitones / 12.0);

	let mut options = RubberBandOptions::default();
	options.set_pitch_scale(pitch_scale);
	options.set_time_ratio(tempo_ratio);

	let mut rubberband = RubberBand::new(sample_rate as usize, channels as usize, options);

	// Process samples
	rubberband.study(samples.as_slice(), false);
	rubberband.process(samples.as_slice(), false);

	// Retrieve processed samples
	samples.clear();
	let mut buffer = vec![0.0f32; 1024 * channels as usize];

	while rubberband.available() > 0 {
		let retrieved = rubberband.retrieve(&mut buffer);
		samples.extend_from_slice(&buffer[..retrieved * channels as usize]);
	}

	Ok(())
}
```

- [ ] **Step 4: Create preview command**

Create `src-tauri/src/commands/preview.rs`:

```rust
use crate::audio::decode_file;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

static PREVIEW_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

#[tauri::command]
pub fn preview_audio(
	input_path: String,
	pitch_semitones: f32,
	tempo_ratio: f32,
) -> Result<(), String> {
	// Stop any existing preview
	stop_preview();

	let decoded = decode_file(std::path::Path::new(&input_path))
		.map_err(|e| format!("Failed to decode: {}", e))?;

	// In a real implementation, this would:
	// 1. Apply pitch/tempo changes
	// 2. Stream to audio output device
	// 3. Support stop/pause/resume

	// For now, this is a placeholder that signals the preview system
	// The frontend will handle actual audio playback via Web Audio API

	Ok(())
}

#[tauri::command]
pub fn stop_preview() -> () {
	PREVIEW_STOP.store(true, Ordering::SeqCst);
}

#[tauri::command]
pub fn is_preview_playing() -> bool {
	!PREVIEW_STOP.load(Ordering::SeqCst)
}
```

- [ ] **Step 5: Update lib.rs to include new commands**

Edit `src-tauri/src/lib.rs`:

```rust
mod commands;
mod audio;
mod types;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.invoke_handler(tauri::generate_handler![
			commands::analyze_audio,
			commands::process_audio,
			commands::preview_audio,
			commands::stop_preview,
			commands::is_preview_playing,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
```

- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "feat: add Tauri commands for audio analysis and processing"
```

---

## Task 7: Frontend Stores and Commands

**Files:**
- Create: `src/lib/commands.ts`
- Create: `src/lib/stores/audio.ts`

- [ ] **Step 1: Create Tauri command bindings**

Create `src/lib/commands.ts`:

```typescript
import { invoke } from '@tauri-apps/api/core';

export interface AudioInfo {
	duration_secs: number;
	sample_rate: number;
	channels: number;
	bit_depth: number | null;
	format: string;
}

export interface PitchDetection {
	note: string;
	semitone_offset: number;
	confidence: number;
}

export interface ProcessingProgress {
	stage: string;
	progress: number;
}

export async function analyzeAudio(path: string): Promise<[AudioInfo, PitchDetection | null]> {
	return invoke<[AudioInfo, PitchDetection | null]>('analyze_audio', { path });
}

export async function processAudio(
	inputPath: string,
	outputPath: string,
	pitchSemitones: number,
	tempoRatio: number,
	format: 'wav' | 'mp3'
): Promise<AudioInfo> {
	return invoke<AudioInfo>('process_audio', {
		inputPath,
		outputPath,
		pitchSemitones,
		tempoRatio,
		format
	});
}

export async function previewAudio(
	inputPath: string,
	pitchSemitones: number,
	tempoRatio: number
): Promise<void> {
	return invoke('preview_audio', { inputPath, pitchSemitones, tempoRatio });
}

export async function stopPreview(): Promise<void> {
	return invoke('stop_preview');
}
```

- [ ] **Step 2: Create audio store**

Create `src/lib/stores/audio.ts`:

```typescript
import { writable, derived } from 'svelte/store';
import type { AudioInfo, PitchDetection } from '$lib/commands';

export interface AudioState {
	filePath: string | null;
	fileName: string | null;
	audioInfo: AudioInfo | null;
	pitchDetection: PitchDetection | null;
	pitchShift: number; // -24 to +24 semitones
	tempoRatio: number; // 0.1 to 5.0
	isProcessing: boolean;
	isPlaying: boolean;
	error: string | null;
}

function createAudioStore() {
	const initialState: AudioState = {
		filePath: null,
		fileName: null,
		audioInfo: null,
		pitchDetection: null,
		pitchShift: 0,
		tempoRatio: 1.0,
		isProcessing: false,
		isPlaying: false,
		error: null
	};

	const { subscribe, set, update } = writable<AudioState>(initialState);

	return {
		subscribe,
		setFile: (filePath: string, fileName: string) =>
			update((s) => ({ ...s, filePath, fileName, error: null })),
		setAudioInfo: (info: AudioInfo) =>
			update((s) => ({ ...s, audioInfo: info })),
		setPitchDetection: (detection: PitchDetection | null) =>
			update((s) => ({ ...s, pitchDetection: detection })),
		setPitchShift: (value: number) =>
			update((s) => ({ ...s, pitchShift: value })),
		setTempoRatio: (value: number) =>
			update((s) => ({ ...s, tempoRatio: value })),
		setProcessing: (isProcessing: boolean) =>
			update((s) => ({ ...s, isProcessing })),
		setPlaying: (isPlaying: boolean) =>
			update((s) => ({ ...s, isPlaying })),
		setError: (error: string | null) =>
			update((s) => ({ ...s, error })),
		reset: () => set(initialState)
	};
}

export const audioStore = createAudioStore();

// Derived stores for computed values
export const targetNote = derived(audioStore, ($audio) => {
	if (!$audio.pitchDetection) return null;

	const notes = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
	const current = $audio.pitchDetection.note;

	// Parse current note (e.g., "A4" -> { note: "A", octave: 4 })
	const match = current.match(/^([A-G]#?)(\d+)$/);
	if (!match) return current;

	const noteName = match[1];
	const octave = parseInt(match[2]);
	const noteIndex = notes.indexOf(noteName);

	// Calculate new note after pitch shift
	const totalSemitones = noteIndex + Math.round($audio.pitchShift);
	const newNoteIndex = ((totalSemitones % 12) + 12) % 12;
	const octaveChange = Math.floor(totalSemitones / 12);
	const newOctave = octave + octaveChange;

	return `${notes[newNoteIndex]}${newOctave}`;
});

export const estimatedDuration = derived(audioStore, ($audio) => {
	if (!$audio.audioInfo) return null;
	return $audio.audioInfo.duration_secs / $audio.tempoRatio;
});
```

- [ ] **Step 3: Commit**

```bash
git add .
git commit -m "feat: add frontend stores and Tauri command bindings"
```

---

## Task 8: DropZone Component

**Files:**
- Create: `src/lib/components/DropZone.svelte`

- [ ] **Step 1: Create DropZone component**

Create `src/lib/components/DropZone.svelte`:

```svelte
<script lang="ts">
	import { audioStore } from '$lib/stores/audio';
	import { analyzeAudio } from '$lib/commands';
	import { open } from '@tauri-apps/plugin-dialog';

	let isDragging = false;
	let isLoading = false;

	const acceptedTypes = ['audio/mpeg', 'audio/wav', 'audio/x-m4a', 'audio/ogg', 'audio/flac', 'audio/aac'];

	async function handleFileDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;

		const file = event.dataTransfer?.files[0];
		if (file) {
			await processFile(file.path || file.name);
		}
	}

	async function handleClick() {
		const selected = await open({
			multiple: false,
			filters: [
				{
					name: 'Audio',
					extensions: ['mp3', 'wav', 'm4a', 'ogg', 'flac', 'aac']
				}
			]
		});

		if (selected && typeof selected === 'string') {
			await processFile(selected);
		}
	}

	async function processFile(path: string) {
		isLoading = true;
		audioStore.setFile(path, path.split('/').pop() || path.split('\\').pop() || path);

		try {
			const [info, pitch] = await analyzeAudio(path);
			audioStore.setAudioInfo(info);
			audioStore.setPitchDetection(pitch);
		} catch (err) {
			audioStore.setError(`Failed to analyze audio: ${err}`);
		} finally {
			isLoading = false;
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		isDragging = true;
	}

	function handleDragLeave() {
		isDragging = false;
	}
</script>

<div
	class="drop-zone"
	class:dragging={isDragging}
	class:loading={isLoading}
	on:drop={handleFileDrop}
	on:dragover={handleDragOver}
	on:dragleave={handleDragLeave}
	on:click={handleClick}
	role="button"
	tabindex="0"
>
	{#if isLoading}
		<div class="loading-content">
			<div class="spinner"></div>
			<p>Analyzing audio...</p>
		</div>
	{:else}
		<div class="drop-content">
			<svg class="upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
				<polyline points="17 8 12 3 7 8" />
				<line x1="12" y1="3" x2="12" y2="15" />
			</svg>
			<p class="main-text">Drop audio file here</p>
			<p class="sub-text">or click to browse</p>
			<p class="formats">MP3, WAV, M4A, OGG, FLAC, AAC</p>
		</div>
	{/if}
</div>

<style>
	.drop-zone {
		background: var(--bg-card);
		border: 2px dashed var(--border-color);
		border-radius: 16px;
		padding: 40px 20px;
		text-align: center;
		cursor: pointer;
		transition: all 0.2s ease;
		min-height: 160px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.drop-zone:hover,
	.drop-zone.dragging {
		border-color: #667eea;
		background: rgba(102, 126, 234, 0.1);
	}

	.drop-zone.loading {
		pointer-events: none;
	}

	.upload-icon {
		width: 48px;
		height: 48px;
		color: var(--text-secondary);
		margin-bottom: 16px;
	}

	.main-text {
		font-size: 16px;
		font-weight: 500;
		color: var(--text-primary);
		margin-bottom: 4px;
	}

	.sub-text {
		font-size: 14px;
		color: var(--text-secondary);
		margin-bottom: 8px;
	}

	.formats {
		font-size: 12px;
		color: var(--text-muted);
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--border-color);
		border-top-color: #667eea;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	.loading-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		color: var(--text-secondary);
	}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add DropZone component for file upload"
```

---

## Task 9: KeyDisplay Component

**Files:**
- Create: `src/lib/components/KeyDisplay.svelte`

- [ ] **Step 1: Create KeyDisplay component**

Create `src/lib/components/KeyDisplay.svelte`:

```svelte
<script lang="ts">
	import { audioStore, targetNote } from '$lib/stores/audio';

	$: hasFile = $audioStore.filePath !== null;
	$: pitch = $audioStore.pitchDetection;
	$: target = $targetNote;
	$: pitchShift = $audioStore.pitchShift;
</script>

<div class="key-display" class:visible={hasFile}>
	{#if hasFile && pitch}
		<div class="keys-grid">
			<div class="key-card original">
				<div class="label">Original Key</div>
				<div class="note gradient-text-primary">
					{pitch.note}
				</div>
				<div class="offset">
					{pitch.semitone_offset >= 0 ? '+' : ''}{pitch.semitone_offset} semitones
				</div>
				<div class="confidence">
					Confidence: {Math.round(pitch.confidence * 100)}%
				</div>
			</div>

			<div class="key-card target">
				<div class="label">Target Key</div>
				<div class="note gradient-text-secondary">
					{target || pitch.note}
				</div>
				<div class="offset">
					{pitchShift >= 0 ? '+' : ''}{pitchShift.toFixed(1)} semitones
				</div>
			</div>
		</div>
	{:else if hasFile}
		<div class="no-detection">
			<p>Could not detect pitch</p>
			<p class="hint">Try a different audio file</p>
		</div>
	{:else}
		<div class="placeholder">
			<p>Upload an audio file to see key information</p>
		</div>
	{/if}
</div>

<style>
	.key-display {
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.key-display.visible {
		opacity: 1;
	}

	.keys-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.key-card {
		background: var(--bg-card);
		backdrop-filter: blur(20px);
		border: 1px solid var(--border-color);
		border-radius: 16px;
		padding: 24px;
		text-align: center;
	}

	.label {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 1px;
		color: var(--text-muted);
		margin-bottom: 8px;
	}

	.note {
		font-size: 48px;
		font-weight: 700;
		line-height: 1;
		margin-bottom: 8px;
		transition: transform 0.2s ease;
	}

	.offset {
		font-size: 14px;
		color: var(--text-secondary);
		margin-bottom: 4px;
	}

	.confidence {
		font-size: 11px;
		color: var(--text-muted);
	}

	.no-detection, .placeholder {
		text-align: center;
		padding: 40px;
		color: var(--text-secondary);
	}

	.hint {
		font-size: 12px;
		color: var(--text-muted);
		margin-top: 8px;
	}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add KeyDisplay component showing original and target keys"
```

---

## Task 10: ParameterSlider Component

**Files:**
- Create: `src/lib/components/ParameterSlider.svelte`

- [ ] **Step 1: Create ParameterSlider component**

Create `src/lib/components/ParameterSlider.svelte`:

```svelte
<script lang="ts">
	export let label: string;
	export let value: number;
	export let min: number;
	export let max: number;
	export let step: number = 0.1;
	export let unit: string = '';
	export let gradient: 'primary' | 'secondary' = 'primary';
	export let formatFn: (v: number) => string = (v) => v.toFixed(1);

	const percentage = (($value - $min) / ($max - $min)) * 100;

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		value = parseFloat(target.value);
	}

	function handleKeydown(event: KeyboardEvent) {
		const target = event.target as HTMLInputElement;
		const currentValue = parseFloat(target.value);

		if (event.key === 'ArrowUp' || event.key === 'ArrowRight') {
			event.preventDefault();
			value = Math.min($max, currentValue + $step);
		} else if (event.key === 'ArrowDown' || event.key === 'ArrowLeft') {
			event.preventDefault();
			value = Math.max($min, currentValue - $step);
		}
	}

	function reset() {
		if (label === 'Pitch Shift') {
			value = 0;
		} else {
			value = 1.0;
		}
	}
</script>

<div class="slider-container">
	<div class="slider-header">
		<span class="slider-label">{label}</span>
		<span class="slider-value">{formatFn($value)}{unit}</span>
	</div>

	<div class="slider-wrapper">
		<input
			type="range"
			min={$min}
			max={$max}
			step={$step}
			value={$value}
			on:input={handleInput}
			on:keydown={handleKeydown}
			class="slider"
			class:gradient-primary={$gradient === 'primary'}
			class:gradient-secondary={$gradient === 'secondary'}
		/>
	</div>

	<div class="slider-labels">
		<span>{$min}{unit}</span>
		<span class="reset-btn" on:click={reset} role="button" tabindex="0">Reset</span>
		<span>{$max}{unit}</span>
	</div>
</div>

<style>
	.slider-container {
		background: var(--bg-card);
		backdrop-filter: blur(20px);
		border: 1px solid var(--border-color);
		border-radius: 16px;
		padding: 16px;
	}

	.slider-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	.slider-label {
		font-size: 13px;
		color: var(--text-secondary);
	}

	.slider-value {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-primary);
	}

	.slider-wrapper {
		position: relative;
		height: 6px;
	}

	.slider {
		-webkit-appearance: none;
		appearance: none;
		width: 100%;
		height: 6px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 3px;
		outline: none;
		cursor: pointer;
	}

	.slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		cursor: pointer;
		transition: transform 0.1s ease, box-shadow 0.2s ease;
	}

	.slider::-webkit-slider-thumb:hover {
		transform: scale(1.1);
	}

	.slider.gradient-primary::-webkit-slider-thumb {
		background: linear-gradient(135deg, #667eea, #764ba2);
		box-shadow: 0 2px 8px rgba(102, 126, 234, 0.5);
	}

	.slider.gradient-secondary::-webkit-slider-thumb {
		background: linear-gradient(135deg, #f093fb, #f5576c);
		box-shadow: 0 2px 8px rgba(245, 87, 108, 0.5);
	}

	.slider::-moz-range-thumb {
		width: 18px;
		height: 18px;
		border-radius: 50%;
		border: none;
		cursor: pointer;
	}

	.slider.gradient-primary::-moz-range-thumb {
		background: linear-gradient(135deg, #667eea, #764ba2);
	}

	.slider.gradient-secondary::-moz-range-thumb {
		background: linear-gradient(135deg, #f093fb, #f5576c);
	}

	.slider-labels {
		display: flex;
		justify-content: space-between;
		font-size: 10px;
		color: var(--text-muted);
		margin-top: 8px;
	}

	.reset-btn {
		color: var(--text-secondary);
		cursor: pointer;
		padding: 2px 8px;
		border-radius: 4px;
		transition: background 0.2s;
	}

	.reset-btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add ParameterSlider component for pitch and tempo control"
```

---

## Task 11: ActionButtons Component

**Files:**
- Create: `src/lib/components/ActionButtons.svelte`

- [ ] **Step 1: Create ActionButtons component**

Create `src/lib/components/ActionButtons.svelte`:

```svelte
<script lang="ts">
	import { audioStore, estimatedDuration } from '$lib/stores/audio';
	import { processAudio } from '$lib/commands';
	import { save } from '@tauri-apps/plugin-dialog';

	export let onPreview: () => void;

	$: hasFile = $audioStore.filePath !== null;
	$: isProcessing = $audioStore.isProcessing;
	$: isPlaying = $audioStore.isPlaying;

	async function handleExport() {
		if (!$audioStore.filePath) return;

		const fileName = $audioStore.fileName || 'audio';
		const pitchStr = $audioStore.pitchShift >= 0
			? `+${$audioStore.pitchShift.toFixed(1)}`
			: $audioStore.pitchShift.toFixed(1);
		const tempoStr = `${$audioStore.tempoRatio.toFixed(2)}x`;
		const defaultName = `${fileName}_pitch${pitchStr}_tempo${tempoStr}`;

		const outputPath = await save({
			defaultPath: defaultName,
			filters: [
				{ name: 'WAV', extensions: ['wav'] },
				{ name: 'MP3', extensions: ['mp3'] }
			]
		});

		if (!outputPath) return;

		audioStore.setProcessing(true);

		try {
			const format = outputPath.endsWith('.mp3') ? 'mp3' : 'wav';
			await processAudio(
				$audioStore.filePath,
				outputPath,
				$audioStore.pitchShift,
				$audioStore.tempoRatio,
				format
			);
		} catch (err) {
			audioStore.setError(`Failed to process audio: ${err}`);
		} finally {
			audioStore.setProcessing(false);
		}
	}

	function handlePreview() {
		audioStore.setPlaying(!$audioStore.isPlaying);
		onPreview();
	}
</script>

<div class="actions">
	<button
		class="btn-secondary"
		disabled={!hasFile || isProcessing}
		on:click={handlePreview}
	>
		{#if isPlaying}
			<svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
				<rect x="6" y="4" width="4" height="16" rx="1" />
				<rect x="14" y="4" width="4" height="16" rx="1" />
			</svg>
			Stop
		{:else}
			<svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
				<path d="M8 5v14l11-7z" />
			</svg>
			Preview [Space]
		{/if}
	</button>

	<button
		class="btn-primary"
		disabled={!hasFile || isProcessing}
		on:click={handleExport}
	>
		{#if isProcessing}
			<div class="btn-spinner"></div>
			Processing...
		{:else}
			Convert & Export
		{/if}
	</button>
</div>

<style>
	.actions {
		display: flex;
		gap: 12px;
	}

	.btn-secondary {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid var(--border-color);
		border-radius: 10px;
		color: white;
		font-size: 14px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.15);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		flex: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px;
		background: linear-gradient(135deg, #667eea, #764ba2);
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 20px rgba(102, 126, 234, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	svg {
		flex-shrink: 0;
	}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add ActionButtons component for preview and export"
```

---

## Task 12: Main Page Assembly

**Files:**
- Create: `src/routes/+page.svelte`

- [ ] **Step 1: Create main page**

Create `src/routes/+page.svelte`:

```svelte
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { audioStore, estimatedDuration } from '$lib/stores/audio';
	import { previewAudio, stopPreview } from '$lib/commands';
	import DropZone from '$lib/components/DropZone.svelte';
	import KeyDisplay from '$lib/components/KeyDisplay.svelte';
	import ParameterSlider from '$lib/components/ParameterSlider.svelte';
	import ActionButtons from '$lib/components/ActionButtons.svelte';

	let pitchShift = 0;
	let tempoRatio = 1.0;

	// Sync with store
	$: {
		audioStore.setPitchShift(pitchShift);
		audioStore.setTempoRatio(tempoRatio);
	}

	function handlePreview() {
		if (!$audioStore.filePath) return;

		if ($audioStore.isPlaying) {
			stopPreview();
		} else {
			previewAudio($audioStore.filePath, pitchShift, tempoRatio);
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.code === 'Space' && $audioStore.filePath) {
			event.preventDefault();
			handlePreview();
		}
	}

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeydown);
	});

	onDestroy(() => {
		window.removeEventListener('keydown', handleKeydown);
	});
</script>

<svelte:head>
	<title>PitchSpeed</title>
</svelte:head>

<main>
	<header>
		<h1>PitchSpeed</h1>
		<p class="tagline">Pitch & Tempo Control</p>
	</header>

	<section class="upload-section">
		<DropZone />
	</section>

	{#if $audioStore.filePath}
		<section class="info-section">
			<div class="file-info">
				<span class="file-name">{$audioStore.fileName}</span>
				{#if $audioStore.audioInfo}
					<span class="file-details">
						{formatDuration($audioStore.audioInfo.duration_secs)} · {$audioStore.audioInfo.sample_rate} Hz · {$audioStore.audioInfo.channels}ch
					</span>
				{/if}
			</div>
		</section>

		<section class="display-section">
			<KeyDisplay />
		</section>

		<section class="controls-section">
			<ParameterSlider
				label="Pitch Shift"
				bind:value={pitchShift}
				min={-24}
				max={24}
				step={0.5}
				unit=" st"
				gradient="primary"
			/>

			<div class="spacer"></div>

			<ParameterSlider
				label="Tempo"
				bind:value={tempoRatio}
				min={0.1}
				max={5.0}
				step={0.01}
				unit="x"
				gradient="secondary"
				formatFn={(v) => v.toFixed(2)}
			/>

			{#if $audioStore.audioInfo}
				<div class="duration-info">
					<span class="original">Original: {formatDuration($audioStore.audioInfo.duration_secs)}</span>
					<span class="new">New: {formatDuration($estimatedDuration || $audioStore.audioInfo.duration_secs)}</span>
				</div>
			{/if}
		</section>

		<section class="actions-section">
			<ActionButtons onPreview={handlePreview} />
		</section>

		{#if $audioStore.error}
			<div class="error-message">
				{$audioStore.error}
			</div>
		{/if}
	{/if}
</main>

<style>
	main {
		max-width: 560px;
		margin: 0 auto;
		padding: 32px 24px;
		min-height: 100vh;
	}

	header {
		text-align: center;
		margin-bottom: 32px;
	}

	h1 {
		font-size: 28px;
		font-weight: 700;
		background: linear-gradient(135deg, #667eea, #764ba2);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		margin-bottom: 4px;
	}

	.tagline {
		font-size: 14px;
		color: var(--text-secondary);
	}

	.upload-section {
		margin-bottom: 24px;
	}

	.info-section {
		margin-bottom: 20px;
	}

	.file-info {
		text-align: center;
	}

	.file-name {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: var(--text-primary);
		margin-bottom: 4px;
	}

	.file-details {
		font-size: 12px;
		color: var(--text-muted);
	}

	.display-section {
		margin-bottom: 20px;
	}

	.controls-section {
		margin-bottom: 24px;
	}

	.spacer {
		height: 12px;
	}

	.duration-info {
		display: flex;
		justify-content: center;
		gap: 24px;
		margin-top: 12px;
		font-size: 12px;
		color: var(--text-secondary);
	}

	.new {
		color: #f5576c;
		font-weight: 500;
	}

	.actions-section {
		margin-bottom: 16px;
	}

	.error-message {
		background: rgba(245, 87, 108, 0.1);
		border: 1px solid rgba(245, 87, 108, 0.3);
		border-radius: 10px;
		padding: 12px 16px;
		font-size: 13px;
		color: #f5576c;
		text-align: center;
	}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: assemble main page with all components"
```

---

## Task 13: Build and Test

**Files:**
- Verify: All files compile
- Run: Dev server

- [ ] **Step 1: Build frontend**

```bash
npm run build
```

Expected: Frontend builds successfully

- [ ] **Step 2: Run Tauri dev**

```bash
npm run tauri dev
```

Expected: App launches with UI

- [ ] **Step 3: Test file upload**

- Drag an MP3 file onto the drop zone
- Verify metadata displays
- Verify pitch detection shows

- [ ] **Step 4: Test sliders**

- Move pitch slider
- Verify target key updates
- Move tempo slider
- Verify duration estimate updates

- [ ] **Step 5: Test export**

- Click Convert & Export
- Select save location
- Verify file is created
- Play output file to verify quality

- [ ] **Step 6: Build production**

```bash
npm run tauri build
```

Expected: Creates .dmg (macOS) or .exe (Windows)

- [ ] **Step 7: Verify bundle size**

```bash
# macOS
ls -la src-tauri/target/release/bundle/dmg/*.dmg

# Windows
ls -la src-tauri/target/release/bundle/msi/*.msi
```

Expected: macOS < 30MB, Windows < 25MB

---

## Verification Summary

1. [ ] Upload Test: Drag MP3, verify metadata displayed
2. [ ] Pitch Detection: Verify note detected with confidence
3. [ ] Pitch Shift: Verify target key updates in real-time
4. [ ] Tempo Change: Verify duration estimate updates
5. [ ] Export WAV: Verify file created with correct parameters
6. [ ] Performance: Verify processing completes quickly
7. [ ] Bundle Size: Verify .dmg < 30MB, .exe < 25MB
8. [ ] Preview: Spacebar toggles playback state