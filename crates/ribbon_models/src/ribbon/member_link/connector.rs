use serde::{ Deserialize, Serialize };
use twilight_model::id::{
	marker::RoleMarker,
	Id
};

#[derive(Deserialize, Serialize)]
pub struct ConnectorsModel {
	pub items: Vec<Connector>
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Connector {
	Nickname,
	Roles {
		target_role_ids: Vec<Id<RoleMarker>>
	}
}