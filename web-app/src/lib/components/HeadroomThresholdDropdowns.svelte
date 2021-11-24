<script>
	import { _ } from 'svelte-i18n';
	import { preferences } from '$lib/stores';
	import Labelled from '$lib/components/Labelled.svelte';
</script>

<Labelled>
	<span slot="label">{$_('settings.risk.landing.risky')}</span>
	<select name="riskyLanding" bind:value={$preferences.riskyLandingHeadroom} class="custom-select">
		<option value={0}>&nbsp;&nbsp;0%</option>
		<option value={-0.05}>&nbsp;-5%</option>
		<option value={-0.1}>-10%</option>
		<option value={-0.15}>-15%</option>
	</select>
</Labelled>
<Labelled>
	<span slot="label">{$_('settings.risk.landing.unsafe')}</span>
	<select
		name="unsafeLanding"
		bind:value={$preferences.unsafeLandingHeadroom}
		class="custom-select"
	>
		<option value={-0.1}>-10%</option>
		<option value={-0.15}>-15%</option>
		<option value={-0.2}>-20%</option>
		<option value={-0.25}>-25%</option>
	</select>
</Labelled>
