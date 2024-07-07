//! Web search provider.
//!
//! Always returns a hit with the minimum score that, when selected,
//! opens the user's default browser and searches for the query.

use gravel_ffi::prelude::*;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub struct WebsearchProvider {
	config: Config,
	url_pattern: &'static str,
}

#[gravel_provider("websearch")]
impl Provider for WebsearchProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		let config = config.get::<Config>(DEFAULT_CONFIG);

		// this avoids a clone on every keystroke
		let url_pattern = Box::leak(Box::new(config.url_pattern.clone()));

		Self { config, url_pattern }
	}

	fn query(&self, query: &str) -> ProviderResult {
		let hit = SimpleHit::new(query, &*self.config.subtitle, |hit, ctx| {
			do_search(self.url_pattern, hit.title().as_str(), ctx);
		})
		.with_score(MIN_SCORE);

		ProviderResult::single(hit)
	}
}

fn do_search(url_pattern: &str, query: &str, context: RefDynHitActionContext<'_>) {
	let encoded = urlencoding::encode(query);
	let url = url_pattern.replace("{}", &encoded);

	log::debug!("opening URL '{url}'");
	open::that(url)
		.inspect_err(|e| log::error!("unable to open URL: {e}"))
		.ok();

	context.hide_frontend();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub url_pattern: String,
	pub subtitle: String,
}
