<script>
	import { onMount, getContext, onDestroy } from 'svelte';
	import { fly } from 'svelte/transition';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { overviewCamera, FLY_SPEED } from './helpers';
	import { createEventDispatcher } from 'svelte';

	import MdClose from 'svelte-icons/md/MdClose.svelte';
	import LocationCardContent from './LocationCardContent.svelte';

	export let location;

	const { getMap } = getContext(contextKey);
	const dispatch = createEventDispatcher();
	const map = getMap();
	const padding = 390;
	const transitionDuration = 500;

	let targetZoom;
	let targetCenter;

	function flyToLocation(location) {
		const newCameraTransform = map.cameraForBounds(location.coordinates, {
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

	$: flyToLocation(location);
</script>

<div
	class="absolute top-0 left-8 h-full overflow-y-auto z-10"
	transition:fly={{ x: -padding, duration: transitionDuration, opacity: 1 }}
>
	<div class="w-80 card mt-8 mb-8">
		<div
			class="absolute top-12 left-4 shadow rounded-full p-1 bg-white cursor-pointer"
			on:click={dismissSelf}
		>
			<div class="w-4 h-4 text-gray-500">
				<MdClose />
			</div>
		</div>
		<LocationCardContent {location} />
	</div>
</div>
