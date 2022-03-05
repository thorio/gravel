use gravel_core::plugin::*;

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins() -> PluginRegistry {
	let mut registry = PluginRegistry::new();
	register_builtins(&mut registry);
	register_externals(&mut registry);

	registry
}

#[allow(unused)]
fn register_externals(registry: &mut PluginRegistry) {
	// TODO: load external plugins
}

/// Registers plugins directly compiled into the binary.
fn register_builtins(registry: &mut PluginRegistry) {
	gravel_provider_calculator::register_plugins(registry);
	gravel_provider_program::register_plugins(registry);
	gravel_provider_websearch::register_plugins(registry);
	gravel_frontend_default::register_plugins(registry);
}
