#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gravel_core::plugin::*;
use gravel_core::{frontend::*, *};
use gravel_frontend_default::DefaultFrontend;
use gravel_provider_calculator::CalculatorProvider;
use gravel_provider_program::ProgramProvider;

fn main() {
	let registry = get_registry();
	let gravel = Gravel::new(registry.providers);

	let mut frontend = get_frontend(gravel);

	frontend.run();
}

fn get_registry() -> PluginRegistry {
	let mut registry = load_plugins();
	registry
		.provider(Box::new(ProgramProvider::new()))
		.provider(Box::new(CalculatorProvider::new()));

	registry
}

fn get_frontend(gravel: Gravel) -> Box<dyn Frontend> {
	Box::new(DefaultFrontend::new(gravel))
}
