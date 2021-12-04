<script>
	import { _ } from 'svelte-i18n';
	import { preferences } from '$lib/stores';
	import Labelled from '$lib/components/Labelled.svelte';
	import { riskCategories } from '$lib/data/constants';
</script>

<Labelled>
	<span slot="label">{$_('settings.risk.humanPresence.eventLocation')}</span>
	<select
		name="eventRisk"
		bind:value={$preferences.eventLocationClassification}
		class="custom-select"
	>
		{#each riskCategories as riskCategory}
			<option value={riskCategory}>{$_(`risk.${riskCategory}`)}</option>
		{/each}
	</select>
</Labelled>
<Labelled>
	<span slot="label">{$_('settings.risk.humanPresence.dense')}</span>
	<select
		name="crowdedRisk"
		bind:value={$preferences.denselyCrowdedClassification}
		class="custom-select"
	>
		{#each riskCategories as riskCategory}
			<option value={riskCategory}>{$_(`risk.${riskCategory}`)}</option>
		{/each}
	</select>
</Labelled>
