<script>
	import { onMount, setContext } from 'svelte';
	import { elsa } from '$lib/simulation/elsa';
	import AircraftCardContent from '$lib/components/AircraftCardContent.svelte';
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';

	let aircraft;

	onMount(async () => {
		await elsa.startup;
		aircraft = await elsa.fetchAircraft('PA28-181');
	});

	const factors = ['mtow', 'takeoff', 'glide', 'landing'];

	setContext(KeyPrefix, 'guide.introduction.pages.aircraft.');
</script>

<Localized key="preface" />

<ul class="p-4 list-disc list-inside">
	{#each factors as factor}
		<li><Localized key={`factors.${factor}`} /></li>
	{/each}
</ul>

<Localized key="conclusion" />

<div class="card w-80 mx-auto mt-12">
	{#if aircraft}
		<AircraftCardContent {aircraft} />
	{/if}
</div>
