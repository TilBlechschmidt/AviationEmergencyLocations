<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';
	import { elsa } from '$lib/simulation/elsa';
	import { createEventDispatcher } from 'svelte';

	export let name;

	const dispatch = createEventDispatcher();
	const { getMap } = getContext(contextKey);
	const map = getMap();

	onMount(async () => {
		await elsa.startup;

		const lowerLayer = firstNonBackgroundLayer(map);

		map.addSource(name, {
			type: 'geojson',
			data: await elsa.locationHitboxes(500)
		});

		map.addLayer(
			{
				id: name,
				type: 'fill',
				source: name,
				paint: {
					'fill-opacity': 0
				}
			},
			lowerLayer
		);

		map.on('mousemove', name, (e) => {
			if (e.features.length > 0) {
				map.getCanvas().style.cursor = 'pointer';
				let feature = null;

				if (e.features.length > 1) {
					const latLng = e.lngLat;
					const features = e.features;

					let currentDistance = null;
					for (const f of features) {
						const distance = latLng.distanceTo({
							lng: f.properties.lng,
							lat: f.properties.lat
						});

						if (feature === null || (distance !== null && distance < currentDistance)) {
							feature = f;
							currentDistance = distance;
						}
					}
				} else {
					feature = e.features[0];
				}

				dispatch('hover', { id: feature.properties.id });
			}
		});

		map.on('mouseleave', name, () => {
			map.getCanvas().style.cursor = '';
			dispatch('hoverEnd');
		});

		return () => {
			map.off('mousemove', name);
			map.off('mouseleave', name);
			map.removeLayer(name);
			map.removeSource(name);
		};
	});
</script>
