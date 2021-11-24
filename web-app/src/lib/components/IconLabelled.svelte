<script>
	export let iconColor = 'text-gray-500';
	export let textColor = 'text-gray-500';
</script>

<div>
	<span class="{iconColor} w-4 h-4 align-middle inline-block -mt-0.5">
		<slot name="icon" />
	</span>
	<span class="{textColor} pl-1 align-middle inline-block">
		<slot />
	</span>
</div>
