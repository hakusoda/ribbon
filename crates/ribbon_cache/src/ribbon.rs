use dashmap::{
	mapref::{
		multiple::RefMulti,
		one::{ Ref, RefMut }
	},
	DashMap, DashSet
};
use rand::{ distributions::Alphanumeric, Rng };
use ribbon_models::{
	discord::InteractionRef,
	ribbon::{
		user::RobloxAccountModel,
		AuthoriseRequestModel, MemberLinkModel, ServerModel, UserModel
	}
};
use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};	

use crate::Result;

#[derive(Default)]
pub struct RibbonCache {
	pub authorise_requests: DashMap<String, AuthoriseRequestModel>,
	member_links: DashMap<u64, MemberLinkModel>,
	pub roblox_accounts: DashMap<u64, RobloxAccountModel>,
	pub servers: DashMap<Id<GuildMarker>, ServerModel>,
	server_member_links: DashMap<Id<GuildMarker>, DashSet<u64>>,
	users: DashMap<Id<UserMarker>, UserModel>,
	pub user_roblox_accounts: DashMap<Id<UserMarker>, DashSet<u64>>
}

impl RibbonCache {
	pub fn authorise_request(&self, token: &str) -> Option<Ref<'_, String, AuthoriseRequestModel>> {
		self.authorise_requests.get(token)
	}

	pub fn create_authorise_request(&self, interaction: impl Into<InteractionRef>) -> Ref<'_, String, AuthoriseRequestModel> {
		let token: String = rand::thread_rng()
			.sample_iter(Alphanumeric)
			.take(24)
			.map(char::from)
			.collect();
		self
			.authorise_requests
			.entry(token)
			.insert(AuthoriseRequestModel {
				interaction: interaction.into()
			})
			.downgrade()
	}

	pub fn member_link(&self, member_link_id: u64) -> Option<Ref<'_, u64, MemberLinkModel>> {
		self.member_links.get(&member_link_id)
	}

	pub fn member_links(&self, member_link_ids: &[u64]) -> Vec<RefMulti<u64, MemberLinkModel>> {
		self.member_links
			.iter()
			.filter(|x| member_link_ids.contains(&x.id))
			.collect()
	}

	pub async fn roblox_account(&self, link_id: u64) -> Result<Ref<'_, u64, RobloxAccountModel>> {
		Ok(match self.roblox_accounts.get(&link_id) {
			Some(model) => model,
			None => unimplemented!()
		})
	}

	pub async fn server(&self, guild_id: Id<GuildMarker>) -> Result<Ref<'_, Id<GuildMarker>, ServerModel>> {
		Ok(match self.servers.get(&guild_id) {
			Some(model) => model,
			None => self
				._insert_server(guild_id)
				.await?
				.downgrade()
		})
	}

	pub async fn server_mut(&self, guild_id: Id<GuildMarker>) -> Result<RefMut<Id<GuildMarker>, ServerModel>> {
		Ok(match self.servers.get_mut(&guild_id) {
			Some(model) => model,
			None => self
				._insert_server(guild_id)
				.await?
		})
	}

	pub async fn server_member_links(&self, guild_id: Id<GuildMarker>) -> Result<Vec<u64>> {
		Ok(match self.server_member_links.get(&guild_id) {
			Some(model) => model
				.iter()
				.map(|x| *x)
				.collect(),
			None => {
				let models = MemberLinkModel::get_server_many(guild_id)
					.await?;
				let model_ids: Vec<_> = models
					.iter()
					.map(|x| x.id)
					.collect();
				for model in models {
					self.member_links.insert(model.id, model);
				}

				self.server_member_links
					.entry(guild_id)
					.or_default()
					.extend(model_ids.clone());
				model_ids
			}
		})
	}

	async fn _insert_server(&self, guild_id: Id<GuildMarker>) -> Result<RefMut<Id<GuildMarker>, ServerModel>> {
		let new_model = ServerModel::get(guild_id)
			.await?
			.unwrap_or_else(|| ServerModel::from(guild_id));
		Ok(self.servers
			.entry(guild_id)
			.insert(new_model)
		)
	}

	pub async fn user(&self, user_id: Id<UserMarker>) -> Result<Ref<'_, Id<UserMarker>, UserModel>> {
		Ok(match self.users.get(&user_id) {
			Some(model) => model,
			None => {
				let new_model = UserModel::get_or_insert(user_id)
					.await?;
				self
					.users
					.entry(user_id)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn user_roblox_accounts(&self, user_id: Id<UserMarker>) -> Result<Vec<u64>> {
		Ok(match self.user_roblox_accounts.get(&user_id) {
			Some(model) => model
				.iter()
				.map(|x| *x)
				.collect(),
			None => {
				let models = RobloxAccountModel::get_user_many(user_id)
					.await?;
				let model_ids: Vec<_> = models
					.iter()
					.map(|x| x.id)
					.collect();
				for model in models {
					self.roblox_accounts.insert(model.id, model);
				}

				self.user_roblox_accounts
					.entry(user_id)
					.or_default()
					.extend(model_ids.clone());
				model_ids
			}
		})
	}
}