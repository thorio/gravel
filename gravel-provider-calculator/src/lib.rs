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
		let result = eval(query);

		let Some(result) = result else {
			return ProviderResult::empty();
		};

		// If the result is the same as the query, e.g. just a single number,
		// then don't return a result.
		if query.trim() == result {
			return ProviderResult::empty();
		}

		let hit = SimpleHit::new(result, self.config.subtitle.clone(), do_copy).with_score(MAX_SCORE);

		ProviderResult::single(Arc::new(hit))
	}
}

fn eval(expression: &str) -> Option<String> {
	match evalexpr::eval(expression) {
		Ok(Value::Float(result)) => Some(result),
		Ok(Value::Int(result)) => Some(result as f64),
		_ => None,
	}
	.map(|r| round(r, 15).to_string())
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

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case("1", "1")]
	#[case("1 + 1", "2")]
	#[case("1 - 1", "0")]
	#[case("1 / 1", "1")]
	#[case("1 * 1", "1")]
	#[case("3 * 0.2", "0.6")]
	#[case("2 ^ 10", "1024")]
	fn should_eval(#[case] expression: &str, #[case] expected: &str) {
		let actual = eval(expression);
		assert_eq!(Some(expected), actual.as_deref(), "{expression}");
	}

	#[rstest]
	#[case("clippy")]
	#[case("1 1")]
	#[case("1 / 0")]
	#[case("x + 5")]
	fn should_err(#[case] expression: &str) {
		let actual = eval(expression);
		assert_eq!(None, actual, "{expression}");
	}
}
