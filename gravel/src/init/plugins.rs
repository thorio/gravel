use external::register_externals;
use gravel_core::{config::ExternalPlugins, plugin::PluginRegistry};

/// Initializes the [`PluginRegistry`] and registers built-in plugins.
pub fn plugins(config: &ExternalPlugins) -> PluginRegistry {
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
	use gravel_core::{config::ExternalPlugins, paths, plugin::PluginRegistry};
	use gravel_ffi::{logging::StaticLogTarget, PluginLibRef};
	use itertools::Itertools;
	use std::path::PathBuf;

	pub fn register_externals(registry: &mut PluginRegistry, config: &ExternalPlugins) {
		const EMPTY: &[String] = &[];

		let filter = match config {
			ExternalPlugins::Disabled => return,
			ExternalPlugins::All => EMPTY,
			ExternalPlugins::Whitelist(names) => names,
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
