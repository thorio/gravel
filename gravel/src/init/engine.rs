use gravel_core::{config::*, plugin::*, *};
use std::sync::mpsc::Sender;

/// Initializes the configured [`Provider`]s and the [`QueryEngine`].
///
/// If a configured provider cannot be found, an error is logged
/// and the provider is skipped.
pub fn engine(sender: Sender<FrontendMessage>, registry: &PluginRegistry, config: &ConfigManager) -> QueryEngine {
	log::trace!("initializing query engine");

	let mut engine = QueryEngine::new(sender);

	for (index, provider_config) in config.root.providers.iter().enumerate() {
		let plugin_name = &provider_config.plugin;

		log::debug!("initializing provider '{plugin_name}' with index '{index}'");

		let adapter = config.get_provider_adapter(index);
		let factory = get_provider_factory(registry, plugin_name);

		let Some(factory) = factory else {
			log::warn!("provider '{}' not found, skipping", plugin_name);
			continue;
		};

		let provider = factory(&adapter);
		engine.register(provider, provider_config.keyword.clone());
	}

	engine
}

fn get_provider_factory<'a>(registry: &'a PluginRegistry, name: &str) -> Option<&'a ProviderFactory> {
	match &registry.get_plugin(name)?.factory {
		PluginFactory::Provider(factory) => Some(factory),
		_ => None,
	}
}
