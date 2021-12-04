<script>
	import TiArrowRight from 'svelte-icons/ti/TiArrowRight.svelte';
	import TiArrowLeft from 'svelte-icons/ti/TiArrowLeft.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import { goto } from '$app/navigation';

	const previousLocation = '/guide/limitations';
	const previousLabel = 'Assumptions & Limitations';
	const nextLocation = '/guide/ranges/intro';
	const nextLabel = 'Tool details';
</script>

<div class="flex justify-center w-full h-full flex-wrap">
	<div class="max-w-prose m-t-auto my-auto p-8 text-center">
		<h1 class="text-4xl pb-2">Reachability tool</h1>

		<div class="my-4 card">
			<img class="rounded-xl" src="/guide/rangeTool.jpg" alt="Reachability tool" />
		</div>

		<div class="mt-8 flex">
			<IconButton reversed on:click={() => goto(previousLocation)}>
				<span slot="icon"><TiArrowLeft /></span>
				{previousLabel}
			</IconButton>
			<div class="flex-grow">
				<div class="w-8" />
			</div>
			<IconButton on:click={() => goto(nextLocation)}>
				{nextLabel}
				<span slot="icon"><TiArrowRight /></span>
			</IconButton>
		</div>
	</div>
</div>

<style>
	img {
		width: 500px;
	}
</style>
