import adapter from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import { join } from 'path';
import { sveltePreprocess } from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
export default {
	extensions: ['.md', '.svelte'],
	kit: {
		adapter: adapter(),
		experimental: {
			instrumentation: {
				server: true
			}
		}
	},
	preprocess: [
		mdsvex({
			extension: '.md',
			layout: join(import.meta.dirname, './src/lib/interface/layouts/markdown.svelte')
		}),
		sveltePreprocess({}),
		vitePreprocess()
	]
};