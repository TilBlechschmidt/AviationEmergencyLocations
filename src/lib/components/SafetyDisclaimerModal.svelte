<script>
	import { disclaimerSeen } from '$lib/stores';
	import Modal from '$lib/components/Modal.svelte';
	import Disclaimer from '$lib/components/Disclaimer.svelte';

	export let noRedirect = false;

	function dismissGuide() {
		disclaimerSeen.set(true);
		if (!noRedirect) window.open('/guide', '_blank');
	}
</script>

<Modal hidden={$disclaimerSeen}>
	<div class="p-8">
		<Disclaimer
			warning
			title="guide.welcome.title"
			text="guide.welcome.text"
			confirmation1="guide.welcome.confirmation1"
			confirmation2="guide.welcome.confirmation2"
			button="guide.welcome.button"
			on:submit={dismissGuide}
		/>
	</div>
</Modal>
