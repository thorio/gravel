use crate::config::*;
use gravel_core::{plugin::PluginRegistry, *};

/// Initializes the configured [`Frontend`].
pub fn frontend(registry: &PluginRegistry, engine: QueryEngine, config: &ConfigManager) -> Box<dyn Frontend> {
	// fall back to the plugin name if no alias is configured
	let plugin_name = &config.root.frontend.plugin;
	let frontend_name = config.root.frontend.alias.as_ref().unwrap_or(plugin_name);
	log::debug!("initializing frontend '{plugin_name}' with alias '{frontend_name}'");

	let adapter = config.get_plugin_adapter(frontend_name);

	let frontend = try_get_frontend(registry, engine, plugin_name, &adapter);

	let Some(frontend) = frontend else {
		log::error!("frontend \"{plugin_name}\" not found, exiting");
		std::process::exit(1);
	};

	frontend
}

fn try_get_frontend(
	registry: &PluginRegistry,
	engine: QueryEngine,
	name: &str,
	config: &PluginConfigAdapter,
) -> Option<Box<dyn Frontend>> {
	registry.get_frontend(name)?.get_frontend(engine, config)
}
