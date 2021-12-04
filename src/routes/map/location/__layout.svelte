<script>
	import { getContext, onDestroy, onMount } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { goto } from '$app/navigation';

	import CriticalArea from '$lib/components/map/layers/CriticalArea.svelte';
	import LocationRanges from '$lib/components/map/layers/LocationRanges.svelte';

	import { altitude, aircraftID } from '$lib/stores';
	import { elsa } from '$lib/simulation/elsa';

	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(async () => {
		map.on('click', onClick);
		await elsa.startup;
	});

	onDestroy(() => {
		map.off('click', onClick);
	});

	async function onClick(e) {
		const { lat, lng } = e.lngLat;
		const locationID = await elsa.closestLocationWithinReach(lat, lng, 2500);

		if (locationID) {
			goto(`/map/location/${locationID}`);
		}
	}
</script>

<CriticalArea />
<LocationRanges name="ranges" aircraft={$aircraftID} altitude={$altitude} />
<slot />
