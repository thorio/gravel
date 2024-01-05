//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

use gravel_core::{performance::Stopwatch, *};
use std::sync::mpsc;

mod init;

fn main() {
	let stopwatch = Stopwatch::start();
	#[cfg(windows)]
	init::windows_console::attach();

	let args = init::cli();
	init::logging(args.verbosity.log_level());

	let config = init::config();
	let (sender, receiver) = mpsc::channel::<FrontendMessage>();

	let registry = init::plugins();
	let engine = init::engine(sender.clone(), &registry, &config);
	let mut frontend = init::frontend(&registry, engine, &config);

	init::single_instance(config.root.single_instance.as_deref());
	init::hotkeys(&config.root.hotkeys, sender);

	log::trace!("initialization complete, took {stopwatch}");
	log::trace!("starting frontend");
	frontend.run(receiver);

	#[cfg(windows)]
	init::windows_console::detach();

	log::trace!("exiting");
}
