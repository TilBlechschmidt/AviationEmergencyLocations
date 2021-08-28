<script>
	import { goto } from '$app/navigation';
	import { Map } from '@beyonk/svelte-mapbox';
	import center from '@turf/center';

	import { altitude, aircraftID } from '$lib/stores';
	import data from '$lib/data/generated.json';

	import SatelliteImagery from '$lib/SatelliteImagery.svelte';
	import LocationRanges from '$lib/LocationRanges.svelte';
	import LocationLines from '$lib/LocationLines.svelte';
	import CriticalArea from '$lib/CriticalArea.svelte';
	import { overviewCamera, FLY_SPEED } from '$lib/map';

	let mapComponent;

	function recenter(map) {
		const camera = overviewCamera(map);
		map.easeTo({
			speed: FLY_SPEED,
			...camera
		});
	}

	function onReady() {
		const map = mapComponent.getMap();
		recenter(map);
		map.on('click', (e) => onClick(e));
	}

	function onClick(e) {
		const mapbox = mapComponent.getMapbox();
		const clickLocation = e.lngLat;

		let shortestDistance = Infinity;
		let closestLocation = undefined;

		for (let locationID in data.locations) {
			const location = data.locations[locationID];
			const locationCenter = center(location.geojson).geometry.coordinates;
			const possibleTarget = new mapbox.LngLat(locationCenter[0], locationCenter[1]);
			const distance = possibleTarget.distanceTo(clickLocation);

			if (distance < location.length * 2 && distance < shortestDistance) {
				closestLocation = location;
				shortestDistance = distance;
			}
		}

		if (closestLocation) {
			goto(`/map/${closestLocation.id}`);
		}
	}

	const locations = Object.values(data.locations);
	$: aircraft = data.aircrafts[$aircraftID];
</script>

<Map
	accessToken="pk.eyJ1IjoidGlsYmxlY2hzY2htaWR0IiwiYSI6ImNqczYxZXplZjA3bnM0M3A5djB1cDl3azUifQ.MEU9Fe4JHD1_3U1BLNJWbg"
	style="mapbox://styles/tilblechschmidt/ckraoako74wms18mx5xv38zea/draft"
	center={[9.99, 53.55]}
	zoom={10.3}
	options={{ customAttribution: ['Til Blechschmidt', 'LGV Hamburg'] }}
	bind:this={mapComponent}
	on:ready={onReady}
>
	<SatelliteImagery />
	<CriticalArea />
	<LocationRanges
		name="ranges"
		{locations}
		{aircraft}
		altitude={$altitude}
	/>
	<LocationLines name="lines" {locations} {aircraft} />
	<slot />
</Map>

<div class="absolute top-0 right-0">
	<select
		bind:value={$aircraftID}
	>
		{#each Object.values(data.aircrafts) as aircraft}
			<option value={aircraft.id}>
				{aircraft.name}
			</option>
		{/each}
	</select>
</div>
