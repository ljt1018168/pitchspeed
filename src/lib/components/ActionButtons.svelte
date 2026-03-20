<script lang="ts">
	import { audioStore } from '$lib/stores/audio';
	import { processAudio } from '$lib/commands';
	import { save } from '@tauri-apps/plugin-dialog';

	let hasFile = $derived($audioStore.filePath !== null);
	let isProcessing = $derived($audioStore.isProcessing);

	async function handleExport() {
		if (!$audioStore.filePath) return;
		const fileName = $audioStore.fileName || 'audio';
		const defaultName = `${fileName}_pitch{$audioStore.pitchShift}_tempo{$audioStore.tempoRatio}x`;

		const outputPath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'WAV', extensions: ['wav'] }]
		});

		if (!outputPath) return;

		audioStore.setProcessing(true);
		try {
			await processAudio($audioStore.filePath, outputPath, $audioStore.pitchShift, $audioStore.tempoRatio, 'wav');
		} catch (err) {
			audioStore.setError(`Failed: ${err}`);
		} finally {
			audioStore.setProcessing(false);
		}
	}
</script>

<div class="actions">
	<button class="btn-secondary" disabled={!hasFile || isProcessing}>Preview</button>
	<button class="btn-primary" disabled={!hasFile || isProcessing} onclick={handleExport}>
		{#if isProcessing}Processing...{:else}Convert & Export{/if}
	</button>
</div>

<style>
	.actions { display: flex; gap: 12px; }
	.btn-secondary { flex: 1; padding: 14px; background: rgba(255,255,255,0.1); border: 1px solid var(--border-color); border-radius: 10px; color: white; font-size: 14px; cursor: pointer; }
	.btn-primary { flex: 2; padding: 14px; background: linear-gradient(135deg, #667eea, #764ba2); border: none; border-radius: 10px; color: white; font-size: 14px; font-weight: 600; cursor: pointer; }
	.btn-primary:disabled, .btn-secondary:disabled { opacity: 0.5; cursor: not-allowed; }
</style>