<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { onMount } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';
	import AircraftCardContent from '$lib/components/AircraftCardContent.svelte';

	let aircraft;

	onMount(async () => {
		await elsa.startup;
		aircraft = await elsa.fetchAircraft('PA28-181');
	});

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);

	const factors = ['mtow', 'takeoff', 'glide', 'landing'];
</script>

<CardDetailView
	previousLocation="/guide/riskFactors"
	previousLabel={$_('guide.riskFactors.title')}
	title={$_('guide.aircraft.title')}
	nextLocation="/guide/limitations"
	nextLabel={$_('guide.limitations.title')}
>
	{$_('guide.aircraft.preface')}

	<ul class="p-4 list-disc list-inside">
		{#each factors as factor}
			<li>{$_(`guide.aircraft.factors.${factor}`)}</li>
		{/each}
	</ul>

	{$_('guide.aircraft.conclusion')}

	<span slot="card">
		{#if aircraft}
			<AircraftCardContent {aircraft} />
		{/if}
	</span>
</CardDetailView>
