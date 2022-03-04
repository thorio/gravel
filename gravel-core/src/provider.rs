use crate::frontend::FrontendMessage;
use std::sync::mpsc::Sender;

/// A provider takes a query and provides some relevant results.
pub trait Provider {
	fn query(&self, query: &str) -> QueryResult;
}

/// A hit holds information about how to display a query result, as well
/// as an action to take if the hit is selected.
///
/// The hit can be given a score, in which case it will not be further
/// scored and simply ordered as-is.
pub trait Hit {
	fn get_data(&self) -> &HitData;
	fn action(&self, sender: &Sender<FrontendMessage>);

	/// Assign a pre-determined score to influence the ordering of this hit.
	///
	/// Usually, this is only set to either [`MAX_SCORE`](`crate::scoring::MAX_SCORE`)
	/// or [`MIN_SCORE`](`crate::scoring::MIN_SCORE`), to "pin" it to the top
	/// or bottom of the results.
	fn set_score(&mut self, score: u32);
}

/// A collection of hits.
pub struct QueryResult {
	pub hits: Vec<Box<dyn Hit>>,
}

impl QueryResult {
	pub fn new(hits: Vec<Box<dyn Hit>>) -> Self {
		QueryResult { hits: hits }
	}

	pub fn empty() -> Self {
		Self::new(Vec::new())
	}

	pub fn single(hit: Box<dyn Hit>) -> Self {
		Self::new(vec![hit])
	}
}

/// Holds common data that must be present on all hits.
pub struct HitData {
	pub title: String,
	pub subtitle: String,
	pub score: u32,
	pub scored: bool,
}

impl HitData {
	pub fn empty() -> Self {
		Self::new("", "")
	}

	pub fn new(title: &str, subtitle: &str) -> Self {
		HitData {
			title: title.to_owned(),
			subtitle: subtitle.to_owned(),
			score: 0,
			scored: false,
		}
	}

	pub fn with_score(mut self, score: u32) -> Self {
		self.score = score;
		self.scored = true;

		self
	}
}

/// Reference implementation for [`Hit`].
///
/// Takes a function for an action and can store extra data.
pub struct SimpleHit<T> {
	data: HitData,
	extra_data: T,
	action_func: Box<dyn Fn(&Self, &Sender<FrontendMessage>)>,
}

impl SimpleHit<()> {
	/// Creates a new instance without extra data.
	pub fn new(data: HitData, func: impl Fn(&Self, &Sender<FrontendMessage>) + 'static) -> Self {
		SimpleHit {
			data: data,
			extra_data: (),
			action_func: Box::new(func),
		}
	}
}

impl<T> SimpleHit<T> {
	/// Creates a new instance with extra data.
	pub fn new_extra(data: HitData, extra_data: T, func: impl Fn(&Self, &Sender<FrontendMessage>) + 'static) -> Self {
		SimpleHit {
			data: data,
			extra_data: extra_data,
			action_func: Box::new(func),
		}
	}

	pub fn get_extra_data(&self) -> &T {
		&self.extra_data
	}
}

impl<T> Hit for SimpleHit<T> {
	fn get_data(&self) -> &HitData {
		&self.data
	}

	fn action(&self, sender: &Sender<FrontendMessage>) {
		(self.action_func)(self, sender)
	}

	fn set_score(&mut self, score: u32) {
		self.data.score = score;
	}
}
