<script>
	import Labelled from './Labelled.svelte';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let aircraft;
	export let selectable = false;
</script>

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
		{Math.floor(aircraft.glide.ratio * 100) / 100}<span class="text-gray-400">&nbsp;:&nbsp;1</span>
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
{#if selectable}
	<div class="flex justify-center">
		<button class="p-4 rounded-xl font-medium" on:click={() => dispatch('select', aircraft.id)}>
			Select
		</button>
	</div>
{/if}
