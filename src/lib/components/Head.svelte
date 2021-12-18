<script>
	const defaultTitle = 'Emergency Landing Site Assessment';

	export let site = 'E.L.S.A.';
	export let title = defaultTitle;
	export let description =
		'Tool to visualise available landing spots for general aviation aircraft in densely populated areas.';
	export let image = 'https://elsa.blechschmidt.dev/social.png';

	$: htmlTitle = `${site} | ${title}`;
</script>

<svelte:head>
	<title>{htmlTitle}</title>

	<meta property="og:site_name" content={site} />
	<meta property="og:type" content="website" />

	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:image" content={image} />

	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:image" content={image} />
</svelte:head>
