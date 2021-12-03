<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);

	const bulletPoints = ['wind', 'glide', 'shortField', 'aboveGroundLevel', 'humanFactors'];
</script>

<CardDetailView
	previousLocation="/guide/aircraft"
	previousLabel={$_('guide.aircraft.title')}
	title={$_('guide.limitations.title')}
	nextLocation="/guide/finalWords"
	nextLabel={$_('guide.finalWords.title')}
>
	{$_('guide.limitations.preface')}
	<br />
	<br />
	{$_('guide.limitations.bulletIntro')}

	<ul class="p-4 pb-8 list-disc list-inside">
		{#each bulletPoints as bulletPoint}
			<li class="pt-4">
				<strong>{$_(`guide.limitations.bulletPoints.${bulletPoint}.title`)}</strong> —
				{@html $_(`guide.limitations.bulletPoints.${bulletPoint}.content`)}
			</li>
		{/each}
	</ul>

	{$_('guide.limitations.conclusion')}
</CardDetailView>
