<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer, generateRangeGeoJSON } from './map';

	export let name;
	export let locations;
	export let aircraft;
	export let altitude;

	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(() => {
		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'geojson',
			data: generateRangeGeoJSON(locations, aircraft, altitude)
		});

		map.addLayer(
			{
				id: name,
				type: 'fill',
				source: name,
				paint: {
					'fill-color': ['get', 'color'],
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

	$: {
		const geoJSON = generateRangeGeoJSON(locations, aircraft, altitude);
		const source = map.getSource(name);

		if (source) {
			source.setData(geoJSON);
		}
	}
</script>
