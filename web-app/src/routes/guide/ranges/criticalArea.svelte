<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);
</script>

<CardDetailView
	previousLocation="/guide/ranges/rangeProfiles"
	previousLabel={$_('guide.ranges.rangeProfiles.title')}
	title={$_('guide.ranges.criticalArea.title')}
	nextLocation="/guide/ranges/details"
	nextLabel={$_('guide.ranges.details.title')}
	noWidth
>
	{@html $_('guide.ranges.criticalArea.content')}

	<span slot="card">
		<img class="rounded-xl" src="/guide/criticalArea.jpg" alt="Critical area" />
	</span>
</CardDetailView>

<style>
	img {
		width: 500px;
	}
</style>
