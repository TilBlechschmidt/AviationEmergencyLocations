<script>
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import LocationCard from '$lib/LocationCard.svelte';
	import data from '$lib/data/generated.json';
	import { aircraftID } from '$lib/stores';

	$: location = data.locations[$page.params.locationID];
	$: aircraft = data.aircrafts[$aircraftID];
</script>

{#if $page.params.locationID}
	<LocationCard {location} {aircraft} on:dismiss={() => goto('/map')} />
{/if}

<slot />
