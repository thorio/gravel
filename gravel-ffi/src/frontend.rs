use crate::{ArcDynHit, PluginConfigAdapter, ScoredHit};
use abi_stable::std_types::{RBox, RStr, RString, RVec};
use abi_stable::{external_types::crossbeam_channel::RReceiver, sabi_trait, StableAbi};

pub type BoxDynFrontend = Frontend_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait Frontend {
	fn run(&mut self, receiver: RReceiver<FrontendMessage>) -> FrontendExitStatus;
}

pub trait FrontendDef {
	fn new(engine: BoxDynFrontendContext, config: &PluginConfigAdapter<'_>) -> Self;
	fn run(&mut self, receiver: RReceiver<FrontendMessage>) -> FrontendExitStatus;
}

pub type BoxDynFrontendContext = FrontendContext_TO<'static, RBox<()>>;

/// Aggregates and scores hits from the given [`Provider`]s.
#[sabi_trait]
pub trait FrontendContext {
	/// Runs the query against configured providers and returns results
	fn query(&self, query: RStr<'_>) -> QueryResult;

	/// Executes the passed hit's action
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

/// Represents actions the [`Frontend`] should take.
///
/// These values are to be received by the frontend via a provided
/// [`Receiver`] and must be handled.
#[derive(StableAbi, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FrontendMessage {
	ShowOrHide,
	Show,
	Hide,
	ShowWithQuery(RString),
	Refresh,
	Exit,
	Restart,
}

// TODO: implement nonexhaustive for these two
#[derive(StableAbi, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FrontendExitStatus {
	Exit,
	Restart,
}
