<script>
	import { onMount, onDestroy, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';

	const { getMap } = getContext(contextKey);
	const map = getMap();
	const name = 'satellite';

	onMount(() => {
		const lowerLayer = firstNonBackgroundLayer(map);
		const paint = {
			'raster-fade-duration': 1000,
			'raster-resampling': 'nearest',
			'raster-opacity': ['interpolate', ['exponential', 0.5], ['zoom'], 12.5, 0, 13.5, 1]
		};

		map.addSource(`${name}-dop20`, {
			type: 'raster',
			tiles: [
				'https://geodienste.hamburg.de/HH_WMS_DOP?bbox={bbox-epsg-3857}&format=image/png&service=WMS&version=1.1.1&request=GetMap&srs=EPSG:3857&width=256&height=256&layers=DOP&transparent=true'
			],
			tileSize: 256
		});

		map.addLayer(
			{
				id: `${name}-mapbox`,
				source: { type: 'raster', url: 'mapbox://mapbox.satellite', tileSize: 256 },
				type: 'raster',
				paint: {
					'raster-brightness-min': 0.3,
					'raster-saturation': 0.1,
					'raster-contrast': 0.25,
					...paint
				}
			},
			lowerLayer
		);

		map.addLayer(
			{
				id: `${name}-dop20`,
				type: 'raster',
				source: `${name}-dop20`,
				paint
			},
			lowerLayer
		);
	});

	onDestroy(() => {
		map.removeLayer(`${name}-mapbox`);
		map.removeLayer(`${name}-dop20`);
		map.removeSource(`${name}-dop20`);
	});
</script>
