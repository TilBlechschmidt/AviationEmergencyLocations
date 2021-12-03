import adapter from '@sveltejs/adapter-node';
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
		adapter: adapter({
			out: 'build',
			precompress: true,
		}),
		target: '#svelte',
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
