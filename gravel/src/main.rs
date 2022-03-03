#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gravel_core::{plugin::*, *};
use gravel_frontend_default::DefaultFrontend;
use gravel_provider_calculator::CalculatorProvider;
use gravel_provider_program::ProgramProvider;
use gravel_provider_websearch::WebsearchProvider;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod init;

fn main() {
	let config = init::config();
	let (sender, receiver): (Sender<FrontendMessage>, Receiver<FrontendMessage>) = mpsc::channel();

	let engine = create_engine(sender.clone());
	let mut frontend = create_frontend(engine);

	init::hotkeys(&config.hotkeys, sender);

	frontend.run(receiver);
}

fn create_engine(sender: Sender<FrontendMessage>) -> QueryEngine {
	let registry = get_registry();

	QueryEngine::new(registry.providers, sender)
}

fn get_registry() -> PluginRegistry {
	let mut registry = load_plugins();
	registry
		.provider(Box::new(ProgramProvider::new()))
		.provider(Box::new(CalculatorProvider::new()))
		.provider(Box::new(WebsearchProvider::new()));

	registry
}

fn create_frontend(engine: QueryEngine) -> Box<dyn Frontend> {
	Box::new(DefaultFrontend::new(engine))
}
