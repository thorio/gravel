use abi_stable::std_types::{RBox, RString};
use abi_stable::{external_types::crossbeam_channel::RReceiver, sabi_trait, StableAbi};

pub type BoxDynFrontend = Frontend_TO<'static, RBox<()>>;

// can't implement From<T> because Provider_TO is generated into a different module
pub trait FrontendExt: Frontend + 'static {
	fn into_dyn(self) -> BoxDynFrontend
	where
		Self: Sized,
	{
		BoxDynFrontend::from_value(self, sabi_trait::TD_Opaque)
	}
}

impl<T: Frontend + 'static> FrontendExt for T {}

#[sabi_trait]
pub trait Frontend {
	fn run(&mut self, receiver: RReceiver<FrontendMessage>) -> FrontendExitStatus;
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

#[derive(StableAbi, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FrontendExitStatus {
	Exit,
	Restart,
}
