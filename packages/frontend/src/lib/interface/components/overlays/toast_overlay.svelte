<script lang="ts">
	import './toast_overlay.scss';

	import { get_toasts } from '$lib/client/store/interface/toasts.svelte';

	import ToastOverlayItem from './toast_overlay_item.svelte';
</script>

<div class="overlay_toasts">
	{#each get_toasts() as toast_item}
		<ToastOverlayItem toast={toast_item}/>
	{/each}
</div>