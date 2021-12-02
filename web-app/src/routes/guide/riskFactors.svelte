<script>
	import HeadroomThresholdDropdowns from '$lib/components/HeadroomThresholdDropdowns.svelte';
	import HumanPresenceDropdowns from '$lib/components/HumanPresenceDropdowns.svelte';
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);

	const factors = ['headroom', 'humanPresence', 'surface'];
</script>

<CardDetailView
	previousLocation="/guide/risk"
	previousLabel={$_('guide.risk.title')}
	title={$_('guide.riskFactors.title')}
	nextLocation="/guide/aircraft"
	nextLabel={$_('guide.aircraft.title')}
>
	{$_('guide.riskFactors.preface')}

	{#each factors as factor}
		<div class="pt-8 pb-4">
			<h2 class="font-bold text-center pb-2">{$_(`guide.riskFactors.factors.${factor}.title`)}</h2>
			{@html $_(`guide.riskFactors.factors.${factor}.text`)}

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
</CardDetailView>
