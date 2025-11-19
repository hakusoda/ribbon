import icons from 'unplugin-icons/vite'
import { sentrySvelteKit } from '@sentry/sveltekit';
import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
export default {
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
		sentrySvelteKit(),
		sveltekit()
	],
	server: {
		port: 5174,
		strictPort: true
	}
} satisfies UserConfig;