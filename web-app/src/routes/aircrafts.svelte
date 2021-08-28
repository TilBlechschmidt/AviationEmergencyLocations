<script>
	import data from '$lib/data/generated.json';
	import Labelled from '$lib/Labelled.svelte';
	import { aircraftID } from '$lib/stores';
	import { goto } from '$app/navigation';

	function select(newAircraftID) {
		$aircraftID = newAircraftID;
		goto('/map');
	}

	const poundsToKilogram = (pounds) => Math.round(pounds * 0.4535924);
	const nauticalMilesToMeters = (nm) => Math.round(nm * 1.852001 * 1000);
	const feetToMeters = (feet) => Math.round(feet * 0.3048);

	$: aircrafts = Object.values(data.aircrafts).sort((a, b) => a.mtow > b.mtow);
</script>

<div class="w-full h-full flex flex-col" id="container">
	<div class="text-4xl font-extralight text-center mt-8 mb-4">Choose your aircraft</div>
	<div class="flex-grow flex flex-wrap items-center justify-center">
		{#each aircrafts as aircraft}
			<div class="bg-white rounded-xl drop-shadow shadow-xl w-72 m-8">
				<h1 class="text-2xl text-center pt-4 pb-4">{aircraft.name}</h1>
				<hr class="text-gray-200" />
				<div class="p-4">
					<div class="font-medium pb-2">General</div>
					<Labelled>
						<span slot="label">MTOW</span>
						{poundsToKilogram(aircraft.mtow)} kg
					</Labelled>
					<Labelled>
						<span slot="label">Glide distance</span>
						{nauticalMilesToMeters(aircraft.glide.distance)} m
					</Labelled>
					<Labelled>
						<span slot="label">Turn radius</span>
						{aircraft.derivedPerformance.glide.turnRadius} m
					</Labelled>
				</div>
				<hr class="text-gray-200" />
				<div class="p-4">
					<div class="font-medium pb-2">Takeoff</div>
					<Labelled>
						<span slot="label">Ground roll</span>
						{feetToMeters(aircraft.takeoff.groundRoll)} m
					</Labelled>
					<Labelled>
						<span slot="label">Total distance</span>
						{feetToMeters(aircraft.takeoff.totalDistance)} m
					</Labelled>
				</div>
				<hr class="text-gray-200" />
				<div class="p-4">
					<div class="font-medium pb-2">Landing</div>
					<Labelled>
						<span slot="label">Ground roll</span>
						{feetToMeters(aircraft.landing.groundRoll)} m
					</Labelled>
					<Labelled>
						<span slot="label">Total distance</span>
						{feetToMeters(aircraft.landing.totalDistance)} m
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

<style>
	#container {
		background: linear-gradient(90deg, #fefefe 31.5px, transparent 1%) center,
			linear-gradient(#fefefe 31.5px, transparent 1%) center, #e0e0e0;
		background-size: 33px 33px;
	}
</style>
