<script>
	import { onMount, setContext } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import { aircraftID, preferences } from '$lib/stores';
	import { guideLocation } from '$lib/data/constants';
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';
	import LocationCardContent from '$lib/components/map/LocationCardContent.svelte';

	let location;

	onMount(async () => {
		await elsa.startup;
		location = await elsa.fetchLocation($preferences, guideLocation, $aircraftID);
	});

	setContext(KeyPrefix, 'guide.introduction.pages.locations.');
</script>

<Localized key="preface" />

<ul class="p-4 list-disc list-inside">
	<li><Localized key="factors.elevation" /></li>
	<li><Localized key="factors.surface" /></li>
	<li><Localized key="factors.usage" /></li>
	<li><Localized key="factors.humanPresence" /></li>
	<li><Localized key="factors.headings" /></li>
	<li><Localized key="factors.remarks" /></li>
</ul>

<Localized key="derivedFactors" />

<ul class="p-4 list-disc list-inside">
	<li><Localized key="factors.length" /></li>
	<li><Localized key="factors.headroom" /><sup class="text-red-600">*</sup></li>
	<li><Localized key="factors.risk" /><sup class="text-red-600">*</sup></li>
</ul>

<Localized key="conclusion" />

<div class="map-component-simulation card w-80 mx-auto mt-12">
	{#if location}
		<LocationCardContent {location} />
	{/if}
</div>
