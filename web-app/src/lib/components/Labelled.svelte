<script>
	export let warning = false;
	export let critical = false;
</script>

<div class="flex flex-row justify-between pb-2">
	<div class="text-gray-500">
		<slot name="label" />
	</div>
	<div
		class="text-right {critical ? 'text-red-500' : warning ? 'text-yellow-500' : ''} {critical ||
		warning
			? 'font-medium'
			: ''}"
	>
		<slot />
	</div>
</div>
