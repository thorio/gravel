use external::register_externals;
use gravel_core::{config::ExternalPluginPolicy, plugin::PluginRegistry};

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins(config: &ExternalPluginPolicy) -> PluginRegistry {
	let mut registry = PluginRegistry::default();
	register_builtins(&mut registry);
	register_externals(&mut registry, config);

	registry
}

/// Registers plugins compiled directly into the binary.
#[allow(unused_variables)]
fn register_builtins(registry: &mut PluginRegistry) {
	log::trace!("registering builtin plugins");

	#[cfg(feature = "fltk")]
	registry.register(gravel_frontend_fltk::__gravel_plugin_inner());

	#[cfg(feature = "calculator")]
	registry.register(gravel_provider_calculator::__gravel_plugin_inner());
	#[cfg(feature = "custom")]
	registry.register(gravel_provider_custom::__gravel_plugin_inner());
	#[cfg(feature = "exec")]
	registry.register(gravel_provider_exec::__gravel_plugin_inner());
	#[cfg(feature = "kill")]
	registry.register(gravel_provider_kill::__gravel_plugin_inner());
	#[cfg(feature = "program")]
	registry.register(gravel_provider_program::__gravel_plugin_inner());
	#[cfg(feature = "system")]
	registry.register(gravel_provider_system::__gravel_plugin_inner());
	#[cfg(feature = "websearch")]
	registry.register(gravel_provider_websearch::__gravel_plugin_inner());
}

mod external {
	use glob::{Paths, glob};
	use gravel_core::plugin::{PluginRegistry, load_library_from_path};
	use gravel_core::{config::ExternalPluginPolicy, paths};
	use gravel_ffi::{PluginLibRef, logging::StaticLogTarget};
	use itertools::Itertools;
	use std::path::PathBuf;

	pub fn register_externals(registry: &mut PluginRegistry, config: &ExternalPluginPolicy) {
		const EMPTY: &[String] = &[];

		let filter = match config {
			ExternalPluginPolicy::Disabled => return,
			ExternalPluginPolicy::All => EMPTY,
			ExternalPluginPolicy::Whitelist(names) => names,
		};

		log::trace!("looking for external plugin libraries");

		let definitions = paths::plugin_globs()
			.filter_map(expand_glob)
			.flatten()
			.filter_map(Result::ok)
			.unique_by(|p| p.file_name().map(ToOwned::to_owned))
			.filter(filter_libs(filter))
			.filter_map(load_lib)
			.flat_map(|l| l.plugin()(StaticLogTarget::get()));

		for definition in definitions {
			registry.register(definition);
		}
	}

	fn filter_libs(names: &[String]) -> impl Fn(&PathBuf) -> bool + '_ {
		|p| {
			if names.is_empty() {
				return true;
			}

			let stem = p
				.file_stem()
				.expect("was matched by glob, so must have a stem")
				.to_string_lossy();

			names.iter().any(|n| n == &stem)
		}
	}

	fn expand_glob(pattern: PathBuf) -> Option<Paths> {
		glob(&pattern.to_string_lossy())
			.inspect_err(|e| log::error!("unable to expand glob {pattern:?}: {e}"))
			.ok()
	}

	#[expect(clippy::print_stderr)]
	fn load_lib(path: PathBuf) -> Option<PluginLibRef> {
		log::trace!("attempting to load plugin library from {path:?}");

		load_library_from_path(&path)
			.inspect_err(|e| {
				// these errors tend to be huge, multiline monsters, so putting them in the logs would flood them
				log::error!("unable to load plugin at {path:?}, writing diagnostics to stderr");
				eprintln!("{e}");
			})
			.ok()
	}
}
