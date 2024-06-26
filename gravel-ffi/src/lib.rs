#![allow(clippy::empty_docs, unused_qualifications, clippy::used_underscore_binding)]

use abi_stable::library::{LibraryError, RootModule};
use abi_stable::{package_version_strings, sabi_types::VersionStrings, StableAbi};
use std::path::Path;

mod config;
mod engine;
mod fns;
mod frontend;
mod hit;
mod plugin;
mod provider;

pub use config::{ConfigLayer, ConfigManager, ConfigSource, MergeStrategy, PluginConfigAdapter};
pub use engine::{BoxDynQueryEngine, QueryEngine, QueryEngineExt, QueryResult};
pub use frontend::{BoxDynFrontend, Frontend, FrontendExitStatus, FrontendMessage};
pub use hit::{ArcDynHit, Hit, HitExt, ScoredHit, SimpleHit};
pub use plugin::{plugin, PluginDefinition};
pub use provider::{BoxDynProvider, Provider, ProviderExt, ProviderResult};

/// This struct is the root module,
/// which must be converted to `ExampleLib_Ref` to be passed through ffi.
///
/// The `#[sabi(kind(Prefix(prefix_ref = ExampleLib_Ref)))]`
/// attribute tells `StableAbi` to create an ffi-safe static reference type
/// for `ExampleLib` called `ExampleLib_Ref`.
///
/// The `#[sabi(missing_field(panic))]` attribute specifies that trying to
/// access a field that doesn't exist must panic with a message saying that
/// the field is inaccessible.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = GravelPluginLibRef)))]
#[sabi(missing_field(panic))]
pub struct GravelPluginLib {
	/// The `#[sabi(last_prefix_field)]` attribute here means that this is the last
	/// field in this struct that was defined in the first compatible version of the library
	/// (0.1.0, 0.2.0, 0.3.0, 1.0.0, 2.0.0 ,etc),
	/// requiring new fields to always be added below preexisting ones.
	///
	/// The `#[sabi(last_prefix_field)]` attribute would stay on this field until the
	/// library bumps its "major" version,
	/// at which point it would be moved to the last field at the time.
	///
	#[sabi(last_prefix_field)]
	pub get_plugin: extern "C" fn() -> PluginDefinition,
}

#[allow(clippy::use_self)]
impl RootModule for GravelPluginLibRef {
	abi_stable::declare_root_module_statics! {GravelPluginLibRef}

	const BASE_NAME: &'static str = "example_library";
	const NAME: &'static str = "example_library";
	const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

/// This loads the root from the library in the `directory` folder.
pub fn load_root_module_in_directory(directory: &Path) -> Result<GravelPluginLibRef, LibraryError> {
	GravelPluginLibRef::load_from_file(&directory.join("libgravel_plugin_test.so"))
}
