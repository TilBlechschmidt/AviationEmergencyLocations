<script>
	import { _ } from 'svelte-i18n';
	import { preferences } from '$lib/stores';
	import Labelled from '$lib/components/Labelled.svelte';
</script>

<Labelled>
    <span slot="label">{$_('settings.risk.humanPresence.eventLocation')}</span>
    <select
        name="eventRisk"
        bind:value={$preferences.eventLocationClassification}
        class="custom-select"
    >
        <option value="safe">{$_('risk.safe')}</option>
        <option value="risky">{$_('risk.risky')}</option>
        <option value="unsafe">{$_('risk.unsafe')}</option>
    </select>
</Labelled>
<Labelled>
    <span slot="label">{$_('settings.risk.humanPresence.dense')}</span>
    <select
        name="crowdedRisk"
        bind:value={$preferences.denselyCrowdedClassification}
        class="custom-select"
    >
        <option value="safe">{$_('risk.safe')}</option>
        <option value="risky">{$_('risk.risky')}</option>
        <option value="unsafe">{$_('risk.unsafe')}</option>
    </select>
</Labelled>