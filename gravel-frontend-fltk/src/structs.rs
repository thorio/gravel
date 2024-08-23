use crate::scrollbar::Scrollbar;
use fltk::{app::App, app::Receiver, app::Sender, frame::Frame, group::Group, input::Input, window::Window};
use gravel_ffi::ActionKind;

/// Holds all necessary elements of the FLTK app.
pub struct Ui {
	pub window: Window,
	pub _app: App,
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
#[derive(Debug)]
pub enum Event {
	Query,
	Confirm(ActionKind),
	CursorUp,
	CursorDown,
	PageUp,
	PageDown,
	CursorTop,
	CursorBottom,
	HideWindow,
	Cancel,
	Exit,
}
