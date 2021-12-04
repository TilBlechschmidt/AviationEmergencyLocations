<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';
	import { riskColors } from '$lib/data/constants';
	import { elsa } from '$lib/simulation/elsa';
	import { preferences } from '$lib/stores';

	export let name;
	export let aircraft;
	export let altitude;

	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(async () => {
		await elsa.startup;

		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'geojson',
			data: await elsa.reachabilityGeoJSON($preferences, aircraft, altitude)
		});

		map.addLayer(
			{
				id: name,
				type: 'fill',
				source: name,
				paint: {
					'fill-color': ['get', ['get', 'risk'], ['literal', riskColors]],
					'fill-opacity': ['interpolate', ['linear'], ['zoom'], 12, 0.25, 13.5, 0]
				}
			},
			lowerLayer
		);

		return () => {
			map.removeLayer(name);
			map.removeSource(name);
		};
	});

	const updateRanges = async (preferences, aircraft, altitude) => {
		const geoJSON = await elsa.reachabilityGeoJSON(preferences, aircraft, altitude);
		const source = map.getSource(name);

		if (source) {
			source.setData(geoJSON);
		}
	};

	$: updateRanges($preferences, aircraft, altitude);
</script>
