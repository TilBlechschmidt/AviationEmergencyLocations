<script>
	import { fly } from 'svelte/transition';

	export let altitude;
	export let min = 1500;
	export let max = 2500;

	const padding = 390;
	const duration = 500;
</script>

<div
	class="absolute bottom-14 right-8 bg-white p-4 rounded w-24 flex flex-col items-center"
	transition:fly={{ x: padding, duration, opacity: 1 }}
>
	<input
		class="h-48 mb-4"
		type="range"
		bind:value={altitude}
		{min}
		{max}
		step="10"
		orient="vertical"
		name="foo"
	/>
	<div class="text-lg">{Math.round(altitude)}ft</div>
</div>

<style>
	input[type='range'] {
		width: 16px;
		writing-mode: bt-lr; /* IE */
		-webkit-appearance: slider-vertical; /* WebKit */
	}
</style>
