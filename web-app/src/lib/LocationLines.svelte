<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer, generateLocationLineGeoJSON } from './map';

	export let name;
	export let locations;
	export let aircraft;

	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(() => {
		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'geojson',
			data: generateLocationLineGeoJSON(locations, aircraft)
		});

		map.addLayer(
			{
				id: name,
				type: 'line',
				source: name,
				paint: {
					'line-opacity': 0.75,
					'line-width': 4,
					'line-color': ['get', 'color']
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
		const geoJSON = generateLocationLineGeoJSON(locations, aircraft);
		const source = map.getSource(name);

		if (source) {
			source.setData(geoJSON);
		}
	}
</script>
