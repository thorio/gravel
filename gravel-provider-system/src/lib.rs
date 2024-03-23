//! System provider.
//! Provides system commands such as shutdown, log out or exiting gravel.

use anyhow::Result;
use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use serde::Deserialize;
use std::{env, sync::Arc};

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = plugin("system").with_provider(Box::new(get_provider));

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
			get_message_hit(config.exit, FrontendMessage::Exit),
			get_message_hit(config.reload, FrontendMessage::Restart),
			get_shell_hit(config.lock, implementation::lock),
			get_shell_hit(config.logout, implementation::logout),
			get_shell_hit(config.restart, implementation::restart),
			get_shell_hit(config.shutdown, implementation::shutdown),
			get_shell_hit(config.sleep, implementation::sleep),
		];

		Self { hits: hits.into() }
	}
}

impl Provider for WebsearchProvider {
	fn query(&self, _query: &str) -> ProviderResult {
		ProviderResult::new(self.hits.to_vec())
	}
}

fn get_message_hit(config: CommandConfig, message: FrontendMessage) -> Arc<dyn Hit> {
	let hit = SimpleHit::new(config.title, config.subtitle, move |_hit, sender| {
		sender.send(message.clone()).ok();
	});

	Arc::new(hit)
}

fn get_shell_hit(
	config: ShellCommandConfig,
	action: impl Fn(&str) -> Result<()> + Send + Sync + 'static,
) -> Arc<SimpleHit> {
	let hit = SimpleHit::new(config.title, config.subtitle, move |hit, sender| {
		if let Err(err) = action(&config.command_linux) {
			log::error!("error during system operation {}: {err}", hit.get_title());
		}

		sender.send(FrontendMessage::Hide).ok();
	});

	Arc::new(hit)
}

#[derive(Clone, Deserialize, Debug)]
struct Config {
	pub exit: CommandConfig,
	pub reload: CommandConfig,
	pub lock: ShellCommandConfig,
	pub logout: ShellCommandConfig,
	pub restart: ShellCommandConfig,
	pub shutdown: ShellCommandConfig,
	pub sleep: ShellCommandConfig,
}

#[derive(Clone, Deserialize, Debug)]
struct CommandConfig {
	pub title: String,
	pub subtitle: String,
}

#[derive(Clone, Deserialize, Debug)]
struct ShellCommandConfig {
	pub title: String,
	pub subtitle: String,
	pub command_linux: String,
}
