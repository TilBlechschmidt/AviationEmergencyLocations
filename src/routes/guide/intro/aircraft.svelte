<script>
	import { _ } from 'svelte-i18n';
	import { onMount } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import AircraftCardContent from '$lib/components/AircraftCardContent.svelte';

	let aircraft;

	onMount(async () => {
		await elsa.startup;
		aircraft = await elsa.fetchAircraft('PA28-181');
	});

	const factors = ['mtow', 'takeoff', 'glide', 'landing'];
</script>

{$_('guide.introduction.pages.aircraft.preface')}

<ul class="p-4 list-disc list-inside">
	{#each factors as factor}
		<li>{$_(`guide.introduction.pages.aircraft.factors.${factor}`)}</li>
	{/each}
</ul>

{$_('guide.introduction.pages.aircraft.conclusion')}

<div class="card w-80 mx-auto mt-12">
	{#if aircraft}
		<AircraftCardContent {aircraft} />
	{/if}
</div>
