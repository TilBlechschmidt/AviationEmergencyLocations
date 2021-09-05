<script>
	import { onMount } from 'svelte';
	import { Map } from '@beyonk/svelte-mapbox';

	import SatelliteImagery from '$lib/SatelliteImagery.svelte';
	import LocationLines from '$lib/LocationLines.svelte';

	import { elsa } from '$lib/elsa';
	import { aircraftID } from '$lib/stores';

	onMount(async () => await elsa.startup);
</script>

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
