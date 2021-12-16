<script>
	import Localized from '$lib/components/Localized.svelte';

	const dubinsPathAge = new Date().getFullYear() - 1957;
</script>

<img class="guide-cover-image" src="/guide/dubinsPath.jpg" alt="Example of a Dubins Path" />
<Localized key="guide.tool.reachability.paths.content" {dubinsPathAge} />
