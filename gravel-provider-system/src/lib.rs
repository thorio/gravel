//! System provider.
//! Provides system commands such as shutdown, log out or exiting gravel.

use anyhow::Result;
use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use serde::Deserialize;
use std::sync::Arc;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("system").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(config: &PluginConfigAdapter) -> Box<dyn Provider> {
	let plugin_config = config.get::<Config>(DEFAULT_CONFIG);

	let provider = WebsearchProvider::new(plugin_config);

	Box::new(provider)
}

pub struct WebsearchProvider {
	hits: Box<[Arc<dyn Hit>]>,
}

impl WebsearchProvider {
	fn new(config: Config) -> Self {
		let hits = vec![
			get_exit(config.exit),
			get_hit(config.lock, implementation::lock),
			get_hit(config.logout, implementation::logout),
			get_hit(config.restart, implementation::restart),
			get_hit(config.shutdown, implementation::shutdown),
			get_hit(config.sleep, implementation::sleep),
		];

		Self { hits: hits.into() }
	}
}

impl Provider for WebsearchProvider {
	fn query(&self, _query: &str) -> ProviderResult {
		ProviderResult::new(self.hits.to_vec())
	}
}

fn get_exit(config: ExitConfig) -> Arc<dyn Hit> {
	let hit = SimpleHit::new(config.title, config.subtitle, |_hit, sender| {
		sender.send(FrontendMessage::Exit).ok();
	});

	Arc::new(hit)
}

fn get_hit(config: SubcommandConfig, action: impl Fn(&str) -> Result<()> + Send + Sync + 'static) -> Arc<SimpleHit> {
	let hit = SimpleHit::new(config.title, config.subtitle, move |_, sender| {
		if let Err(err) = action(&config.command_linux) {
			log::error!("error during system operation: {err}");
		}

		sender.send(FrontendMessage::Hide).ok();
	});

	Arc::new(hit)
}

#[derive(Clone, Deserialize, Debug)]
struct Config {
	pub exit: ExitConfig,
	pub lock: SubcommandConfig,
	pub logout: SubcommandConfig,
	pub restart: SubcommandConfig,
	pub shutdown: SubcommandConfig,
	pub sleep: SubcommandConfig,
}

#[derive(Clone, Deserialize, Debug)]
struct ExitConfig {
	pub title: String,
	pub subtitle: String,
}

#[derive(Clone, Deserialize, Debug)]
struct SubcommandConfig {
	pub title: String,
	pub subtitle: String,
	#[allow(unused)]
	pub command_linux: String,
}
