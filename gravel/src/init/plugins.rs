use gravel_core::plugin::*;

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins() -> PluginRegistry {
	log::trace!("loading plugins");

	let mut registry = PluginRegistry::default();
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
	gravel_frontend_fltk::register_plugins(registry);

	gravel_provider_calculator::register_plugins(registry);
	gravel_provider_kill::register_plugins(registry);
	gravel_provider_exec::register_plugins(registry);
	gravel_provider_program::register_plugins(registry);
	gravel_provider_system::register_plugins(registry);
	gravel_provider_websearch::register_plugins(registry);
}
