<script>
	import { onMount } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import { aircraftID, preferences } from '$lib/stores';
	import { guideLocation } from '$lib/data/constants';
	import { _ } from 'svelte-i18n';

	import LocationCardContent from '$lib/components/map/LocationCardContent.svelte';

	let location;

	onMount(async () => {
		await elsa.startup;
		location = await elsa.fetchLocation($preferences, guideLocation, $aircraftID);
	});
</script>

{$_('guide.introduction.pages.locations.preface')}

<ul class="p-4 list-disc list-inside">
	<li>{$_('guide.introduction.pages.locations.factors.elevation')}</li>
	<li>{$_('guide.introduction.pages.locations.factors.surface')}</li>
	<li>{$_('guide.introduction.pages.locations.factors.usage')}</li>
	<li>{$_('guide.introduction.pages.locations.factors.humanPresence')}</li>
	<li>{$_('guide.introduction.pages.locations.factors.headings')}</li>
	<li>{$_('guide.introduction.pages.locations.factors.remarks')}</li>
</ul>

{$_('guide.introduction.pages.locations.derivedFactors')}

<ul class="p-4 list-disc list-inside">
	<li>{$_('guide.introduction.pages.locations.factors.length')}</li>
	<li>{$_('guide.introduction.pages.locations.factors.headroom')}<sup class="text-red-600">*</sup></li>
	<li>{$_('guide.introduction.pages.locations.factors.risk')}<sup class="text-red-600">*</sup></li>
</ul>

{$_('guide.introduction.pages.locations.conclusion')}

<div class="map-component-simulation card w-80 mx-auto mt-12">
	{#if location}
		<LocationCardContent {location} />
	{/if}
</div>
