<script>
	import { _ } from 'svelte-i18n';

	const bulletPoints = ['wind', 'glide', 'shortField', 'aboveGroundLevel', 'humanFactors'];
</script>

{$_('guide.introduction.pages.limitations.preface')}
<br />
<br />
{$_('guide.introduction.pages.limitations.bulletIntro')}

<ul class="p-4 pb-8 list-disc list-inside">
	{#each bulletPoints as bulletPoint}
		<li class="pt-4">
			<strong>{$_(`guide.introduction.pages.limitations.bulletPoints.${bulletPoint}.title`)}</strong> —
			{@html $_(`guide.introduction.pages.limitations.bulletPoints.${bulletPoint}.content`)}
		</li>
	{/each}
</ul>

{$_('guide.introduction.pages.limitations.conclusion')}
