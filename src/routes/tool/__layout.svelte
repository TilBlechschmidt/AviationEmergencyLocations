<script>
	import { onMount } from 'svelte';
	import { Map } from '@beyonk/svelte-mapbox';

	import Modal from '$lib/components/Modal.svelte';
	import Disclaimer from '$lib/components/Disclaimer.svelte';
	import SatelliteImagery from '$lib/components/map/layers/SatelliteImagery.svelte';
	import LocationLines from '$lib/components/map/layers/LocationLines.svelte';

	import { elsa } from '$lib/simulation/elsa';
	import { aircraftID, disclaimerSeen } from '$lib/stores';

	onMount(async () => {
		await elsa.startup;
	});

	let innerWidth = 0;

	function dismissGuide() {
		disclaimerSeen.set(true);
		window.open('/guide', '_blank');
	}
</script>

<svelte:window bind:innerWidth />

{#if innerWidth > 650}
	<Map
		accessToken="pk.eyJ1IjoidGlsYmxlY2hzY2htaWR0IiwiYSI6ImNqczYxZXplZjA3bnM0M3A5djB1cDl3azUifQ.MEU9Fe4JHD1_3U1BLNJWbg"
		style="mapbox://styles/tilblechschmidt/ckraoako74wms18mx5xv38zea/draft"
		center={[9.99, 53.95]}
		zoom={7.5}
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

	<Modal hidden={$disclaimerSeen}>
		<div class="p-8">
			<Disclaimer
				warning
				title="guide.welcome.title"
				text="guide.welcome.text"
				confirmation1="guide.welcome.confirmation1"
				confirmation2="guide.welcome.confirmation2"
				button="guide.welcome.button"
				on:submit={dismissGuide}
			/>
		</div>
	</Modal>
{:else}
	<div class="flex items-center justify-center h-full">
		<div class="p-8 text-center mx-auto">
			<h1 class="text-2xl">Uh oh.</h1>
			Your screen is too narrow to use E.L.S.A.!<br />
			Either rotate your device or use another one.
		</div>
	</div>
{/if}
