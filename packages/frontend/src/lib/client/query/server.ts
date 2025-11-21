import { createQuery } from '@tanstack/svelte-query';

import { agent } from '../agent';
export function create_server_query(server_id: string) {
	return createQuery(() => ({
		queryKey: ['server', server_id],
		queryFn: () => agent.servers.get(server_id)
	}));
}

export function create_server_member_links_query(server_id: string) {
	return createQuery(() => ({
		queryKey: ['server', server_id, 'member_links'],
		queryFn: () => agent.servers.get_member_links(server_id)
	}));
}