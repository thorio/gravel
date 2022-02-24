pub mod environment;
pub mod frontend;
pub mod plugin;
pub mod provider;
pub mod scoring;

use crate::frontend::*;
use crate::provider::*;
use crate::scoring::*;
use std::process::exit;

pub struct QueryEngine {
	providers: Vec<Box<dyn Provider>>,
}

impl QueryEngine {
	pub fn new(providers: Vec<Box<dyn Provider>>) -> Self {
		QueryEngine {
			providers: providers,
		}
	}

	pub fn query(&self, query: &str) -> QueryResult {
		if !query.trim().is_empty() {
			self.inner_query(query)
		} else {
			QueryResult::empty()
		}
	}

	pub fn run_hit_action(&self, _frontend: Box<&dyn Frontend>, hit: &Box<dyn Hit>) {
		hit.action();
		exit(0);
	}

	fn inner_query(&self, query: &str) -> QueryResult {
		let mut results = Vec::new();

		for provider in &self.providers {
			let result = provider.query(query);
			results.push(result);
		}

		let mut aggregate = aggregate_results(results);
		score_hits(query, &mut aggregate);
		trim_hits(&mut aggregate);
		order_hits(&mut aggregate);

		aggregate
	}
}

fn aggregate_results(results: Vec<QueryResult>) -> QueryResult {
	let mut hits = Vec::new();

	for mut result in results {
		hits.append(&mut result.hits);
	}

	QueryResult::new(hits)
}
