use crate::ArcDynHit;
use std::mem;
use std::ops::DerefMut;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

/// Container for unchanging hits.
pub struct StaticHitCache {
	data: Box<[ArcDynHit]>,
}

impl StaticHitCache {
	/// Creates a new cache from any iterator if hits.
	pub fn new<H>(items: impl IntoIterator<Item = H>) -> Self
	where
		H: Into<ArcDynHit>,
	{
		Self {
			data: items.into_iter().map(Into::into).collect(),
		}
	}

	/// Gets a slice of the contained hits.
	pub fn get(&self) -> &[ArcDynHit] {
		&self.data
	}
}

/// Container for hits than can be re-used.
///
/// By default it caches hits indefinitely, only allowing manual clearing.  
/// See [`Self::max_age`].
#[derive(Default, Debug)]
pub struct HitCache {
	data: RwLock<Data>,
	strategy: Strategy,
}

impl HitCache {
	/// Converts this cache to using the MaxAge strategy,
	/// discarding the contents after the specified time has passed.
	pub fn max_age(mut self, age: Duration) -> Self {
		self.strategy = Strategy::MaxAge(age);
		self
	}

	/// Clears the cache, setting it to empty.
	pub fn clear(&self) {
		let mut guard = self.write();
		guard.hits.clear();
		guard.created = None;
	}

	/// Gets the cached hits, if there are any.
	pub fn get(&self) -> Option<Guard<'_>> {
		let guard = self.read();

		if self.is_stale(&guard) {
			return None;
		}

		Some(Guard(guard))
	}

	/// Gets the cached hits or computes them from `f`, storing them in the cache.
	pub fn get_or<I, H>(&self, f: impl FnOnce() -> I) -> Guard<'_>
	where
		I: Iterator<Item = H>,
		H: Into<ArcDynHit>,
	{
		if let Some(a) = self.get() {
			return a;
		};

		self.set(f().map(Into::into));
		Guard(self.read())
	}

	/// Replaces any current contents with `hits`.
	pub fn set(&self, hits: impl Iterator<Item = ArcDynHit>) {
		let mut guard = self.write();
		guard.hits.clear();
		guard.hits.extend(hits);
		guard.created = Some(Instant::now());
	}

	/// Private API, may change at any time.
	#[doc(hidden)]
	pub fn _is_poisoned(&self) -> bool {
		self.data.is_poisoned()
	}

	fn is_stale(&self, data: &Data) -> bool {
		match self.strategy {
			Strategy::Lazy => data.created.is_none(),
			Strategy::MaxAge(max_age) => data.created.map(|c| c.elapsed() > max_age).unwrap_or(true),
		}
	}

	fn read(&self) -> RwLockReadGuard<'_, Data> {
		match self.data.read() {
			Ok(g) => g,
			Err(e) => {
				drop(e);

				// write resets the contents to a known-good state and clears the poison
				drop(self.write());
				self.read()
			}
		}
	}

	fn write(&self) -> RwLockWriteGuard<'_, Data> {
		self.data.write().unwrap_or_else(|e| {
			log::error!("cache rwlock was poisoned! resetting cache and continuing");

			let mut guard = e.into_inner();
			mem::take(guard.deref_mut());
			self.data.clear_poison();

			guard
		})
	}
}

#[derive(Debug)]
enum Strategy {
	Lazy,
	MaxAge(Duration),
}

impl Default for Strategy {
	fn default() -> Self {
		Self::Lazy
	}
}

#[derive(Default, Debug)]
struct Data {
	hits: Vec<ArcDynHit>,
	created: Option<Instant>,
}

#[derive(Debug)]
pub struct Guard<'a>(RwLockReadGuard<'a, Data>);

impl Guard<'_> {
	pub fn get(&self) -> &[ArcDynHit] {
		&self.0.hits
	}
}
