<script lang="ts">
	import { audioStore } from '$lib/stores/audio';
	import { analyzeAudio } from '$lib/commands';
	import { open } from '@tauri-apps/plugin-dialog';

	let isLoading = false;

	async function handleClick() {
		const selected = await open({
			multiple: false,
			filters: [{ name: 'Audio', extensions: ['mp3', 'wav', 'm4a', 'ogg', 'flac', 'aac'] }]
		});

		if (selected && typeof selected === 'string') {
			await processFile(selected);
		}
	}

	async function processFile(path: string) {
		isLoading = true;
		audioStore.setFile(path, path.split('/').pop() || path);

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
</script>

<div class="drop-zone" class:loading={isLoading} on:click={handleClick} on:keydown={(e) => e.key === 'Enter' && handleClick()} role="button" tabindex="0">
	{#if isLoading}
		<div class="spinner"></div>
		<p>Analyzing...</p>
	{:else}
		<p class="main-text">Click to upload audio file</p>
		<p class="sub-text">MP3, WAV, M4A, OGG, FLAC, AAC</p>
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
		transition: all 0.2s;
	}
	.drop-zone:hover { border-color: #667eea; background: rgba(102, 126, 234, 0.1); }
	.main-text { font-size: 16px; font-weight: 500; color: var(--text-primary); }
	.sub-text { font-size: 14px; color: var(--text-secondary); margin-top: 8px; }
	.spinner { width: 32px; height: 32px; border: 3px solid var(--border-color); border-top-color: #667eea; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 16px; }
</style>