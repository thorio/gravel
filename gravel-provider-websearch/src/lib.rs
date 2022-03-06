//! Web search provider.
//!
//! Always returns a hit with the minimum score that, when selected,
//! opens the user's default browser and searches for the query.

use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MIN_SCORE, *};
use serde::Deserialize;
use std::sync::mpsc::Sender;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("websearch").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(config: &PluginConfigAdapter) -> Box<dyn Provider> {
	let plugin_config = config.get::<Config>(DEFAULT_CONFIG);

	let provider = WebsearchProvider { config: plugin_config };

	Box::new(provider)
}

pub struct WebsearchProvider {
	config: Config,
}

impl Provider for WebsearchProvider {
	fn query(&self, query: &str) -> QueryResult {
		let data = HitData::new(query, &self.config.subtitle).with_score(MIN_SCORE);
		let extra = ExtraData {
			url_pattern: self.config.url_pattern.to_owned(),
		};

		let hit = Box::new(SimpleHit::new_extra(data, extra, do_search));
		QueryResult::single(hit)
	}
}

struct ExtraData {
	pub url_pattern: String,
}

fn do_search(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	let encoded = urlencoding::encode(&hit.get_data().title);
	let url = hit.get_extra_data().url_pattern.replace("{}", encoded.as_ref());

	implementation::open_url(&url).expect("failed to open url");

	sender.send(FrontendMessage::Hide).unwrap();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub url_pattern: String,
	pub subtitle: String,
}
