import type { GetServerResponse } from '../../api/server';
import type {
	CreateServerMemberLinkResponse,
	GetServerMemberLinksResponse,
	UpdateServerMemberLinkPayload
} from '../../api/server/member_link';

export interface RibbonAgentServerApi {
	get(server_id: string): Promise<GetServerResponse>
	get_member_links(server_id: string): Promise<GetServerMemberLinksResponse>
	
	create_member_link(server_id: string, display_name: string): Promise<CreateServerMemberLinkResponse>
	update_member_link(server_id: string, link_id: number, payload: UpdateServerMemberLinkPayload): Promise<void>
	delete_member_link(server_id: string, link_id: number): Promise<void>
}