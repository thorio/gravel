//! System provider.
//! Provides system commands such as shutdown, log out or exiting gravel.

use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use serde::Deserialize;

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

	let provider = WebsearchProvider { config: plugin_config };

	Box::new(provider)
}

pub struct WebsearchProvider {
	config: Config,
}

impl Provider for WebsearchProvider {
	fn query(&self, _query: &str) -> ProviderResult {
		let hits = vec![
			get_exit(&self.config),
			get_hit(&self.config.lock, implementation::lock),
			get_hit(&self.config.logout, implementation::logout),
			get_hit(&self.config.restart, implementation::restart),
			get_hit(&self.config.shutdown, implementation::shutdown),
			get_hit(&self.config.sleep, implementation::sleep),
		];

		ProviderResult::new(hits)
	}
}

fn get_exit(config: &Config) -> Box<dyn Hit> {
	let hit = SimpleHit::new(&*config.exit.title, &*config.exit.subtitle, |_hit, sender| {
		sender
			.send(FrontendMessage::Exit)
			.expect("failed to send frontend message");
	});

	Box::new(hit)
}

fn get_hit(config: &SubcommandConfig, action: impl Fn(&SubcommandConfig) + 'static) -> Box<SimpleHit<()>> {
	let cloned_config = config.to_owned();
	let hit = SimpleHit::new(&*config.title, &*config.subtitle, move |_, sender| {
		action(&cloned_config);

		sender
			.send(FrontendMessage::Hide)
			.expect("failed to send frontend message");
	});

	Box::new(hit)
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
