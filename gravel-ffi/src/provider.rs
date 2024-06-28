use abi_stable::sabi_trait;
use abi_stable::std_types::{RBox, RStr, RVec};
use abi_stable::StableAbi;

use crate::hit::ArcDynHit;

pub type BoxDynProvider = Provider_TO<'static, RBox<()>>;

// can't implement From<T> because Provider_TO is generated into a different module
pub trait ProviderExt: Provider + 'static {
	fn into_dyn(self) -> BoxDynProvider
	where
		Self: Sized,
	{
		BoxDynProvider::from_value(self, sabi_trait::TD_Opaque)
	}
}

impl<T: Provider + 'static> ProviderExt for T {}

/// A provider takes a query and provides some relevant results.
#[sabi_trait]
pub trait Provider {
	fn query(&self, query: RStr<'_>) -> ProviderResult;
}

/// A collection of hits.
#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct ProviderResult {
	pub hits: RVec<ArcDynHit>,
}

impl ProviderResult {
	#[must_use]
	pub fn new(hits: impl Into<RVec<ArcDynHit>>) -> Self {
		Self { hits: hits.into() }
	}

	#[must_use]
	pub fn empty() -> Self {
		Self::new(vec![])
	}

	#[must_use]
	pub fn single(hit: ArcDynHit) -> Self {
		Self::new(vec![hit])
	}
}
