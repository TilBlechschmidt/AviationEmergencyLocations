<script>
	import GoIssueOpened from 'svelte-icons/go/GoIssueOpened.svelte';
	import GoMention from 'svelte-icons/go/GoMention.svelte';
	import MdMap from 'svelte-icons/md/MdMap.svelte';
	import TiArrowRight from 'svelte-icons/ti/TiArrowRight.svelte';
	import TiArrowLeft from 'svelte-icons/ti/TiArrowLeft.svelte';
	import Head from '$lib/components/Head.svelte';
	import SafetyDisclaimerModal from '$lib/components/SafetyDisclaimerModal.svelte';

	import { _ } from 'svelte-i18n';
	import { page as pageStore } from '$app/stores';

	const sections = [
		{
			title: '',
			pages: [{ title: $_('guide.overview.title'), path: '/guide' }]
		},
		{
			title: $_('guide.introduction.title'),
			pages: [
				{
					title: $_('guide.introduction.pages.locations.title'),
					path: '/guide/intro/locations'
				},
				{
					title: $_('guide.introduction.pages.riskAssessment.title'),
					path: '/guide/intro/riskAssessment'
				},
				{
					title: $_('guide.introduction.pages.riskFactors.title'),
					path: '/guide/intro/riskFactors'
				},
				{
					title: $_('guide.introduction.pages.aircraft.title'),
					path: '/guide/intro/aircraft'
				},
				{
					title: $_('guide.introduction.pages.limitations.title'),
					path: '/guide/intro/limitations'
				}
			]
		},
		{
			title: $_('guide.tool.reachability.title'),
			pages: [
				{
					title: $_('guide.tool.reachability.overview.title'),
					path: '/guide/tool/reachability'
				},
				{
					title: $_('guide.tool.reachability.paths.title'),
					path: '/guide/tool/reachability/paths'
				},
				{
					title: $_('guide.tool.reachability.rangeProfiles.title'),
					path: '/guide/tool/reachability/rangeProfiles'
				},
				{
					title: $_('guide.tool.reachability.criticalArea.title'),
					path: '/guide/tool/reachability/criticalArea'
				},
				{
					title: $_('guide.tool.reachability.details.title'),
					path: '/guide/tool/reachability/details'
				}
			]
		},
		{
			title: '',
			pages: [{ title: $_('guide.finalRemarks.title'), path: '/guide/remarks' }]
		}
	];

	let scrollContainer;

	function findPageContext(path) {
		let previous = null;
		let current = null;
		let next = null;

		outer: for (const section of sections) {
			for (const page of section.pages) {
				const data = Object.assign(page, { section });

				if (page.path == path) {
					current = data;
				} else if (current !== null) {
					next = data;
					break outer;
				} else {
					previous = data;
				}
			}
		}

		return { previous, current, next };
	}

	function scrollToTop(_path) {
		if (scrollContainer) scrollContainer.scrollTop = 0;
	}

	$: page = findPageContext($pageStore.path);
	$: scrollToTop($pageStore.path);
</script>

<Head
	site="E.L.S.A. Guide"
	title={`${page.current.section.title ? `${page.current.section.title} — ` : ''}${
		page.current.title
	}`}
	description="Knowledge base containing information on how to use E.L.S.A. as well as useful tips for emergency situations like e.g. what to watch out for when ditching an aircraft."
/>

<SafetyDisclaimerModal noRedirect />

<div class="flex w-full h-full">
	<div
		class="from-gray-200 via-gray-200 to-white bg-gradient-to-l p-12 pr-0 whitespace-nowrap h-full overflow-y-auto flex-shrink-0"
	>
		<div class="mb-4">
			<div class="text-2xl">E.L.S.A.</div>
			<div class="flex links text-gray-500 space-x-4 pt-1">
				<a href="/tool">
					<span class="w-4 h-4 align-middle inline-block -mt-0.5"><MdMap /></span>
				</a>
				<a href="/imprint">
					<span class=" w-4 h-4 align-middle inline-block -mt-0.5"><GoMention /></span>
				</a>
				<a href="https://github.com/TilBlechschmidt/ELSA/issues/new" target="_blank">
					<span class=" w-4 h-4 align-middle inline-block -mt-0.5"><GoIssueOpened /></span>
				</a>
			</div>
		</div>

		<hr class="border-gray-300 mt-8 mr-12" />

		{#each sections as section}
			<div class="mt-8 mb-2 text-gray-500 tracking-widest text-sm font-extralight uppercase">
				{section.title}
			</div>

			<div class="text-sm font-medium tracking-wide text-gray-600">
				{#each section.pages as page}
					<a
						href={page.path}
						class="no-default block py-2 -ml-2 pl-2 pr-12 hover:bg-white hover:shadow-sm transition-all"
						class:current-page-link={page.path == $pageStore.path}
						sveltekit:noscroll
					>
						{page.title}
					</a>
				{/each}
			</div>
		{/each}
	</div>
	<div class="h-full overflow-y-auto flex flex-grow" bind:this={scrollContainer}>
		<div class="m-12 max-w-prose">
			<div class="pb-4 align-bottom flex flex-wrap">
				<h1 class="text-3xl whitespace-nowrap">{page.current.title}</h1>
				<div class="flex-grow" />
				<h3 class="text-gray-500 mt-auto whitespace-nowrap">
					{page.current.section.title}
				</h3>
			</div>
			<hr class="p-4" />

			<slot />

			<div class="mt-8 flex">
				{#if page.previous}
					<a
						href={page.previous.path}
						class="text-sm font-medium"
						sveltekit:prefetch
						sveltekit:noscroll
					>
						<span class="w-6 h-6 align-middle inline-block -mt-0.5"><TiArrowLeft /></span>
						{#if page.previous.section !== page.current.section && page.previous.section.title}
							{page.previous.section.title}:
						{/if}
						{page.previous.title}
					</a>
				{/if}
				<div class="flex-grow">
					<div class="w-8" />
				</div>
				{#if page.next}
					<a
						href={page.next.path}
						class="text-sm font-medium"
						sveltekit:prefetch
						sveltekit:noscroll
					>
						{#if page.next.section !== page.current.section && page.next.section.title}
							{page.next.section.title}:
						{/if}
						{page.next.title}
						<span class="w-6 h-6 align-middle inline-block -mt-0.5"><TiArrowRight /></span>
					</a>
				{/if}
			</div>
			<div class="h-12" />
		</div>
	</div>
</div>

<style>
	.links > a[href] {
		border: none;
	}

	.current-page-link {
		@apply text-blue-500 bg-white shadow-sm;
	}
</style>
