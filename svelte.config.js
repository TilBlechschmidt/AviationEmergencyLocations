import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import { resolve } from 'path';

const elsaPackageDir = resolve(resolve(), 'elsa', 'target', 'pkg');

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [preprocess({
		"postcss": true
	})],
	kit: {
		// Disabled due to issues with hydration+i18n duplicating HTML elements after {@html $_('...')} tags
		ssr: true,
		target: '#svelte',
		adapter: adapter({ fallback: '200.html' }),
		vite: {
			resolve: {
				alias: {
					'elsa': elsaPackageDir,
				},
			}
		}
	}
};

export default config;
