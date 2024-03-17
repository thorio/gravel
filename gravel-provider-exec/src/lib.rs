//! gravel's command runner
//!
//! Always returns a hit with the minimum score that, when selected,
//! runs the command with the system shell.

use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MIN_SCORE, *};
use serde::Deserialize;
use std::sync::{mpsc::Sender, Arc};

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = plugin("exec").with_provider(Box::new(get_provider));

	registry.register(definition);
}

fn get_provider(config_adapter: &PluginConfigAdapter) -> Box<dyn Provider> {
	let config = config_adapter.get::<Config>(DEFAULT_CONFIG);

	let provider = ExecProvider { config };

	Box::new(provider)
}

pub struct ExecProvider {
	config: Config,
}

impl Provider for ExecProvider {
	fn query(&self, query: &str) -> ProviderResult {
		let hit = SimpleHit::new(query, &*self.config.subtitle, run_command).with_score(MIN_SCORE);

		ProviderResult::single(Arc::new(hit))
	}
}

fn run_command(hit: &SimpleHit, sender: &Sender<FrontendMessage>) {
	if let Err(err) = implementation::run_command(hit.get_title()) {
		log::error!("{err}");
	}

	sender.send(FrontendMessage::Hide).ok();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub subtitle: String,
}
