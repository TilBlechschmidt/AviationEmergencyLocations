<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';

	export let name;
	export let geojson;
	export let paint = {};

	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(() => {
		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'geojson',
			data: geojson
		});

		map.addLayer(
			{
				id: name,
				type: 'line',
				source: name,
				paint,
				layout: {
					'line-join': 'round'
				}
			},
			lowerLayer
		);

		return () => {
			map.removeLayer(name);
			map.removeSource(name);
		};
	});

	const updateData = async (data) => {
		const source = map.getSource(name);
		if (source) source.setData(data);
	};

	const updatePaint = async (paint) => {
		if (map.getLayer(name)) {
			for (const key in paint) {
				map.setPaintProperty(name, key, paint[key]);
			}
		}
	};

	$: updateData(geojson);
	$: updatePaint(paint);
</script>
