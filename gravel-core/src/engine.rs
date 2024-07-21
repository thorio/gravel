use crate::performance::Stopwatch;
use crate::scoring;
use abi_stable::std_types::RString;
use abi_stable::{external_types::crossbeam_channel::RSender, sabi_trait, std_types::RStr, traits::IntoReprRust};
use gravel_ffi::{ActionKind, ArcDynHit, FrontendMessage, HitActionContext, ProviderResult, RefDynHitActionContext};
use gravel_ffi::{BoxDynProvider, FrontendMessageNe, QueryResult};
use itertools::Itertools;
use std::iter::once;

/// Holds a [`Provider`] and some additional metadata.
struct ProviderInfo {
	pub name: String,
	pub provider: BoxDynProvider,
	pub keyword: Option<String>,
}

/// Aggregates and scores hits from the given [`gravel_ffi::Provider`]s.
pub struct QueryEngine {
	providers: Vec<ProviderInfo>,
	action_context: ActionContext,
}

impl QueryEngine {
	pub fn query(&self, query: RStr<'_>) -> QueryResult {
		fn inner(engine: &QueryEngine, query: &str) -> QueryResult {
			if let Some(result) = engine.try_keyword_query(query) {
				return result;
			}

			engine.full_query(query)
		}

		let query = query.into_rust();

		if query.trim().is_empty() {
			return QueryResult::default();
		}

		let stopwatch = Stopwatch::start();

		let result = inner(self, query);

		log::trace!("query took {stopwatch}");
		result
	}

	pub fn run_hit_action(&self, hit: &ArcDynHit, kind: ActionKind) {
		let context = (&self.action_context).into();

		match kind {
			ActionKind::Primary => hit.action(context),
			ActionKind::Secondary => hit.secondary_action(context),
		}
	}

	pub fn new(sender: RSender<FrontendMessageNe>) -> Self {
		Self {
			providers: vec![],
			action_context: ActionContext::new(sender),
		}
	}

	/// Adds the provider to the engine's collection.
	pub fn register(&mut self, name: String, provider: BoxDynProvider, keyword: Option<String>) -> &mut Self {
		let info = ProviderInfo {
			name,
			provider,
			keyword,
		};

		self.providers.push(info);
		self
	}

	/// Runs the query against all available providers.
	fn full_query(&self, query: &str) -> QueryResult {
		let providers = self.providers.iter().filter(|provider| provider.keyword.is_none());

		query_all(providers, query)
	}

	/// Tries to find a provider with the a keyword that matches the query's.
	/// If one is found, the keyword is stripped from the query and the
	/// resulting new query is ran against that provider only.
	fn try_keyword_query(&self, query: &str) -> Option<QueryResult> {
		let first_word = query.split(' ').next()?;

		let provider = self.check_keywords(first_word)?;

		// remove the keyword from the query
		let new_query = &query[first_word.len()..query.len()].trim_start();

		Some(query_all(once(provider), new_query))
	}

	/// Tries to find a provider with the a keyword that matches the given string.
	fn check_keywords(&self, first_word: &str) -> Option<&ProviderInfo> {
		self.providers
			.iter()
			.find(|p| matches!(&p.keyword, Some(k) if k == first_word))
	}
}

/// Queries providers; aggregates, scores and orders [`Hit`]s.
#[allow(single_use_lifetimes)]
fn query_all<'a>(providers: impl Iterator<Item = &'a ProviderInfo>, query: &str) -> QueryResult {
	let hits = providers.flat_map(|p| query_one(p, query).hits).collect_vec();

	let hits = match query.trim() {
		"*" => scoring::to_unscored(hits),
		_ => scoring::to_scored(hits, query),
	};

	QueryResult::new(hits)
}

fn query_one(info: &ProviderInfo, query: &str) -> ProviderResult {
	let stopwatch = Stopwatch::start();

	let result = info.provider.query(query.into());

	log::trace!(
		"query for provider '{}' took {stopwatch} and produced {} hits",
		info.name,
		result.hits.len()
	);

	result
}

struct ActionContext {
	sender: RSender<FrontendMessageNe>,
}

impl ActionContext {
	pub fn new(sender: RSender<FrontendMessageNe>) -> Self {
		Self { sender }
	}

	fn send(&self, message: FrontendMessage) {
		self.sender
			.send(FrontendMessageNe::new(message))
			.inspect_err(|e| log::error!("unable to frontend message from hit action: {e}"))
			.ok();
	}
}

impl<'a> From<&'a ActionContext> for RefDynHitActionContext<'a> {
	fn from(value: &'a ActionContext) -> Self {
		Self::from_ptr(value, sabi_trait::TD_Opaque)
	}
}

impl HitActionContext for ActionContext {
	fn hide_frontend(&self) {
		self.send(FrontendMessage::Hide);
	}

	fn refresh_frontend(&self) {
		self.send(FrontendMessage::Refresh);
	}

	fn exit(&self) {
		self.send(FrontendMessage::Exit);
	}

	fn restart(&self) {
		self.send(FrontendMessage::Restart);
	}

	fn set_query(&self, query: RString) {
		self.send(FrontendMessage::ShowWithQuery(query));
	}
}
