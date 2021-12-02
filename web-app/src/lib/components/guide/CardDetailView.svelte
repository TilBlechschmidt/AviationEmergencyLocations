<script>
	import TiArrowRight from 'svelte-icons/ti/TiArrowRight.svelte';
	import TiArrowLeft from 'svelte-icons/ti/TiArrowLeft.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import { goto } from '$app/navigation';

	export let title;

	export let previousLabel;
	export let previousLocation;

	export let nextLabel;
	export let nextLocation;
</script>

<div class="flex justify-center w-full h-full flex-wrap">
	<div class="max-w-prose m-t-auto my-auto p-8 text-justify">
		<h1 class="text-3xl pb-4">{title}</h1>
		<hr class="p-4" />

		<slot />

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
	{#if $$slots.card}
		<div class="my-auto">
			<div class="m-8 w-80 card">
				<slot name="card" />
			</div>
		</div>
	{/if}
</div>
