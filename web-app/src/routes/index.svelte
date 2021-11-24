<script>
	import MdArrowForward from 'svelte-icons/md/MdArrowForward.svelte';
	import { goto } from '$app/navigation';
	import { fade, fly } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';

	let shown = false;

	onMount(() => {
		shown = true;
	});
</script>

<div class="w-full h-full flex flex-col justify-center items-center" out:fade={{ duration: 500 }}>
	{#if shown}
		<h1 class="text-5xl" in:fly={{ y: 200, duration: 1000 }}>E.L.S.A.</h1>
		<p class="text-3xl text-gray-500 pt-4" in:fade={{ duration: 1000, delay: 750 }}>
			Emergency Landing Site Assessment
		</p>

		<button
			in:fade={{ y: 200, duration: 1000, delay: 750 }}
			class="m-8 p-4 rounded border-gray-200 border-solid border text-gray-500 hover:shadow-md transition-all"
			on:click={() => goto('/guide/welcome')}
		>
			{$_('welcomeButton')}
			<span class="w-6 h-6 text-gray-500 align-middle inline-block -mt-1 ml-2">
				<MdArrowForward />
			</span>
		</button>
	{/if}
</div>
