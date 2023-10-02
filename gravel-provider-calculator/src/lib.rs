//! Calculator provider based on [`meval`].
//!
//! Whenever the input can be parsed as a mathematical expression, shows the
//! result as the first hit.
//!
//! Selecting the hit copies the calculated value to the system's clipboard.

use arboard::Clipboard;
use evalexpr::Value;
use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MAX_SCORE, *};
use serde::Deserialize;
use std::{
	error::Error,
	sync::{mpsc::Sender, Arc},
};

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("calculator").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(config: &PluginConfigAdapter) -> Box<dyn Provider> {
	let plugin_config = config.get::<Config>(DEFAULT_CONFIG);

	Box::new(CalculatorProvider { config: plugin_config })
}

struct CalculatorProvider {
	config: Config,
}

impl Provider for CalculatorProvider {
	fn query(&self, query: &str) -> ProviderResult {
		match evalexpr::eval(query) {
			Ok(Value::Float(result)) => self.get_result(query, result),
			Ok(Value::Int(result)) => self.get_result(query, result as f64),
			_ => ProviderResult::empty(),
		}
	}
}

impl CalculatorProvider {
	fn get_result(&self, query: &str, result: f64) -> ProviderResult {
		let title = round(result, 15).to_string();

		// If the result is the same as the query, e.g. just a single number,
		// then don't return a result.
		if query.trim() == title {
			return ProviderResult::empty();
		}

		let hit = SimpleHit::new(title, self.config.subtitle.clone(), do_copy).with_score(MAX_SCORE);

		ProviderResult::single(Arc::new(hit))
	}
}

fn do_copy(hit: &SimpleHit<()>, sender: &Sender<FrontendMessage>) {
	set_clipboard(hit.get_title()).ok();

	sender
		.send(FrontendMessage::Hide)
		.expect("receiver should live for the lifetime of the program");
}

fn round(number: f64, precision: u32) -> f64 {
	let factor = 10_u64.pow(precision) as f64;
	(number * factor).round() / factor
}

fn set_clipboard(str: &str) -> Result<(), Box<dyn Error>> {
	Clipboard::new()?.set_text(str)?;
	Ok(())
}

#[derive(Deserialize, Debug)]
struct Config {
	pub subtitle: String,
}
