<script lang="ts">
	import { audioStore, estimatedDuration } from '$lib/stores/audio';
	import DropZone from '$lib/components/DropZone.svelte';
	import KeyDisplay from '$lib/components/KeyDisplay.svelte';
	import ParameterSlider from '$lib/components/ParameterSlider.svelte';
	import ActionButtons from '$lib/components/ActionButtons.svelte';

	let pitchShift = 0;
	let tempoRatio = 1.0;

	$: audioStore.setPitchShift(pitchShift);
	$: audioStore.setTempoRatio(tempoRatio);

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}
</script>

<svelte:head><title>PitchSpeed</title></svelte:head>

<main>
	<header>
		<h1>PitchSpeed</h1>
		<p class="tagline">Pitch & Tempo Control</p>
	</header>

	<section class="upload"><DropZone /></section>

	{#if $audioStore.filePath}
		<section class="info">
			<span class="file-name">{$audioStore.fileName}</span>
			{#if $audioStore.audioInfo}
				<span class="file-details">{formatDuration($audioStore.audioInfo.duration_secs)} · {$audioStore.audioInfo.sample_rate} Hz</span>
			{/if}
		</section>

		<section class="display"><KeyDisplay /></section>

		<section class="controls">
			<ParameterSlider label="Pitch Shift" bind:value={pitchShift} min={-24} max={24} step={0.5} unit=" st" gradient="primary" />
			<ParameterSlider label="Tempo" bind:value={tempoRatio} min={0.1} max={5.0} step={0.01} unit="x" gradient="secondary" />
		</section>

		<section class="actions"><ActionButtons /></section>

		{#if $audioStore.error}
			<div class="error">{$audioStore.error}</div>
		{/if}
	{/if}
</main>

<style>
	main { max-width: 560px; margin: 0 auto; padding: 32px 24px; }
	header { text-align: center; margin-bottom: 32px; }
	h1 { font-size: 28px; font-weight: 700; background: linear-gradient(135deg, #667eea, #764ba2); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }
	.tagline { font-size: 14px; color: var(--text-secondary); }
	.upload { margin-bottom: 24px; }
	.info { text-align: center; margin-bottom: 20px; }
	.file-name { display: block; font-size: 14px; font-weight: 500; }
	.file-details { font-size: 12px; color: var(--text-muted); }
	.display { margin-bottom: 20px; }
	.controls { margin-bottom: 24px; display: flex; flex-direction: column; gap: 12px; }
	.actions { margin-bottom: 16px; }
	.error { background: rgba(245, 87, 108, 0.1); border: 1px solid rgba(245, 87, 108, 0.3); border-radius: 10px; padding: 12px; font-size: 13px; color: #f5576c; text-align: center; }
</style>