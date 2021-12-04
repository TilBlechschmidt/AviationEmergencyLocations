<script>
	import { elsa } from '$lib/simulation/elsa';
	import { onMount } from 'svelte';
	import { aircraftID } from '$lib/stores';

	let svg = '';
	let aircrafts = [];

	async function updateSVG(aircraftID) {
		svg = await elsa.takeoffProfile(aircraftID);
	}

	onMount(async () => {
		await elsa.startup;
		aircrafts = await elsa.fetchAircraftList();
		await updateSVG($aircraftID);
	});

	$: updateSVG($aircraftID);
</script>

<div class="flex flex-col h-5/6">
	<div>
		<select bind:value={$aircraftID}>
			{#each aircrafts as aircraft}
				<option value={aircraft.id}>
					{aircraft.name}
				</option>
			{/each}
		</select>
	</div>

	<div class="profile-container flex-grow">
		{@html svg}
	</div>
</div>

<style>
	div.profile-container :global(svg) {
		height: 100%;
		width: 100%;
	}
</style>
