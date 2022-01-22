use crate::scrollbar::Scrollbar;
use fltk::{app::App, app::Receiver, frame::Frame, group::Group, input::Input, window::Window};

pub struct Ui {
	pub window: Window,
	pub app: App,
	pub input: Input,
	pub scrollbar: Scrollbar,
	pub hits: Vec<HitUi>,
	pub receiver: Receiver<Message>,
}

pub struct HitUi {
	pub group: Group,
	pub title: Frame,
	pub subtitle: Frame,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Message {
	Query,
	Confirm,
	CursorUp,
	CursorDown,
	CursorPageUp,
	CursorPageDown,
	CursorTop,
	CursorBottom,
	Cancel,
}
