import { createQuery } from '@tanstack/svelte-query';

import { agent } from '../agent';
export function create_server_query(server_id: string) {
	return createQuery(() => ({
		queryKey: ['server', server_id],
		queryFn: () => agent.servers.get(server_id)
	}));
}

export function create_server_roles_query(server_id: string) {
	return createQuery(() => ({
		queryKey: ['server', server_id, 'roles'],
		queryFn: () => agent.servers.get_roles(server_id)
	}));
}