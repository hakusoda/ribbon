use futures::TryStreamExt;
use ribbon_util::PG_POOL;
use serde::Serialize;
use std::pin::Pin;
use twilight_model::id::{
	marker::GuildMarker,
	Id
};

use crate::Result;

pub mod connector;
pub mod criteria;

pub use connector::ConnectorsModel;
pub use criteria::CriteriaModel;

#[derive(Serialize)]
pub struct MemberLinkModel {
	pub connectors: ConnectorsModel,
	pub criteria: CriteriaModel,
	pub display_name: String,
	pub id: u64
}

impl MemberLinkModel {
	pub async fn get_server_many(guild_id: Id<GuildMarker>) -> Result<Vec<Self>> {
		Ok(sqlx::query!(
			"
			SELECT connectors, criteria, display_name, id
			FROM server_member_links
			WHERE server_id = $1
			",
			guild_id.get() as i64
		)
			.fetch(&*Pin::static_ref(&PG_POOL).await)
			.try_fold(Vec::new(), |mut acc, record| {
				acc.push(Self {
					connectors: serde_json::from_value(record.connectors).unwrap(),
					criteria: serde_json::from_value(record.criteria).unwrap(),
					display_name: record.display_name,
					id: record.id as u64
				});

				async move { Ok(acc) }
			})
			.await?
		)
	}
}