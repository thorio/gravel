use crate::frontend::FrontendMessage;
use std::sync::{mpsc::Sender, Arc};

/// A provider takes a query and provides some relevant results.
pub trait Provider {
	fn query(&self, query: &str) -> ProviderResult;
}

/// A collection of hits.
pub struct ProviderResult {
	pub hits: Vec<Arc<dyn Hit>>,
}

impl ProviderResult {
	#[must_use]
	pub fn new(hits: Vec<Arc<dyn Hit>>) -> Self {
		Self { hits }
	}

	#[must_use]
	pub fn empty() -> Self {
		Self::new(vec![])
	}

	#[must_use]
	pub fn single(hit: Arc<dyn Hit>) -> Self {
		Self::new(vec![hit])
	}
}

/// A hit holds information about how to display a query result, as well
/// as an action to take if the hit is selected.
///
/// The hit can be given a score, in which case it will not be further
/// scored and simply ordered as-is.
pub trait Hit: Sync + Send {
	fn get_title(&self) -> &str;
	fn get_subtitle(&self) -> &str;
	fn get_override_score(&self) -> Option<u32>;
	fn action(&self, sender: &Sender<FrontendMessage>);
}

/// Reference implementation for [`Hit`].
///
/// Takes a function for an action and can store extra data.
pub struct SimpleHit {
	title: Box<str>,
	subtitle: Box<str>,
	override_score: Option<u32>,

	// I think inlining it is easier to read in this case, due to T.
	#[allow(clippy::type_complexity)]
	action_func: Box<dyn Fn(&Self, &Sender<FrontendMessage>) + Send + Sync>,
}

impl SimpleHit {
	/// Creates a new instance without extra data.
	pub fn new(
		title: impl Into<Box<str>>,
		subtitle: impl Into<Box<str>>,
		func: impl Fn(&Self, &Sender<FrontendMessage>) + Send + Sync + 'static,
	) -> Self {
		Self {
			title: title.into(),
			subtitle: subtitle.into(),
			override_score: None,
			action_func: Box::new(func),
		}
	}

	#[must_use]
	pub fn with_score(mut self, score: u32) -> Self {
		self.override_score = Some(score);
		self
	}
}

impl Hit for SimpleHit {
	fn action(&self, sender: &Sender<FrontendMessage>) {
		(self.action_func)(self, sender);
	}

	fn get_title(&self) -> &str {
		&self.title
	}

	fn get_subtitle(&self) -> &str {
		&self.subtitle
	}

	fn get_override_score(&self) -> Option<u32> {
		self.override_score
	}
}
