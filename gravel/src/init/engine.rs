use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_core::config::ConfigManager;
use gravel_core::{engine::QueryEngineImpl, plugin::PluginRegistry};
use gravel_ffi::FrontendMessage;

/// Initializes the configured [`Provider`]s and the [`QueryEngine`].
///
/// If a configured provider cannot be found, an error is logged
/// and the provider is skipped.
pub fn engine(sender: RSender<FrontendMessage>, registry: &PluginRegistry, config: &ConfigManager) -> QueryEngineImpl {
	log::trace!("initializing query engine");

	let mut engine = QueryEngineImpl::new(sender);

	for (index, provider_config) in config.root().providers.iter().enumerate() {
		let plugin_name = &provider_config.plugin;

		log::debug!("initializing provider '{plugin_name}' with index '{index}'");

		let factory = registry.get(plugin_name).and_then(|p| p.factory.provider());

		let Some(factory) = factory else {
			log::warn!("provider '{}' not found, skipping", plugin_name);
			continue;
		};

		let provider = factory(&config.adapt_provider(index));
		engine.register(provider, provider_config.keyword.clone());
	}

	engine
}
