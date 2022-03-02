#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gravel_core::{frontend::*, plugin::*, *};
use gravel_frontend_default::DefaultFrontend;
use gravel_provider_calculator::CalculatorProvider;
use gravel_provider_program::ProgramProvider;
use gravel_provider_websearch::WebsearchProvider;
use gravel_util::hotkeys;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

fn main() {
	let (sender, receiver): (Sender<ControlMessage>, Receiver<ControlMessage>) = mpsc::channel();

	let engine = create_engine(sender.clone());
	let mut frontend = create_frontend(engine);

	init_hotkeys(sender);

	frontend.run(receiver);
}

fn init_hotkeys(sender: Sender<ControlMessage>) {
	hotkeys::Listener::<ControlMessage>::new()
		.register_emacs("S-<Space>", ControlMessage::ShowOrHide)
		.unwrap()
		.spawn_listener(sender);
}

fn create_engine(sender: Sender<ControlMessage>) -> QueryEngine {
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
