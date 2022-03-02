use crate::frontend::FrontendMessage;
use std::sync::mpsc::Sender;

pub trait Provider {
	fn query(&self, query: &str) -> QueryResult;
}

pub trait Hit {
	fn get_data(&self) -> &HitData;
	fn action(&self, sender: &Sender<FrontendMessage>);
	fn set_score(&mut self, score: u32);
}

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

pub struct SimpleHit<T> {
	data: HitData,
	extra_data: T,
	action_func: Box<dyn Fn(&Self, &Sender<FrontendMessage>)>,
}

impl SimpleHit<()> {
	pub fn new(data: HitData, func: impl Fn(&Self, &Sender<FrontendMessage>) + 'static) -> Self {
		SimpleHit {
			data: data,
			extra_data: (),
			action_func: Box::new(func),
		}
	}
}

impl<T> SimpleHit<T> {
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
