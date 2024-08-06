use ribbon_commands_core::{ Context, Error, Result, command };
use ribbon_syncing::SyncOperation;

#[command(slash, context = "guild", description = "test command for guilds")]
pub async fn guild_test(context: Context) -> Result<()> {
	context.reply("omg, haiii!!!")
		.ephemeral()
		.await
}

#[command(slash, context = "guild", description = "Sync your server profile with the Roblox platform.")]
pub async fn sync(context: Context) -> Result<()> {
	SyncOperation::from_interaction(&context, false)
		.await
		.map_err(|x| {
			println!("{x}");
			Error::Unknown
		})?;

	Ok(())
}