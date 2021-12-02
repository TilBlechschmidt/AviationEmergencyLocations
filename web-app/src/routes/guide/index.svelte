<script>
	import { goto } from '$app/navigation';
	import { _ } from 'svelte-i18n';
	import Disclaimer from '$lib/components/guide/Disclaimer.svelte';
	import { completeDisclaimer, DISCLAIMERS } from '$lib/components/guide/guard';

	function nextStep() {
		completeDisclaimer(DISCLAIMERS.INTRODUCTION);

		// Not using `restorePreDisclaimerDestination` here because in 99% of cases,
		// the user that ended up here has not read the guide and this redirecting him
		// to the "final words" of the guide makes no sense.
		goto('/guide/locations');
	}
</script>

<div class="w-full h-full flex justify-center">
	<div class="my-auto">
		<div class="max-w-prose card p-8 m-8 text-center">
			<Disclaimer
				warning
				title={$_('guide.welcome.title')}
				text={$_('guide.welcome.text')}
				confirmation1={$_('guide.welcome.confirmation1')}
				confirmation2={$_('guide.welcome.confirmation2')}
				button={$_('guide.welcome.button')}
				on:submit={nextStep}
			/>
		</div>
	</div>
</div>
