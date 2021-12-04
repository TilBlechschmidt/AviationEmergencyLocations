<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);
</script>

<CardDetailView
	previousLocation="/guide/ranges/criticalArea"
	previousLabel={$_('guide.ranges.criticalArea.title')}
	title={$_('guide.ranges.details.title')}
	nextLocation="/guide/finalWords"
	nextLabel={$_('guide.finalWords.title')}
	noWidth
>
	{@html $_('guide.ranges.details.content')}

	<span slot="card">
		<img class="rounded-xl" src="/guide/locationDetails.jpg" alt="Zoomed in location detail view" />
	</span>
</CardDetailView>

<style>
	img {
		width: 400px;
	}
</style>
