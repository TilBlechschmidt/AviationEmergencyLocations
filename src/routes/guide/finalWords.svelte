<script>
	import { _ } from 'svelte-i18n';
	import Disclaimer from '$lib/components/guide/Disclaimer.svelte';
	import {
		restorePreDisclaimerDestination,
		completeDisclaimer,
		requireDisclaimer,
		DISCLAIMERS
	} from '$lib/components/guide/guard';

	requireDisclaimer([DISCLAIMERS.INTRODUCTION]);

	function nextStep() {
		completeDisclaimer(DISCLAIMERS.SAFETY_GUIDE);
		restorePreDisclaimerDestination('/aircrafts');
	}
</script>

<div class="w-full h-full flex justify-center">
	<div class="my-auto">
		<div class="max-w-prose card p-8 m-8 text-center">
			<Disclaimer
				title={$_('guide.finalWords.title')}
				text={$_('guide.finalWords.text')}
				confirmation1={$_('guide.finalWords.confirmation')}
				button={$_('guide.finalWords.button')}
				on:submit={nextStep}
			/>
		</div>
	</div>
</div>
