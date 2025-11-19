use ribbon_worker::RibbonWorker;
use starlight::App; // silly library i am making using twilight

mod commands;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
	dotenvy::dotenv().unwrap();
	
	let token = std::env::var("DISCORD_BOT_TOKEN")
		.unwrap();
	
	let app: App<RibbonWorker> = App::new(token)
		.start()
		.await;
	println!("app started");
	
	tokio::signal::ctrl_c()
		.await
		.unwrap();
	println!("shutdown signal received");
	
	app
		.close()
		.await;
}