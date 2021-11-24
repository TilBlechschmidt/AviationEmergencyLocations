<script>
	export let reversed = false;
</script>

<button
	type="button"
	class="text-sm font-medium border-b border-dashed border-gray-400 hover:border-solid hover:border-blue-400"
	on:click
>
	{#if !reversed}<slot />{/if}
	<span class="w-6 h-6 align-middle inline-block -mt-0.5"><slot name="icon" /></span>
	{#if reversed}<slot />{/if}
</button>
