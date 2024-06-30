// abi_stable derives generate code that doesn't gel with these lints
// since you can't just slap these on generated code they're disabled for the whole crate
#![allow(
	clippy::empty_docs,
	clippy::used_underscore_binding,
	unused_qualifications,
	single_use_lifetimes
)]

use abi_stable::library::RootModule;
use abi_stable::{package_version_strings, sabi_types::VersionStrings, StableAbi};

mod config;
mod engine;
mod fns;
mod frontend;
mod hit;
pub mod paths;
mod plugin;
mod provider;

pub mod prelude {
	pub use crate::config::{ConfigLayer, ConfigManager, ConfigSource, MergeStrategy, PluginConfigAdapter};
	pub use crate::engine::{BoxDynQueryEngine, QueryEngine, QueryResult};
	pub use crate::frontend::{BoxDynFrontend, Frontend, FrontendDef, FrontendExitStatus, FrontendMessage};
	pub use crate::hit::{ArcDynHit, Hit, ScoredHit, SimpleHit};
	pub use crate::plugin::{PluginDefinition, PluginMetadata};
	pub use crate::provider::{BoxDynProvider, Provider, ProviderDef, ProviderResult};
	pub use crate::{PluginLib, PluginLibRef};

	pub use gravel_ffi_macros::*;

	pub const MAX_SCORE: u32 = u32::MAX;
	pub const MIN_SCORE: u32 = u32::MIN;
}

pub use prelude::*;

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
#[sabi(kind(Prefix(prefix_ref = PluginLibRef)))]
#[sabi(missing_field(panic))]
pub struct PluginLib {
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
	pub plugin: extern "C" fn() -> PluginDefinition,
}

// TODO: make log crate work in dynamic libs

#[allow(clippy::use_self)]
impl RootModule for PluginLibRef {
	abi_stable::declare_root_module_statics! {PluginLibRef}

	const BASE_NAME: &'static str = "example_library";
	const NAME: &'static str = "example_library";
	const VERSION_STRINGS: VersionStrings = package_version_strings!();
}
