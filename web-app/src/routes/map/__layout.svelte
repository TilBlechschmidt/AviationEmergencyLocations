<script>
	import { onMount } from 'svelte';
	import { Map } from '@beyonk/svelte-mapbox';

	import SatelliteImagery from '$lib/components/map/layers/SatelliteImagery.svelte';
	import LocationLines from '$lib/components/map/layers/LocationLines.svelte';

	import { elsa } from '$lib/simulation/elsa';
	import { aircraftID } from '$lib/stores';

	onMount(async () => await elsa.startup);

	let innerWidth = 0;
</script>

<svelte:window bind:innerWidth />

{#if innerWidth > 650}
	<Map
		accessToken="pk.eyJ1IjoidGlsYmxlY2hzY2htaWR0IiwiYSI6ImNqczYxZXplZjA3bnM0M3A5djB1cDl3azUifQ.MEU9Fe4JHD1_3U1BLNJWbg"
		style="mapbox://styles/tilblechschmidt/ckraoako74wms18mx5xv38zea/draft"
		center={[9.99, 53.55]}
		zoom={10.3}
		options={{
			customAttribution: ['Til Blechschmidt', 'LGV Hamburg'],
			maxPitch: 0,
			bearingSnap: 180
		}}
	>
		<SatelliteImagery />
		<LocationLines aircraft={$aircraftID} />
		<slot />
	</Map>
{:else}
	<div class="flex items-center justify-center h-full">
		<div class="p-8 text-center mx-auto">
			<h1 class="text-2xl">Uh oh.</h1>
			Your screen is too narrow to use E.L.S.A.!<br />
			Either rotate your device or use another one.
		</div>
	</div>
{/if}
