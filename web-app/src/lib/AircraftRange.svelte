<script>
	import { onMount } from 'svelte';
	import { elsa } from '$lib/elsa';
	import { riskColors } from '$lib/constants';
	import { feetToMeters } from '$lib/units';
	import Line from '$lib/Line.svelte';
	import { Marker } from '@beyonk/svelte-mapbox';
	import MdAirplanemodeActive from 'svelte-icons/md/MdAirplanemodeActive.svelte';

	export let aircraftID;
	export let latitude;
	export let longitude;
	export let altitude;
	export let heading;

	let icon;

	$: paint = {
		'line-opacity': [
			'interpolate',
			['linear'],
			['get', 'heightLoss'],
			feetToMeters(altitude * 0.5),
			1,
			feetToMeters(altitude),
			0
		],
		'line-width': 2.5,
		'line-color': ['get', ['get', 'risk'], ['literal', riskColors]],
		'line-dasharray': [2, 0.5]
	};

	let geojson = {
		type: 'FeatureCollection',
		features: []
	};

	async function updateGeoJSON(latitude, longitude, heading, altitude, aircraftID) {
		geojson = await elsa.landingOptions(latitude, longitude, heading, altitude, aircraftID);
	}

	$: updateGeoJSON(latitude, longitude, heading, altitude, aircraftID);
	$: if (icon) icon.style.transform = `rotate(${heading}deg)`;

	onMount(async () => await elsa.startup);
</script>

<Line name="route-emergency-paths" {geojson} {paint} />
<Marker lat={latitude} lng={longitude} markerOptions={{ rotationAlignment: 'map' }}>
	<div class="w-6 h-6 text-white" bind:this={icon}>
		<MdAirplanemodeActive />
	</div>
</Marker>
