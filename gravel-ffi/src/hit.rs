use crate::fns::RBoxFn;
use crate::FrontendMessage;
use abi_stable::external_types::crossbeam_channel::RSender;
use abi_stable::sabi_trait;
use abi_stable::std_types::{RArc, ROption, RStr, RString};
use abi_stable::StableAbi;
use std::fmt::Debug;

pub type ArcDynHit = Hit_TO<'static, RArc<()>>;

// can't implement directly because Hit_TO is generated into a different module
pub trait HitExt: Hit + 'static {
	fn into_dyn(self) -> ArcDynHit
	where
		Self: Sized,
	{
		ArcDynHit::from_ptr(RArc::new(self), sabi_trait::TD_Opaque)
	}
}

impl<T: Hit + 'static> HitExt for T {}

// can't implement directly because Hit_TO is generated into a different module
pub trait CloneHit {
	fn clone(&self) -> ArcDynHit;
}

impl CloneHit for ArcDynHit {
	fn clone(&self) -> ArcDynHit {
		Self::from_sabi(self.obj.shallow_clone())
	}
}

#[sabi_trait]
pub trait Hit: Sync + Send + Debug {
	fn get_title(&self) -> RStr<'_>;
	fn get_subtitle(&self) -> RStr<'_>;
	fn get_override_score(&self) -> ROption<u32>;
	fn action(&self, sender: &RSender<FrontendMessage>);
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
	pub action: RBoxFn<RSender<FrontendMessage>, ()>,
}

impl SimpleHit {
	/// Creates a new instance without extra data.
	#[must_use]
	pub fn new(
		title: impl Into<RString>,
		subtitle: impl Into<RString>,
		func: impl Fn(&RSender<FrontendMessage>) + Send + Sync + 'static,
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

impl Hit for SimpleHit {
	fn action(&self, sender: &RSender<FrontendMessage>) {
		self.action.call(sender);
	}

	#[must_use]
	fn get_title(&self) -> RStr<'_> {
		self.title.as_rstr()
	}

	#[must_use]
	fn get_subtitle(&self) -> RStr<'_> {
		self.subtitle.as_rstr()
	}

	#[must_use]
	fn get_override_score(&self) -> ROption<u32> {
		self.override_score
	}
}
