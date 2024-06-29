use abi_stable::library::{lib_header_from_path, LibHeader};
use glob::{glob, Paths};
use gravel_core::{paths, plugin::PluginRegistry};
use gravel_ffi::prelude::*;
use itertools::Itertools;
use std::path::PathBuf;

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins() -> PluginRegistry {
	log::trace!("loading plugins");

	let mut registry = PluginRegistry::default();
	register_builtins(&mut registry);
	register_externals(&mut registry);

	registry
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

#[allow(unused)]
fn register_externals(registry: &mut PluginRegistry) {
	fn expand_glob(pattern: PathBuf) -> Option<Paths> {
		glob(&pattern.to_string_lossy())
			.inspect_err(|e| log::error!("unable to expand glob {pattern:?}: {e}"))
			.ok()
	}

	#[allow(clippy::print_stderr)]
	fn load_lib(path: PathBuf) -> Option<PluginLibRef> {
		lib_header_from_path(&path)
			.and_then(LibHeader::init_root_module)
			.inspect_err(|e| {
				// these errors tend to be huge, multiline monsters, so putting them in the logs would flood them
				log::error!("unable to load plugin at {path:?}, writing diagnostics to stderr");
				eprintln!("{e}");
			})
			.ok()
	}

	let definitions = paths::plugin_globs()
		.filter_map(expand_glob)
		.flatten()
		.filter_map(Result::ok)
		.unique_by(|p| p.file_name().map(ToOwned::to_owned))
		.filter_map(load_lib)
		.map(|l| l.plugin()());

	for definition in definitions {
		registry.register(definition);
	}
}
