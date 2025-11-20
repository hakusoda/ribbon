import icons from 'unplugin-icons/vite'
import { sentrySvelteKit } from '@sentry/sveltekit';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
export default defineConfig({
	build: {
		target: 'esnext'
	},
	plugins: [
		icons({
			compiler: 'svelte',
			scale: 1,
			iconCustomizer(_collection, _icon, props) {
				props['font-size'] = '16px';
			}
		}),
		sentrySvelteKit() as any,
		sveltekit()
	],
	server: {
		port: 5173,
		strictPort: true
	}
});