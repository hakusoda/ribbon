import type UserRibbonAgent from '../agent';
import type { RibbonAgentServerApi } from '../../types/agent/apis/server';
import type { GetServerResponse } from '../../types/api/server';
import type {
	CreateServerMemberLinkResponse,
	GetServerRolesResponse,
	UpdateServerMemberLinkPayload
} from '../../types/api/server/member_link';

export default class UserRibbonAgentServerApi implements RibbonAgentServerApi {
	public constructor(private agent: UserRibbonAgent) {}
	
	public get(server_id: string): Promise<GetServerResponse> {
		return this.agent.fetch_json(`/v1/server/${server_id}`);
	}
	
	public get_roles(server_id: string): Promise<GetServerRolesResponse> {
		return this.agent.fetch_json(`/v1/server/${server_id}/roles`);
	}
	
	public create_member_link(server_id: string, display_name: string): Promise<CreateServerMemberLinkResponse> {
		return this.agent.fetch_json(`/v1/server/${server_id}/member_links`, {
			body: JSON.stringify({ display_name }),
			method: 'POST',
			headers: { 'content-type': 'application/json' }
		});
	}
	
	public update_member_link(server_id: string, link_id: number, payload: UpdateServerMemberLinkPayload): Promise<void> {
		return this.agent.fetch_json(`/v1/server/${server_id}/member_link/${link_id}`, {
			body: JSON.stringify(payload),
			headers: { 'content-type': 'application/json' },
			method: 'PATCH'
		});
	}
	
	public delete_member_link(server_id: string, link_id: number): Promise<void> {
		return this.agent.fetch_json(`/v1/server/${server_id}/member_link/${link_id}`, {
			method: 'DELETE'
		});
	}
}