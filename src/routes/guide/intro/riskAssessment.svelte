<script>
	import FaCircle from 'svelte-icons/fa/FaCircle.svelte';
	import IconLabelled from '$lib/components/IconLabelled.svelte';
	import { riskCategories } from '$lib/data/constants';
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';
	import { setContext } from 'svelte';

	setContext(KeyPrefix, 'guide.introduction.pages.riskAssessment.');

	let selectedRisk = 'safe';
	const riskColors = {
		safe: 'text-green-500',
		risky: 'text-yellow-500',
		unsafe: 'text-red-500'
	};
</script>

<Localized key="preface" />

<div class="flex justify-around pt-8">
	{#each riskCategories as risk}
		<div
			class:border-dashed={selectedRisk !== risk}
			class="border-b border-gray-300 hover:border-solid hover:border-blue-400 transition-all cursor-pointer"
			on:click={() => (selectedRisk = risk)}
		>
			<IconLabelled iconColor={riskColors[risk]} textColor="text-black">
				<span slot="icon"><FaCircle /></span>
				<Localized key={`^risk.${risk}`} />
			</IconLabelled>
		</div>
	{/each}
</div>

<div class="p-8 text-gray-500 text-sm">
	<Localized key={`explanation.${selectedRisk}`} />
</div>

<Localized key="conclusion" />
