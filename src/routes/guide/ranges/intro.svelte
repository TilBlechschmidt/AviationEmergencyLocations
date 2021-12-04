<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);
</script>

<CardDetailView
	previousLocation="/guide/limitations"
	previousLabel={$_('guide.limitations.title')}
	title={$_('guide.ranges.intro.title')}
	nextLocation="/guide/ranges/dubinsPath"
	nextLabel={$_('guide.ranges.dubinsPath.title')}
	noWidth
>
	{@html $_('guide.ranges.intro.content')}

	<span slot="card">
		<img class="rounded-xl" src="/guide/rangeTool.jpg" alt="Reachability tool" />
	</span>
</CardDetailView>

<style>
	img {
		width: 500px;
	}
</style>
