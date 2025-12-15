use ribbon_worker::RibbonWorker;
use starlight::App; // silly library i am making using twilight

mod commands;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
	dotenvy::dotenv().unwrap();
	
	rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
	
	let token = std::env::var("DISCORD_BOT_TOKEN")
		.unwrap();
	
	let app: App<RibbonWorker> = App::new(token)
		.start()
		.await;
	println!("app started");
	
	#[cfg(unix)] {
		use tokio::signal::unix::{ SignalKind, signal };
		
		let mut sigterm = signal(SignalKind::terminate()).unwrap();
		tokio::select! {
			_ = tokio::signal::ctrl_c() => {}
			_ = sigterm.recv() => {}
		}
	}
	#[cfg(not(unix))] {
		tokio::signal::ctrl_c()
			.await
			.unwrap();
	}
	println!("shutdown signal received");
	
	app
		.close()
		.await;
}