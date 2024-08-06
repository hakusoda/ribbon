#![feature(let_chains, type_alias_impl_trait)]
use dashmap::DashSet;
use ribbon_cache::CACHE;
use ribbon_models::{
	discord::InteractionRef,
	ribbon::member_link::{
		connector::Connector,
		criteria::CriteriaItem
	}
};
use ribbon_util::{
	emoji::{ EMOJI_ARROW_CLOCKWISE, EMOJI_ICON_DISCORD, EMOJI_ICON_ROBLOX },
	DISCORD_CLIENT, DISCORD_INTERACTION_CLIENT, WEBSITE_URL
};
use std::fmt::Display;
use twilight_http::request::AuditLogReason;
use twilight_model::{
	channel::message::{
		component::{ ActionRow, Button, ButtonStyle },
		MessageFlags, ReactionType
	},
	http::interaction::{ InteractionResponse, InteractionResponseData, InteractionResponseType },
	id::{
		marker::{ GuildMarker, RoleMarker, UserMarker },
		Id
	}
};
use twilight_util::builder::InteractionResponseDataBuilder;

pub mod error;

pub use error::*;

pub struct SyncOperation {
	guild_id: Id<GuildMarker>,
	user_id: Id<UserMarker>,

	interaction: Option<(InteractionRef, bool)>
}

// TODO: queue system, of some sorts? i think that would be good
impl SyncOperation {
	pub fn from_interaction(interaction: impl Into<InteractionRef>, is_acknowledged: bool) -> Self {
		let interaction_ref: InteractionRef = interaction.into();
		Self {
			guild_id: interaction_ref.guild_id.unwrap(),
			user_id: interaction_ref.user_id.unwrap(),

			interaction: Some((interaction_ref, is_acknowledged))
		}
	}

	async fn execute(self) -> Result<SyncOperationResult> {
		let guild_id = self.guild_id;
		let user_id = self.user_id;

		let link_ids = CACHE
			.ribbon
			.user_roblox_accounts(user_id)
			.await?;
		let Some(link_id) = link_ids.into_iter().next() else {
			if let Some((interaction,_)) = self.interaction {
				let request = CACHE
					.ribbon
					.create_authorise_request(interaction.clone());
				let callback_url = format!("{WEBSITE_URL}/roblox_request/{}", request.key());
				DISCORD_INTERACTION_CLIENT
					.create_response(interaction.id, &interaction.token, &InteractionResponse {
						kind: InteractionResponseType::ChannelMessageWithSource,
						data: Some(InteractionResponseData {
							components: Some(vec![ActionRow {
								components: vec![Button {
									custom_id: None,
									disabled: false,
									emoji: Some(ReactionType::Custom {
										animated: false,
										id: EMOJI_ICON_ROBLOX,
										name: None
									}),
									label: Some("Connect Roblox Account".into()),
									style: ButtonStyle::Link,
									url: Some(callback_url)
								}.into()]
							}.into()]),
							content: Some("you don't have a roblox account connected!!!!!\nAHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH!!!!!!!!!!!".to_string()),
							flags: Some(MessageFlags::EPHEMERAL),
							..Default::default()
						})
					})
					.await?;
				return Ok(SyncOperationResult::Cancelled);
			} else {
				unimplemented!()
			}
		};

		let account = CACHE
			.ribbon
			.roblox_account(link_id)
			.await?;
		
		let roblox_id = account.roblox_id;
		if let Some((interaction, acknowledged)) = &self.interaction && !acknowledged {
			DISCORD_INTERACTION_CLIENT
				.create_response(interaction.id, &interaction.token, &InteractionResponse {
					kind: InteractionResponseType::DeferredChannelMessageWithSource,
					data: Some(InteractionResponseDataBuilder::new()
						.flags(MessageFlags::EPHEMERAL)
						.build()
					)
				})
				.await?;
		}
			
		let member_link_ids = CACHE
			.ribbon
			.server_member_links(guild_id)
			.await?;

		let mut role_changes: Vec<RoleChange> = Vec::new();
		for member_link_id in member_link_ids {
			if let Some(member_link) = CACHE.ribbon.member_link(member_link_id) {
				println!("{}", member_link.display_name);
				let mut is_criteria_met = true;
				for item in &member_link.criteria.items {
					let is_item_met = match item {
						CriteriaItem::GroupMembership { group_id } => {
							let group_ids = CACHE
								.roblox
								.user_memberships(roblox_id)
								.await?;

							group_ids
								.iter()
								.any(|x| x == group_id)
						},
						CriteriaItem::GroupMembershipRole { group_id, role_id } => {
							CACHE
								.roblox
								.user_memberships(roblox_id)
								.await?;
							
							CACHE
								.roblox
								.membership(*group_id, roblox_id)
								.is_some_and(|x| &x.role_id() == role_id)
						},
						CriteriaItem::ValidAccount => true
					};
					if !is_item_met {
						is_criteria_met = false;
						break;
					}
				}

				for connector in &member_link.connectors.items {
					match connector {
						Connector::Roles { target_role_ids } => RoleChange::extend_with_many(
							&mut role_changes,
							if is_criteria_met { RoleChangeKind::Assign } else { RoleChangeKind::Remove },
							target_role_ids
						),
						_ => unimplemented!()
					}
				}
			}
		}
		println!("{role_changes:#?}");

		if !role_changes.is_empty() {
			let member = CACHE
				.discord
				.member(guild_id, user_id)
				.await?;
			if let Some(new_roles) = RoleChange::apply_changes(&role_changes, &member.roles) {
				DISCORD_CLIENT
					.update_guild_member(guild_id, user_id)
					.reason("sync operation")
					.roles(&new_roles)
					.await?;
			}
		}

		let profile_changed = !role_changes.is_empty();
		let operation_result = SyncOperationResult::Success {
			profile_changed,
			role_changes
		};

		if let Some((interaction, _acknowledged)) = self.interaction {
			DISCORD_INTERACTION_CLIENT
				.update_response(&interaction.token)
				.content(Some(&operation_result.to_string()))
				.components(Some(&[ActionRow {
					components: vec![
						Button {
							custom_id: Some("sync_again".into()),
							disabled: false,
							emoji: Some(ReactionType::Custom {
								animated: false,
								id: EMOJI_ARROW_CLOCKWISE,
								name: None
							}),
							label: Some("Sync Again".into()),
							style: ButtonStyle::Primary,
							url: None
						}.into(),
						Button {
							custom_id: None,
							disabled: false,
							emoji: Some(ReactionType::Custom {
								animated: false,
								id: EMOJI_ICON_DISCORD,
								name: None
							}),
							label: Some("Get Support".into()),
							style: ButtonStyle::Link,
							url: Some("https://discord.com/invite/rs3r4dQu9P".into())
						}.into()
					]
				}.into()]))
				.await?;
		}

		Ok(operation_result)
	}
}

