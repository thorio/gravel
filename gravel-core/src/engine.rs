use crate::frontend::FrontendMessage;
use crate::provider::*;
use crate::scoring::*;
use std::sync::mpsc::Sender;

pub struct QueryEngine {
	providers: Vec<Box<dyn Provider>>,
	sender: Sender<FrontendMessage>,
}

impl QueryEngine {
	pub fn new(providers: Vec<Box<dyn Provider>>, sender: Sender<FrontendMessage>) -> Self {
		QueryEngine {
			providers: providers,
			sender: sender,
		}
	}

	pub fn query(&self, query: &str) -> QueryResult {
		if !query.trim().is_empty() {
			self.inner_query(query)
		} else {
			QueryResult::empty()
		}
	}

	pub fn run_hit_action(&self, hit: &Box<dyn Hit>) {
		hit.action(&self.sender);
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
