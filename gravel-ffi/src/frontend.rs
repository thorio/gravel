use crate::{ActionKind, ArcDynHit, PluginConfigAdapter, ScoredHit};
use abi_stable::std_types::{RBox, ROption, RString, RVec};
use abi_stable::traits::IntoReprRust;
use abi_stable::{sabi_trait, StableAbi};

/// FFI-safe [`FrontendInner`] trait object.
pub type BoxDynFrontend = FrontendInner_TO<'static, RBox<()>>;

/// This trait is auto-implemented with the [`crate::gravel_frontend`] macro.
///
/// It does some boilerplate conversions to reduce complexity in the real [`Frontend`].
#[sabi_trait]
pub trait FrontendInner {
	fn run(&mut self) -> FrontendExitStatusNe;
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
///     fn run(&mut self) -> FrontendExitStatus {
///         // run UI here
///
///         // gravel will exit when this function returns
///         FrontendExitStatus::Exit
///     }
/// }
/// ```
pub trait Frontend {
	/// Constructs a new frontend. Events must be received and handled by calling [`FrontendContextExt::recv`].
	fn new(engine: BoxDynFrontendContext, config: &PluginConfigAdapter<'_>) -> Self;

	/// Runs the UI; must block until ready to exit.
	fn run(&mut self) -> FrontendExitStatus;
}

/// FFI-safe [`FrontendContext`] trait object.
pub type BoxDynFrontendContext = FrontendContext_TO<'static, RBox<()>>;

/// Context object providing core functionality to the [`Frontend`].
#[sabi_trait]
pub trait FrontendContext {
	/// Use [`FrontendContextExt::recv`] instead.
	#[doc(hidden)]
	fn recv_raw(&self) -> ROption<FrontendMessageNe>;

	/// Runs the query against configured providers and returns results.
	///
	/// The return value is a token unique to each query, allowing frontends to
	/// ignore out-of-date results in lieu of cancellations.
	fn query(&self, query: RString) -> u32;

	/// Executes the passed hit's action.
	fn run_hit_action(&self, hit: &ArcDynHit, kind: ActionKind);
}

pub trait FrontendContextExt: FrontendContext {
	/// Attempts to receive one [`FrontendMessage`].
	///
	/// Returns [`None`] if there is no message or the message is from a newer version of the plugin interface.
	fn recv(&self) -> Option<FrontendMessage> {
		self.recv_raw()
			.into_rust()?
			.into_enum()
			.inspect_err(|e| log::warn!("unknown FrontendMessage, this plugin is out of date: {e}"))
			.ok()
	}
}

impl<T: FrontendContext> FrontendContextExt for T {}

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

/// Non-exhaustive wrapper around [`FrontendMessage`].
pub type FrontendMessageNe = FrontendMessage_NE;

/// Represents actions the [`Frontend`] should take.
///
/// These values are to be received by the frontend via
/// [`FrontendContextExt::recv`] and must be handled.
#[repr(u8)]
#[derive(StableAbi, Debug)]
#[sabi(kind(WithNonExhaustive(size = 40, traits(Debug))))]
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

	/// Clear all caches the frontend may keep.
	ClearCaches,

	/// Result from a previous query.
	///
	/// The first value is a token unique to each query, allowing frontends to
	/// ignore out-of-date results in lieu of cancellations.
	QueryResult(u32, QueryResult),
}

/// Non-exhaustive wrapper around [`FrontendExitStatus`].
pub type FrontendExitStatusNe = FrontendExitStatus_NE;

/// This is returned when a [`Frontend`] exits and tells gravel what to do.
#[repr(u8)]
#[derive(StableAbi, Debug, Copy, Clone)]
#[sabi(kind(WithNonExhaustive(size = 40, traits(Debug, Clone))))]
pub enum FrontendExitStatus {
	Exit,
	Restart,
}
