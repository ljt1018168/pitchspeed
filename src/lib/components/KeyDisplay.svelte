<script lang="ts">
	import { audioStore, targetNote } from '$lib/stores/audio';

	let pitch = $derived($audioStore.pitchDetection);
	let target = $derived($targetNote);
</script>

{#if pitch}
<div class="keys-grid">
	<div class="key-card">
		<div class="label">Original Key</div>
		<div class="note gradient-text-primary">{pitch.note}</div>
		<div class="offset">{pitch.semitone_offset >= 0 ? '+' : ''}{pitch.semitone_offset} st</div>
		<div class="confidence">Confidence: {Math.round(pitch.confidence * 100)}%</div>
	</div>
	<div class="key-card">
		<div class="label">Target Key</div>
		<div class="note gradient-text-secondary">{target || pitch.note}</div>
		<div class="offset">{$audioStore.pitchShift >= 0 ? '+' : ''}{$audioStore.pitchShift.toFixed(1)} st</div>
	</div>
</div>
{/if}

<style>
	.keys-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
	.key-card { background: var(--bg-card); border: 1px solid var(--border-color); border-radius: 16px; padding: 24px; text-align: center; }
	.label { font-size: 12px; text-transform: uppercase; letter-spacing: 1px; color: var(--text-muted); margin-bottom: 8px; }
	.note { font-size: 48px; font-weight: 700; margin-bottom: 8px; }
	.offset { font-size: 14px; color: var(--text-secondary); }
	.confidence { font-size: 11px; color: var(--text-muted); margin-top: 4px; }
</style>