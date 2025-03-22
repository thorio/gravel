//! This is the interface library between the main gravel application and individual plugins.
//!
//! [`abi_stable`] is used to facilitate safe FFI across compiler versions.

// the abi_stable derives/macros generate code that doesn't gel with these lints
// since you can't just slap these on generated code they're disabled for the whole crate
#![allow(
	clippy::empty_docs,
	clippy::needless_lifetimes,
	clippy::used_underscore_binding,
	non_local_definitions,
	single_use_lifetimes,
	unused_qualifications
)]

mod cache;
#[doc(hidden)]
pub mod config;
mod frontend;
mod hit;
pub mod logging;
pub mod paths;
mod plugin;
mod prefix;
mod provider;

/// This module re-exports the types needed to write plugins.
///
/// Bring them all into scope with `use gravel_ffi::prelude::*;`.  
/// <sub>may not actually contain *all* types required; terms and conditions apply</sub>
pub mod prelude {
	pub use crate::{
		ActionKind, ArcDynHit, BoxDynFrontendContext, Frontend, FrontendContext, FrontendContextExt,
		FrontendExitStatus, FrontendMessage, FrontendMessageNe, Hit, HitCache, MAX_SCORE, MIN_SCORE,
		PluginConfigAdapter, Provider, ProviderResult, QueryResult, RefDynHitActionContext, ScoredHit, SimpleHit,
		StaticHitCache, gravel_frontend, gravel_provider,
	};

	pub use abi_stable::external_types::crossbeam_channel::RReceiver;
	pub use abi_stable::std_types::{ROption, RStr};
	pub use abi_stable::traits::{IntoReprC, IntoReprRust};
}

pub use cache::{HitCache, StaticHitCache};
pub use config::PluginConfigAdapter;
pub use frontend::{
	BoxDynFrontend, BoxDynFrontendContext, Frontend, FrontendContext, FrontendContextExt, FrontendExitStatus,
	FrontendExitStatusNe, FrontendInner, FrontendMessage, FrontendMessageNe, QueryResult,
};
pub use hit::{
	ActionKind, ArcDynHit, Hit, HitActionContext, MAX_SCORE, MIN_SCORE, RefDynHitActionContext, ScoredHit, SimpleHit,
	clone_hit_arc,
};
pub use plugin::{PluginDefinition, PluginMetadata};
pub use prefix::{PluginLib, PluginLibRef};
pub use provider::{BoxDynProvider, Provider, ProviderInner, ProviderResult};

pub use gravel_ffi_macros::*;

#[cfg(test)]
mod clippy_shut_up {
	// this has to be put *somewhere* so clippy doesn't complain that the crate is unused
	// (even though it's used in the integration tests)
	use gravel_core as _;
	use gravel_test_utils as _;
}
