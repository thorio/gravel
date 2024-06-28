use crate::scrollbar::Scrollbar;
use fltk::{app::App, app::Receiver, app::Sender, frame::Frame, group::Group, input::Input, window::Window};
use gravel_ffi::prelude::*;

/// Holds all necessary elements of the FLTK app.
pub struct Ui {
	pub window: Window,
	pub app: App,
	pub input: Input,
	pub scrollbar: Scrollbar,
	pub hits: Vec<HitUi>,
	pub receiver: Receiver<Event>,
	pub sender: Sender<Event>,
}

/// Holds UI elements for displaying a single hit.
pub struct HitUi {
	pub group: Group,
	pub title: Frame,
	pub subtitle: Frame,
}

/// Represents Actions the UI should carry out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
	Query,
	ForceQuery,
	Confirm,
	CursorUp,
	CursorDown,
	CursorPageUp,
	CursorPageDown,
	CursorTop,
	CursorBottom,
	ShowWindow,
	HideWindow,
	ShowOrHideWindow,
	ShowWithQuery(String),
	Cancel,
	Exit,
	Restart,
}

impl From<FrontendMessage> for Event {
	fn from(message: FrontendMessage) -> Self {
		use FrontendMessage as M;
		match message {
			M::ShowOrHide => Self::ShowOrHideWindow,
			M::Show => Self::ShowWindow,
			M::Hide => Self::HideWindow,
			M::ShowWithQuery(query) => Self::ShowWithQuery(query.into()),
			M::Refresh => Self::ForceQuery,
			M::Exit => Self::Exit,
			M::Restart => Self::Restart,
		}
	}
}
