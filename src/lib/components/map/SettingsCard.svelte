<script>
	import { fly } from 'svelte/transition';
	import Labelled from '$lib/components/Labelled.svelte';
	import { aircraftID, preferences, altitude } from '$lib/stores';
	import { elsa } from '$lib/simulation/elsa';
	import { onMount } from 'svelte';
	import { degreesToRadians } from '@turf/helpers';
	import HeadroomThresholdDropdowns from '../HeadroomThresholdDropdowns.svelte';
	import HumanPresenceDropdowns from '../HumanPresenceDropdowns.svelte';
	import { bankAngles } from '$lib/data/constants';
	import GoChevronDown from 'svelte-icons/go/GoChevronDown.svelte';
	import GoBook from 'svelte-icons/go/GoBook.svelte';
	import GoMention from 'svelte-icons/go/GoMention.svelte';
	import GoIssueOpened from 'svelte-icons/go/GoIssueOpened.svelte';
	import IconLabelled from '$lib/components/IconLabelled.svelte';
	import Localized from '$lib/components/Localized.svelte';

	let aircraftName = 'Loading ...';

	onMount(async () => {
		await elsa.startup;
		updateAircraft($aircraftID);
	});

	const padding = 390;
	const duration = 500;

	async function updateAircraft(id) {
		aircraftName = (await elsa.fetchAircraft(id)).name;
	}

	$: updateAircraft($aircraftID);
</script>

{#if $preferences}
	<div
		class="absolute top-0 right-0 h-full overflow-y-auto p-8 z-10"
		transition:fly={{ x: padding, duration, opacity: 1 }}
	>
		<div class="w-80 text-sm card">
			<a
				class="no-default mx-4 pt-4 mb-4 text-center text-xl cursor-pointer block tool-title"
				href="/tool"
			>
				<span class="border-b border-solid border-transparent transition-all"
					><Localized key="tool.reachability.title" /></span
				>
				<span class="w-4 h-4 -mb-0.5 inline-block icon"><GoChevronDown /></span>
			</a>
			<div class="flex justify-around p-4 pt-0 links">
				<a href="/guide/tool/reachability">
					<IconLabelled>
						<GoBook slot="icon" />
						<Localized key="settings.links.guide" />
					</IconLabelled>
				</a>
				<a href="/imprint">
					<IconLabelled>
						<GoMention slot="icon" />
						<Localized key="settings.links.imprint" />
					</IconLabelled>
				</a>
				<a href="https://github.com/TilBlechschmidt/ELSA/issues/new" target="_blank">
					<IconLabelled>
						<GoIssueOpened slot="icon" />
						<Localized key="settings.links.report" />
					</IconLabelled>
				</a>
			</div>
		</div>
		<div class="w-80 text-sm mt-8 card">
			<div class="p-4 text-center text-lg"><Localized key="settings.flight.title" /></div>
			<hr class="text-gray-200" />
			<div class="p-4 pt-6">
				<Labelled>
					<span slot="label"><Localized key="settings.flight.aircraft" /></span>
					<a class="font-mono" href="/aircrafts">{aircraftName}</a>
				</Labelled>
				<Labelled>
					<span slot="label"><Localized key="settings.flight.bank" /></span>
					<select name="bank" bind:value={$preferences.bank} class="custom-select">
						{#each bankAngles as bank}
							<option value={degreesToRadians(bank)}>{bank}º</option>
						{/each}
					</select>
				</Labelled>
			</div>
			<hr class="text-gray-200" />
			<div class="p-4 pt-6">
				<Labelled>
					<span slot="label"><Localized key="settings.flight.altitude" /></span>
					<span class="font-mono">{$altitude}ft</span>
				</Labelled>
				<input
					type="range"
					bind:value={$altitude}
					min="1500"
					max="2500"
					step="10"
					class="w-full mt-2"
				/>
			</div>
		</div>
		<div class="w-80 text-sm mt-8 card">
			<div class="p-4 text-center text-lg"><Localized key="settings.risk.title" /></div>
			<hr class="text-gray-200" />
			<div class="p-4">
				<div class="pb-4 font-medium"><Localized key="settings.risk.landing.title" /></div>
				<HeadroomThresholdDropdowns />
			</div>
			<hr class="text-gray-200" />
			<div class="p-4">
				<div class="pb-4 font-medium">
					<Localized key="settings.risk.humanPresence.eventLocation" />
				</div>
				<HumanPresenceDropdowns />
			</div>
		</div>
	</div>
{/if}

<style>
	.links a[href]:not(:hover) {
		border-bottom-color: rgba(0, 0, 0, 0);
	}

	.tool-title:hover span.icon {
		@apply drop-shadow;
	}

	.tool-title:hover span:not(.icon) {
		@apply border-blue-400;
	}
</style>
