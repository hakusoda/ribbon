use log::info;
use twilight_gateway::Event;

use crate::Result;

pub mod guild;
pub mod interaction;

pub fn handle_event(event: Event) {
	let event_kind = event.kind();
	info!("handle_event {event_kind:?}");

	if let Err(error) = match event {
		Event::GuildCreate(x) => guild::guild_create(*x),
		Event::GuildUpdate(x) => guild::guild_update(*x),
		Event::GuildDelete(x) => guild::guild_delete(x),
		Event::InteractionCreate(x) => spawn(interaction::interaction_create(*x)),
		_ => Ok(())
	} {
		println!("error occurred in event handler! {error}");
	}
}

fn spawn<F: Future<Output = Result<()>> + Send + 'static>(future: F) -> Result<()> {
	tokio::spawn(async move {
		if let Err(error) = future.await {
			println!("error occurred in async event handler! {error}");
		}
	});

	Ok(())
}