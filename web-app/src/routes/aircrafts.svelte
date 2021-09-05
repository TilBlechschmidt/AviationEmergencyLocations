<script>
	import Labelled from '$lib/Labelled.svelte';
	import { aircraftID } from '$lib/stores';
	import { goto } from '$app/navigation';
	import { elsa } from '$lib/elsa';
	import { onMount } from 'svelte';
	import { fly, fade } from 'svelte/transition';

	let aircrafts = [];

	onMount(async () => {
		await elsa.startup;
		aircrafts = await elsa.fetchAircraftList();
	});

	function select(newAircraftID) {
		$aircraftID = newAircraftID;
		goto('/map/location');
	}
</script>

<div class="w-full h-full flex flex-col" transition:fade={{ duration: 500 }}>
	<div class="text-4xl font-extralight text-center mt-8 mb-4">Choose your aircraft</div>
	<div class="flex-grow flex flex-wrap items-center justify-center">
		{#each aircrafts as aircraft, i}
			<div
				class="bg-white rounded-xl drop-shadow shadow-xl w-72 m-8"
				in:fly={{ y: 200, duration: 1000, delay: 500 + i * 200 }}
			>
				<h1 class="text-2xl text-center pt-4 pb-4">{aircraft.name}</h1>
				<hr class="text-gray-200" />
				<div class="p-4">
					<div class="font-medium pb-2">General</div>
					<Labelled>
						<span slot="label">MTOW</span>
						{Math.round(aircraft.mtow)} kg
					</Labelled>
					<Labelled>
						<span slot="label">Glide ratio</span>
						{Math.floor(aircraft.glide.ratio * 100) / 100}<span class="text-gray-400"
							>&nbsp;:&nbsp;1</span
						>
					</Labelled>
					<Labelled>
						<span slot="label">Turn radius</span>
						{Math.round(aircraft.glide.turnRadius)} m
					</Labelled>
				</div>
				<hr class="text-gray-200" />
				<div class="p-4">
					<div class="font-medium pb-2">Takeoff</div>
					<Labelled>
						<span slot="label">Ground roll</span>
						{Math.round(aircraft.takeoff.groundRoll)} m
					</Labelled>
					<Labelled>
						<span slot="label">Total distance</span>
						{Math.round(aircraft.takeoff.totalDistance)} m
					</Labelled>
				</div>
				<hr class="text-gray-200" />
				<div class="p-4">
					<div class="font-medium pb-2">Landing</div>
					<Labelled>
						<span slot="label">Ground roll</span>
						{Math.round(aircraft.landing.groundRoll)} m
					</Labelled>
					<Labelled>
						<span slot="label">Total distance</span>
						{Math.round(aircraft.landing.totalDistance)} m
					</Labelled>
				</div>
				<div class="flex justify-center">
					<button class="p-4 rounded-xl font-medium" on:click={() => select(aircraft.id)}
						>Select</button
					>
				</div>
			</div>
		{/each}
	</div>
</div>
