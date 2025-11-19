use starlight::Worker;
use starlight::{ Event, Shard };

pub struct RibbonWorker {
	
}

impl Worker for RibbonWorker {
	fn new() -> Self {
		Self {}
	}
	
	async fn handle_event(&self, shard: &mut Shard, event: Event) {
		println!("event on {}: {:?}", shard.id(), event.kind());
	}
}