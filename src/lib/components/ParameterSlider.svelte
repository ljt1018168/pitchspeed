<script lang="ts">
	export let label: string;
	export let value: number;
	export let min: number;
	export let max: number;
	export let step: number = 0.1;
	export let unit: string = '';
	export let gradient: 'primary' | 'secondary' = 'primary';
</script>

<div class="slider-container">
	<div class="slider-header">
		<span class="label">{label}</span>
		<span class="value">{value.toFixed(1)}{unit}</span>
	</div>
	<input type="range" {min} {max} {step} bind:value class="slider" class:gradient-primary={gradient === 'primary'} class:gradient-secondary={gradient === 'secondary'} />
</div>

<style>
	.slider-container { background: var(--bg-card); border: 1px solid var(--border-color); border-radius: 16px; padding: 16px; }
	.slider-header { display: flex; justify-content: space-between; margin-bottom: 12px; }
	.label { font-size: 13px; color: var(--text-secondary); }
	.value { font-size: 13px; font-weight: 600; }
	.slider { -webkit-appearance: none; width: 100%; height: 6px; background: rgba(255,255,255,0.1); border-radius: 3px; cursor: pointer; }
	.slider::-webkit-slider-thumb { -webkit-appearance: none; width: 18px; height: 18px; border-radius: 50%; cursor: pointer; }
	.slider.gradient-primary::-webkit-slider-thumb { background: linear-gradient(135deg, #667eea, #764ba2); }
	.slider.gradient-secondary::-webkit-slider-thumb { background: linear-gradient(135deg, #f093fb, #f5576c); }
</style>