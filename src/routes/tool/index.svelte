<script>
	import MdFlightLand from 'svelte-icons/md/MdFlightLand.svelte';
	import MdNavigation from 'svelte-icons/md/MdNavigation.svelte';
	import GoBook from 'svelte-icons/go/GoBook.svelte';
	import ToolButton from '$lib/components/ToolButton.svelte';
	import { disclaimerSeen } from '$lib/stores';
	import { fly } from 'svelte/transition';
	import Localized from '$lib/components/Localized.svelte';

	import { getContext } from 'svelte';
	import { contextKey } from '@beyonk/svelte-mapbox';

	const { getMap } = getContext(contextKey);
	const map = getMap();

	if (!map.isMoving()) {
		map.easeTo({
			duration: 1000,
			center: [9.99, 53.95],
			zoom: 7.5
		});
	}
</script>

{#if $disclaimerSeen}
	<div
		class="absolute bottom-8 left-0 w-full flex justify-center"
		in:fly={{ y: 200, duration: 500, delay: 500 }}
		out:fly={{ y: 200, duration: 500 }}
	>
		<div class="card p-8 pt-4">
			<div class="w-full text-center mb-4 text-xl"><Localized key="tool.selection.prompt" /></div>
			<div class="flex justify-evenly space-x-4">
				<ToolButton
					title="tool.reachability.title"
					subtitle="tool.reachability.subtitle"
					href="/tool/reachability"
				>
					<MdFlightLand />
				</ToolButton>

				<ToolButton
					title="tool.routePlanner.title"
					subtitle="tool.routePlanner.subtitle"
					href="/tool/route"
					disabled
				>
					<MdNavigation />
				</ToolButton>

				<ToolButton title="tool.guide.title" subtitle="tool.guide.subtitle" href="/guide">
					<GoBook />
				</ToolButton>
			</div>
		</div>
	</div>
{/if}
