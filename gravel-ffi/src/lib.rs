// abi_stable derives generate code that doesn't gel with these lints
// since you can't just slap these on generated code they're disabled for the whole crate
#![allow(
	clippy::empty_docs,
	clippy::used_underscore_binding,
	unused_qualifications,
	single_use_lifetimes
)]

pub mod config;
mod frontend;
mod hit;
pub mod logging;
pub mod paths;
mod plugin;
mod prefix;
mod provider;

pub mod prelude {
	pub use crate::{
		gravel_frontend, gravel_provider, ArcDynHit, BoxDynFrontendContext, FrontendContext, FrontendDef,
		FrontendExitStatus, FrontendMessage, FrontendMessageNe, Hit, PluginConfigAdapter, ProviderDef, ProviderResult,
		QueryResult, RefDynHitActionContext, ScoredHit, SimpleHit, MAX_SCORE, MIN_SCORE,
	};
}

pub use config::PluginConfigAdapter;
pub use frontend::{
	BoxDynFrontend, BoxDynFrontendContext, Frontend, FrontendContext, FrontendDef, FrontendExitStatus,
	FrontendExitStatusNe, FrontendMessage, FrontendMessageNe, QueryResult,
};
pub use hit::{ArcDynHit, Hit, HitActionContext, RefDynHitActionContext, ScoredHit, SimpleHit};
pub use plugin::{PluginDefinition, PluginMetadata};
pub use prefix::{PluginLib, PluginLibRef};
pub use provider::{BoxDynProvider, Provider, ProviderDef, ProviderResult};

pub use gravel_ffi_macros::*;

pub const MAX_SCORE: u32 = u32::MAX;
pub const MIN_SCORE: u32 = u32::MIN;
