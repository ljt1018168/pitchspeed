<script lang="ts">
	import { audioStore, targetNote } from '$lib/stores/audio';

	let pitch = $derived($audioStore.pitchDetection);
	let target = $derived($targetNote);
	let pitchShift = $derived($audioStore.pitchShift);
</script>

<div class="keys-grid">
	{#if pitch}
		<div class="key-card">
			<div class="label">原始音调</div>
			<div class="note gradient-text-primary">{pitch.note}</div>
			<div class="offset">{pitch.semitone_offset >= 0 ? '+' : ''}{pitch.semitone_offset} 半音偏差</div>
			<div class="confidence">置信度: {Math.round(pitch.confidence * 100)}%</div>
		</div>
		<div class="key-card">
			<div class="label">目标音调</div>
			<div class="note gradient-text-secondary">{target || pitch.note}</div>
			<div class="offset">
				{#if pitchShift !== 0}
					<span class="shift-value">{pitchShift >= 0 ? '+' : ''}{pitchShift.toFixed(1)} 半音</span>
				{:else}
					<span class="no-shift">未调整</span>
				{/if}
			</div>
		</div>
	{:else if $audioStore.filePath}
		<div class="key-card full-width">
			<div class="label">音高检测</div>
			<div class="no-detection">正在检测或无法检测</div>
			<div class="hint">请确保音频包含清晰的音调</div>
		</div>
	{/if}
</div>

<style>
	.keys-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
	.key-card { background: var(--bg-card); border: 1px solid var(--border-color); border-radius: 16px; padding: 24px; text-align: center; }
	.key-card.full-width { grid-column: 1 / -1; }
	.label { font-size: 12px; text-transform: uppercase; letter-spacing: 1px; color: var(--text-muted); margin-bottom: 8px; }
	.note { font-size: 48px; font-weight: 700; margin-bottom: 8px; }
	.offset { font-size: 14px; color: var(--text-secondary); }
	.confidence { font-size: 11px; color: var(--text-muted); margin-top: 4px; }
	.no-detection { font-size: 24px; font-weight: 500; color: var(--text-secondary); margin: 16px 0; }
	.hint { font-size: 12px; color: var(--text-muted); }
	.shift-value { color: #667eea; font-weight: 500; }
	.no-shift { color: var(--text-muted); }
</style>