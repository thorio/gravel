use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_core::config::Hotkey;
use gravel_core::hotkeys::Listener;
use gravel_ffi::FrontendMessageNe;

/// Initializes a hotkey listener on a different thread.
/// See [`Listener`].
pub fn hotkeys(hotkeys: &[Hotkey], sender: RSender<FrontendMessageNe>) {
	if hotkeys.is_empty() {
		log::debug!("no hotkeys configured");
		return;
	}

	log::trace!("initializing hotkeys");

	let mut listener = Listener::<FrontendMessageNe>::default();

	for hotkey in hotkeys {
		let binding = &hotkey.binding;
		let action = &hotkey.action;
		let message = FrontendMessageNe::new(action.clone().into());

		match listener.register_emacs(binding, message) {
			Ok(_) => log::debug!("registered hotkey '{binding}' with action '{action:?}'"),
			Err(e) => log::warn!("invalid binding '{}', {e}. skipping", binding),
		};
	}

	listener.spawn_listener(sender);
}
