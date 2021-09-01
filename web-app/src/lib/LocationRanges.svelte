<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from './map';
	import { riskColors } from '$lib/constants';
	import { elsa } from '$lib/elsa';

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
			data: await elsa.reachabilityGeoJSON(aircraft, altitude)
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

	const updateRanges = async (aircraft, altitude) => {
		const geoJSON = await elsa.reachabilityGeoJSON(aircraft, altitude);
		const source = map.getSource(name);

		if (source) {
			source.setData(geoJSON);
		}
	};

	$: updateRanges(aircraft, altitude);
</script>
