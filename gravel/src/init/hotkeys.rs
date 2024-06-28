use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_core::config::HotkeyConfig;
use gravel_core::hotkeys::Listener;
use gravel_ffi::prelude::*;

/// Initializes a hotkey listener on a different thread.
/// See [`Listener`].
pub fn hotkeys(hotkeys: &[HotkeyConfig], sender: RSender<FrontendMessage>) {
	log::trace!("initializing hotkeys");

	let mut listener = Listener::<FrontendMessage>::default();

	for hotkey in hotkeys {
		let binding = &hotkey.binding;
		let action = &hotkey.action;

		match listener.register_emacs(binding, (&hotkey.action).into()) {
			Ok(_) => log::debug!("registered hotkey '{binding}' with action '{action:?}'"),
			Err(err) => log::warn!("invalid binding '{}', {err}. skipping", binding),
		};
	}

	listener.spawn_listener(sender);
}
