use gravel_core::config::ConfigManager;
use gravel_core::{engine::QueryEngine, plugin::PluginRegistry};
use gravel_ffi::BoxDynFrontend;

/// Initializes the configured [`gravel_ffi::Frontend`].
pub fn frontend(registry: &PluginRegistry, engine: QueryEngine, config: &ConfigManager) -> BoxDynFrontend {
	let root_config = config.root();

	// fall back to the plugin name if no alias is configured
	let plugin_name = &root_config.frontend.plugin;
	log::debug!("initializing frontend '{plugin_name}'");

	let factory = registry.get(plugin_name).and_then(|p| p.factory.frontend());
	let Some(factory) = factory else {
		log::error!("frontend '{plugin_name}' not found, exiting");
		std::process::exit(1);
	};

	factory(engine.into(), &config.adapt_frontend())
}
