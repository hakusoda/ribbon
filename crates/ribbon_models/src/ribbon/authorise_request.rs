use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};

use crate::discord::InteractionRef;

pub struct AuthoriseRequestModel {
	pub interaction: InteractionRef
}

impl AuthoriseRequestModel {
	pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
		self.interaction.guild_id
	}

	pub fn user_id(&self) -> Option<Id<UserMarker>> {
		self.interaction.user_id
	}
}