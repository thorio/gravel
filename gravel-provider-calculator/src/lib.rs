use clipboard::{ClipboardContext, ClipboardProvider};
use gravel_core::{frontend::ControlMessage, provider::*, scoring::*};
use meval::eval_str;
use std::sync::mpsc::Sender;

pub struct CalculatorProvider {}

impl CalculatorProvider {
	pub fn new() -> Self {
		CalculatorProvider {}
	}
}

impl Provider for CalculatorProvider {
	fn query(&self, query: &str) -> QueryResult {
		let result = eval_str(query);

		if result.is_ok() {
			QueryResult::single(get_hit(result.unwrap()))
		} else {
			QueryResult::empty()
		}
	}
}

fn get_hit(result: f64) -> Box<dyn Hit> {
	let title = round(result, 15).to_string();

	let hitdata = HitData::new(&title, "Copy to clipboard").with_score(MAX_SCORE);

	Box::new(SimpleHit::new(hitdata, set_clipboard))
}

fn round(number: f64, precision: u32) -> f64 {
	let factor = (10 as u64).pow(precision) as f64;
	(number * factor).round() / factor
}

fn set_clipboard(hit: &SimpleHit<()>, sender: &Sender<ControlMessage>) {
	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
	ctx.set_contents(hit.get_data().title.clone()).unwrap();

	sender.send(ControlMessage::Hide);
}
