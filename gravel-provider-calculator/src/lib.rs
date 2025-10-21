//! Calculator provider based on [`mexprp`].
//!
//! Whenever the input can be parsed as a mathematical expression, shows the
//! result as the first hit.
//!
//! Selecting the hit copies the calculated value to the system's clipboard.

use abi_stable::reexports::SelfOps;
use gravel_ffi::prelude::*;
use mexprp::Answer;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

struct CalculatorProvider {
	config: Config,
}

#[gravel_provider("calculator")]
impl Provider for CalculatorProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		Self {
			config: config.get(DEFAULT_CONFIG),
		}
	}

	fn query(&self, query: &str) -> ProviderResult {
		let query = query.trim();

		eval(query)
			.filter(|r| !query_was_const(query, r))
			.map(|r| self.get_hit(r))
			.piped(ProviderResult::from_option)
	}
}

impl CalculatorProvider {
	fn get_hit(&self, result: String) -> SimpleHit {
		SimpleHit::new(result, self.config.subtitle.clone(), move |hit, ctx| {
			ctx.set_clipboard_text(hit.title().to_string().into_c());
			ctx.hide_frontend();
		})
		.with_secondary(|hit, ctx| {
			ctx.set_query(hit.title().as_str().to_owned().into_c());
		})
		.with_score(MAX_SCORE)
	}
}

// queries that do not require any calculation should be ignored
fn query_was_const(query: &str, result: &str) -> bool {
	query == result || matches!(query, "e" | "pi" | "i")
}

fn eval(expression: &str) -> Option<String> {
	match mexprp::eval(expression) {
		Ok(Answer::Single(result)) => Some(result),
		Ok(Answer::Multiple(results)) => results.into_iter().next(),
		_ => None,
	}
	.map(|r| round(r, 10).to_string())
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
