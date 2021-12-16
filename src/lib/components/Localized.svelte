<script context="module">
	export const KeyPrefix = 'i18n-keyPrefix';

	function generateOptions(props) {
		let options = {
			values: Object.assign({}, props)
		};
		delete options.values.key;

		return options;
	}

	function parseLinks(input) {
		const matches = input.matchAll(/<a href="(.+?)"(?: (external))?>(.+?)<\/a>/g);

		let ast = [];
		let previous = 0;

		for (const match of matches) {
			const leading = input.substring(previous, match.index);
			if (leading.length > 0) ast.push({ text: leading, type: 'string' });

			const href = match[1];
			const external = match.length == 4;
			const text = external ? match[3] : match[2];
			ast.push({ text, href, external, type: 'a' });

			previous = match.index + match[0].length;
		}

		const trailing = input.substring(previous);
		if (trailing.length > 0) ast.push({ text: trailing, type: 'string' });

		return ast;
	}

	function parseAST(input) {
		const value = input.replaceAll(/\n/g, '<br></br>');
		const matches = value.matchAll(/<(\/)?([^a]+?)>/g);

		let openTag = null;
		let ast = [];
		let previous = 0;

		const processText = (text) => parseLinks(text);

		for (const match of matches) {
			const closing = match[1] !== undefined;
			const tag = match[2];
			const length = match[0].length;
			const leading = value.substring(previous, match.index);

			if (closing && openTag === tag) {
				ast.push({ text: leading, type: tag });
				openTag = null;
				previous = match.index + length;

				if (tag == 'br' && leading.length > 0) {
					console.warn(
						'Encountered <br> tag with content which is invalid! Content will be discarded.'
					);
				}
			} else if (closing) {
				throw new Error('Attempted to close opened tag of different type!');
			} else {
				ast = ast.concat(processText(leading));
				openTag = tag;
				previous = match.index + length;
			}
		}

		const trailing = value.substring(previous);
		if (trailing.length > 0) ast = ast.concat(processText(trailing));

		return ast;
	}

	function buildKey(key) {
		if (key.length > 0 && key[0] == '^') return key.substring(1);

		const prefix = getContext(KeyPrefix) || '';
		return `${prefix}${key}`;
	}
</script>

<script>
	import { getContext } from 'svelte';
	import { _ } from 'svelte-i18n';

	export let key;

	$: options = generateOptions($$props);
	$: ast = parseAST($_(buildKey(key), options));
</script>

{#each ast as block}
	{#if block.type == 'string'}
		{block.text}
	{:else if block.type == 'br'}
		<br />
	{:else if block.type == 'strong'}
		<strong>{block.text}</strong>
	{:else if block.type == 'em'}
		<em>{block.text}</em>
	{:else if block.type == 'code'}
		<code>{block.text}</code>
	{:else if block.type == 'a' && block.external}
		<a href={block.href} target="_blank" rel="noopener noreferrer">{block.text}</a>
	{:else if block.type == 'a'}
		<a href={block.href}>{block.text}</a>
	{/if}
{/each}
