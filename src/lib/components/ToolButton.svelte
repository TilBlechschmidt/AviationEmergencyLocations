<script>
	import { _ } from 'svelte-i18n';

	export let title;
	export let subtitle;
	export let href;
	export let disabled = false;
</script>

<a
	{href}
	class="no-default rounded-xl border border-solid border-gray-300 flex p-4 hover:shadow transition-all"
	class:bg-gray-200={disabled}
	class:pointer-events-none={disabled}
>
	<span class="w-12 h-12 text-blue-500 mr-4"><slot /></span>
	<div>
		<div class:text-gray-400={disabled}>{title}</div>
		<div class="text-sm text-gray-400" class:text-red-500={disabled}>
			{disabled ? $_('tool.selection.soon') : subtitle}
		</div>
	</div>
</a>
