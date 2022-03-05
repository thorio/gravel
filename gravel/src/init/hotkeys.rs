use crate::config::*;
use gravel_core::{hotkeys::*, *};
use std::sync::mpsc::Sender;

/// Initializes a hotkey listener on a different thread.
/// See [`Listener`].
pub fn hotkeys(hotkeys: &Vec<HotkeyConfig>, sender: Sender<FrontendMessage>) {
	let mut listener = Listener::<FrontendMessage>::new();

	for hotkey in hotkeys {
		match listener.register_emacs(&hotkey.binding, get_control_message(hotkey)) {
			Ok(_) => (),
			Err(_) => println!("invalid hotkey {}, skipping", &hotkey.binding),
		};
	}

	listener.spawn_listener(sender);
}

fn get_control_message(hotkey: &HotkeyConfig) -> FrontendMessage {
	match &hotkey.action {
		HotkeyAction::ShowHide => FrontendMessage::ShowOrHide,
		HotkeyAction::Show => FrontendMessage::Show,
		HotkeyAction::Hide => FrontendMessage::Hide,
		HotkeyAction::ShowWith => FrontendMessage::ShowWithQuery(hotkey.query.as_deref().unwrap_or("").to_owned()),
	}
}
