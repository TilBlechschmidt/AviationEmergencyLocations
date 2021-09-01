<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from './map';
	import { riskColors } from '$lib/constants';
	import { elsa } from '$lib/elsa';

	export let name;
	export let aircraft;

	const { getMap } = getContext(contextKey);
	const map = getMap();

	async function generateGeoJSON(aircraft) {
		const lines = await elsa.locationGeoJSON(aircraft);
		return lines;
	}

	onMount(async () => {
		await elsa.startup;

		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'geojson',
			data: await generateGeoJSON(aircraft)
		});

		map.addLayer(
			{
				id: name,
				type: 'line',
				source: name,
				paint: {
					'line-opacity': 0.75,
					'line-width': 4,
					'line-color': ['get', ['get', 'risk'], ['literal', riskColors]]
				}
			},
			lowerLayer
		);

		return () => {
			map.removeLayer(name);
			map.removeSource(name);
		};
	});

	const updateRanges = async (aircraft) => {
		const geoJSON = await generateGeoJSON(aircraft);
		const source = map.getSource(name);

		if (source) {
			source.setData(geoJSON);
		}
	};

	$: updateRanges(aircraft);

	// $: {
	// 	const geoJSON = generateLocationLineGeoJSON(locations, aircraft);
	// 	const source = map.getSource(name);

	// 	if (source) {
	// 		source.setData(geoJSON);
	// 	}
	// }
</script>
