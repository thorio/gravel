use crate::scrollbar::Scrollbar;
use fltk::{app::App, app::Receiver, app::Sender, frame::Frame, group::Group, input::Input, window::Window};
use gravel_core::FrontendMessage;

/// Holds all necessary elements of the FLTK app.
pub struct Ui {
	pub window: Window,
	pub app: App,
	pub input: Input,
	pub scrollbar: Scrollbar,
	pub hits: Vec<HitUi>,
	pub receiver: Receiver<Message>,
	pub sender: Sender<Message>,
}

/// Holds UI elements for displaying a single hit.
pub struct HitUi {
	pub group: Group,
	pub title: Frame,
	pub subtitle: Frame,
}

/// Represents Actions the UI should carry out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
	Query,
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
}

impl From<FrontendMessage> for Message {
	fn from(message: FrontendMessage) -> Self {
		match message {
			FrontendMessage::ShowOrHide => Message::ShowOrHideWindow,
			FrontendMessage::Show => Message::ShowWindow,
			FrontendMessage::Hide => Message::HideWindow,
			FrontendMessage::ShowWithQuery(query) => Message::ShowWithQuery(query),
			FrontendMessage::Exit => Message::Exit,
		}
	}
}
