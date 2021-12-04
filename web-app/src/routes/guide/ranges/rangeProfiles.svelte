<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);
</script>

<CardDetailView
	previousLocation="/guide/ranges/dubinsPath"
	previousLabel={$_('guide.ranges.dubinsPath.title')}
	title={$_('guide.ranges.rangeProfiles.title')}
	nextLocation="/guide/ranges/criticalArea"
	nextLabel={$_('guide.ranges.criticalArea.title')}
	noWidth
>
	{@html $_('guide.ranges.rangeProfiles.content')}

	<span slot="card">
		<img class="rounded-xl" src="/guide/range.jpg" alt="Example of a range profile" />
	</span>
</CardDetailView>

<style>
	img {
		width: 500px;
	}
</style>
