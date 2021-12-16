<script>
	import FaExclamationTriangle from 'svelte-icons/fa/FaExclamationTriangle.svelte';
	import { createEventDispatcher } from 'svelte';
	import Localized from './Localized.svelte';

	export let warning = false;

	export let title;
	export let text;
	export let confirmation1;
	export let confirmation2 = null;
	export let button;

	let confirmation1Checked = false;
	let confirmation2Checked = false;

	const dispatch = createEventDispatcher();

	function dismiss() {
		dispatch('submit');
	}
</script>

<div class="text-2xl text-gray-700 pt-2 pb-6 flex justify-around">
	{#if warning}
		<div class="w-8 h-8 mr-2 text-yellow-400 align-middle inline-block">
			<FaExclamationTriangle />
		</div>
	{/if}
	<Localized key={title} />
	{#if warning}
		<div class="w-8 h-8 ml-2 text-yellow-400 align-middle inline-block">
			<FaExclamationTriangle />
		</div>
	{/if}
</div>
<hr class="text-gray-200" />
<div class="text-base text-gray-500 text-justify pt-6 pb-6">
	<Localized key={text} />
</div>

<div>
	<div class="flex items-center mb-4 text-left">
		<div>
			<input
				aria-describedby="disclaimer-read"
				type="checkbox"
				class="bg-gray-50 border-gray-300 focus:ring-3 focus:ring-blue-300 h-4 w-4 rounded cursor-pointer"
				bind:checked={confirmation1Checked}
			/>
		</div>
		<label
			for="disclaimer-read"
			class="text-sm ml-3 font-medium text-gray-900 cursor-pointer"
			on:click={() => (confirmation1Checked = !confirmation1Checked)}
		>
			<Localized key={confirmation1} />
		</label>
	</div>
	{#if confirmation2}
		<div class="flex items-center mb-4 text-left">
			<div>
				<input
					aria-describedby="disclaimer-read"
					type="checkbox"
					class="bg-gray-50 border-gray-300 focus:ring-3 focus:ring-blue-300 h-4 w-4 rounded cursor-pointer"
					bind:checked={confirmation2Checked}
				/>
			</div>
			<label
				for="disclaimer-read"
				class="text-sm ml-3 font-medium text-gray-900 cursor-pointer"
				on:click={() => (confirmation2Checked = !confirmation2Checked)}
			>
				<Localized key={confirmation2} />
			</label>
		</div>
	{/if}

	<button
		type="button"
		disabled={!confirmation1Checked || (confirmation2 && !confirmation2Checked)}
		on:click={dismiss}
		class="disabled:bg-gray-200 disabled:text-gray-400 w-full rounded-lg border border-gray-200 bg-white text-sm font-medium px-4 py-2 text-gray-900 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 mr-3 mb-3"
	>
		<Localized key={button} />
	</button>
</div>
