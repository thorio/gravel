use crate::config::PluginConfigAdapter;
use crate::hit::{clone_hit_arc, ArcDynHit};
use abi_stable::sabi_trait;
use abi_stable::std_types::{RBox, RStr, RVec};
use abi_stable::StableAbi;

/// FFI-safe [`ProviderInner`] trait object.
pub type BoxDynProvider = ProviderInner_TO<'static, RBox<()>>;

/// This trait is auto-implemented with the [`crate::gravel_provider`] macro.
///
/// It does some boilerplate conversions to reduce complexity in the real [`Provider`].
#[sabi_trait]
pub trait ProviderInner {
	fn query(&self, query: RStr<'_>) -> ProviderResult;
	fn clear_caches(&self) {
		// do nothing
	}
}

/// Abstracts functionality required for a provider.
///
/// Implement this and add the [`crate::gravel_provider`] macro to write a provider plugin:
/// ```
/// use gravel_ffi::prelude::*;
///
/// pub struct MyProvider {
///     // put state here
/// }
///
/// // give the plugin a memorable name
/// #[gravel_provider("my_provider")]
/// impl Provider for MyProvider {
///     fn new(config: &PluginConfigAdapter<'_>) -> Self {
///         Self { }
///     }
///
///     fn query(&self, query: &str) -> ProviderResult {
///
///         ProviderResult::empty()
///     }
/// }
/// ```
pub trait Provider {
	/// Constructs a new provider.
	fn new(config: &PluginConfigAdapter<'_>) -> Self;

	/// Queries the provider and returns hits.
	fn query(&self, query: &str) -> ProviderResult;

	/// Clears all caches the provider may keep.
	fn clear_caches(&self) {
		// do nothing
	}
}

/// A collection of hits returned by a provider.
#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct ProviderResult {
	pub hits: RVec<ArcDynHit>,
}

impl ProviderResult {
	/// Constructs a new [`ProviderResult`] from some hits.
	#[must_use]
	pub fn new(hits: impl IntoIterator<Item = impl Into<ArcDynHit>>) -> Self {
		let hits = hits.into_iter().map(Into::into).collect();
		Self { hits }
	}

	/// Constructs a new [`ProviderResult`] from some borrowed hits, cloning them (incrementing the Rc).
	#[must_use]
	pub fn from_cached<'a>(hits: impl IntoIterator<Item = &'a ArcDynHit>) -> Self {
		let hits = hits.into_iter().map(clone_hit_arc).collect();
		Self { hits }
	}

	/// Constructs an empty [`ProviderResult`].
	#[must_use]
	pub fn empty() -> Self {
		Self { hits: RVec::new() }
	}

	/// Constructs a new [`ProviderResult`] from a single hit.
	#[must_use]
	pub fn single(hit: impl Into<ArcDynHit>) -> Self {
		let hits = vec![hit.into()].into();
		Self { hits }
	}
}
