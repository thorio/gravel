use itertools::Itertools;

use crate::frontend::FrontendMessage;
use crate::performance::Stopwatch;
use crate::scoring::ScoredHit;
use crate::{provider::*, scoring};
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

pub struct QueryResult {
	pub hits: Vec<ScoredHit>,
}

impl QueryResult {
	pub fn new(hits: Vec<ScoredHit>) -> Self {
		Self { hits }
	}

	pub fn empty() -> Self {
		Self::new(vec![])
	}
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
	pub fn register(&mut self, provider: Box<dyn Provider>, keyword: Option<String>) -> &mut Self {
		let info = ProviderInfo { provider, keyword };

		self.providers.push(info);
		self
	}

	/// Queries all providers with the given query.
	pub fn query(&self, query: &str) -> QueryResult {
		let stopwatch = Stopwatch::start();

		if query.trim().is_empty() {
			return QueryResult { hits: vec![] };
		}

		log::trace!("starting query '{query}'");

		if let Some(result) = self.try_keyword_query(query) {
			return result;
		}

		let result = self.full_query(query);

		log::trace!("query complete, took {stopwatch}");
		result
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
		let Some(first_word) = query.split(' ').next() else {
			return None;
		};

		let Some(provider) = self.check_keywords(first_word) else {
			return None;
		};

		// remove the keyword from the query
		let new_query = &query[first_word.len()..query.len()].trim_start();

		Some(inner_query(&[provider], new_query))
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
fn inner_query(providers: &[&ProviderInfo], query: &str) -> QueryResult {
	let hits = providers
		.iter()
		.flat_map(|p| p.provider.query(query).hits)
		.collect_vec();

	let hits = scoring::get_scored_hits(hits, query);

	QueryResult { hits }
}
