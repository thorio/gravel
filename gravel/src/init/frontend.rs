use crate::config::*;
use gravel_core::{plugin::PluginRegistry, *};

/// Initializes the configured [`Frontend`].
pub fn frontend(registry: &PluginRegistry, engine: QueryEngine, config: &RootConfig) -> Box<dyn Frontend> {
	let frontend = try_get_frontend(registry, engine, config);

	if frontend.is_none() {
		println!("frontend \"{}\" not found, exiting", config.frontend.name);
		std::process::exit(1);
	}

	frontend.unwrap()
}

fn try_get_frontend(registry: &PluginRegistry, engine: QueryEngine, config: &RootConfig) -> Option<Box<dyn Frontend>> {
	registry.get_frontend(&config.frontend.name)?.get_frontend(engine)
}
