use crate::fns::RBoxFn;
use crate::frontend::FrontendMessage;
use abi_stable::external_types::crossbeam_channel::RSender;
use abi_stable::sabi_trait;
use abi_stable::std_types::{RArc, ROption, RStr, RString};
use abi_stable::StableAbi;
use std::fmt::Debug;

pub type ArcDynHit = Hit_TO<'static, RArc<()>>;

#[sabi_trait]
pub trait Hit: Sync + Send + Debug {
	fn title(&self) -> RStr<'_>;
	fn subtitle(&self) -> RStr<'_>;
	fn override_score(&self) -> ROption<u32>;
	// TODO factor the sender out, provide a nicer interface
	fn action(&self, sender: &RSender<FrontendMessage>);
}

pub fn clone_hit_ptr(hit: &ArcDynHit) -> ArcDynHit {
	ArcDynHit::from_sabi(hit.obj.shallow_clone())
}

#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct ScoredHit {
	pub hit: ArcDynHit,
	pub score: u32,
}

impl ScoredHit {
	pub fn from(hit: ArcDynHit, score: u32) -> Self {
		Self { hit, score }
	}
}

#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct SimpleHit {
	pub title: RString,
	pub subtitle: RString,
	pub override_score: ROption<u32>,
	pub action: RBoxFn<Self, RSender<FrontendMessage>, ()>,
}

impl SimpleHit {
	/// Creates a new instance without extra data.
	#[must_use]
	pub fn new(
		title: impl Into<RString>,
		subtitle: impl Into<RString>,
		func: impl Fn(&Self, &RSender<FrontendMessage>) + Send + Sync + 'static,
	) -> Self {
		Self {
			title: title.into(),
			subtitle: subtitle.into(),
			override_score: ROption::RNone,
			action: func.into(),
		}
	}

	#[must_use]
	pub fn with_score(mut self, score: u32) -> Self {
		self.override_score = ROption::RSome(score);
		self
	}
}

impl From<SimpleHit> for ArcDynHit {
	fn from(value: SimpleHit) -> Self {
		Self::from_ptr(RArc::new(value), sabi_trait::TD_Opaque)
	}
}

impl Hit for SimpleHit {
	fn action(&self, sender: &RSender<FrontendMessage>) {
		self.action.call(self, sender);
	}

	#[must_use]
	fn title(&self) -> RStr<'_> {
		self.title.as_rstr()
	}

	#[must_use]
	fn subtitle(&self) -> RStr<'_> {
		self.subtitle.as_rstr()
	}

	#[must_use]
	fn override_score(&self) -> ROption<u32> {
		self.override_score
	}
}
