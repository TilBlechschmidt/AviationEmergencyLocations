<script>
	import { getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';
	import { overviewCamera, FLY_SPEED } from '$lib/components/map/helpers';

	import SettingsCard from '$lib/components/map/SettingsCard.svelte';

	const { getMap } = getContext(contextKey);
	const map = getMap();
	const camera = overviewCamera(map);

	if (!map.isMoving()) {
		map.easeTo({
			speed: FLY_SPEED,
			...camera
		});
	}
</script>

<SettingsCard />
<slot />
