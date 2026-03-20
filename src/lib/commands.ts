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