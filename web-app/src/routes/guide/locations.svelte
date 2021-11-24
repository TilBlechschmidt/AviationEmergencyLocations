<script>
	import { onMount } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import { aircraftID, preferences } from '$lib/stores';
	import { guideLocation } from '$lib/data/constants';
	import { _ } from 'svelte-i18n';

	import LocationCardContent from '$lib/components/map/LocationCardContent.svelte';
	import CardDetailView from '../../lib/components/guide/CardDetailView.svelte';

	let location;

	onMount(async () => {
		await elsa.startup;
		location = await elsa.fetchLocation($preferences, guideLocation, $aircraftID);
	});
</script>

<CardDetailView
	previousLocation="/guide/welcome"
	previousLabel={$_('guide.welcome.title')}
	title={$_('guide.locations.title')}
	nextLocation="/guide/risk"
	nextLabel={$_('guide.risk.title')}
>
	{$_('guide.locations.preface')}

	<ul class="p-4 list-disc list-inside">
		<li>{$_('guide.locations.factors.elevation')}</li>
		<li>{$_('guide.locations.factors.surface')}</li>
		<li>{$_('guide.locations.factors.usage')}</li>
		<li>{$_('guide.locations.factors.humanPresence')}</li>
		<li>{$_('guide.locations.factors.headings')}</li>
		<li>{$_('guide.locations.factors.remarks')}</li>
	</ul>

	{$_('guide.locations.derivedFactors')}

	<ul class="p-4 list-disc list-inside">
		<li>{$_('guide.locations.factors.length')}</li>
		<li>{$_('guide.locations.factors.headroom')}<sup class="text-red-600">*</sup></li>
		<li>{$_('guide.locations.factors.risk')}<sup class="text-red-600">*</sup></li>
	</ul>

	{$_('guide.locations.conclusion')}

	<span slot="card">
		{#if location}
			<LocationCardContent {location} />
		{/if}
	</span>
</CardDetailView>
