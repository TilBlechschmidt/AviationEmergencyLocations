<script>
	import { onMount } from 'svelte';
	import { elsa } from '$lib/elsa';

	onMount(async () => await elsa.startup);

	let altitude = 300;
	let time = 42;

	async function doStuff(altitude) {
		const start = new Date();
		const geoJSON = await elsa.reachabilityGeoJSON('C150', altitude);
		time = new Date() - start;
		console.log(geoJSON);
	}

	$: doStuff(altitude);
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<br />
<input type="range" min="300" max="1000" step="10" bind:value={altitude} />
{altitude}m — took {time}ms
