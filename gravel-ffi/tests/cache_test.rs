//! Really a unit test, but can't be in-module due to
//! circular dependency gravel-ffi <-> gravel-test-utils

#![allow(unused_crate_dependencies)]

use gravel_ffi::{ArcDynHit, HitCache, StaticHitCache};
use gravel_test_utils::{data::TestHit, util::slices_equal};
use rstest::rstest;
use std::{iter::once, panic::catch_unwind, thread::sleep, time::Duration};

use gravel_test_utils::fixtures::test_hits;

#[rstest]
fn cache_static(test_hits: &[TestHit]) {
	let cache = StaticHitCache::new(test_hits.iter().cloned());

	assert!(slices_equal(test_hits, cache.get()));
}

#[rstest]
fn cache_lazy(test_hits: &[TestHit]) {
	let cache = HitCache::default();

	assert!(cache.get().is_none());

	let guard = cache.get_or(|| test_hits.iter().cloned());

	assert!(slices_equal(test_hits, guard.get()));
}

#[rstest]
fn cache_max_age(test_hits: &[TestHit]) {
	let max_age = Duration::from_millis(1);

	let cache = HitCache::default().max_age(max_age);

	assert!(cache.get().is_none());

	let guard = cache.get_or(|| test_hits.iter().cloned());

	assert!(slices_equal(test_hits, guard.get()));

	sleep(max_age);

	assert!(cache.get().is_none());
}

#[rstest]
fn cache_clear(test_hits: &[TestHit]) {
	let cache = HitCache::default();

	cache.get_or(|| test_hits.iter().cloned());

	cache.clear();

	assert!(cache.get().is_none());
}

#[rstest]
fn cache_poison_recovery(test_hits: &[TestHit]) {
	struct CausesPanics;

	impl From<CausesPanics> for ArcDynHit {
		fn from(_: CausesPanics) -> Self {
			panic!("expected panic");
		}
	}

	let max_age = Duration::from_millis(1);

	// set the cache up with stale data
	let cache = HitCache::default().max_age(max_age);
	_ = cache.get_or(|| test_hits.iter().cloned());
	sleep(max_age);

	// then cause a panic while trying to write new data
	_ = catch_unwind(|| {
		_ = cache.get_or(|| once(CausesPanics));
	});

	// cache is now poisoned
	assert!(cache._is_poisoned());

	// check if recovery works correctly
	assert!(cache.get().is_none());
	let guard = cache.get_or(|| test_hits.iter().cloned());

	assert!(slices_equal(test_hits, guard.get()));
}
