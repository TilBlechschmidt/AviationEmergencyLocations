<script>
	import { elsa } from '$lib/simulation/elsa';
	import { onMount } from 'svelte';
	import { aircraftID } from '$lib/stores';

	let profile = { svg: '', points: [] };
	let aircrafts = [];
	let distance = 2270;

	const presets = {
		'EDDH 23 – A1': 3800,
		'EDDH 23 – D8': 2800,
		'EDDH 23 – D6': 2270
	};

	async function updateSVG(aircraftID, distance) {
		profile = await elsa.takeoffProfile(aircraftID, distance);
	}

	function loadPreset(e) {
		distance = e.target.value;
	}

	onMount(async () => {
		await elsa.startup;
		aircrafts = await elsa.fetchAircraftList();
		await updateSVG($aircraftID, distance);
	});

	$: svg = profile.svg;
	$: updateSVG($aircraftID, distance || 3800);
</script>

<div class="flex flex-col h-5/6">
	<div class="flex justify-between">
		<div>
			Available distance:&nbsp;
			<input
				bind:value={distance}
				placeholder="3800"
				class="border border-gray-400 text-right"
				style="width: 5ch"
			/>
			m

			<select on:change={loadPreset}>
				<option disabled selected>Load preset</option>
				{#each Object.keys(presets) as key}
					<option value={presets[key]}>{key}</option>
				{/each}
			</select>
		</div>

		<select bind:value={$aircraftID}>
			{#each aircrafts as aircraft}
				<option value={aircraft.id}>
					{aircraft.name}
				</option>
			{/each}
		</select>
	</div>

	<div class="profile-container flex-grow">
		{@html svg}
	</div>

	<div class="pt-4 text-sm max-w-lg text-center mx-auto">
		<strong>NOTE:</strong>
		This calculation assumes perfect conditions with no wind and with estimated descent rates. It might
		be possible to achieve a faster descent using a slip or full flaps or a shorter ground track with
		a headwind. However, human factors like reaction/decision time are not taking into consideration
		either. Neither are things like the effectiveness of brakes or potentially remaining thrust by the
		engine. Thus it is expected that overall, this resembles quite an accurate "general purpose" risk
		picture.
	</div>
</div>

<style>
	div.profile-container :global(svg) {
		height: 100%;
		width: 100%;
	}
</style>
