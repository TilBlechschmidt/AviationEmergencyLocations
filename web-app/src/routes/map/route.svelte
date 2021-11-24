<script>
	import AircraftRange from '$lib/components/map/layers/AircraftRange.svelte';
	import AltitudeSlider from '$lib/components/map/AltitudeSlider.svelte';
	import Line from '$lib/components/map/layers/Line.svelte';
	import { aircraftID, altitude } from '$lib/stores';

	import { lineString } from '@turf/helpers';
	import along from '@turf/along';
	import length from '@turf/length';
	import bearing from '@turf/bearing';

	let route = lineString([
		[9.98892306752411, 53.63074627851191],
		[9.922593871660233, 53.6005796926256],
		[9.917713362158718, 53.56247379922515],
		[9.92641266242496, 53.43493721367993]
	]);

	const paint = {
		'line-opacity': 0.65,
		'line-width': 3,
		'line-color': '#2196F3'
	};

	let progress = 0;
	let interval;

	let latitude = 56;
	let longitude = 10;
	let heading = 210;

	$: {
		const total = length(route);
		const distance = Math.min(progress * total, total - 0.01);
		const point = along(route, distance);
		const nextPoint = along(route, distance + 0.01);
		const [lng, lat] = point.geometry.coordinates;

		heading = bearing(point, nextPoint);

		latitude = lat;
		longitude = lng;
	}

	function start() {
		interval = setInterval(() => {
			if (progress < 1) progress += 0.0005;
			else stop();
		}, 10);
	}

	function stop() {
		clearInterval(interval);
		interval = undefined;
	}
</script>

<AltitudeSlider bind:altitude={$altitude} />
<AircraftRange aircraftID={$aircraftID} altitude={$altitude} {latitude} {longitude} {heading} />
<Line name="route" geojson={route} {paint} />
<slot />

<div class="absolute top-0 left-0 text-black bg-white rounded-br pl-4">
	<input class=" w-96" type="range" min="0" max="1" step="0.0001" bind:value={progress} />
	{#if interval}
		<button on:click={stop} class="p-4">Stop</button>
	{:else}
		<button on:click={start} class="p-4">Start</button>
	{/if}
</div>
