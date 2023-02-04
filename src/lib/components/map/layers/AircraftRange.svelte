<script>
	import { onMount } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import { riskColors } from '$lib/data/constants';
	import { feetToMeters } from '$lib/units';
	import { preferences } from '$lib/stores';
	import Line from './Line.svelte';
	import Marker from './Marker.svelte';
	import MdAirplanemodeActive from 'svelte-icons/md/MdAirplanemodeActive.svelte';

	export let aircraftID;
	export let latitude;
	export let longitude;
	export let altitude;
	export let heading;
	export let availableCount = 0;

	let icon;

	$: paint = {
		'line-opacity':
			altitude > 0
				? [
						'interpolate',
						['linear'],
						['get', 'heightLoss'],
						// TODO Make this a user-configurable parameter :)
						feetToMeters(altitude * 0.5),
						1,
						feetToMeters(altitude),
						0
				  ]
				: 1,
		'line-width': 2.5,
		'line-color': ['get', ['get', 'risk'], ['literal', riskColors]],
		'line-dasharray': [2, 0.5]
	};

	let geojson = {
		type: 'FeatureCollection',
		features: []
	};

	async function updateGeoJSON(preferences, latitude, longitude, heading, altitude, aircraftID) {
		geojson = await elsa.landingOptions(
			preferences,
			latitude,
			longitude,
			heading,
			altitude,
			aircraftID
		);

		availableCount = geojson.features.length;
	}

	$: updateGeoJSON($preferences, latitude, longitude, heading, altitude, aircraftID);
	$: if (icon) icon.style.transform = `rotate(${heading}deg)`;

	onMount(async () => await elsa.startup);
</script>

<Line name="route-emergency-paths" {geojson} {paint} />
<Marker lat={latitude} lng={longitude} markerOptions={{ rotationAlignment: 'map' }}>
	<div class="w-6 h-6 text-white" bind:this={icon}>
		<MdAirplanemodeActive />
	</div>
</Marker>
