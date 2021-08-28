<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from './map';

	const { getMap } = getContext(contextKey);
	const map = getMap();
	const name = 'dop20';

	onMount(() => {
		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'raster',
			tiles: [
				'https://geodienste.hamburg.de/HH_WMS_DOP?bbox={bbox-epsg-3857}&format=image/png&service=WMS&version=1.1.1&request=GetMap&srs=EPSG:3857&width=256&height=256&layers=DOP'
			],
			tileSize: 256
		});

		map.addLayer(
			{
				id: name,
				type: 'raster',
				source: name,
				paint: {
					'raster-fade-duration': 1000,
					'raster-resampling': 'nearest',
					'raster-opacity': ['interpolate', ['exponential', 0.5], ['zoom'], 12.5, 0, 13.5, 1]
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
