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
	fn query(&self, _query: &str) -> QueryResult {
		let hits = vec![
			get_exit(&self.config),
			get_lock(&self.config),
			get_logout(&self.config),
			get_restart(&self.config),
			get_shutdown(&self.config),
			get_sleep(&self.config),
		];

		QueryResult::new(hits)
	}
}

fn get_exit(config: &Config) -> Box<dyn Hit> {
	let data = HitData::new(&config.exit.title, &config.exit.subtitle);
	Box::new(SimpleHit::new(data, |_hit, sender| {
		sender
			.send(FrontendMessage::Exit)
			.expect("failed to send frontend message");
	}))
}

fn get_lock(config: &Config) -> Box<dyn Hit> {
	let data = HitData::new(&config.lock.title, &config.lock.subtitle);
	Box::new(SimpleHit::new_extra(data, config.lock.clone(), |hit, sender| {
		implementation::lock(hit.get_extra_data());
		sender
			.send(FrontendMessage::Hide)
			.expect("failed to send frontend message");
	}))
}

fn get_logout(config: &Config) -> Box<dyn Hit> {
	let data = HitData::new(&config.logout.title, &config.logout.subtitle);
	Box::new(SimpleHit::new_extra(data, config.logout.clone(), |hit, sender| {
		implementation::logout(hit.get_extra_data());
		sender
			.send(FrontendMessage::Hide)
			.expect("failed to send frontend message");
	}))
}

fn get_restart(config: &Config) -> Box<dyn Hit> {
	let data = HitData::new(&config.restart.title, &config.restart.subtitle);
	Box::new(SimpleHit::new_extra(data, config.restart.clone(), |hit, sender| {
		implementation::restart(hit.get_extra_data());
		sender
			.send(FrontendMessage::Hide)
			.expect("failed to send frontend message");
	}))
}

fn get_shutdown(config: &Config) -> Box<dyn Hit> {
	let data = HitData::new(&config.shutdown.title, &config.shutdown.subtitle);
	Box::new(SimpleHit::new_extra(data, config.shutdown.clone(), |hit, sender| {
		implementation::shutdown(hit.get_extra_data());
		sender
			.send(FrontendMessage::Hide)
			.expect("failed to send frontend message");
	}))
}

fn get_sleep(config: &Config) -> Box<dyn Hit> {
	let data = HitData::new(&config.sleep.title, &config.sleep.subtitle);
	Box::new(SimpleHit::new_extra(data, config.sleep.clone(), |hit, sender| {
		implementation::sleep(hit.get_extra_data());
		sender
			.send(FrontendMessage::Hide)
			.expect("failed to send frontend message");
	}))
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
