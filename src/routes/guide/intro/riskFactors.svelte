<script>
	import HeadroomThresholdDropdowns from '$lib/components/HeadroomThresholdDropdowns.svelte';
	import HumanPresenceDropdowns from '$lib/components/HumanPresenceDropdowns.svelte';
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';
	import { setContext } from 'svelte';

	setContext(KeyPrefix, 'guide.introduction.pages.riskFactors.');

	const factors = ['headroom', 'humanPresence', 'surface'];
</script>

<Localized key="preface" />

{#each factors as factor}
	<div class="pt-8 pb-4">
		<h2 class="font-bold text-center pb-2">
			<Localized key={`factors.${factor}.title`} />
		</h2>
		<Localized key={`factors.${factor}.text`} />

		{#if factor == 'headroom'}
			<div class="max-w-xs items-center mx-auto pt-4">
				<HeadroomThresholdDropdowns />
			</div>
		{:else if factor == 'humanPresence'}
			<div class="max-w-xs items-center mx-auto pt-4">
				<HumanPresenceDropdowns />
			</div>
		{/if}
	</div>
{/each}
