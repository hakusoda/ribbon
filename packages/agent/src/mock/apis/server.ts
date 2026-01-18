import type UserRibbonAgent from '../agent';
import type { RibbonAgentServerApi } from '../../types/agent/apis/server';
import type { GetServerResponse } from '../../types/api/server';
import type {
	GetServerRolesResponse,
	UpdateServerRolePayload,
	ServerRole
} from '../../types/api/server/member_link';

export default class MockRibbonAgentServerApi implements RibbonAgentServerApi {
	private servers: Map<string, InternalServer> = new Map();
	
	public constructor(private agent: UserRibbonAgent) {}
	
	public get(server_id: string): Promise<GetServerResponse> {
		return Promise.resolve(this.internal_get(server_id));
	}
	
	public get_roles(server_id: string): Promise<GetServerRolesResponse> {
		return Promise.resolve(this.internal_get(server_id).roles.values().toArray());
	}
	
	public update_role(server_id: string, role_id: string, payload: UpdateServerRolePayload): Promise<void> {
		const server = this.internal_get(server_id);
		const role = server.roles.get(role_id);
		if (!role)
			return Promise.reject(new Error("no such mock item"));
		
		return Promise.resolve();
	}
	
	private internal_get(server_id: string): InternalServer {
		const server = this.servers.get(server_id);
		if (!server) {
			const new_server: InternalServer = {
				id: server_id,
				display_name: `${server_id} (Mock)`,
				roles: new Map([
					['1111111111', { id: '1111111111', name: 'Admin', member_count: 0, requirements_mock: 'Inactive' }],
					['1234567890', { id: '1234567890', name: 'Special Role', member_count: 5, requirements_mock: 'Group Role' }],
					['1430000000', { id: '1430000000', name: 'Verified', member_count: 143, requirements_mock: 'Group Membership' }],
				])
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
	roles: Map<string, ServerRole>
}