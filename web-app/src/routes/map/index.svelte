<script>
	import { altitude } from '$lib/stores';
	import AltitudeSlider from '$lib/AltitudeSlider.svelte';
</script>

<AltitudeSlider bind:altitude={$altitude} />
<slot />
