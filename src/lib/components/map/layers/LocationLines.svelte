<script>
	import { riskColors } from '$lib/data/constants';
	import { elsa } from '$lib/simulation/elsa';
	import Line from './Line.svelte';
	import { onMount } from 'svelte';
	import { preferences } from '$lib/stores';

	export let aircraft;

	const paint = {
		'line-opacity': 0.75,
		'line-width': 4,
		'line-color': ['get', ['get', 'risk'], ['literal', riskColors]]
	};

	let geojson = {
		type: 'FeatureCollection',
		features: []
	};

	async function updateGeoJSON(preferences, aircraft) {
		// Normally, one would have to worry about race-conditions.
		// However, since WASM blocks the worker thread for each calculation, we can reasonably
		// assume that those are a non-issue and will disregard their possibility *fingers crossed*
		geojson = await elsa.locationLinesGeoJSON(preferences, aircraft);
	}

	$: updateGeoJSON($preferences, aircraft);

	onMount(async () => await elsa.startup);
</script>

<Line name="location-lines" {geojson} {paint} />
