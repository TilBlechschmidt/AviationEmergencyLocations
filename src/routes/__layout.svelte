<script context="module">
	import { waitLocale } from 'svelte-i18n';

	export async function load() {
		await waitLocale();
		return {};
	}
</script>

<script>
	import '../app.postcss';
	import '$lib/i18n';
</script>

<div id="container">
	<slot />
</div>

<style>
	#container {
		width: 100%;
		height: 100%;

		background: linear-gradient(90deg, #fefefe 31.5px, transparent 1%) center,
			linear-gradient(#fefefe 31.5px, transparent 1%) center, #e0e0e0;
		background-size: 33px 33px;
	}
</style>
