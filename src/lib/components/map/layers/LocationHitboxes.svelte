<script>
	import { onMount, onDestroy, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';
	import { elsa } from '$lib/simulation/elsa';
	import { createEventDispatcher } from 'svelte';

	export let name;

	const dispatch = createEventDispatcher();
	const { getMap } = getContext(contextKey);
	const map = getMap();

	const emptyCollection = {
		type: 'FeatureCollection',
		features: []
	};

	function getFeatureFromEvent(e) {
		let feature = null;

		if (e.features.length > 0) {
			map.getCanvas().style.cursor = 'pointer';

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
		}

		return feature;
	}

	function handleMouseMove(e) {
		const feature = getFeatureFromEvent(e);
		if (feature) dispatch('hover', { id: feature.properties.id });
	}

	function handleMouseLeave(e) {
		map.getCanvas().style.cursor = '';
		dispatch('hoverEnd');
	}

	function handleClick(e) {
		const feature = getFeatureFromEvent(e);
		if (feature) dispatch('click', { id: feature.properties.id });
	}

	onMount(async () => {
		await elsa.startup;

		const lowerLayer = firstNonBackgroundLayer(map);
		let cacheBusted = false;

		if (!map.getSource(name)) {
			cacheBusted = true;
			map.addSource(name, {
				type: 'geojson',
				data: emptyCollection
			});
		}

		if (!map.getLayer(name)) {
			cacheBusted = true;
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
		}

		map.on('mousemove', name, handleMouseMove);
		map.on('mouseleave', name, handleMouseLeave);
		map.on('click', name, handleClick);

		if (!cacheBusted) {
			map.once('idle', updateHitboxData);
		} else {
			await updateHitboxData();
		}
	});

	onDestroy(() => {
		map.off('mousemove', name, handleMouseMove);
		map.off('mouseleave', name, handleMouseLeave);
		map.off('click', name, handleClick);
	});

	async function updateHitboxData() {
		const hitboxes = await elsa.locationHitboxes(500);
		const source = map.getSource(name);
		if (source) {
			source.setData(hitboxes);
		}
	}
</script>