#[derive(Debug)]
pub struct RoleChange {
	pub kind: RoleChangeKind,
	pub role_id: Id<RoleMarker>
}

impl RoleChange {
	pub fn new(kind: RoleChangeKind, role_id: Id<RoleMarker>) -> Self {
		Self { kind, role_id }
	}

	pub fn extend_with_many(target: &mut Vec<RoleChange>, kind: RoleChangeKind, role_ids: &[Id<RoleMarker>]) {
		target.extend(
			role_ids
				.iter()
				.map(|role_id| Self::new(kind.clone(), *role_id))	
		);
	}

	pub fn is_assign(&self) -> bool {
		matches!(self.kind, RoleChangeKind::Assign)
	}

	pub fn is_remove(&self) -> bool {
		matches!(self.kind, RoleChangeKind::Remove)
	}

	pub fn format(&self) -> String {
		let role = CACHE
			.discord
			.role(self.role_id);
		let role_name = role
			.as_ref()
			.map_or("", |x| &x.name);
		format!("{} {role_name}", match self.kind {
			RoleChangeKind::Assign => "+",
			RoleChangeKind::Remove => "-"
		})
	}

	pub fn apply_changes(role_changes: &[RoleChange], roles: &[Id<RoleMarker>]) -> Option<Vec<Id<RoleMarker>>> {
		let removed_roles: Vec<_> = role_changes
			.iter()
			.filter_map(|x| match x.is_remove() {
				true => Some(x.role_id),
				false => None
			})
			.collect();
		let new_roles: DashSet<_> = roles
			.iter()
			.filter(|x| !removed_roles.contains(x))
			.copied()
			.collect();
		let mut added_role = false;
		for role_change in role_changes {
			if role_change.is_assign() {
				new_roles.insert(role_change.role_id);
				added_role = true;
			}
		}

		if removed_roles.is_empty() && !added_role { None } else {
			Some(new_roles
				.into_iter()
				.collect()
			)
		}
	}
}

#[derive(Clone, Debug)]
pub enum RoleChangeKind {
	Assign,
	Remove
}

pub enum SyncOperationResult {
	Success {
		profile_changed: bool,
		role_changes: Vec<RoleChange>
	},
	Cancelled
}

impl Display for SyncOperationResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Success { profile_changed, role_changes } => if *profile_changed {
				writeln!(f, "## <:person_fill_check:1269956304143913011>  Server profile updated!")?;
				if role_changes.is_empty() {
					write!(f, "uhhhh")
				} else {
					write!(f, "Your roleset has been updated, see changes below.\n```diff\n{}```", role_changes
						.iter()
						.map(RoleChange::format)
						.collect::<Vec<_>>()
						.join("\n")
					)
				}
			} else {
				write!(f, "## profile did NOT change")
			},
			Self::Cancelled => write!(f, "cancelled")
		}
	}
}

pub type SyncOperationFuture = impl Future<Output = Result<SyncOperationResult>>;

impl IntoFuture for SyncOperation {
	type IntoFuture = SyncOperationFuture;
	type Output = Result<SyncOperationResult>;

	fn into_future(self) -> Self::IntoFuture {
		self.execute()
	}
}