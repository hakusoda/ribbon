import type UserRibbonAgent from '../agent';
import type { RibbonAgentServerApi } from '../../types/agent/apis/server';
import type { GetServerResponse } from '../../types/api/server';
import type {
	CreateServerMemberLinkResponse,
	GetServerMemberLinksResponse,
	UpdateServerMemberLinkPayload,
	MemberLink
} from '../../types/api/server/member_link';

export default class MockRibbonAgentServerApi implements RibbonAgentServerApi {
	private servers: Map<string, InternalServer> = new Map();
	
	public constructor(private agent: UserRibbonAgent) {}
	
	public get(server_id: string): Promise<GetServerResponse> {
		return Promise.resolve(this.internal_get(server_id));
	}
	
	public get_member_links(server_id: string): Promise<GetServerMemberLinksResponse> {
		return Promise.resolve(this.internal_get(server_id).member_links.values().toArray());
	}
	
	public create_member_link(server_id: string, display_name: string): Promise<CreateServerMemberLinkResponse> {
		const server = this.internal_get(server_id);
		
		const new_link: MemberLink = {
			id: server.member_links.size + 1,
			display_name,
			connectors: { items: [] },
			criteria: { items: [] }
		};
		server.member_links.set(new_link.id, new_link);
		
		return Promise.resolve(new_link);
	}
	
	public update_member_link(server_id: string, link_id: number, payload: UpdateServerMemberLinkPayload): Promise<void> {
		const server = this.internal_get(server_id);
		const link = server.member_links.get(link_id);
		if (!link)
			return Promise.reject(new Error("no such mock item"));
		
		if (payload.display_name != undefined)
			link.display_name = payload.display_name;
		if (payload.connectors)
			link.connectors = payload.connectors;
		if (payload.criteria)
			link.criteria = payload.criteria;
		
		return Promise.resolve();
	}
	
	public delete_member_link(server_id: string, link_id: number): Promise<void> {
		const server = this.internal_get(server_id);
		server.member_links.delete(link_id);
		
		return Promise.resolve();
	}
	
	private internal_get(server_id: string): InternalServer {
		const server = this.servers.get(server_id);
		if (!server) {
			const new_server: InternalServer = {
				id: server_id,
				display_name: `${server_id} (Mock)`,
				member_links: new Map()
			};
			this.servers.set(server_id, new_server);
			
			return new_server;
		}
		
		return server;
	}
}

export interface InternalServer {
	id: string,
	display_name: string,
	member_links: Map<number, MemberLink>
}