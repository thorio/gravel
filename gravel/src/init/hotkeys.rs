use abi_stable::external_types::crossbeam_channel::RSender;
use abi_stable::std_types::RString;
use gravel_core::config::{HotkeyAction, HotkeyConfig};
use gravel_core::hotkeys::Listener;
use gravel_ffi::FrontendMessage;

/// Initializes a hotkey listener on a different thread.
/// See [`Listener`].
pub fn hotkeys(hotkeys: &[HotkeyConfig], sender: RSender<FrontendMessage>) {
	log::trace!("initializing hotkeys");

	let mut listener = Listener::<FrontendMessage>::default();

	for hotkey in hotkeys {
		let binding = &hotkey.binding;
		let action = &hotkey.action;

		match listener.register_emacs(binding, get_control_message(hotkey)) {
			Ok(_) => log::debug!("registered hotkey '{binding}' with action '{action:?}'"),
			Err(err) => log::warn!("invalid binding '{}', {err}. skipping", binding),
		};
	}

	listener.spawn_listener(sender);
}

fn get_control_message(hotkey: &HotkeyConfig) -> FrontendMessage {
	match &hotkey.action {
		HotkeyAction::ShowHide => FrontendMessage::ShowOrHide,
		HotkeyAction::Show => FrontendMessage::Show,
		HotkeyAction::Hide => FrontendMessage::Hide,
		HotkeyAction::ShowWith(query) => FrontendMessage::ShowWithQuery(RString::from(query.as_str())),
	}
}
