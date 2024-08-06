use dashmap::{
	mapref::one::Ref,
	DashMap
};
use twilight_model::id::{
	marker::{ ChannelMarker, GuildMarker, RoleMarker, UserMarker },
	Id
};	
use ribbon_models::discord::{
	guild::{ MemberModel, RoleModel },
	ChannelModel, GuildModel
};

use crate::Result;

#[derive(Default)]
pub struct DiscordCache {
	pub channels: DashMap<Id<ChannelMarker>, ChannelModel>,
	pub guilds: DashMap<Id<GuildMarker>, GuildModel>,
	pub members: DashMap<(Id<GuildMarker>, Id<UserMarker>), MemberModel>,
	pub private_channels: DashMap<Id<UserMarker>, Id<ChannelMarker>>,
	pub roles: DashMap<Id<RoleMarker>, RoleModel>
}

impl DiscordCache {
	pub async fn channel(&self, channel_id: Id<ChannelMarker>) -> Result<Ref<'_, Id<ChannelMarker>, ChannelModel>> {
		Ok(match self.channels.get(&channel_id) {
			Some(model) => model,
			None => {
				let new_model = ChannelModel::get(channel_id)
					.await?;
				self.channels.entry(channel_id)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn guild(&self, guild_id: Id<GuildMarker>) -> Result<Ref<'_, Id<GuildMarker>, GuildModel>> {
		Ok(match self.guilds.get(&guild_id) {
			Some(model) => model,
			None => {
				let new_model = GuildModel::get(guild_id)
					.await?;
				self.guilds
					.entry(guild_id)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn member(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<Ref<'_, (Id<GuildMarker>, Id<UserMarker>), MemberModel>> {
		let key = (guild_id, user_id);
		Ok(match self.members.get(&key) {
			Some(model) => model,
			None => {
				let new_model = MemberModel::get(guild_id, user_id)
					.await?;
				self.members
					.entry(key)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn private_channel(&self, user_id: Id<UserMarker>) -> Result<Id<ChannelMarker>> {
		Ok(*match self.private_channels.get(&user_id) {
			Some(model) => model,
			None => {
				let new_model = ChannelModel::get_private(user_id)
					.await?;
				let new_model_id = new_model.id;

				self.channels.insert(new_model_id, new_model);
				self.private_channels.entry(user_id)
					.insert(new_model_id)
					.downgrade()
			}
		})
	}

	pub fn role(&self, role_id: Id<RoleMarker>) -> Option<Ref<'_, Id<RoleMarker>, RoleModel>> {
		self.roles.get(&role_id)
	}
}