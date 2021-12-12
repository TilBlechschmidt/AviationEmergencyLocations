<script>
	import { aircraftID } from '$lib/stores';
	import { goto } from '$app/navigation';
	import { elsa } from '$lib/simulation/elsa';
	import { onMount } from 'svelte';
	import { fly, fade } from 'svelte/transition';
	import AircraftCardContent from '$lib/components/AircraftCardContent.svelte';

	let aircrafts = [];

	onMount(async () => {
		await elsa.startup;
		aircrafts = await elsa.fetchAircraftList();
	});

	function onSelect(e) {
		$aircraftID = e.detail;
		goto('/map/location');
	}
</script>

<div class="w-full h-full flex flex-col" transition:fade={{ duration: 500 }}>
	<div class="text-4xl font-extralight text-center mt-8 mb-4">Choose your aircraft</div>
	<div class="flex-grow flex flex-wrap items-center justify-center">
		{#each aircrafts as aircraft, i}
			<div class="card w-72 m-8" in:fly={{ y: 200, duration: 1000, delay: 500 + i * 200 }}>
				<AircraftCardContent selectable {aircraft} on:select={onSelect} />
			</div>
		{/each}
	</div>
</div>
