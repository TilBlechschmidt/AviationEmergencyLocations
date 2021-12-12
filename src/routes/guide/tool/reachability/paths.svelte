<script>
	import { _ } from 'svelte-i18n';

	const dubinsPathAge = new Date().getFullYear() - 1957;
</script>

<img class="guide-cover-image" src="/guide/dubinsPath.jpg" alt="Example of a Dubins Path" />
{@html $_('guide.tool.reachability.paths.content', { values: { dubinsPathAge } })}
