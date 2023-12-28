//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

use gravel_core::*;
use std::sync::mpsc;

mod init;

fn main() {
	#[cfg(windows)]
	init::windows_console::attach();

	let config = init::config();
	let (sender, receiver) = mpsc::channel::<FrontendMessage>();

	let registry = init::plugins();
	let engine = init::engine(sender.clone(), &registry, &config);
	let mut frontend = init::frontend(&registry, engine, &config);

	init::single_instance(config.root.single_instance.as_ref());
	init::hotkeys(&config.root.hotkeys, sender);

	frontend.run(receiver);

	#[cfg(windows)]
	init::windows_console::detach();
}
