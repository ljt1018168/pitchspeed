<script lang="ts">
	import { audioStore, targetNote, type ExportFormat } from '$lib/stores/audio';
	import { processAudio, analyzeAudio } from '$lib/commands';
	import { save } from '@tauri-apps/plugin-dialog';
	import { readFile } from '@tauri-apps/plugin-fs';
	import { get } from 'svelte/store';

	let hasFile = $derived($audioStore.filePath !== null);
	let isProcessing = $derived($audioStore.isProcessing);
	let exportFormat = $derived($audioStore.exportFormat);

	// 预览播放状态
	let isPlaying = $state(false);
	let audioContext: AudioContext | null = null;
	let audioBuffer: AudioBuffer | null = null;
	let sourceNode: AudioBufferSourceNode | null = null;

	function handleFormatChange(format: ExportFormat) {
		audioStore.setExportFormat(format);
	}

	async function handlePreview() {
		if (!$audioStore.filePath) return;

		// 如果正在播放，停止
		if (isPlaying) {
			stopPlayback();
			return;
		}

		try {
			// 读取音频文件
			const fileData = await readFile($audioStore.filePath);

			// 创建 AudioContext
			if (!audioContext) {
				audioContext = new AudioContext();
			}

			// 解码音频
			audioBuffer = await audioContext.decodeAudioData(fileData.buffer);

			// 播放
			playWithPitchAndTempo();
		} catch (err) {
			console.error('预览失败:', err);
			audioStore.setError(`预览失败: ${err}`);
		}
	}

	function playWithPitchAndTempo() {
		if (!audioContext || !audioBuffer) return;

		// 停止之前的播放
		if (sourceNode) {
			try {
				sourceNode.stop();
			} catch (e) {
				// 忽略已停止的错误
			}
		}

		// 创建新的源节点
		sourceNode = audioContext.createBufferSource();
		sourceNode.buffer = audioBuffer;

		// 获取当前 store 值
		const state = get(audioStore);

		// 设置播放速率（影响速度和音高）
		sourceNode.playbackRate.value = state.tempoRatio;

		// 音高调整（通过 detune，单位是 cents，100 cents = 1 半音）
		sourceNode.detune.value = state.pitchShift * 100;

		// 连接到输出
		sourceNode.connect(audioContext.destination);

		// 开始播放
		sourceNode.start(0);
		isPlaying = true;

		// 播放结束后重置状态
		sourceNode.onended = () => {
			isPlaying = false;
		};
	}

	function stopPlayback() {
		if (sourceNode) {
			try {
				sourceNode.stop();
			} catch (e) {
				// 忽略
			}
			sourceNode = null;
		}
		isPlaying = false;
	}

	async function handleExport() {
		// 获取当前 store 的完整状态
		const state = get(audioStore);

		if (!state.filePath) return;

		// 移除原文件扩展名
		let baseFileName = state.fileName || 'audio';
		const lastDotIndex = baseFileName.lastIndexOf('.');
		if (lastDotIndex > 0) {
			baseFileName = baseFileName.substring(0, lastDotIndex);
		}

		// 获取目标音调
		const targetNoteValue = get(targetNote);

		// 格式化音调值（如 G, A#, C4 等）
		const noteStr = targetNoteValue || 'unknown';
		// 移除八度数字，只保留音名
		const noteName = noteStr.replace(/\d+$/, '');

		// 格式化速度值
		const tempoStr = state.tempoRatio.toFixed(2);

		// 构建默认文件名：原名_音调_速度
		const defaultName = `${baseFileName}_${noteName}_${tempoStr}x`;
		const ext = state.exportFormat;

		const filters = ext === 'mp3'
			? [{ name: 'MP3', extensions: ['mp3'] }]
			: [{ name: 'WAV', extensions: ['wav'] }];

		const outputPath = await save({
			defaultPath: defaultName,
			filters
		});

		if (!outputPath) return;

		// 停止预览
		stopPlayback();

		audioStore.setProcessing(true);
		try {
			await processAudio(state.filePath, outputPath, state.pitchShift, state.tempoRatio, ext);
		} catch (err) {
			audioStore.setError(`处理失败: ${err}`);
		} finally {
			audioStore.setProcessing(false);
		}
	}
</script>

<div class="export-options">
	<div class="format-selector">
		<span class="format-label">导出格式：</span>
		<button
			class="format-btn"
			class:active={exportFormat === 'wav'}
			disabled={!hasFile || isProcessing}
			onclick={() => handleFormatChange('wav')}
		>
			WAV
		</button>
		<button
			class="format-btn"
			class:active={exportFormat === 'mp3'}
			disabled={!hasFile || isProcessing}
			onclick={() => handleFormatChange('mp3')}
		>
			MP3
		</button>
	</div>
</div>

<div class="actions">
	<button
		class="btn-secondary"
		class:playing={isPlaying}
		disabled={!hasFile || isProcessing}
		onclick={handlePreview}
	>
		{#if isPlaying}停止预览{:else}预览{/if}
	</button>
	<button class="btn-primary" disabled={!hasFile || isProcessing} onclick={handleExport}>
		{#if isProcessing}处理中...{:else}转换并导出{/if}
	</button>
</div>

<style>
	.export-options {
		margin-bottom: 12px;
	}

	.format-selector {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.format-label {
		font-size: 13px;
		color: rgba(255, 255, 255, 0.6);
	}

	.format-btn {
		padding: 8px 16px;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		color: white;
		font-size: 13px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.format-btn.active {
		background: linear-gradient(135deg, #667eea, #764ba2);
		border-color: transparent;
	}

	.format-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.actions { display: flex; gap: 12px; }
	.btn-secondary { flex: 1; padding: 14px; background: rgba(255,255,255,0.1); border: 1px solid var(--border-color); border-radius: 10px; color: white; font-size: 14px; cursor: pointer; transition: all 0.2s; }
	.btn-secondary.playing { background: rgba(245, 87, 108, 0.2); border-color: #f5576c; color: #f5576c; }
	.btn-primary { flex: 2; padding: 14px; background: linear-gradient(135deg, #667eea, #764ba2); border: none; border-radius: 10px; color: white; font-size: 14px; font-weight: 600; cursor: pointer; }
	.btn-primary:disabled, .btn-secondary:disabled { opacity: 0.5; cursor: not-allowed; }
</style>