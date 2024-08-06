use ribbon_util::PG_POOL;
use twilight_model::id::{
	marker::GuildMarker,
	Id
};

use crate::Result;

pub struct ServerModel {
	pub id: Id<GuildMarker>
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
				id: Id::new(record.id as u64)
			})
		)
	}
}

impl From<Id<GuildMarker>> for ServerModel {
	fn from(value: Id<GuildMarker>) -> Self {
		Self {
			id: value
		}
	}
}