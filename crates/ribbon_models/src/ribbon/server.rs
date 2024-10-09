use ribbon_util::PG_POOL;
use serde::Serialize;
use twilight_model::id::{
	marker::GuildMarker,
	Id
};

use crate::Result;

#[derive(Serialize)]
pub struct ServerModel {
	pub id: Id<GuildMarker>,
	pub display_name: String
}

impl ServerModel {
	pub async fn get(guild_id: Id<GuildMarker>) -> Result<Option<Self>> {
		Ok(sqlx::query!(
			"
			SELECT id
			FROM servers
			WHERE id = $1
			",
			guild_id.get() as i64
		)
			.fetch_optional(&*std::pin::Pin::static_ref(&PG_POOL).await)
			.await?
			.map(|record| Self {
				id: Id::new(record.id as u64),
				display_name: "placeholder".into()
			})
		)
	}
}

impl From<Id<GuildMarker>> for ServerModel {
	fn from(value: Id<GuildMarker>) -> Self {
		Self {
			id: value,
			display_name: "placeholder".into()
		}
	}
}