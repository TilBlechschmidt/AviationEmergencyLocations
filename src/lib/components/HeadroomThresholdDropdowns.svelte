<script>
	import { _ } from 'svelte-i18n';
	import { preferences } from '$lib/stores';
	import Labelled from '$lib/components/Labelled.svelte';
	import { riskyLandingHeadrooms, unsafeLandingHeadrooms } from '$lib/data/constants';
	import Localized from './Localized.svelte';

	function formatPercentage(value) {
		const formatted = `${value * 100}%`;
		const delta = 4 - formatted.length;
		const padding = '&nbsp;'.repeat(delta);
		return `${padding}${formatted}`;
	}
</script>

<Labelled>
	<span slot="label"><Localized key="^settings.risk.landing.risky" /></span>
	<select name="riskyLanding" bind:value={$preferences.riskyLandingHeadroom} class="custom-select">
		{#each riskyLandingHeadrooms as headroom}
			<option value={headroom}>{@html formatPercentage(headroom)}</option>
		{/each}
	</select>
</Labelled>
<Labelled>
	<span slot="label"><Localized key="^settings.risk.landing.unsafe" /></span>
	<select
		name="unsafeLanding"
		bind:value={$preferences.unsafeLandingHeadroom}
		class="custom-select"
	>
		{#each unsafeLandingHeadrooms as headroom}
			<option value={headroom}>{@html formatPercentage(headroom)}</option>
		{/each}
	</select>
</Labelled>
