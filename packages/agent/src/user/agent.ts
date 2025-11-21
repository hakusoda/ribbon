import UserRibbonAgentServerApi from './apis/server';
import type { RibbonAgent } from '../types/agent';

export default class UserRibbonAgent implements RibbonAgent {
	public servers: UserRibbonAgentServerApi
	
	public constructor(private base_url: string) {
		this.servers = new UserRibbonAgentServerApi(this);
	}
	
	public async fetch_json<T>(path: string, { body, headers, method = 'GET' }: FetchOptions = {}): Promise<T> {
		const response = await fetch(`${this.base_url}${path}`, {
			body,
			credentials: 'include',
			headers: {
				accept: 'application/json',
				...headers
			},
			method
		});
		if (response.status != 200)
			throw new FetchError(await response.text(), response.status);
		
		return response.json();
	}
}

export interface FetchOptions {
	body?: string,
	headers?: Record<string, string>,
	method?: 'GET' | 'POST' | 'PATCH' | 'DELETE'
}

export class FetchError extends Error {
	constructor(message: string, public status: number) {
		super(message);
		this.name = "FetchError";
	}
}