<script lang="ts">
	import { page } from '$app/state';

	import type { SideNavigationItem } from '$lib/client/types/interface/side_navigation';
	
	let { items }: {
		items: SideNavigationItem[]
	} = $props();
	
	const pathname = $derived(page.url.pathname);
</script>

<div class="side_navigation">
	<p>NAVIGATION???</p>
	{#each items as navigation_item}
		{@const item_pathname = navigation_item.pathname.replaceAll(/\[(\w+)\]/g, (_,param) => (page.params as any)[param])}
		<a href={item_pathname} class:active={item_pathname === pathname}>
			{navigation_item.display_label}
		</a>
	{/each}
</div>

<style lang="scss">
	.side_navigation {
		display: flex;
		flex-direction: column;
		padding: 24px 64px;

		p {
			color: #ffffff40;
			font-size: .8em;
		}
		a {
			color: #ffffff80;
			text-decoration: none;
			&.active {
				color: #fff;
			}
		}
	}
</style>