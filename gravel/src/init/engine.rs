use crate::config::*;
use gravel_core::{plugin::*, *};
use std::sync::mpsc::Sender;

/// Initializes the configured [`Provider`]s and the [`QueryEngine`].
///
/// If a configured provider cannot be found, an error is logged
/// and the provider is skipped.
pub fn engine(sender: Sender<FrontendMessage>, registry: &PluginRegistry, config: &RootConfig) -> QueryEngine {
	let mut engine = QueryEngine::new(sender);

	for provider_config in config.providers.iter() {
		let provider = try_get_provider(registry, &provider_config.name);

		if provider.is_none() {
			println!("frontend \"{}\" not found, exiting", config.frontend.name);
			continue;
		}

		engine.register(provider.unwrap(), &provider_config.keyword);
	}

	engine
}

fn try_get_provider(registry: &PluginRegistry, name: &str) -> Option<Box<dyn Provider>> {
	registry.get_provider(name)?.get_provider()
}