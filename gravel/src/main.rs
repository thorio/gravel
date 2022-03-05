//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// When compiling in release mode, disable the cmd window that pops up on windows.
// This also disables console output, that's why it isn't enabled in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gravel_core::{plugin::*, *};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod init;

fn main() {
	let config = init::config();
	let (sender, receiver): (Sender<FrontendMessage>, Receiver<FrontendMessage>) = mpsc::channel();

	let registry = init::plugins();
	let engine = create_engine(sender.clone(), &registry);
	let mut frontend = get_frontend(&registry, engine);

	init::single_instance(&config.single_instance);
	init::hotkeys(&config.hotkeys, sender);

	frontend.run(receiver);
}

/// placeholder
fn create_engine(sender: Sender<FrontendMessage>, registry: &PluginRegistry) -> QueryEngine {
	let providers = vec![
		registry.get_provider("calculator").unwrap().get_provider().unwrap(),
		registry.get_provider("program").unwrap().get_provider().unwrap(),
		registry.get_provider("websearch").unwrap().get_provider().unwrap(),
	];

	QueryEngine::new(providers, sender)
}

/// placeholder
fn get_frontend(registry: &PluginRegistry, engine: QueryEngine) -> Box<dyn Frontend> {
	registry.get_frontend("default").unwrap().get_frontend(engine).unwrap()
}
