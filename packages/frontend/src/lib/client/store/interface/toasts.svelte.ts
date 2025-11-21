import type { CreateToastOptions, Toast } from '$lib/client/types/store/interface/toasts';

const toast_items: Toast[] = $state([]);

export function get_toasts(): Toast[] {
	return toast_items;
}

export function create_toast(options: CreateToastOptions) {
	const new_toast: Toast = {
		kind: 'default',

		actions: options.actions ?? [],
		content_id: options.content_id,
		metadata: options.metadata ?? {},

		timeout: setTimeout(() => remove_toast(new_toast), 6000)
	};
	toast_items.push(new_toast);
}

export function remove_toast(toast: Toast) {
	const toast_index = toast_items.indexOf(toast);
	if (toast_index != -1)
		toast_items.splice(toast_index, 1);
}