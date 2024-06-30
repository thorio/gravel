use gravel_core::config::{RootConfig, FRONTEND};
use gravel_core::{engine::QueryEngineImpl, plugin::PluginRegistry};
use gravel_ffi::prelude::*;

/// Initializes the configured [`Frontend`].
pub fn frontend(registry: &PluginRegistry, engine: QueryEngineImpl, config: &ConfigManager) -> BoxDynFrontend {
	let root_config = config.root::<RootConfig>();

	// fall back to the plugin name if no alias is configured
	let plugin_name = &root_config.frontend.plugin;
	let frontend_name = root_config.frontend.alias.as_ref().unwrap_or(plugin_name);
	log::debug!("initializing frontend '{plugin_name}' with alias '{frontend_name}'");

	let factory = registry.get(plugin_name).and_then(|p| p.factory.frontend());
	let Some(factory) = factory else {
		log::error!("frontend '{plugin_name}' not found, exiting");
		std::process::exit(1);
	};

	factory(engine.into(), &config.adapt(FRONTEND))
}
