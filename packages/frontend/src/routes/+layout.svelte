<script lang="ts">
	import { onMount } from 'svelte';
	import { QueryCache, QueryClient, QueryClientProvider } from '@tanstack/svelte-query';
	
	import { create_toast } from '$lib/client/store/interface/toasts.svelte';
	import { FetchError } from '$lib/fetch';
	
	import { browser } from '$app/environment';
	import { PUBLIC_SOCIAL_LINK_DISCORD, PUBLIC_SOCIAL_LINK_GITHUB, PUBLIC_SOCIAL_LINK_KOFI } from '$env/static/public';
	
	import '$lib/interface/styles/root.scss';
	
	import BrandLogo from '$lib/interface/visuals/brand_logo.svelte';
	import DiscordIcon from '$lib/interface/visuals/socials/discord_icon.svelte';
	import HakumiLogo from '$lib/interface/visuals/hakumi_logo.svelte';
	import KofiIcon from '$lib/interface/visuals/socials/kofi_icon.svelte';
	
	import ToastOverlay from '$lib/interface/components/overlays/toast_overlay.svelte';
	const query_client = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser,
				retry(failure_count, error) {
					if (error instanceof FetchError && error.status === 401)
						return false;
					
					return failure_count < 3;
				}
			}
		},
		queryCache: new QueryCache({
			onError(error, query) {
				if (error instanceof FetchError && error.status === 401)
					create_toast({ content_id: 'error.resource.unauthenticated' });
			}
		})
	});
	
	let header_hover = $state(false);
	onMount(() => header_hover = document.body.scrollTop > 0);
	
	const { children } = $props();
</script>

<QueryClientProvider client={query_client}>
	<div class="cool_top_blur" class:hover={header_hover}></div>
	<nav>
		<div class="header" class:hover={header_hover}>
			<a class="brand_logo" href="/" title="Ribbon">
				<BrandLogo height={30}/>
			</a>
			<div class="links">
				<a href="/">
					Dashboard
				</a>
			</div>
		</div>
	</nav>
	<main>
		{@render children()}
	</main>
	<footer>
		<div class="footer_gradient"></div>
		<div class="footer_contents">
			<div class="brand">
				<div class="studio_logos">
					<a class="brand_logo" href="/" title="Ribbon">
						<BrandLogo height={40}/>
					</a>
					<a class="brand_logo" href="https://hakumi.cafe" title="HAKUMI">
						<HakumiLogo height={56}/>
					</a>
				</div>
				<div class="lower_section">
					<a class="social_logo" href={PUBLIC_SOCIAL_LINK_DISCORD} title="Discord" target="_blank">
						<DiscordIcon size={24}/>
					</a>
					<a class="social_logo" href={PUBLIC_SOCIAL_LINK_KOFI} title="Ko-fi" target="_blank">
						<KofiIcon size={24}/>
					</a>
					<p class="legal" aria-hidden="true">
						© {new Date().getFullYear()} HAKUMI
					</p>
				</div>
			</div>
			<div class="links">
				<a href="/terms">
					Terms
				</a>
				<a href="/privacy">
					Privacy
				</a>
				<a href={PUBLIC_SOCIAL_LINK_GITHUB}>
					Open Source
				</a>
			</div>
		</div>
	</footer>
	<ToastOverlay/>
</QueryClientProvider>

<svelte:document on:scroll={() => header_hover = document.documentElement.scrollTop > 0}/>

<style lang="scss">
	.cool_top_blur {
		backdrop-filter: blur(6px);
		background: color-mix(in oklab, hsl(315 10% 9%) 80%, transparent);
		height: 96px;
		left: 0;
		mask-image: linear-gradient(#000 20%, transparent 80%);
		opacity: 0;
		position: fixed;
		top: 0;
		transition: opacity .5s;
		width: 100%;
		&.hover {
			opacity: 1;
		}
	}
	nav {
		left: 0;
		padding: 0 16px;
		pointer-events: none;
		position: fixed;
		top: 0;
		width: 100%;
		.header {
			align-items: center;
			backdrop-filter: blur(16px);
			border-radius: 24px;
			display: flex;
			height: 64px;
			margin: 16px auto;
			max-width: 1200px;
			padding: 16px 32px;
			pointer-events: all;
			transition: background .2s, box-shadow .2s, padding .5s;
			&.hover {
				background: hsla(315, 10%, 5%, .3);
				box-shadow: inset 0 0 0 1px hsla(315, 80%, 90%, .15);
				padding: 16px 24px;
			}
			.brand_logo {
				color: #fff;
				margin: 0 16px 0 0;
			}
			.links {
				margin: 0 0 0 auto;
				a {
					color: #fff;
					font-weight: 500;
					text-decoration: none;
				}
			}
		}
	}
	main {
		min-height: 100vh;
		margin: 0 auto;
		max-width: 1200px;
		padding: 80px 16px;
		width: 100%;
	}
	footer {
		background: linear-gradient(to bottom, hsl(350, 12%, 7%), hsl(10, 40%, 6%));
		border-radius: 24px 24px 0 0;
		border-top: 1px solid hsl(350, 20%, 13%);
		overflow: hidden;
		position: relative;
		.footer_gradient {
			background: #d22f661a;
			border-radius: 50%;
			bottom: 0;
			content: '';
			filter: blur(60px);
			height: 50px;
			left: 50%;
			max-width: 1000px;
			position: absolute;
			transform: translate(-50%, 60%);
			width: 70%;
		}
		.footer_contents {
			align-items: center;
			display: flex;
			margin: auto auto 0 auto;
			max-width: 1200px;
			padding: 48px 16px;
			width: 100%;
		}
		.studio_logos {
			align-items: center;
			display: flex;
			gap: 40px;
			.brand_logo {
				color: #fff;
				line-height: 0;
			}
		}
		.lower_section {
			align-items: center;
			display: flex;
			gap: 12px;
			margin-top: 24px;
			.social_logo {
				color: #fff;
				line-height: 0;
			}
			.legal {
				font-family: 'Outfit', sans-serif;
				font-size: .9em;
				font-weight: 500;
				margin: 0 12px 0 auto;
			}
		}
		.links {
			display: flex;
			gap: 32px;
			margin-left: auto;
			a {
				color: var(--color-secondary);
				font-size: .9em;
				font-weight: 450;
				text-decoration: none;
				transition: color .5s;
				&:hover {
					color: var(--color-secondary-hover);
				}
			}
		}
	}
</style>