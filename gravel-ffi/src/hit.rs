use abi_stable::pointer_trait::ImmutableRef;
use abi_stable::std_types::{RArc, ROption, RStr, RString};
use abi_stable::{sabi_trait, RRef, StableAbi};
use std::fmt::Debug;

pub type ArcDynHit = Hit_TO<'static, RArc<()>>;

#[sabi_trait]
pub trait Hit: Sync + Send + Debug {
	fn title(&self) -> RStr<'_>;
	fn subtitle(&self) -> RStr<'_>;
	fn override_score(&self) -> ROption<u32>;
	fn action(&self, context: RefDynHitActionContext<'_>);
}

pub(crate) fn clone_hit_arc(hit: &ArcDynHit) -> ArcDynHit {
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
pub struct SimpleHit {
	pub title: RString,
	pub subtitle: RString,
	pub override_score: ROption<u32>,

	#[allow(clippy::type_complexity)]
	pub action: Box<dyn Fn(&SimpleHit, RefDynHitActionContext<'_>) + Send + Sync>,
}

impl Debug for SimpleHit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SimpleHit")
			.field("title", &self.title)
			.field("subtitle", &self.subtitle)
			.field("override_score", &self.override_score)
			.field("action", &format_args!("Fn@{:p}", self.action.to_raw_ptr()))
			.finish()
	}
}

impl SimpleHit {
	/// Creates a new instance without extra data.
	#[must_use]
	pub fn new(
		title: impl Into<RString>,
		subtitle: impl Into<RString>,
		func: impl Fn(&Self, RefDynHitActionContext<'_>) + Send + Sync + 'static,
	) -> Self {
		Self {
			title: title.into(),
			subtitle: subtitle.into(),
			override_score: ROption::RNone,
			action: Box::new(func),
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
	fn action(&self, context: RefDynHitActionContext<'_>) {
		(self.action)(self, context);
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

pub type RefDynHitActionContext<'a> = HitActionContext_TO<'static, RRef<'a, ()>>;

#[sabi_trait]
pub trait HitActionContext {
	fn hide_frontend(&self);
	fn refresh_frontend(&self);
	fn exit(&self);
	fn restart(&self);
}
