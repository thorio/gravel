use gravel_core::plugin::PluginRegistry;

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins() -> PluginRegistry {
	log::trace!("loading plugins");

	let mut registry = PluginRegistry::default();
	register_builtins(&mut registry);
	register_externals(&mut registry);

	registry
}

fn register_externals(_registry: &mut PluginRegistry) {
	// TODO: load external plugins
}

/// Registers plugins compiled directly into the binary.
#[allow(unused_variables)]
fn register_builtins(registry: &mut PluginRegistry) {
	#[cfg(feature = "fltk")]
	registry.register(gravel_frontend_fltk::get_plugin());

	#[cfg(feature = "calculator")]
	registry.register(gravel_provider_calculator::get_plugin());
	#[cfg(feature = "exec")]
	registry.register(gravel_provider_exec::get_plugin());
	#[cfg(feature = "kill")]
	registry.register(gravel_provider_kill::get_plugin());
	#[cfg(feature = "program")]
	registry.register(gravel_provider_program::get_plugin());
	#[cfg(feature = "system")]
	registry.register(gravel_provider_system::get_plugin());
	#[cfg(feature = "websearch")]
	registry.register(gravel_provider_websearch::get_plugin());
}
