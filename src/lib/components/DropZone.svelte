<script lang="ts">
	import { audioStore, targetNote } from '$lib/stores/audio';
	import { analyzeAudio } from '$lib/commands';
	import { open } from '@tauri-apps/plugin-dialog';

	let isLoading = $state(false);

	let hasFile = $derived($audioStore.filePath !== null);
	let fileName = $derived($audioStore.fileName);
	let audioInfo = $derived($audioStore.audioInfo);
	let pitchDetection = $derived($audioStore.pitchDetection);

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

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
			audioStore.setError(`音频分析失败: ${err}`);
		} finally {
			isLoading = false;
		}
	}
</script>

<div
	class="drop-zone"
	class:loading={isLoading}
	class:has-file={hasFile}
	onclick={handleClick}
	onkeydown={(e) => e.key === 'Enter' && handleClick()}
	role="button"
	tabindex="0"
>
	{#if isLoading}
		<div class="spinner"></div>
		<p class="loading-text">分析中...</p>
	{:else if hasFile}
		<div class="file-info">
			<div class="file-icon">🎵</div>
			<div class="file-details">
				<p class="file-name">{fileName}</p>
				{#if audioInfo}
					<p class="file-meta">
						{formatDuration(audioInfo.duration_secs)} · {audioInfo.sample_rate} Hz · {audioInfo.channels} 声道
					</p>
				{/if}
				{#if pitchDetection}
					<p class="file-pitch">
						检测音调: <span class="note">{pitchDetection.note}</span>
						<span class="confidence">({Math.round(pitchDetection.confidence * 100)}%)</span>
					</p>
				{/if}
			</div>
			<div class="change-hint">
				<span class="change-icon">↻</span>
				<span class="change-text">点击更换</span>
			</div>
		</div>
	{:else}
		<div class="upload-hint">
			<div class="upload-icon">📁</div>
			<p class="main-text">点击上传音频文件</p>
			<p class="sub-text">支持 MP3, WAV, M4A, OGG, FLAC, AAC 格式</p>
		</div>
	{/if}
</div>

<style>
	.drop-zone {
		background: var(--bg-card);
		border: 2px dashed var(--border-color);
		border-radius: 16px;
		padding: 32px 20px;
		text-align: center;
		cursor: pointer;
		transition: all 0.2s;
	}

	.drop-zone:hover {
		border-color: #667eea;
		background: rgba(102, 126, 234, 0.1);
	}

	.drop-zone.has-file {
		border-style: solid;
		border-color: rgba(102, 126, 234, 0.3);
		padding: 20px;
	}

	.upload-hint {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.upload-icon {
		font-size: 32px;
		margin-bottom: 12px;
		opacity: 0.7;
	}

	.main-text {
		font-size: 16px;
		font-weight: 500;
		color: var(--text-primary);
	}

	.sub-text {
		font-size: 14px;
		color: var(--text-secondary);
		margin-top: 8px;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--border-color);
		border-top-color: #667eea;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 16px;
	}

	.loading-text {
		font-size: 14px;
		color: var(--text-secondary);
	}

	.file-info {
		display: flex;
		align-items: center;
		gap: 16px;
		text-align: left;
	}

	.file-icon {
		font-size: 28px;
		flex-shrink: 0;
	}

	.file-details {
		flex: 1;
		min-width: 0;
	}

	.file-name {
		font-size: 15px;
		font-weight: 500;
		color: var(--text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.file-meta {
		font-size: 12px;
		color: var(--text-muted);
		margin-top: 4px;
	}

	.file-pitch {
		font-size: 12px;
		color: var(--text-secondary);
		margin-top: 4px;
	}

	.file-pitch .note {
		color: #667eea;
		font-weight: 600;
	}

	.file-pitch .confidence {
		color: var(--text-muted);
		font-size: 11px;
	}

	.change-hint {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2px;
		flex-shrink: 0;
		padding: 8px 12px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.drop-zone:hover .change-hint {
		background: rgba(102, 126, 234, 0.2);
	}

	.change-icon {
		font-size: 18px;
		color: var(--text-muted);
	}

	.change-text {
		font-size: 11px;
		color: var(--text-muted);
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>