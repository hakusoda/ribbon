use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct CriteriaModel {
	pub items: Vec<CriteriaItem>
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CriteriaItem {
	GroupMembership {
		group_id: u64
	},
	GroupMembershipRole {
		group_id: u64,
		role_id: u64
	},
	ValidAccount
}