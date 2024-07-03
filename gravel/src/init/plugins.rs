use external::register_externals;
use gravel_core::plugin::PluginRegistry;

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins() -> PluginRegistry {
	let mut registry = PluginRegistry::default();
	register_builtins(&mut registry);
	register_externals(&mut registry);

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
	use abi_stable::library::{lib_header_from_path, LibHeader, LibraryError, RootModule};
	use abi_stable::sabi_types::VersionNumber;
	use glob::{glob, Paths};
	use gravel_core::{paths, plugin::PluginRegistry};
	use gravel_ffi::{logging::StaticLogTarget, PluginLibRef};
	use itertools::Itertools;
	use std::path::PathBuf;

	pub fn register_externals(registry: &mut PluginRegistry) {
		log::trace!("looking for external plugin libraries");

		let definitions = paths::plugin_globs()
			.filter_map(expand_glob)
			.flatten()
			.filter_map(Result::ok)
			.unique_by(|p| p.file_name().map(ToOwned::to_owned))
			.filter_map(load_lib)
			.flat_map(|l| l.plugin()(StaticLogTarget::get()));

		for definition in definitions {
			registry.register(definition);
		}
	}

	fn expand_glob(pattern: PathBuf) -> Option<Paths> {
		glob(&pattern.to_string_lossy())
			.inspect_err(|e| log::error!("unable to expand glob {pattern:?}: {e}"))
			.ok()
	}

	fn check_version<M: RootModule>(header: &'static LibHeader) -> Result<&'static LibHeader, LibraryError> {
		let expected_version = VersionNumber::new(M::VERSION_STRINGS)?;
		let actual_version = VersionNumber::new(header.version_strings())?;

		if expected_version.major != actual_version.major
			|| expected_version.minor < actual_version.minor
			|| (expected_version.major == 0) && expected_version.minor > actual_version.minor
		{
			return Err(LibraryError::IncompatibleVersionNumber {
				library_name: M::NAME,
				expected_version,
				actual_version,
			});
		}

		Ok(header)
	}

	#[allow(clippy::print_stderr)]
	fn load_lib(path: PathBuf) -> Option<PluginLibRef> {
		log::trace!("attempting to load plugin library from {path:?}");

		lib_header_from_path(&path)
			.and_then(check_version::<PluginLibRef>)
			.and_then(LibHeader::check_layout)
			.inspect_err(|e| {
				// these errors tend to be huge, multiline monsters, so putting them in the logs would flood them
				log::error!("unable to load plugin at {path:?}, writing diagnostics to stderr");
				eprintln!("{e}");
			})
			.ok()
	}
}
