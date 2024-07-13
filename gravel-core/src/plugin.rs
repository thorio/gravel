use abi_stable::library::{IsLayoutChecked, LibHeader, LibraryError, RootModule, RootModuleError};
use abi_stable::{abi_stability::abi_checking::check_layout_compatibility, library::lib_header_from_path};
use abi_stable::{sabi_types::VersionNumber, std_types::RBoxError, type_layout::TypeLayout};
use gravel_ffi::PluginDefinition;
use std::{collections::HashMap, path::Path};

/// Facilitates registering and finding plugins.
#[derive(Default)]
pub struct PluginRegistry {
	plugins: HashMap<String, PluginDefinition>,
}

impl PluginRegistry {
	/// Registers the plugin.
	///
	/// If the plugin is incorrectly defined or another plugin with identical
	/// name and type is already registered, an error is logged and the plugin
	/// is skipped.
	pub fn register(&mut self, plugin: PluginDefinition) -> &mut Self {
		let name = plugin.meta.name.as_str();

		if self.plugins.contains_key(name) {
			log::warn!("attempted to register duplicate plugin '{}', skipping", name);
			return self;
		}

		log::trace!("registered plugin '{name}'");

		self.plugins.insert(name.to_owned(), plugin);
		self
	}

	#[must_use]
	pub fn get(&self, name: &str) -> Option<&PluginDefinition> {
		self.plugins.get(name)
	}
}

pub fn load_library_from_path<M: RootModule>(path: &Path) -> Result<M, LibraryError> {
	let header = lib_header_from_path(path)?;
	check_module_version::<M>(header)?;

	if let IsLayoutChecked::Yes(layout) = header.root_mod_consts().layout() {
		check_layout::<M>(layout)?;
	};

	let lib = unsafe { header.unchecked_layout::<M>() }.map_err(RootModuleError::into_library_error::<M>)?;
	Ok(lib)
}

fn check_layout<M: RootModule>(plugin_layout: &'static TypeLayout) -> Result<(), LibraryError> {
	let application_layout = M::LAYOUT;

	// `abi_stable` normally checks if the library has an equal or newer version, but
	// this doesn't work for a plugin system. We want to inverse behaviour, where the
	// library can be *older* than the caller, missing certain functionality.
	//
	// By passing the loaded lib's layout as the "interface" we achieve this,
	// checking compatibility in reverse.
	check_layout_compatibility(plugin_layout, application_layout).map_err(|e| {
		let formatted = RBoxError::new(e).to_formatted_error();
		LibraryError::AbiInstability(formatted)
	})
}

fn check_module_version<M: RootModule>(header: &'static LibHeader) -> Result<(), LibraryError> {
	let app_version = VersionNumber::new(M::VERSION_STRINGS)?;
	let plugin_version = VersionNumber::new(header.version_strings())?;

	if !are_compatible(&app_version, &plugin_version) {
		return Err(LibraryError::IncompatibleVersionNumber {
			library_name: M::NAME,
			expected_version: app_version,
			actual_version: plugin_version,
		});
	}

	Ok(())
}

/// Checks like normal semver, except patch bumbs in 0.x are acceptable.
fn are_compatible(app: &VersionNumber, plugin: &VersionNumber) -> bool {
	app.major == plugin.major
		&& app.minor >= plugin.minor
		&& (app.major != 0 || app.minor == plugin.minor)
		&& (app.minor != plugin.minor || app.patch >= plugin.patch)
}

#[cfg(test)]
mod test {
	use super::*;
	use abi_stable::sabi_types::VersionStrings;
	use rstest::rstest;

	#[rstest]
	// major
	#[case("1.0.0", "1.0.0", true)]
	#[case("1.0.0", "2.0.0", false)]
	// minor (>=1.0)
	#[case("1.1.0", "1.1.0", true)]
	#[case("1.2.0", "1.1.0", true)]
	#[case("1.0.0", "1.1.0", false)]
	#[case("1.0.7", "1.1.5", false)]
	// patch (>=1.0)
	#[case("1.0.1", "1.0.0", true)]
	#[case("1.1.0", "1.0.4", true)]
	#[case("1.0.2", "1.0.4", false)]
	// minor (<1.0)
	#[case("0.1.0", "0.1.0", true)]
	#[case("0.2.0", "0.1.0", false)]
	#[case("0.0.0", "0.1.0", false)]
	#[case("0.0.7", "0.1.5", false)]
	// patch (<1.0)
	#[case("0.0.1", "0.0.0", true)]
	#[case("0.3.4", "0.3.1", true)]
	#[case("0.1.0", "0.0.4", false)]
	#[case("0.0.2", "0.0.4", false)]
	fn version_compatibility(
		#[case] application: &'static str,
		#[case] plugin: &'static str,
		#[case] compatible: bool,
	) {
		let app_version = VersionStrings::new(application).parsed().expect("input error");
		let plugin_version = VersionStrings::new(plugin).parsed().expect("input error");

		assert_eq!(compatible, are_compatible(&app_version, &plugin_version));
	}
}
