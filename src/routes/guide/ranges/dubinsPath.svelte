<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);

	const dubinsPathAge = new Date().getFullYear() - 1957;
</script>

<CardDetailView
	previousLocation="/guide/ranges"
	previousLabel={$_('guide.ranges.intro.title')}
	title={$_('guide.ranges.dubinsPath.title')}
	nextLocation="/guide/ranges/rangeProfiles"
	nextLabel={$_('guide.ranges.rangeProfiles.title')}
	noWidth
>
	{@html $_('guide.ranges.dubinsPath.content', { values: { dubinsPathAge } })}

	<span slot="card">
		<img class="rounded-xl" src="/guide/dubinsPath.jpg" alt="Example of a Dubins Path" />
	</span>
</CardDetailView>
