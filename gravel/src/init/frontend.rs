use crate::config::*;
use gravel_core::{plugin::PluginRegistry, *};
use log::*;

/// Initializes the configured [`Frontend`].
pub fn frontend(registry: &PluginRegistry, engine: QueryEngine, config: &ConfigManager) -> Box<dyn Frontend> {
	let frontend_plugin = &config.root.frontend.plugin;

	// fall back to the plugin name if no explicit name is configured
	let frontend_name = config.root.frontend.alias.as_ref().unwrap_or(frontend_plugin);
	let adapter = config.get_plugin_adapter(frontend_name);

	let frontend = try_get_frontend(registry, engine, frontend_plugin, &adapter);

	let Some(frontend) = frontend else {
		warn!("frontend \"{frontend_plugin}\" not found, exiting");
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
