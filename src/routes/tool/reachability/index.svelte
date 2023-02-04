<script>
	import { getContext, onMount } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { overviewCamera } from '$lib/components/map/helpers';
	import { altitude } from '$lib/stores';

	import SettingsCard from '$lib/components/map/SettingsCard.svelte';

	const { getMap } = getContext(contextKey);
	const map = getMap();
	const camera = overviewCamera(map);

	if (!map.isMoving()) {
		map.easeTo({
			duration: 1000,
			...camera
		});
	}

	onMount(() => {
		if ($altitude < 1500) $altitude = 1500;
	});
</script>

<SettingsCard title="tool.reachability.title" />
<slot />
