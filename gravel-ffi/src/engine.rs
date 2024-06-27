use crate::{hit::ScoredHit, ArcDynHit};
use abi_stable::std_types::{RBox, RStr, RVec};
use abi_stable::{sabi_trait, StableAbi};

pub type BoxDynQueryEngine = QueryEngine_TO<'static, RBox<()>>;

// can't implement From<T> because QueryEngine_TO is generated into a different module
pub trait QueryEngineExt: QueryEngine + 'static {
	fn into_dyn(self) -> BoxDynQueryEngine
	where
		Self: Sized,
	{
		BoxDynQueryEngine::from_value(self, sabi_trait::TD_Opaque)
	}
}

impl<T: QueryEngine + 'static> QueryEngineExt for T {}

/// Aggregates and scores hits from the given [`Provider`]s.
#[sabi_trait]
pub trait QueryEngine {
	/// Queries all providers with the given query.
	fn query(&self, query: RStr<'_>) -> QueryResult;
	fn run_hit_action(&self, hit: &ArcDynHit);
}

#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct QueryResult {
	pub hits: RVec<ScoredHit>,
}

impl QueryResult {
	pub fn new(hits: impl Into<RVec<ScoredHit>>) -> Self {
		Self { hits: hits.into() }
	}

	pub fn empty() -> Self {
		Self::new(vec![])
	}
}
