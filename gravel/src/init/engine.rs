use gravel_core::{config::*, plugin::*, *};
use log::*;
use std::sync::mpsc::Sender;

/// Initializes the configured [`Provider`]s and the [`QueryEngine`].
///
/// If a configured provider cannot be found, an error is logged
/// and the provider is skipped.
pub fn engine(sender: Sender<FrontendMessage>, registry: &PluginRegistry, config: &ConfigManager) -> QueryEngine {
	let mut engine = QueryEngine::new(sender);

	for provider_config in &config.root.providers {
		// fall back to the plugin name if no explicit name is configured
		let provider_name = provider_config.alias.as_ref().unwrap_or(&provider_config.plugin);

		let adapter = config.get_plugin_adapter(provider_name);
		let provider = try_get_provider(registry, &provider_config.plugin, &adapter);

		let Some(provider) = provider else {
			warn!("provider \"{}\" not found, skipping", provider_config.plugin);
			continue;
		};

		engine.register(provider, provider_config.keyword.clone());
	}

	engine
}

fn try_get_provider(registry: &PluginRegistry, name: &str, config: &PluginConfigAdapter) -> Option<Box<dyn Provider>> {
	registry.get_provider(name)?.get_provider(config)
}
