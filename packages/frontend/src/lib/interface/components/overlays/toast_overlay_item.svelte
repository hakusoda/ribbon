<script lang="ts">
	import './toast_overlay_item.scss';
	import { remove_toast } from '$lib/client/store/interface/toasts.svelte';
	import type { Toast } from '$lib/client/types/store/interface/toasts';

	import IconInfoCircle from 'virtual:icons/bi/info-circle';

	let { toast }: {
		toast: Toast
	} = $props();
	let fade = $state(false);
</script>

<div class="toast_overlay_item" class:fade>
	<div class="icon">
		<IconInfoCircle font-size={18}/>
	</div>
	<p>
		<!-- this is a temporary thing -->
		{#if toast.content_id === 'action.server.member_link.created'}
			<w>{toast.metadata.display_name}</w> member link has been created
		{:else if toast.content_id === 'action.server.member_link.saved'}
			<w>{toast.metadata.display_name}</w> member link has been saved
		{:else if toast.content_id === 'error.resource.unauthenticated'}
			You need to sign-in to access this resource.
		{:else}
			{toast.content_id}
		 {/if}
	</p>
	{#each toast.actions as action_item}
		<button type="button" onclick={() => {
			fade = true;
			clearTimeout(toast.timeout);
			setTimeout(remove_toast, 500);

			action_item.callback();
		}}>
			{action_item.content_id}
		</button>
	{/each}
</div>