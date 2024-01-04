//! Web search provider.
//!
//! Always returns a hit with the minimum score that, when selected,
//! opens the user's default browser and searches for the query.

use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MIN_SCORE, *};
use log::*;
use serde::Deserialize;
use std::sync::{mpsc::Sender, Arc};

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("websearch").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(config_adapter: &PluginConfigAdapter) -> Box<dyn Provider> {
	let config = config_adapter.get::<Config>(DEFAULT_CONFIG);

	// this avoids a clone on every keystroke
	let url_pattern = Box::leak(Box::new(config.url_pattern.clone()));

	let provider = WebsearchProvider { config, url_pattern };

	Box::new(provider)
}

pub struct WebsearchProvider {
	config: Config,
	url_pattern: &'static str,
}

impl Provider for WebsearchProvider {
	fn query(&self, query: &str) -> ProviderResult {
		let hit = SimpleHit::new(query, &*self.config.subtitle, |h, s| do_search(self.url_pattern, h, s))
			.with_score(MIN_SCORE);

		ProviderResult::single(Arc::new(hit))
	}
}

fn do_search(url_pattern: &str, hit: &SimpleHit, sender: &Sender<FrontendMessage>) {
	let encoded = urlencoding::encode(hit.get_title());
	let url = url_pattern.replace("{}", &encoded);

	if let Err(err) = open::that(url) {
		error!("unable to open URL: {err}")
	}

	sender.send(FrontendMessage::Hide).ok();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub url_pattern: String,
	pub subtitle: String,
}
