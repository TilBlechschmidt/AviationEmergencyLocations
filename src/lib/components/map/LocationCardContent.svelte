<script>
	import FaCompass from 'svelte-icons/fa/FaCompass.svelte';
	import FaRulerHorizontal from 'svelte-icons/fa/FaRulerHorizontal.svelte';
	import FaMountain from 'svelte-icons/fa/FaMountain.svelte';
	import FaExclamationTriangle from 'svelte-icons/fa/FaExclamationTriangle.svelte';
	import MdReport from 'svelte-icons/md/MdReport.svelte';
	import { parseISO, formatDistanceToNow } from 'date-fns';

	import IconLabelled from '$lib/components/IconLabelled.svelte';
	import Labelled from '$lib/components/Labelled.svelte';
	import Localized from '$lib/components/Localized.svelte';

	export let location;

	function formatBearing(bearing) {
		if (bearing < 0) {
			bearing += 360;
		}

		const number = Math.round(bearing / 10);

		if (String(number).length == 1) {
			return `0${number}`;
		} else if (number == 0) {
			return '36';
		} else {
			return number;
		}
	}

	$: landingHeadroom = location.landingHeadroom;
	$: formattedLandingHeadroom = `${Math.round(landingHeadroom * 100)}%`;
	// TODO Put the smaller bearing first when the location is reversible
	$: formattedBearing = `${formatBearing(location.bearing)} / ${
		location.reverseBearing ? `${formatBearing(location.reverseBearing)}` : '--'
	}`;
</script>

<img src="/exampleAerial.jpg" class="rounded-t-xl object-cover h-48 w-full" alt="" />
<div class="w-full text-xs leading-5">
	<div class="p-4 flex items-center justify-between">
		<div>
			<div class="text-2xl">{location.name}</div>
			<div class="text-gray-500"><Localized key={`landUsage.${location.usage}`} /></div>
		</div>
		<div class="pr-2">
			{#if location.risk == 'Risky'}
				<div class="w-6 h-6 text-yellow-500">
					<FaExclamationTriangle />
				</div>
			{:else if location.risk == 'Unsafe'}
				<div class="w-7 h-7 text-red-500">
					<MdReport />
				</div>
			{/if}
		</div>
	</div>
	<hr class="text-gray-200" />
	<div class="p-4 flex flex-row justify-around">
		<IconLabelled>
			<FaCompass slot="icon" />
			{formattedBearing}
		</IconLabelled>
		<IconLabelled>
			<FaRulerHorizontal slot="icon" />
			{Math.round(location.length)}m
		</IconLabelled>
		<IconLabelled>
			<FaMountain slot="icon" />
			{location.elevation}ft
		</IconLabelled>
	</div>
	<hr class="text-gray-200" />
	<div class="p-4 pt-6">
		<Labelled>
			<span slot="label"><Localized key="location.surface" /></span>
			<Localized key={`surfaceType.${location.surface}`} />
		</Labelled>
		<Labelled>
			<span slot="label"><Localized key="location.humanPresence" /></span>
			<Localized key={`humanPresence.${location.humanPresence}`} />
		</Labelled>
		<Labelled critical={landingHeadroom < 0}>
			<span slot="label"><Localized key="location.landingHeadroom" /></span>
			{formattedLandingHeadroom}
		</Labelled>
	</div>
	<hr class="text-gray-200" />
	<div class="p-4 pt-6">
		<Labelled>
			<span slot="label"><Localized key="location.lastSurveyed" /></span>
			{formatDistanceToNow(parseISO(location.surveyDate), { addSuffix: true })}
		</Labelled>
	</div>
	{#if location.remarks}
		<hr class="text-gray-200" />
		<div class="p-4">
			<Labelled><span slot="label"><Localized key="location.remarks" /></span></Labelled>
			{location.remarks}
		</div>
	{/if}
</div>
