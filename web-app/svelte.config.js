import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import { resolve } from 'path';

const gaelicPackageDir = resolve(resolve(), 'pkg');

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [preprocess({
		"postcss": true
	})],

	kit: {
		adapter: adapter(),
		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		vite: {
			resolve: {
				alias: {
					'gaelic': gaelicPackageDir,
				},
			}
		}
	}
};

export default config;
