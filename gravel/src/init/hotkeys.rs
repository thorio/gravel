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

	let mut listener = Listener::default();

	for hotkey in hotkeys {
		let binding = &hotkey.binding;
		let action = hotkey.action.clone();
		let sender_clone = sender.clone();

		let result = listener.register_emacs(binding, move || {
			sender_clone
				.send(FrontendMessageNe::new((&action).into()))
				.inspect_err(|e| log::error!("unable to send hotkey action message: {e}"))
				.ok();
		});

		match result {
			Ok(_) => log::debug!("registered hotkey '{binding}' with action '{:?}'", hotkey.action),
			Err(e) => log::warn!("invalid binding '{}', {e}. skipping", binding),
		};
	}

	listener.spawn_listener();
}
