use gravel_core::{config::*, plugin::*, *};
use std::sync::mpsc::Sender;

/// Initializes the configured [`Provider`]s and the [`QueryEngine`].
///
/// If a configured provider cannot be found, an error is logged
/// and the provider is skipped.
pub fn engine(sender: Sender<FrontendMessage>, registry: &PluginRegistry, config: &ConfigManager) -> QueryEngine {
	log::trace!("initializing query engine");

	let mut engine = QueryEngine::new(sender);

	for provider_config in &config.root.providers {
		// fall back to the plugin name if no alias is configured
		let plugin_name = &provider_config.plugin;
		let provider_name = provider_config.alias.as_ref().unwrap_or(plugin_name);

		log::debug!("initializing provider '{plugin_name}' with alias '{provider_name}'");

		let adapter = config.get_plugin_adapter(provider_name);
		let provider = try_get_provider(registry, plugin_name, &adapter);

		let Some(provider) = provider else {
			log::warn!("provider \"{}\" not found, skipping", plugin_name);
			continue;
		};

		engine.register(provider, provider_config.keyword.clone());
	}

	engine
}

fn try_get_provider(registry: &PluginRegistry, name: &str, config: &PluginConfigAdapter) -> Option<Box<dyn Provider>> {
	registry.get_provider(name)?.get_provider(config)
}
