<script>
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	import LocationCard from '$lib/components/map/LocationCard.svelte';

	import { elsa } from '$lib/simulation/elsa';
	import { aircraftID, preferences } from '$lib/stores';

	let location;

	onMount(async () => await elsa.startup);

	async function fetchLocation(preferences, locationID, aircraftID) {
		// TODO Show a loading indicator or smth
		location = null;

		if (locationID && aircraftID)
			location = await elsa.fetchLocation(preferences, locationID, aircraftID);
	}

	$: fetchLocation($preferences, $page.params.locationID, $aircraftID);
</script>

{#if location}
	<LocationCard
		{location}
		on:dismiss={() => {
			if (location) goto('/map/location');
		}}
	/>
{/if}

<slot />
