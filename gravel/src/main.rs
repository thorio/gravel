//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// When compiling in release mode, disable the cmd window that pops up on windows.
// This also disables console output, that's why it isn't enabled in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gravel_core::*;
use std::sync::mpsc;

mod init;

fn main() {
	let config = init::config();
	let (sender, receiver) = mpsc::channel::<FrontendMessage>();

	let registry = init::plugins();
	let engine = init::engine(sender.clone(), &registry, &config);
	let mut frontend = init::frontend(&registry, engine, &config);

	init::single_instance(config.root.single_instance.as_ref());
	init::hotkeys(&config.root.hotkeys, sender);

	frontend.run(receiver);
}
