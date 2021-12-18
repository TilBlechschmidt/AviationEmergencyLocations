<script>
	import { getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { overviewCamera } from '$lib/components/map/helpers';

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
</script>

<SettingsCard />
<slot />
