<script lang="ts">
	import { audioStore, estimatedDuration } from '$lib/stores/audio';
	import DropZone from '$lib/components/DropZone.svelte';
	import KeyDisplay from '$lib/components/KeyDisplay.svelte';
	import ParameterSlider from '$lib/components/ParameterSlider.svelte';
	import ActionButtons from '$lib/components/ActionButtons.svelte';

	let pitchShift = $state(0);
	let tempoRatio = $state(1.0);

	$effect(() => {
		audioStore.setPitchShift(pitchShift);
	});
	$effect(() => {
		audioStore.setTempoRatio(tempoRatio);
	});

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}
</script>

<svelte:head><title>音速调 - PitchSpeed</title></svelte:head>

<main>
	<header>
		<h1>音速调</h1>
		<p class="tagline">音高与速度调整工具</p>
	</header>

	<section class="upload"><DropZone /></section>

	{#if $audioStore.filePath}
		<section class="display"><KeyDisplay /></section>

		<section class="controls">
		<div class="sliders-row">
			<ParameterSlider label="音高调整" bind:value={pitchShift} min={-24} max={24} step={0.5} unit=" 半音" gradient="primary" />
			<ParameterSlider label="速度调整" bind:value={tempoRatio} min={0.1} max={5.0} step={0.01} unit="x" gradient="secondary" />
		</div>
	</section>

		<section class="duration-info">
			{#if $estimatedDuration}
				<span class="duration-label">预计时长：</span>
				<span class="duration-value">{formatDuration($estimatedDuration)}</span>
			{/if}
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
	.display { margin-bottom: 20px; }
	.controls { margin-bottom: 16px; }
	.sliders-row { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
	.duration-info { text-align: center; margin-bottom: 20px; font-size: 13px; color: var(--text-secondary); }
	.duration-label { color: var(--text-muted); }
	.duration-value { font-weight: 500; color: var(--text-primary); }
	.actions { margin-bottom: 16px; }
	.error { background: rgba(245, 87, 108, 0.1); border: 1px solid rgba(245, 87, 108, 0.3); border-radius: 10px; padding: 12px; font-size: 13px; color: #f5576c; text-align: center; }

	@media (max-width: 480px) {
		.sliders-row { grid-template-columns: 1fr; }
	}
</style>