<script>
	import Labelled from '$lib/components/Labelled.svelte';
	import { _ } from 'svelte-i18n';
	import GoMarkGithub from 'svelte-icons/go/GoMarkGithub.svelte';
</script>

<div class="flex justify-center w-full h-full flex-wrap">
	<div class="imprint m-t-auto my-auto p-8">
		<h1 class="text-5xl pb-8 flex w-full title">
			<a href="/">E.L.S.A.</a>
			<div class="flex-grow" />
			<a
				class="h-10 w-10 hover:drop-shadow hover:shadow rounded-full"
				href="https://github.com/TilBlechschmidt/ELSA"
				target="_blank"
				rel="noopener noreferrer"
			>
				<GoMarkGithub />
			</a>
		</h1>

		<Labelled>
			<span slot="label">{$_('imprint.idea')}</span>
			Roy von Buchwaldt
		</Labelled>
		<Labelled>
			<span slot="label">{$_('imprint.dev')}</span>
			Til Blechschmidt
		</Labelled>

		<hr class="text-gray-200 my-8" />

		<h2 class="text-xl pb-4">{$_('imprint.content.title')}</h2>
		<Labelled>
			<span slot="label">{$_('imprint.content.disclaimer')}</span>
			Max Zager
		</Labelled>
		<Labelled>
			<span slot="label">{$_('imprint.content.safety')}</span>
			Roy von Buchwaldt
		</Labelled>
		<Labelled>
			<span slot="label">{$_('imprint.content.tools')}</span>
			Til Blechschmidt
		</Labelled>
		<Labelled>
			<span slot="label"
				>{$_('imprint.content.translation')} ({$_('imprint.content.languages.german')})</span
			>
			???
		</Labelled>

		<hr class="text-gray-200 my-8" />

		<h2 class="text-xl pb-4">{$_('imprint.survey.title')}</h2>
		<Labelled>
			<span slot="label">{$_('imprint.survey.drone')}</span>
			Til Blechschmidt
		</Labelled>
		<Labelled>
			<span slot="label">{$_('imprint.survey.sites')}</span>
			Roy von Buchwaldt
		</Labelled>

		<hr class="text-gray-200 my-8" />

		<h2 class="text-xl pb-4">{$_('imprint.special.title')}</h2>
		<Labelled>
			<span slot="label">{$_('imprint.special.drone')}</span>
			Bernd Peeters
		</Labelled>
		<Labelled>
			<span slot="label">{$_('imprint.special.eduAndFinances')}</span>
			HFC Hamburg e.V.
		</Labelled>

		<div class="p-8 text-sm text-gray-500 text-center">
			{@html $_('imprint.bugs')}
			<a
				href="https://github.com/TilBlechschmidt/ELSA/issues/new"
				target="_blank"
				rel="noopener noreferrer"
			>
				<code>GitHub</code>
			</a>.
			<div class="lg:my-2 my-6" />
			{@html $_('imprint.inquiry')}
			<a
				href="mailto:elsa@blechschmidt.dev?subject=E.L.S.A. Inquiry"
				target="_blank"
				rel="noopener noreferrer"
			>
				<code>elsa@blechschmidt.dev</code>
			</a>.
		</div>
	</div>
</div>

<style>
	.imprint {
		width: 65ch;
	}

	.title > a[href] {
		border: none;
		border-bottom: none;
		color: black;
	}
</style>
