use std::sync::mpsc::Receiver;

/// A frontend is a user-interactive interface with which the user can issue
/// queries and browse and select their results.
pub trait Frontend {
	fn run(&mut self, receiver: Receiver<FrontendMessage>);
}

/// Represents actions the [`Frontend`] should take.
///
/// These values are to be received by the frontend via a provided
/// [`Receiver`] and must be handled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontendMessage {
	ShowOrHide,
	Show,
	Hide,
	ShowWithQuery(String),
	Exit,
}
