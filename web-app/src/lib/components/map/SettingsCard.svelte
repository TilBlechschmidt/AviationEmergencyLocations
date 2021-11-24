<script>
	import { goto } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import Labelled from '$lib/components/Labelled.svelte';
	import { aircraftID, preferences, altitude } from '$lib/stores';
	import { elsa } from '$lib/simulation/elsa';
	import { onMount } from 'svelte';
	import { degreesToRadians } from '@turf/helpers';
	import { _ } from 'svelte-i18n';
	import HeadroomThresholdDropdowns from '../HeadroomThresholdDropdowns.svelte';
	import HumanPresenceDropdowns from '../HumanPresenceDropdowns.svelte';

	let aircraftName = 'Loading ...';

	onMount(async () => {
		await elsa.startup;
		$preferences = await elsa.verifyPreferences($preferences);
		updateAircraft($aircraftID);
	});

	const padding = 390;
	const duration = 500;

	function changeAircraft() {
		goto('/aircrafts');
	}

	async function updateAircraft(id) {
		aircraftName = (await elsa.fetchAircraft(id)).name;
	}

	$: updateAircraft($aircraftID);
</script>

{#if $preferences}
	<div
		class="absolute top-0 right-0 h-full overflow-y-auto p-8"
		transition:fly={{ x: padding, duration, opacity: 1 }}
	>
		<div class="w-80 text-sm card">
			<div class="p-4 text-center text-lg">{$_('settings.flight.title')}</div>
			<hr class="text-gray-200" />
			<div class="p-4 pt-6">
				<Labelled>
					<span slot="label">{$_('settings.flight.aircraft')}</span>
					<div class="cursor-pointer dashed-underline font-mono" on:click={changeAircraft}>
						{aircraftName}
					</div>
				</Labelled>
				<Labelled>
					<span slot="label">{$_('settings.flight.bank')}</span>
					<select name="bank" bind:value={$preferences.bank} class="custom-select">
						<option value={degreesToRadians(45)}>45º</option>
						<option value={degreesToRadians(60)}>60º</option>
					</select>
				</Labelled>
			</div>
			<hr class="text-gray-200" />
			<div class="p-4 pt-6">
				<Labelled>
					<span slot="label">{$_('settings.flight.altitude')}</span>
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
			<div class="p-4 text-center text-lg">{$_('settings.risk.title')}</div>
			<hr class="text-gray-200" />
			<div class="p-4">
				<div class="pb-4 font-medium">{$_('settings.risk.landing.title')}</div>
				<HeadroomThresholdDropdowns />
			</div>
			<hr class="text-gray-200" />
			<div class="p-4">
				<div class="pb-4 font-medium">{$_('settings.risk.humanPresence.eventLocation')}</div>
				<HumanPresenceDropdowns />
			</div>
		</div>
	</div>
{/if}

<style>
	.dashed-underline {
		border-bottom: 1px dashed #999;
	}
</style>
