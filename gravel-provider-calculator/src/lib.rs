//! Calculator provider based on [`mexprp`].
//!
//! Whenever the input can be parsed as a mathematical expression, shows the
//! result as the first hit.
//!
//! Selecting the hit copies the calculated value to the system's clipboard.

use arboard::Clipboard;
use gravel_ffi::prelude::*;
use mexprp::Answer;
use serde::Deserialize;
use std::cell::OnceCell;
use std::sync::{Arc, Mutex};

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

struct CalculatorProvider {
	config: Config,
	clipboard: OnceCell<Option<Arc<Mutex<Clipboard>>>>,
}

impl CalculatorProvider {
	fn get_clipboard(&self) -> Option<Arc<Mutex<Clipboard>>> {
		self.clipboard.get_or_init(create_clipboard).clone()
	}
}

#[gravel_provider("calculator")]
impl Provider for CalculatorProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		Self {
			config: config.get(DEFAULT_CONFIG),
			clipboard: OnceCell::new(),
		}
	}

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

		let hit = SimpleHit::new(result, self.config.subtitle.clone(), move |hit, ctx| {
			do_copy(clipboard.clone(), hit.title().as_str(), ctx);
		})
		.with_score(MAX_SCORE);

		ProviderResult::single(hit)
	}
}

fn create_clipboard() -> Option<Arc<Mutex<Clipboard>>> {
	log::trace!("spawning clipboard instance");

	Clipboard::new()
		.inspect_err(|e| log::error!("unable to initialize clipboard: {e}"))
		.ok()
		.map(|c| Arc::new(Mutex::new(c)))
}

fn eval(expression: &str) -> Option<String> {
	match mexprp::eval(expression) {
		Ok(Answer::Single(result)) => Some(result),
		Ok(Answer::Multiple(results)) => results.into_iter().next(),
		_ => None,
	}
	.map(|r| round(r, 10).to_string())
}

fn do_copy(clipboard: Option<Arc<Mutex<Clipboard>>>, result: &str, context: RefDynHitActionContext<'_>) {
	let Some(clipboard_mutex) = clipboard else {
		return;
	};

	log::debug!("copying value to clipboard: {result}");

	let mut guard = clipboard_mutex.lock().expect("thread holding the mutex can't panic");
	guard
		.set_text(result)
		.inspect_err(|e| log::error!("unable to set clipboard: {e}"))
		.ok();

	context.hide_frontend();
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
