use gravel_core::{frontend::ControlMessage, provider::*, scoring::*};
use std::sync::mpsc::Sender;

pub struct WebsearchProvider {}

impl WebsearchProvider {
	pub fn new() -> Self {
		WebsearchProvider {}
	}
}

impl Provider for WebsearchProvider {
	fn query(&self, query: &str) -> QueryResult {
		let data = HitData::new(query, "Web Search").with_score(MIN_SCORE);
		let hit = Box::new(SimpleHit::new(data, do_search));

		QueryResult::single(hit)
	}
}

fn do_search(hit: &SimpleHit<()>, sender: &Sender<ControlMessage>) {
	let encoded = urlencoding::encode(&hit.get_data().title);
	let url = format!("https://www.google.com/search?q={}", encoded);

	gravel_util::process::open_url(&url).expect("failed to open url");

	sender.send(ControlMessage::Hide).unwrap();
}
