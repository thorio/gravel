use crate::config::PluginConfigAdapter;
use crate::hit::{clone_hit_ptr, ArcDynHit};
use abi_stable::sabi_trait;
use abi_stable::std_types::{RBox, RStr, RVec};
use abi_stable::StableAbi;

pub type BoxDynProvider = Provider_TO<'static, RBox<()>>;

/// A provider takes a query and provides some relevant results.
#[sabi_trait]
pub trait Provider {
	fn query(&self, query: RStr<'_>) -> ProviderResult;
}

pub trait ProviderDef {
	fn new(config: &PluginConfigAdapter<'_>) -> Self;
	fn query(&self, query: &str) -> ProviderResult;
}

/// A collection of hits.
#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct ProviderResult {
	pub hits: RVec<ArcDynHit>,
}

impl ProviderResult {
	#[must_use]
	pub fn new(hits: impl IntoIterator<Item = impl Into<ArcDynHit>>) -> Self {
		let hits = hits.into_iter().map(Into::into).collect();
		Self { hits }
	}

	#[must_use]
	pub fn from_cached<'a>(hits: impl IntoIterator<Item = &'a ArcDynHit>) -> Self {
		let hits = hits.into_iter().map(clone_hit_ptr).collect();
		Self { hits }
	}

	#[must_use]
	pub fn empty() -> Self {
		Self { hits: RVec::new() }
	}

	#[must_use]
	pub fn single(hit: impl Into<ArcDynHit>) -> Self {
		let hits = vec![hit.into()].into();
		Self { hits }
	}
}
