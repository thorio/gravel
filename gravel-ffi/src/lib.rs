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

pub mod config;
mod engine;
mod fns;
mod frontend;
mod hit;
pub mod paths;
mod plugin;
mod provider;

pub mod prelude {
	pub use crate::{gravel_frontend, gravel_provider};
	pub use crate::{ArcDynHit, BoxDynHitActionContext, Hit, ScoredHit, SimpleHit};
	pub use crate::{BoxDynQueryEngine, QueryEngine, QueryResult};
	pub use crate::{FrontendDef, FrontendExitStatus, FrontendMessage};
	pub use crate::{PluginConfigAdapter, ProviderDef, ProviderResult};
	pub use crate::{MAX_SCORE, MIN_SCORE};
}

pub use config::PluginConfigAdapter;
pub use engine::{BoxDynQueryEngine, QueryEngine, QueryResult};
pub use frontend::{BoxDynFrontend, Frontend, FrontendDef, FrontendExitStatus, FrontendMessage};
pub use hit::{ArcDynHit, BoxDynHitActionContext, Hit, HitActionContext, ScoredHit, SimpleHit};
pub use plugin::{PluginDefinition, PluginMetadata};
pub use provider::{BoxDynProvider, Provider, ProviderDef, ProviderResult};

pub use gravel_ffi_macros::*;

pub const MAX_SCORE: u32 = u32::MAX;
pub const MIN_SCORE: u32 = u32::MIN;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginLibRef)))]
pub struct PluginLib {
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
