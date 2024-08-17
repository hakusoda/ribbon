#![feature(let_chains, try_blocks, const_async_blocks, type_alias_impl_trait)]
use clap::Parser;
use env_logger::Env;
use log::info;
use once_cell::sync::Lazy;
use std::pin::Pin;
use ribbon_cache::CACHE;
use ribbon_commands::commands::COMMANDS;
use ribbon_util::{ DISCORD_APP_ID, DISCORD_CLIENT, DISCORD_INTERACTION_CLIENT };
use twilight_gateway::CloseFrame;
use twilight_model::application::command::CommandType;

use discord::command::ApplicationCommand;
pub use error::Result;

mod discord;
mod error;

#[derive(Parser)]
struct Args {
	#[clap(long, short)]
    update_commands: bool
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

	info!("starting Ribbon v{}", env!("CARGO_PKG_VERSION"));
	
	let args = Args::parse();
	if args.update_commands {
		let mut commands: Vec<ApplicationCommand> = vec![];
		for command in COMMANDS.iter() {
			if command.is_slash {
				commands.push(ApplicationCommand::new(command, CommandType::ChatInput).unwrap());
			}
			if command.is_message {
				commands.push(ApplicationCommand::new(command, CommandType::Message).unwrap());
			}
			if command.is_user {
				commands.push(ApplicationCommand::new(command, CommandType::User).unwrap());
			}
		}

		DISCORD_CLIENT.request::<()>(
			twilight_http::request::Request::builder(&twilight_http::routing::Route::SetGlobalCommands {
				application_id: DISCORD_APP_ID.get()
			})
				.json(&commands)
				.build()
				.unwrap()
		).await.unwrap();

		info!("successfully updated global commands");
	} else {
		Lazy::force(&CACHE);
		Lazy::force(&COMMANDS);
		Lazy::force(&DISCORD_INTERACTION_CLIENT); // also evaluates DISCORD_CLIENT & DISCORD_APP_ID
		Pin::static_ref(&discord::DISCORD_APP_COMMANDS).await;

		let message_sender = discord::gateway::initialise();
		ribbon_frontend::setup_frontend()
			.await
			.unwrap();

		info!("shutdown signal received, saving stuff...");
		
		message_sender.close(CloseFrame::NORMAL).unwrap();

		info!("now shutting down...");
	}
}