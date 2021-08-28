<script>
	import { browser } from '$app/env';
	import { onMount } from 'svelte';
	import MyWorker from '$lib/worker.js?worker';

	onMount(() => {
		const worker = new MyWorker();
		worker.onerror = (e) => console.error('worker error', e);
		worker.onmessage = (msg) => console.log('worker msg', msg);
	});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
