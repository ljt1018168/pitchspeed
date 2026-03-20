import { writable, derived } from 'svelte/store';
import type { AudioInfo, PitchDetection } from '$lib/commands';

export interface AudioState {
	filePath: string | null;
	fileName: string | null;
	audioInfo: AudioInfo | null;
	pitchDetection: PitchDetection | null;
	pitchShift: number;
	tempoRatio: number;
	isProcessing: boolean;
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
		setError: (error: string | null) =>
			update((s) => ({ ...s, error })),
		reset: () => set(initialState)
	};
}

export const audioStore = createAudioStore();

export const targetNote = derived(audioStore, ($audio) => {
	if (!$audio.pitchDetection) return null;

	const notes = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
	const current = $audio.pitchDetection.note;

	const match = current.match(/^([A-G]#?)(\d+)$/);
	if (!match) return current;

	const noteName = match[1];
	const octave = parseInt(match[2]);
	const noteIndex = notes.indexOf(noteName);

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