<script>
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';
	import { setContext } from 'svelte';

	setContext(KeyPrefix, 'guide.introduction.pages.limitations.');

	const bulletPoints = ['wind', 'glide', 'shortField', 'aboveGroundLevel', 'humanFactors'];
</script>

<Localized key="preface" />

<ul class="p-4 pb-8 list-disc list-inside">
	{#each bulletPoints as bulletPoint}
		<li class="pt-4">
			<strong><Localized key={`bulletPoints.${bulletPoint}.title`} /></strong> —
			<Localized key={`bulletPoints.${bulletPoint}.content`} />
		</li>
	{/each}
</ul>

<Localized key="conclusion" />
