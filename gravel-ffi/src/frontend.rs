use crate::{ArcDynHit, PluginConfigAdapter, ScoredHit};
use abi_stable::std_types::{RBox, RStr, RString, RVec};
use abi_stable::{external_types::crossbeam_channel::RReceiver, sabi_trait, StableAbi};

/// FFI-safe [`FrontendInner`] trait object.
pub type BoxDynFrontend = FrontendInner_TO<'static, RBox<()>>;

/// This trait is auto-implemented with the [`crate::gravel_frontend`] macro.
///
/// It does some boilerplate conversions to reduce complexity in the real [`Frontend`].
#[sabi_trait]
pub trait FrontendInner {
	fn run(&mut self, receiver: RReceiver<FrontendMessageNe>) -> FrontendExitStatusNe;
}

/// Abstracts functionality required for a frontend.
///
/// Implement this and add the [`crate::gravel_frontend`] macro to write a frontend plugin:
/// ```
/// use gravel_ffi::prelude::*;
///
/// pub struct MyFrontend {
///     // put state here
/// }
///
/// // give the plugin a memorable name
/// #[gravel_frontend("my_frontend")]
/// impl Frontend for MyFrontend {
///     fn new(context: BoxDynFrontendContext, config: &PluginConfigAdapter<'_>) -> Self {
///         // initialize UI here
///         Self { }
///     }
///
///     fn run(&mut self, receiver: RReceiver<FrontendMessageNe>) -> FrontendExitStatus {
///         // run UI here
///
///         // gravel will exit when this function returns
///         FrontendExitStatus::Exit
///     }
/// }
/// ```
pub trait Frontend {
	/// Constructs a new frontend.
	fn new(engine: BoxDynFrontendContext, config: &PluginConfigAdapter<'_>) -> Self;

	/// Runs the UI. Messages sent to the `receiver` must be handled.
	fn run(&mut self, receiver: RReceiver<FrontendMessageNe>) -> FrontendExitStatus;
}

/// FFI-safe [`FrontendContext`] trait object.
pub type BoxDynFrontendContext = FrontendContext_TO<'static, RBox<()>>;

/// Context object providing core functionality to the [`Frontend`].
#[sabi_trait]
pub trait FrontendContext {
	/// Runs the query against configured providers and returns results.
	fn query(&self, query: RStr<'_>) -> QueryResult;

	/// Executes the passed hit's action.
	fn run_hit_action(&self, hit: &ArcDynHit);

	/// Executes the passed hit's secondary action.
	fn run_secondary_hit_action(&self, hit: &ArcDynHit);
}

/// A Collection of scored hits returned by the [`FrontendContext`].
#[repr(C)]
#[derive(StableAbi, Default, Debug)]
pub struct QueryResult {
	pub hits: RVec<ScoredHit>,
}

impl QueryResult {
	pub fn new(hits: impl Into<RVec<ScoredHit>>) -> Self {
		Self { hits: hits.into() }
	}
}

/// Non-exhaustive variant of [`FrontendMessage`].
pub type FrontendMessageNe = FrontendMessage_NE;

/// Represents actions the [`Frontend`] should take.
///
/// These values are to be received by the frontend via a provided
/// [`RReceiver`] and must be handled.
#[repr(u8)]
#[derive(StableAbi, Debug, Clone)]
#[sabi(kind(WithNonExhaustive(size = 40, traits(Debug, Clone))))]
pub enum FrontendMessage {
	ShowOrHide,
	Show,
	Hide,

	/// Show the window and pre-populate the query with the parameter.
	ShowWithQuery(RString),

	/// Run the same query again.
	Refresh,

	/// Exit the appplication with [`FrontendExitStatus::Exit`].
	Exit,

	/// Exit the appplication with [`FrontendExitStatus::Restart`].
	Restart,
}

/// Non-exhaustive variant of [`FrontendExitStatus`].
pub type FrontendExitStatusNe = FrontendExitStatus_NE;

/// This is returned when a [`Frontend`] exits and tells gravel what to do.
#[repr(u8)]
#[derive(StableAbi, Debug, Copy, Clone)]
#[sabi(kind(WithNonExhaustive(size = 40, traits(Debug, Clone))))]
pub enum FrontendExitStatus {
	Exit,
	Restart,
}
