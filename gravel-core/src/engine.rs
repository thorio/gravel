use crate::frontend::FrontendMessage;
use crate::provider::*;
use crate::scoring::*;
use std::sync::mpsc::Sender;

/// Holds a [`Provider`] and some additional metadata.
struct ProviderInfo {
	pub provider: Box<dyn Provider>,
	pub keyword: Option<String>,
}

pub struct QueryEngine {
	providers: Vec<ProviderInfo>,
	sender: Sender<FrontendMessage>,
}

/// Aggregates and scores hits from the given [`Provider`]s.
impl QueryEngine {
	pub fn new(sender: Sender<FrontendMessage>) -> Self {
		Self {
			providers: vec![],
			sender,
		}
	}

	/// Adds the provider to the engine's collection.
	pub fn register(&mut self, provider: Box<dyn Provider>, keyword: &Option<String>) -> &mut Self {
		let info = ProviderInfo {
			provider,
			keyword: keyword.clone(),
		};

		self.providers.push(info);
		self
	}

	/// Queries all providers with the given query.
	pub fn query(&self, query: &str) -> QueryResult {
		if query.trim().is_empty() {
			return QueryResult::empty();
		}

		if let Some(result) = self.try_keyword_query(query) {
			return result;
		}

		self.full_query(query)
	}

	pub fn run_hit_action(&self, hit: &dyn Hit) {
		hit.action(&self.sender);
	}

	/// Runs the query against all available providers.
	fn full_query(&self, query: &str) -> QueryResult {
		let providers = self
			.providers
			.iter()
			.filter(|provider| provider.keyword.is_none())
			.collect::<Vec<&ProviderInfo>>();

		inner_query(&providers, query)
	}

	/// Tries to find a provider with the a keyword that matches the query's.
	/// If one is found, the keyword is stripped from the query and the
	/// resulting new query is run against that provider only.
	fn try_keyword_query(&self, query: &str) -> Option<QueryResult> {
		let words = query.split(' ').collect::<Vec<&str>>();
		let first_word: &str = words.first().unwrap();

		if let Some(provider) = self.check_keywords(first_word) {
			let providers = vec![provider];

			// remove the keyword from the query
			let new_query = &query[first_word.len()..query.len()].trim_start();

			return Some(inner_query(&providers, new_query));
		}

		None
	}

	/// Tries to find a provider with the a keyword that matches the given string.
	fn check_keywords(&self, first_word: &str) -> Option<&ProviderInfo> {
		for provider in &self.providers {
			match provider.keyword.as_ref() {
				Some(keyword) if keyword == first_word => return Some(provider),
				_ => (),
			};
		}

		None
	}
}

/// Queries providers; aggregates, scores and orders [`Hit`]s.
fn inner_query(providers: &Vec<&ProviderInfo>, query: &str) -> QueryResult {
	let mut results = Vec::new();

	for provider in providers {
		let result = provider.provider.query(query);
		results.push(result);
	}

	let mut aggregate = aggregate_results(results);
	score_hits(query, &mut aggregate);
	trim_hits(&mut aggregate);
	order_hits(&mut aggregate);

	aggregate
}

/// Combines the hits from all given [`QueryResult`]s into a single, new result.
fn aggregate_results(results: Vec<QueryResult>) -> QueryResult {
	let mut hits = Vec::new();

	for mut result in results {
		hits.append(&mut result.hits);
	}

	QueryResult::new(hits)
}
