<script>
	import { onMount, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';
	import { riskColors } from '$lib/data/constants';
	import { elsa } from '$lib/simulation/elsa';
	import { preferences } from '$lib/stores';
	import LocationHitboxes from './LocationHitboxes.svelte';

	export let name;
	export let aircraft;
	export let altitude;

	const { getMap } = getContext(contextKey);
	const map = getMap();

	let hoverID = null;
	let hoverData = {};
	let hoverSource;

	const emptyCollection = {
		type: 'FeatureCollection',
		features: []
	};

	onMount(async () => {
		await elsa.startup;

		const lowerLayer = firstNonBackgroundLayer(map);
		const { byRisk, byID } = await elsa.reachabilityGeoJSON($preferences, aircraft, altitude);
		hoverData = byID;

		map.addSource(name, {
			type: 'geojson',
			data: byRisk
		});

		map.addSource(`${name}-individual`, {
			type: 'geojson',
			data: emptyCollection
		});
		hoverSource = map.getSource(`${name}-individual`);

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

		map.addLayer({
			id: `${name}-individual`,
			type: 'line',
			source: `${name}-individual`,
			paint: {
				'line-width': 3,
				'line-color': ['get', ['get', 'risk'], ['literal', riskColors]],
				'line-opacity': ['interpolate', ['linear'], ['zoom'], 12, 0.35, 13.5, 0]
			}
		});

		return () => {
			map.removeLayer(name);
			map.removeSource(name);
			map.removeLayer(`${name}-individual`);
			map.removeSource(`${name}-individual`);
		};
	});

	const updateRanges = async (preferences, aircraft, altitude) => {
		const { byRisk, byID } = await elsa.reachabilityGeoJSON(preferences, aircraft, altitude);
		const byRiskSource = map.getSource(name);
		if (byRiskSource) byRiskSource.setData(byRisk);
		hoverData = byID;
		if (hoverID) hoverSource.setData(hoverData[hoverID]);
	};

	$: updateRanges($preferences, aircraft, altitude);

	function onHover(e) {
		const id = e.detail.id;

		if (hoverID !== id) {
			hoverID = id;
			hoverSource.setData(hoverData[id]);
		}
	}

	function onHoverEnd() {
		hoverSource.setData(emptyCollection);
		hoverID = null;
	}
</script>

<LocationHitboxes name="hitboxes" on:hover={onHover} on:hoverEnd={onHoverEnd} />
