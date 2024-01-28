use crate::config::*;
use gravel_core::plugin::{FrontendFactory, PluginFactory, PluginRegistry};
use gravel_core::*;

/// Initializes the configured [`Frontend`].
pub fn frontend(registry: &PluginRegistry, engine: QueryEngine, config: &ConfigManager) -> Box<dyn Frontend> {
	// fall back to the plugin name if no alias is configured
	let plugin_name = &config.root.frontend.plugin;
	let frontend_name = config.root.frontend.alias.as_ref().unwrap_or(plugin_name);
	log::debug!("initializing frontend '{plugin_name}' with alias '{frontend_name}'");

	let adapter = config.get_plugin_adapter(frontend_name);

	let factory = get_frontend_factory(registry, plugin_name);

	let Some(factory) = factory else {
		log::error!("frontend '{plugin_name}' not found, exiting");
		std::process::exit(1);
	};

	factory(engine, &adapter)
}

fn get_frontend_factory<'a>(registry: &'a PluginRegistry, name: &str) -> Option<&'a FrontendFactory> {
	match &registry.get_plugin(name)?.factory {
		PluginFactory::Frontend(factory) => Some(factory),
		_ => None,
	}
}
