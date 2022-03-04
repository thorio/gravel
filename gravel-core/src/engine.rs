use crate::frontend::FrontendMessage;
use crate::provider::*;
use crate::scoring::*;
use std::sync::mpsc::Sender;

pub struct QueryEngine {
	providers: Vec<Box<dyn Provider>>,
	sender: Sender<FrontendMessage>,
}

/// Aggregates and scores hits from the given [`Provider`]s.
impl QueryEngine {
	pub fn new(providers: Vec<Box<dyn Provider>>, sender: Sender<FrontendMessage>) -> Self {
		QueryEngine {
			providers: providers,
			sender: sender,
		}
	}

	/// Queries all providers with the given query.
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

	/// Queries providers; aggregates, scores and orders [`Hit`]s.
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

/// Combines the hits from all given [`QueryResult`]s into a single, new result.
fn aggregate_results(results: Vec<QueryResult>) -> QueryResult {
	let mut hits = Vec::new();

	for mut result in results {
		hits.append(&mut result.hits);
	}

	QueryResult::new(hits)
}
