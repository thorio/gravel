use crate::performance::Stopwatch;
use crate::scoring;
use abi_stable::std_types::RString;
use abi_stable::{external_types::crossbeam_channel::RSender, sabi_trait, std_types::RStr, traits::IntoReprRust};
use gravel_ffi::{ArcDynHit, FrontendMessage, HitActionContext, RefDynHitActionContext};
use gravel_ffi::{BoxDynFrontendContext, BoxDynProvider, FrontendContext, FrontendMessageNe, QueryResult};
use itertools::Itertools;

/// Holds a [`Provider`] and some additional metadata.
struct ProviderInfo {
	pub provider: BoxDynProvider,
	pub keyword: Option<String>,
}

/// Aggregates and scores hits from the given [`gravel_ffi::Provider`]s.
pub struct QueryEngine {
	providers: Vec<ProviderInfo>,
	action_context: ActionContext,
}

/// For now the [`QueryEngine`] _is_ the [`FrontendContext`],
/// but this can later be changed without breaking the interface
impl FrontendContext for QueryEngine {
	fn query(&self, query: RStr<'_>) -> QueryResult {
		let stopwatch = Stopwatch::start();

		let query = query.into_rust();

		if query.trim().is_empty() {
			return QueryResult::default();
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
		hit.action((&self.action_context).into());
	}

	fn run_secondary_hit_action(&self, hit: &ArcDynHit) {
		hit.secondary_action((&self.action_context).into());
	}
}

impl From<QueryEngine> for BoxDynFrontendContext {
	fn from(value: QueryEngine) -> Self {
		Self::from_value(value, sabi_trait::TD_Opaque)
	}
}

impl QueryEngine {
	pub fn new(sender: RSender<FrontendMessageNe>) -> Self {
		Self {
			providers: vec![],
			action_context: ActionContext::new(sender),
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
	/// resulting new query is ran against that provider only.
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
