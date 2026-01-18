import type { GetServerResponse } from '../../api/server';
import type {
	GetServerRolesResponse,
	UpdateServerRolePayload
} from '../../api/server/member_link';

export interface RibbonAgentServerApi {
	get(server_id: string): Promise<GetServerResponse>
	get_roles(server_id: string): Promise<GetServerRolesResponse>
	
	update_role(server_id: string, role_id: string, payload: UpdateServerRolePayload): Promise<void>
}