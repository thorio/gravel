//! Calculator provider based on [`meval`].
//!
//! Whenever the input can be parsed as a mathematical expression, shows the
//! result as the first hit.
//!
//! Selecting the hit copies the calculated value to the system's clipboard.

use arboard::Clipboard;
use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MAX_SCORE, *};
use mexprp::Answer;
use serde::Deserialize;
use std::{
	cell::OnceCell,
	sync::{mpsc::Sender, Arc, Mutex},
};

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = plugin("calculator").with_provider(Box::new(get_provider));

	registry.register(definition);
}

fn get_provider(config: &PluginConfigAdapter) -> Box<dyn Provider> {
	let plugin_config = config.get::<Config>(DEFAULT_CONFIG);

	Box::new(CalculatorProvider {
		config: plugin_config,
		clipboard: OnceCell::new(),
	})
}

fn create_clipboard() -> Option<Arc<Mutex<Clipboard>>> {
	log::trace!("spawning clipboard instance");

	match Clipboard::new() {
		Err(err) => {
			log::error!("unable to initialize clipboard: {err}");
			None
		}
		Ok(clipboard) => Some(Arc::new(Mutex::new(clipboard))),
	}
}

struct CalculatorProvider {
	config: Config,
	clipboard: OnceCell<Option<Arc<Mutex<Clipboard>>>>,
}

impl CalculatorProvider {
	fn get_clipboard(&self) -> Option<Arc<Mutex<Clipboard>>> {
		self.clipboard.get_or_init(create_clipboard).clone()
	}
}

impl Provider for CalculatorProvider {
	fn query(&self, query: &str) -> ProviderResult {
		let query = query.trim();
		let result = eval(query);

		let Some(result) = result else {
			return ProviderResult::empty();
		};

		if query == result || matches!(query, "e" | "pi" | "i") {
			return ProviderResult::empty();
		}

		let clipboard = self.get_clipboard();

		let hit = SimpleHit::new(result, self.config.subtitle.clone(), move |h, s| {
			do_copy(clipboard.clone(), h, s)
		})
		.with_score(MAX_SCORE);

		ProviderResult::single(Arc::new(hit))
	}
}

fn eval(expression: &str) -> Option<String> {
	match mexprp::eval(expression) {
		Ok(Answer::Single(result)) => Some(result),
		Ok(Answer::Multiple(results)) => results.into_iter().next(),
		_ => None,
	}
	.map(|r| round(r, 10).to_string())
}

fn do_copy(clipboard: Option<Arc<Mutex<Clipboard>>>, hit: &SimpleHit, sender: &Sender<FrontendMessage>) {
	let value = hit.get_title();
	log::debug!("copying value to clipboard: {value}");

	if let Some(clipboard_mutex) = clipboard {
		let mut guard = clipboard_mutex.lock().expect("thread holding the mutex can't panic");
		guard.set_text(value).ok();
	}

	sender.send(FrontendMessage::Hide).ok();
}

fn round(number: f64, precision: u32) -> f64 {
	let factor = 10_u64.pow(precision) as f64;
	(number * factor).round() / factor
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
	#[case("1 / 20", "0.05")]
	#[case("2 ^ 10", "1024")]
	#[case("0.1 + 0.2", "0.3")]
	#[case("(2 + 3) * (3 - 5)", "-10")]
	#[case("-2 ^ 3", "-8")]
	#[case("round(2pi)", "6")]
	#[case("sqrt(2)", "1.4142135624")]
	#[case("sin(asin(0.5))", "0.5")]
	fn should_eval(#[case] expression: &str, #[case] expected: &str) {
		let actual = eval(expression);
		assert_eq!(Some(expected), actual.as_deref(), "{expression}");
	}

	#[rstest]
	#[case("clippy")]
	#[case("1 1")]
	#[case("1 / 0")]
	#[case("x + 5")]
	fn should_fail(#[case] expression: &str) {
		let actual = eval(expression);
		assert_eq!(None, actual, "{expression}");
	}
}
