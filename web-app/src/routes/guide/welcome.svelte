<script>
	import { goto } from '$app/navigation';
	import { _ } from 'svelte-i18n';
	import Disclaimer from '$lib/components/guide/Disclaimer.svelte';

	function nextStep() {
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
				text2={$_('guide.welcome.text2')}
				confirmation1={$_('guide.welcome.confirmation1')}
				confirmation2={$_('guide.welcome.confirmation2')}
				button={$_('guide.welcome.button')}
				on:submit={nextStep}
			/>
		</div>
	</div>
</div>
