<script>
	import { _ } from 'svelte-i18n';
</script>

<img
	src="/guide/learning.gif"
	alt="Bad joke, I don't even like Star Wars 🤷‍♂️"
	title="I don't even like Star Wars 🤷‍♂️"
	class="rounded-xl w-full mb-8"
/>

{@html $_('guide.overview.preface')}

<ul class="p-4 list-disc list-inside flex flex-col space-y-2">
	<li>{$_('guide.overview.sections.introduction')}</li>
	<li>{$_('guide.overview.sections.tools')}</li>
	<li>{$_('guide.overview.sections.knowledge')}</li>
</ul>

{@html $_('guide.overview.conclusion')}
