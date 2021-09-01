<script>
	import { onMount, getContext, onDestroy } from 'svelte';
	import { fly } from 'svelte/transition';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { overviewCamera, FLY_SPEED } from './map';
	import { createEventDispatcher } from 'svelte';
	import { aircraftID } from './stores';

	import MdClose from 'svelte-icons/md/MdClose.svelte';
	import FaCompass from 'svelte-icons/fa/FaCompass.svelte';
	import FaRulerHorizontal from 'svelte-icons/fa/FaRulerHorizontal.svelte';
	import FaMountain from 'svelte-icons/fa/FaMountain.svelte';
	import FaExclamationTriangle from 'svelte-icons/fa/FaExclamationTriangle.svelte';
	import MdReport from 'svelte-icons/md/MdReport.svelte';

	import IconLabelled from './IconLabelled.svelte';
	import Labelled from './Labelled.svelte';

	export let location;
	export let aircraft;

	const { getMap } = getContext(contextKey);
	const dispatch = createEventDispatcher();
	const map = getMap();
	const padding = 390;
	const transitionDuration = 500;

	let targetZoom;
	let targetCenter;

	function flyToLocation(location) {
		const bbox = location.geojson.geometry.coordinates;
		const newCameraTransform = map.cameraForBounds(bbox, {
			padding: { top: 100, bottom: 100, left: 100 + padding, right: 100 }
		});

		targetZoom = newCameraTransform.zoom;
		targetCenter = newCameraTransform.center;

		map.flyTo(
			{
				speed: FLY_SPEED,
				...newCameraTransform
			},
			{ zoomingToLocation: true }
		);
	}

	function dismissSelf() {
		map.flyTo({
			speed: FLY_SPEED,
			...overviewCamera(map)
		});
	}

	function onMove(e) {
		if (e.zoomingToLocation) return;

		const zoom = e.target.getZoom();
		const center = e.target.getCenter();
		const zoomDelta = Math.max(0, targetZoom - zoom);

		if (zoomDelta > 2) {
			dispatch('dismiss');
			return;
		}

		const distance = center.distanceTo(targetCenter);

		if (distance > location.length * 2) {
			dispatch('dismiss');
			return;
		}
	}

	onMount(() => {
		map.on('move', onMove);
	});

	onDestroy(() => {
		map.off('move', onMove);
	});

	const usageLabels = {
		Agricultural: 'Landwirtschaftsfläche',
		Aeronautical: 'Luftfahrtgelände',
		Nature: 'Natur',
		Waterway: 'Wasserstraße',
		Event: 'Event Location',
		Park: 'Parkanlage'
	};

	const humanPresenceLabels = {
		Dense: 'Dense',
		Sparse: 'Sparse',
		EventOnly: 'During Events',
		Unlikely: 'Unlikely'
	};

	function formatBearing(bearing) {
		const number = Math.round(bearing / 10);

		if (String(number).length == 1) {
			return `0${number}`;
		} else if (number == 0) {
			return '36';
		} else {
			return number;
		}
	}

	// TODO Exchange the risk (and all other data sources ...)
	$: risk = 'risky';
	$: flyToLocation(location);
	$: landingHeadroom = location.landingHeadroomRatios[$aircraftID];
	$: formattedLandingHeadroom = `${Math.round(landingHeadroom * 100)}%`;
	$: formattedBearing = `${formatBearing(location.bearing)} / ${
		location.reverseBearing ? `${formatBearing(location.reverseBearing)}` : '--'
	}`;
</script>

<div
	class="absolute top-8 bottom-12 left-8 rounded-xl w-80 shadow-2xl drop-shadow-2xl bg-white "
	transition:fly={{ x: -padding, duration: transitionDuration, opacity: 1 }}
>
	<div
		class="absolute top-4 left-4 shadow rounded-full p-1 bg-white cursor-pointer"
		on:click={dismissSelf}
	>
		<div class="w-4 h-4 text-gray-500">
			<MdClose />
		</div>
	</div>
	<img src="/exampleAerial.jpg" class="rounded-t-xl object-cover h-48 w-full" alt="" />
	<div class="w-full">
		<div class="p-4 flex items-center justify-between">
			<div>
				<div class="text-2xl">{location.name}</div>
				<div class="text-gray-500">{usageLabels[location.usage]}</div>
			</div>
			<div class="pr-2">
				{#if risk == 'risky'}
					<div class="w-6 h-6 text-yellow-500">
						<FaExclamationTriangle />
					</div>
				{:else if risk == 'unsafe'}
					<div class="w-7 h-7 text-red-500">
						<MdReport />
					</div>
				{/if}
			</div>
		</div>
		<hr class="text-gray-200" />
		<div class="p-4 flex flex-row justify-around">
			<IconLabelled>
				<FaCompass slot="icon" />
				{formattedBearing}
			</IconLabelled>
			<IconLabelled>
				<FaRulerHorizontal slot="icon" />
				{location.length}m
			</IconLabelled>
			<IconLabelled>
				<FaMountain slot="icon" />
				{location.elevation || 42}m
			</IconLabelled>
		</div>
		<hr class="text-gray-200" />
		<div class="p-4">
			<Labelled>
				<span slot="label">Surface</span>
				{location.surface}
			</Labelled>
			<Labelled>
				<span slot="label">Human presence</span>
				{humanPresenceLabels[location.humanPresence]}
			</Labelled>
			<Labelled critical={landingHeadroom < 0}>
				<span slot="label">Landing headroom</span>
				{formattedLandingHeadroom}
			</Labelled>
		</div>
		<hr class="text-gray-200" />
		<div class="p-4">
			Some very important notes about the location which people will totally read!
		</div>
	</div>
</div>
