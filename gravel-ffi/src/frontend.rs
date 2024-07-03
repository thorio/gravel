use crate::{ArcDynHit, PluginConfigAdapter, ScoredHit};
use abi_stable::std_types::{RBox, RStr, RString, RVec};
use abi_stable::{external_types::crossbeam_channel::RReceiver, sabi_trait, StableAbi};

pub type BoxDynFrontend = Frontend_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait Frontend {
	fn run(&mut self, receiver: RReceiver<FrontendMessageNe>) -> FrontendExitStatusNe;
}

pub trait FrontendDef {
	fn new(engine: BoxDynFrontendContext, config: &PluginConfigAdapter<'_>) -> Self;
	fn run(&mut self, receiver: RReceiver<FrontendMessageNe>) -> FrontendExitStatus;
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

pub type FrontendMessageNe = FrontendMessage_NE;

/// Represents actions the [`Frontend`] should take.
///
/// These values are to be received by the frontend via a provided
/// [`Receiver`] and must be handled.
#[repr(u8)]
#[derive(StableAbi, Debug, Clone)]
#[sabi(kind(WithNonExhaustive(size = 40, traits(Debug, Clone))))]
pub enum FrontendMessage {
	ShowOrHide,
	Show,
	Hide,
	ShowWithQuery(RString),
	Refresh,
	Exit,
	Restart,
}

pub type FrontendExitStatusNe = FrontendExitStatus_NE;

#[repr(u8)]
#[derive(StableAbi, Debug, Copy, Clone)]
#[sabi(kind(WithNonExhaustive(size = 40, traits(Debug, Clone))))]
pub enum FrontendExitStatus {
	Exit,
	Restart,
}
