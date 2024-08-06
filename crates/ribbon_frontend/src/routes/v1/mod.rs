use actix_web::web;

pub mod roblox_callback;

pub fn config(config: &mut web::ServiceConfig) {
	config.service(
		web::scope("v1")
			.wrap(crate::util::default_cors())
			.configure(roblox_callback::config)
	);
}