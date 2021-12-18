<script>
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';
	import { setContext } from 'svelte';

	setContext(KeyPrefix, 'guide.finalRemarks.');
</script>

<img
	src="/guide/dopeCat.gif"
	alt="Cool cat with sunglasses."
	title="These sunglasses definitely qualify this cat to be a co-pilot 😎"
	class="rounded-xl w-full mb-8"
/>

<Localized key="content" />

<hr class="my-8" />

<Localized key="closingWords" />

<button
	class="my-8 disabled:bg-gray-200 disabled:text-gray-400 w-full rounded-lg border border-gray-200 bg-white text-sm font-medium px-4 py-2 text-gray-900 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 mr-3 mb-3"
>
	<Localized key="button" />
</button>
