<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';
	import criticalArea from '$lib/data/criticalArea.json';
	import smoothPolygon from '@turf/polygon-smooth';

	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(() => {
		const name = 'critical-area';
		const lowerLayer = firstNonBackgroundLayer(map);
		const criticalAreaPolygon = smoothPolygon(criticalArea, { iterations: 10 });

		map.addSource(name, {
			type: 'geojson',
			data: criticalAreaPolygon
		});

		map.addLayer(
			{
				id: name,
				type: 'line',
				source: name,
				paint: {
					'line-opacity': 0.75,
					'line-width': 2,
					'line-color': '#E53935',
					'line-dasharray': [2, 1, 4, 1]
				}
			},
			lowerLayer
		);

		return () => {
			map.removeLayer(name);
			map.removeSource(name);
		};
	});
</script>
