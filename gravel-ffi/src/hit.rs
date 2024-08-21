use abi_stable::pointer_trait::ImmutableRef;
use abi_stable::std_types::{RArc, ROption, RStr, RString};
use abi_stable::{sabi_trait, traits::IntoReprC, RRef, StableAbi};
use std::fmt::Debug;

/// The maximum score a [`Hit`] can have.
pub const MAX_SCORE: u32 = u32::MAX;

/// The minimum score a [`Hit`] can have.
pub const MIN_SCORE: u32 = u32::MIN;

/// FFI-safe [`Hit`] trait object.
///
/// It uses [`RArc`] to facilitate caching hits.
pub type ArcDynHit = Hit_TO<'static, RArc<()>>;

/// This is the trait for hits returned from [`crate::Provider`]s.
///
/// It must contain some information about the hit, as well as
/// a function to execute when it is selected.
///
/// For most situations, a [`SimpleHit`] is sufficient.
#[sabi_trait]
pub trait Hit: Sync + Send + Debug {
	fn title(&self) -> RStr<'_>;

	fn subtitle(&self) -> RStr<'_> {
		"".into_c()
	}

	/// If [`ROption::RSome`], skips normal scoring for this hit,
	/// instead using the contained score as-is.
	///
	/// This is useful for pinning a hit to the top or bottom of the results,
	/// but probably not very useful for actual scoring, as the underlying scoring
	/// implementation used in gravel may change.
	fn override_score(&self) -> ROption<u32> {
		ROption::RNone
	}

	fn action(&self, context: RefDynHitActionContext<'_>);

	/// Secondary action for related functionality.
	///
	/// How this is triggered and what it does is plugin-defined. By default,
	/// it calls the regular action.
	fn secondary_action(&self, context: RefDynHitActionContext<'_>) {
		log::trace!("no secondary action set for hit '{}'", self.title());

		self.action(context);
	}
}

/// Clones an [`ArcDynHit`], as this is not straightforward.
///
/// Like [`std::sync::Arc`], cloning just increments the reference counter.
pub(crate) fn clone_hit_arc(hit: &ArcDynHit) -> ArcDynHit {
	ArcDynHit::from_sabi(hit.obj.shallow_clone())
}

#[repr(C)]
#[derive(StableAbi, Debug, PartialEq, Eq, Copy, Clone)]
pub enum ActionKind {
	Primary,
	Secondary,
}

/// Wraps an [`ArcDynHit`] with scoring metadata.
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

/// FFI-safe reference to a [`HitActionContext`] trait object.
pub type RefDynHitActionContext<'a> = HitActionContext_TO<'static, RRef<'a, ()>>;

/// Abstracts interaction between a hit action and the frontend.
#[sabi_trait]
pub trait HitActionContext {
	/// Asks the frontend to hide.
	fn hide_frontend(&self);

	/// Asks the frontend to query again.
	///
	/// Useful if the plugin just did something that changes the results of the next query.
	fn refresh_frontend(&self);

	/// Exits the whole application.
	fn exit(&self);

	/// Restarts the whole application.
	fn restart(&self);

	/// Sets the query and runs it.
	fn set_query(&self, query: RString);

	/// Clears caches in the entire application.
	fn clear_caches(&self);
}

type SimpleHitAction = Box<dyn Fn(&SimpleHit, RefDynHitActionContext<'_>) + Send + Sync>;

/// Standard implementation of [`Hit`] using closures.
#[repr(C)]
pub struct SimpleHit {
	pub title: RString,
	pub subtitle: RString,
	pub override_score: ROption<u32>,

	#[allow(clippy::type_complexity)]
	pub action: SimpleHitAction,
	pub secondary_action: Option<SimpleHitAction>,
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
			secondary_action: None,
		}
	}

	/// Sets the override score for this hit.
	///
	/// See [`Hit::override_score`] for more information.
	#[must_use]
	pub fn with_score(mut self, score: impl Into<Option<u32>>) -> Self {
		let option: Option<u32> = score.into();
		self.override_score = option.into_c();
		self
	}

	/// Sets the secondary action for this hit.
	///
	/// See [`Hit::secondary_action`] for more information.
	#[must_use]
	pub fn with_secondary(
		mut self,
		action: impl Fn(&Self, RefDynHitActionContext<'_>) + Send + Sync + 'static,
	) -> Self {
		self.secondary_action = Some(Box::new(action));
		self
	}
}

impl Hit for SimpleHit {
	fn action(&self, context: RefDynHitActionContext<'_>) {
		(self.action)(self, context);
	}

	fn secondary_action(&self, context: RefDynHitActionContext<'_>) {
		let Some(action) = &self.secondary_action else {
			log::trace!("no secondary action set for hit '{}'", self.title());

			return self.action(context);
		};

		(action)(self, context);
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

impl From<SimpleHit> for ArcDynHit {
	fn from(value: SimpleHit) -> Self {
		Self::from_ptr(RArc::new(value), sabi_trait::TD_Opaque)
	}
}
