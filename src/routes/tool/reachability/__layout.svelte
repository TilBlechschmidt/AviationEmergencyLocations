<script>
	import { goto } from '$app/navigation';

	import CriticalArea from '$lib/components/map/layers/CriticalArea.svelte';
	import LocationRanges from '$lib/components/map/layers/LocationRanges.svelte';
	import { altitude, aircraftID } from '$lib/stores';

	function handleClick(e) {
		goto(`/tool/location/${e.detail.id}?ref=reachability`);
	}
</script>

<CriticalArea />
<LocationRanges name="ranges" aircraft={$aircraftID} altitude={$altitude} on:click={handleClick} />
<slot />
