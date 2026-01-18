export type GetServerRolesResponse = ServerRole[]
export interface UpdateServerRolePayload {
	
}

export interface ServerRole {
	id: string,
	name: string,
	
	member_count: number,
	requirements_mock: string
}

export interface MemberLink {
	id: number,
	display_name: string,

	connectors: MemberLinkConnectors,
	criteria: MemberLinkCriteria
}

export interface MemberLinkConnectors {
	items: MemberLinkConnector[]
}

export type MemberLinkConnector =
	MemberLinkConnectorNickname |
	MemberLinkConnectorRoles

export interface MemberLinkConnectorNickname {
	kind: 'nickname'
}

export interface MemberLinkConnectorRoles {
	kind: 'roles',
	target_role_ids: string[]
}

export interface MemberLinkCriteria {
	items: MemberLinkCriteriaItem[]
}

export type MemberLinkCriteriaItem =
	MemberLinkCriteriaItemGroupMembership |
	MemberLinkCriteriaItemGroupMembershipRole |
	MemberLinkCriteriaItemValidAccount

export interface MemberLinkCriteriaItemGroupMembership {
	kind: 'group_membership',
	group_id: string
}
export interface MemberLinkCriteriaItemGroupMembershipRole {
	kind: 'group_membership_role',
	group_id: string,
	role_id: string
}
export interface MemberLinkCriteriaItemValidAccount {
	kind: 'valid_account'
}