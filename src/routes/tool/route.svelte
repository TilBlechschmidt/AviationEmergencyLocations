<script>
	import AircraftRange from '$lib/components/map/layers/AircraftRange.svelte';
	import AltitudeSlider from '$lib/components/map/AltitudeSlider.svelte';
	import MdWarning from 'svelte-icons/md/MdWarning.svelte';
	import MdPlayArrow from 'svelte-icons/md/MdPlayArrow.svelte';
	import MdPause from 'svelte-icons/md/MdPause.svelte';
	import Line from '$lib/components/map/layers/Line.svelte';
	import { aircraftID, altitude } from '$lib/stores';
	import { elsa } from '$lib/simulation/elsa';
	import { metersToFeet } from '$lib/units';
	import routes from '$lib/data/routes.json';

	import { lineString } from '@turf/helpers';
	import along from '@turf/along';
	import length from '@turf/length';
	import bearing from '@turf/bearing';
	import { fade, fly } from 'svelte/transition';
	import SettingsCard from '../../lib/components/map/SettingsCard.svelte';
	import Localized from '../../lib/components/Localized.svelte';

	import { getContext, onMount } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { overviewCamera } from '$lib/components/map/helpers';

	let selectedRoute = 'RWY23 @ D6 -> S';

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
	let distance = 0;
	let availableCount = 0;
	let inDanger = true;

	let targetAltitude = 2000;
	let profile = { svg: '', points: [] };

	const { getMap } = getContext(contextKey);
	const map = getMap();
	const camera = overviewCamera(map);

	if (!map.isMoving()) {
		map.easeTo({
			duration: 1000,
			...camera
		});
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

	function lerp(start, end, amt) {
		return (1 - amt) * start + amt * end;
	}

	function altitudeForDistance(distance) {
		if (distance <= 0) return 0;

		const high = profile.points.find((p) => p[0] >= distance);
		const low = profile.points
			.slice()
			.reverse()
			.find((p) => p[0] <= distance);

		if (low && high) {
			return lerp(low[1], high[1], (distance - low[0]) / (high[0] - low[0]));
		} else if (low) {
			return (distance - low[1]) * profile.slope;
		} else {
			console.warn('No low or high point available for altitude calculation!');
			return 0;
		}
	}

	async function updateProfile(aircraftID, distance) {
		profile = await elsa.takeoffProfile(aircraftID, distance);
	}

	function resetProgress(_route) {
		progress = 0;
	}

	$: route = lineString(routes[selectedRoute].points);
	$: takeoffProfileDistance = routes[selectedRoute].availableDistance;

	$: resetProgress(route);
	$: updateProfile($aircraftID, takeoffProfileDistance || 3800);
	$: $altitude = Math.min(metersToFeet(altitudeForDistance(distance)), targetAltitude);
	$: takeoffProfileLineProgress = (distance / takeoffProfileDistance) * 100;
	$: inDanger =
		availableCount == 0 && (profile.points.length > 3 ? distance > profile.points[3][0] : true);

	$: {
		const total = length(route);
		const distanceKm = Math.min(progress * total, total - 0.01);
		const point = along(route, distanceKm);
		const nextPoint = along(route, distanceKm + 0.01);
		const [lng, lat] = point.geometry.coordinates;

		latitude = lat;
		longitude = lng;
		heading = bearing(point, nextPoint);
		distance = distanceKm * 1000;
	}
</script>

<AircraftRange
	aircraftID={$aircraftID}
	altitude={$altitude}
	{latitude}
	{longitude}
	{heading}
	bind:availableCount
/>
<Line name="route" geojson={route} {paint} />
<slot />

<SettingsCard
	title="tool.routePlanner.title"
	routeChoices={Object.keys(routes)}
	disableAltitude
	aircraftDropdown
	bind:selectedRoute
	bind:targetAltitude
/>

<div class="absolute bottom-8 left-0 w-full z-30">
	{#if inDanger}
		<div class="w-full flex justify-center" transition:fly={{ y: 10, duration: 150 }}>
			<div
				class="flex items-center backdrop-blur bg-red-500 bg-opacity-20 rounded-lg px-4 py-2"
				style="color: #ff3d00"
			>
				<span class="w-4 h-4 inline-block mr-2"><MdWarning /></span>
				No landing option available
			</div>
		</div>
	{/if}
	<div
		class="m-8 flex items-center bg-white rounded-lg pr-4"
		in:fly={{ y: 100, duration: 250, delay: 250 }}
		out:fly={{ y: 100, duration: 250 }}
	>
		{#if interval}
			<button on:click={stop} class="p-2 flex items-center">
				<span class="w-6 h-6 inline-block"><MdPause /></span>
			</button>
		{:else}
			<button on:click={start} class="p-2 flex items-center">
				<span class="w-6 h-6 inline-block"><MdPlayArrow /></span>
			</button>
		{/if}
		<input
			class="flex-grow"
			type="range"
			min="0"
			max="1"
			step="0.0001"
			bind:value={progress}
			on:mousedown={() => stop()}
		/>
	</div>
</div>

{#if takeoffProfileLineProgress < 100}
	<div class="absolute left-8 top-8" transition:fade={{ duration: 250 }}>
		<div class="rounded-lg backdrop-blur bg-black bg-opacity-40 text-white text-opacity-50">
			<div class="h-64 w-135 relative overflow-hidden rounded-t-lg">
				{@html profile.svg}
				<div
					class="absolute top-0 left-0 h-full w-px bg-red-500"
					style={`margin-left: ${takeoffProfileLineProgress}%`}
				/>
			</div>
			<div class="p-4 text-center text-base -mt-12">
				<Localized key="tool.routePlanner.profile" />
			</div>
		</div>
	</div>
{/if}

<style lang="postcss">
	.w-135 {
		width: 28rem;
	}

	:global(svg) {
		height: 100%;
		width: 100%;
	}

	:global(text) {
		@apply text-sm;
		fill: #ff3d00;
	}
</style>
