<script>
	import { _ } from 'svelte-i18n';
	import Localized, { KeyPrefix } from '$lib/components/Localized.svelte';
	import { setContext } from 'svelte';

	setContext(KeyPrefix, 'guide.overview.');
</script>

<img
	src="/guide/learning.gif"
	alt="Bad joke, I don't even like Star Wars 🤷‍♂️"
	title="I don't even like Star Wars 🤷‍♂️"
	class="rounded-xl w-full mb-8"
/>

<Localized key="preface" />

<ul class="p-4 list-disc list-inside flex flex-col space-y-2">
	<li><Localized key="sections.introduction" /></li>
	<li><Localized key="sections.tools" /></li>
	<li><Localized key="sections.knowledge" /></li>
</ul>

<Localized key="conclusion" />
