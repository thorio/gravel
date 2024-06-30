use crate::performance::Stopwatch;
use crate::scoring;
use abi_stable::{external_types::crossbeam_channel::RSender, sabi_trait, std_types::RStr, traits::IntoReprRust};
use gravel_ffi::prelude::*;
use itertools::Itertools;

/// Holds a [`Provider`] and some additional metadata.
struct ProviderInfo {
	pub provider: BoxDynProvider,
	pub keyword: Option<String>,
}

pub struct QueryEngineImpl {
	providers: Vec<ProviderInfo>,
	sender: RSender<FrontendMessage>,
}

impl QueryEngine for QueryEngineImpl {
	fn query(&self, query: RStr<'_>) -> QueryResult where {
		let stopwatch = Stopwatch::start();

		let query = query.into_rust();

		if query.trim().is_empty() {
			return QueryResult::empty();
		}

		log::trace!("starting query '{query}'");

		if let Some(result) = self.try_keyword_query(query) {
			log::trace!("query complete, took {stopwatch}");
			return result;
		}

		let result = self.full_query(query);

		log::trace!("query complete, took {stopwatch}");
		result
	}

	fn run_hit_action(&self, hit: &ArcDynHit) {
		hit.action(&self.sender);
	}
}

impl From<QueryEngineImpl> for BoxDynQueryEngine {
	fn from(value: QueryEngineImpl) -> Self {
		Self::from_value(value, sabi_trait::TD_Opaque)
	}
}

/// Aggregates and scores hits from the given [`Provider`]s.
impl QueryEngineImpl {
	pub fn new(sender: RSender<FrontendMessage>) -> Self {
		Self {
			providers: vec![],
			sender,
		}
	}

	/// Adds the provider to the engine's collection.
	pub fn register(&mut self, provider: BoxDynProvider, keyword: Option<String>) -> &mut Self {
		let info = ProviderInfo { provider, keyword };

		self.providers.push(info);
		self
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
		let first_word = query.split(' ').next()?;

		let provider = self.check_keywords(first_word)?;

		// remove the keyword from the query
		let new_query = &query[first_word.len()..query.len()].trim_start();

		Some(inner_query(&[provider], new_query))
	}

	/// Tries to find a provider with the a keyword that matches the given string.
	fn check_keywords(&self, first_word: &str) -> Option<&ProviderInfo> {
		self.providers
			.iter()
			.find(|p| matches!(&p.keyword, Some(k) if k == first_word))
	}
}

/// Queries providers; aggregates, scores and orders [`Hit`]s.
fn inner_query(providers: &[&ProviderInfo], query: &str) -> QueryResult {
	let hits = providers
		.iter()
		.flat_map(|p| p.provider.query(query.into()).hits)
		.collect_vec();

	let hits = match query.trim() {
		"*" => scoring::to_unscored(hits),
		_ => scoring::to_scored(hits, query),
	};

	QueryResult::new(hits)
}
