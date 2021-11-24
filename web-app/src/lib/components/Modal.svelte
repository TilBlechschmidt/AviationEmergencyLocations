<script>
	export let hidden = false;
</script>

{#if !hidden}
	<div class="fixed inset-0 bg-black bg-opacity-50 overflow-y-auto h-full w-full">
		<div class="w-full h-full flex justify-center">
			<div class="my-auto">
				<div class="card max-w-prose m-8">
					<slot />
				</div>
			</div>
		</div>
	</div>
{/if}
