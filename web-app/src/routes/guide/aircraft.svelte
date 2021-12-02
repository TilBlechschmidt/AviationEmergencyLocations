<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
	import { requireDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);
</script>

<CardDetailView
	previousLocation="/guide/riskFactors"
	previousLabel={$_('guide.riskFactors.title')}
	title={$_('guide.aircraft.title')}
	nextLocation="/guide/limitations"
	nextLabel={$_('guide.limitations.title')}
>
	Since each aircraft has different takeoff, glide, and landing characteristics, the tool allows you
	to select an aircraft to use for all calculations. For the most common aircraft, we provide
	presets for you to select later on. However, make sure the numbers shown actually line up with the
	POH of your aircraft!
	<br />
	<br />
	TODO Figure out what I actually want to say here ^^
	<br />
	And add an example card to the right!
</CardDetailView>
