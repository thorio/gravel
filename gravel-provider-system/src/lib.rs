//! System provider.
//! Provides system commands such as shutdown, log out or exiting gravel.

use abi_stable::{sabi_extern_fn, std_types::RStr};
use anyhow::Result;
use gravel_ffi::prelude::*;
use serde::Deserialize;
use std::env;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn get_plugin() -> PluginDefinition {
	PluginMetadata::new("system").with_provider(get_provider)
}

#[sabi_extern_fn]
fn get_provider(config_adapter: &PluginConfigAdapter<'_>) -> BoxDynProvider {
	let plugin_config = config_adapter.get::<Config>(DEFAULT_CONFIG);

	WebsearchProvider::new(plugin_config).into_dyn()
}

pub struct WebsearchProvider {
	hits: Box<[ArcDynHit]>,
}

impl WebsearchProvider {
	fn new(config: Config) -> Self {
		let hits = vec![
			message_hit(config.exit, FrontendMessage::Exit),
			message_hit(config.reload, FrontendMessage::Restart),
			shell_hit(config.lock, implementation::lock),
			shell_hit(config.logout, implementation::logout),
			shell_hit(config.restart, implementation::restart),
			shell_hit(config.shutdown, implementation::shutdown),
			shell_hit(config.sleep, implementation::sleep),
		];

		Self { hits: hits.into() }
	}
}

impl Provider for WebsearchProvider {
	fn query(&self, _query: RStr<'_>) -> ProviderResult {
		ProviderResult::new(self.hits.to_vec())
	}
}

fn message_hit(config: CommandConfig, message: FrontendMessage) -> ArcDynHit {
	let hit = SimpleHit::new(config.title, config.subtitle, move |_hit, sender| {
		sender.send(message.clone()).ok();
	});

	hit.into_dyn()
}

fn shell_hit(config: ShellCommandConfig, action: impl Fn(&str) -> Result<()> + Send + Sync + 'static) -> ArcDynHit {
	let hit = SimpleHit::new(config.title, config.subtitle, move |hit, sender| {
		if let Err(err) = action(&config.command_linux) {
			log::error!("couldn't perform system operation {}: {err}", hit.title());
		}

		sender.send(FrontendMessage::Hide).ok();
	});

	hit.into_dyn()
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
