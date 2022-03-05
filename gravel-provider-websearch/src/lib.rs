//! Web search provider.
//!
//! Always returns a hit with the minimum score that, when selected,
//! opens the user's default browser and searches for the query.

use gravel_core::{plugin::*, scoring::MIN_SCORE, *};
use std::sync::mpsc::Sender;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("websearch").with_provider(|| Box::new(WebsearchProvider::new()));

	registry.register(definition);
}

pub struct WebsearchProvider;

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

fn do_search(hit: &SimpleHit<()>, sender: &Sender<FrontendMessage>) {
	let encoded = urlencoding::encode(&hit.get_data().title);
	let url = format!("https://www.google.com/search?q={}", encoded);

	implementation::open_url(&url).expect("failed to open url");

	sender.send(FrontendMessage::Hide).unwrap();
}