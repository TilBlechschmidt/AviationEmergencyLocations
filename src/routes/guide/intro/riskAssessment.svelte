<script>
	import FaCircle from 'svelte-icons/fa/FaCircle.svelte';
	import IconLabelled from '$lib/components/IconLabelled.svelte';
	import { _ } from 'svelte-i18n';
	import { riskCategories } from '$lib/data/constants';

	let selectedRisk = 'safe';
	const riskColors = {
		safe: 'text-green-500',
		risky: 'text-yellow-500',
		unsafe: 'text-red-500'
	};
</script>

{$_('guide.introduction.pages.riskAssessment.preface')}

<div class="flex justify-around pt-8">
	{#each riskCategories as risk}
		<div
			class="{selectedRisk == risk ? 'border-b border-gray-300' : ''} cursor-pointer"
			on:click={() => (selectedRisk = risk)}
		>
			<IconLabelled iconColor={riskColors[risk]} textColor="text-black">
				<span slot="icon"><FaCircle /></span>
				{$_(`risk.${risk}`)}
			</IconLabelled>
		</div>
	{/each}
</div>

<div class="p-8 text-gray-500 text-sm">
	{$_(`guide.introduction.pages.riskAssessment.explanation.${selectedRisk}`)}
</div>

{$_('guide.introduction.pages.riskAssessment.conclusion')}
