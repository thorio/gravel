//! Web search provider.
//!
//! Always returns a hit with the minimum score that, when selected,
//! opens the user's default browser and searches for the query.

use abi_stable::std_types::RStr;
use abi_stable::{external_types::crossbeam_channel::RSender, sabi_extern_fn};
use gravel_ffi::prelude::*;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn get_plugin() -> PluginDefinition {
	PluginMetadata::new("websearch").with_provider(get_provider)
}

#[sabi_extern_fn]
fn get_provider(config_adapter: &PluginConfigAdapter<'_>) -> BoxDynProvider {
	let config = config_adapter.get::<Config>(DEFAULT_CONFIG);

	// this avoids a clone on every keystroke
	let url_pattern = Box::leak(Box::new(config.url_pattern.clone()));

	WebsearchProvider { config, url_pattern }.into_dyn()
}

pub struct WebsearchProvider {
	config: Config,
	url_pattern: &'static str,
}

impl Provider for WebsearchProvider {
	fn query(&self, query: RStr<'_>) -> ProviderResult {
		let hit = SimpleHit::new(query, &*self.config.subtitle, |hit, sender| {
			do_search(self.url_pattern, hit.title().as_str(), sender);
		})
		.with_score(MIN_SCORE);

		ProviderResult::single(hit.into_dyn())
	}
}

fn do_search(url_pattern: &str, query: &str, sender: &RSender<FrontendMessage>) {
	let encoded = urlencoding::encode(query);
	let url = url_pattern.replace("{}", &encoded);

	log::debug!("opening URL '{url}'");
	if let Err(err) = open::that(url) {
		log::error!("unable to open URL: {err}");
	}

	sender.send(FrontendMessage::Hide).ok();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub url_pattern: String,
	pub subtitle: String,
}
