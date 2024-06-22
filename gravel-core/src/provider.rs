use crate::frontend::FrontendMessage;
use nameof::name_of;
use std::fmt::{Debug, Formatter, Result};
use std::sync::{mpsc::Sender, Arc};

/// A provider takes a query and provides some relevant results.
pub trait Provider {
	fn query(&self, query: &str) -> ProviderResult;
}

/// A collection of hits.
#[derive(Debug)]
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
pub trait Hit: Sync + Send + Debug {
	fn get_title(&self) -> &str;
	fn get_subtitle(&self) -> &str;
	fn get_override_score(&self) -> Option<u32>;
	fn action(&self, sender: &Sender<FrontendMessage>);
}

pub type SimpleHitAction = Box<dyn Fn(&SimpleHit, &Sender<FrontendMessage>) + Send + Sync>;

/// Reference implementation for [`Hit`].
///
/// Takes a function for an action and can store extra data.
pub struct SimpleHit {
	title: Box<str>,
	subtitle: Box<str>,
	override_score: Option<u32>,

	action_func: SimpleHitAction,
}

impl SimpleHit {
	/// Creates a new instance without extra data.
	#[must_use]
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

// name_of! on types doesn't work with Self
#[allow(clippy::use_self)]
impl Debug for SimpleHit {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
		fmt.debug_struct(name_of!(type SimpleHit))
			.field(name_of!(title in Self), &self.title)
			.field(name_of!(subtitle in Self), &self.subtitle)
			.field(name_of!(override_score in Self), &self.override_score)
			.finish_non_exhaustive()
	}
}

impl Hit for SimpleHit {
	fn action(&self, sender: &Sender<FrontendMessage>) {
		(self.action_func)(self, sender);
	}

	#[must_use]
	fn get_title(&self) -> &str {
		&self.title
	}

	#[must_use]
	fn get_subtitle(&self) -> &str {
		&self.subtitle
	}

	#[must_use]
	fn get_override_score(&self) -> Option<u32> {
		self.override_score
	}
}
