<script>
	import { onMount, onDestroy, getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { firstNonBackgroundLayer } from '../helpers';
	import { riskColors } from '$lib/data/constants';
	import { elsa } from '$lib/simulation/elsa';
	import { preferences } from '$lib/stores';
	import LocationHitboxes from './LocationHitboxes.svelte';
	import { page } from '$app/stores';

	export let name;
	export let aircraft;
	export let altitude;

	const { getMap } = getContext(contextKey);
	const map = getMap();

	let fullyMounted = false;
	let cacheBusted = false;
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

		if (!map.getSource(name)) {
			cacheBusted = true;
			console.info(`Mounting source ${name}`);
			map.addSource(name, {
				type: 'geojson',
				data: emptyCollection
			});
		}

		if (!map.getSource(`${name}-individual`)) {
			cacheBusted = true;
			console.info(`Mounting source ${name}-individual`);
			map.addSource(`${name}-individual`, {
				type: 'geojson',
				data: emptyCollection
			});
		}

		if (!map.getLayer(name)) {
			cacheBusted = true;
			console.info(`Mounting layer ${name}`);
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
		}

		if (!map.getLayer(`${name}-individual`)) {
			cacheBusted = true;
			console.info(`Mounting layer ${name}-individual`);
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
		}

		map.setLayoutProperty(name, 'visibility', 'visible');
		map.setLayoutProperty(`${name}-individual`, 'visibility', 'visible');

		fullyMounted = true;
		hoverSource = map.getSource(`${name}-individual`);
		updateRanges($preferences, aircraft, altitude);
	});

	onDestroy(() => {
		const timeout = $page.path.startsWith('/tool/location') ? 2000 : 0;

		setTimeout(() => {
			hoverSource.setData(emptyCollection);
			map.setLayoutProperty(name, 'visibility', 'none');
			map.setLayoutProperty(`${name}-individual`, 'visibility', 'none');
		}, timeout);
	});

	const updateRanges = async (local_preferences, local_aircraft, local_altitude) => {
		if (!fullyMounted) return;

		// If the component mounted with a valid cache, skip the first update.
		if (!cacheBusted) {
			console.info('Encountered fully cached LocationRanges, deferring update');
			cacheBusted = true;
			map.once('idle', async () => {
				console.info('Executing deferred update');
				await updateRanges($preferences, aircraft, altitude);
			});
			return;
		}

		const { byRisk, byID } = await elsa.reachabilityGeoJSON(
			local_preferences,
			local_aircraft,
			local_altitude
		);

		const byRiskSource = map.getSource(name);
		if (byRiskSource) byRiskSource.setData(byRisk);

		hoverData = byID;
		if (hoverID) hoverSource.setData(hoverData[hoverID]);
	};

	$: updateRanges($preferences, aircraft, altitude);

	function handleHover(e) {
		const id = e.detail.id;

		if (hoverID !== id) {
			hoverID = id;
			if (hoverData.hasOwnProperty(id)) hoverSource.setData(hoverData[id]);
		}
	}

	function handleHoverEnd() {
		hoverSource.setData(emptyCollection);
		hoverID = null;
	}
</script>

<LocationHitboxes name="hitboxes" on:hover={handleHover} on:hoverEnd={handleHoverEnd} on:click />
