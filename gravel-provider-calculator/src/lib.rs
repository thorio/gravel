//! Calculator provider based on [`meval`].
//!
//! Whenever the input can be parsed as a mathematical expression, shows the
//! result as the first hit.
//!
//! Selecting the hit copies the calculated value to the system's clipboard.

use clipboard::{ClipboardContext, ClipboardProvider};
use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MAX_SCORE, *};
use meval::eval_str;
use serde::Deserialize;
use std::sync::mpsc::Sender;

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
	fn query(&self, query: &str) -> QueryResult {
		let result = eval_str(query);

		match result {
			Ok(result) => QueryResult::single(get_hit(&self.config, result)),
			Err(_err) => QueryResult::empty(),
		}
	}
}

fn get_hit(config: &Config, result: f64) -> Box<dyn Hit> {
	let title = round(result, 15).to_string();

	let hitdata = HitData::new(&title, &config.subtitle).with_score(MAX_SCORE);

	Box::new(SimpleHit::new(hitdata, set_clipboard))
}

fn round(number: f64, precision: u32) -> f64 {
	let factor = 10_u64.pow(precision) as f64;
	(number * factor).round() / factor
}

fn set_clipboard(hit: &SimpleHit<()>, sender: &Sender<FrontendMessage>) {
	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
	ctx.set_contents(hit.get_data().title.clone()).unwrap();

	sender.send(FrontendMessage::Hide).unwrap();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub subtitle: String,
}
