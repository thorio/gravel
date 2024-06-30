use abi_stable::traits::IntoReprRust;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gravel_ffi::{ArcDynHit, ScoredHit};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;

lazy_static! {
	static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

/// Like [`to_scored`], but skips the actual scoring step, defaulting to 0
pub fn to_unscored(hits: impl IntoIterator<Item = ArcDynHit>) -> Vec<ScoredHit> {
	hits.into_iter()
		.map(|hit| {
			let score = hit.override_score().unwrap_or(0);
			ScoredHit::from(hit, score)
		})
		.sorted_by(compare_hits)
		.collect()
}

/// Assigns each hit a score based on how closely its title matches the query,
/// discards non-matching hits and orders them highest to lowest.
pub fn to_scored(hits: Vec<ArcDynHit>, query: &str) -> Vec<ScoredHit> {
	hits.into_iter()
		.filter_map(|h| scored(h, query))
		.sorted_by(compare_hits)
		.collect()
}

fn scored(hit: ArcDynHit, query: &str) -> Option<ScoredHit> {
	let override_score = hit.override_score().into_rust();

	let score = override_score.or_else(|| score(&hit, query))?;

	Some(ScoredHit::from(hit, score))
}

fn score(hit: &ArcDynHit, query: &str) -> Option<u32> {
	let title = hit.title().into_rust();
	MATCHER.fuzzy_match(title, query).map(|s| s as u32)
}

fn compare_hits(a: &ScoredHit, b: &ScoredHit) -> Ordering {
	match b.score.cmp(&a.score) {
		Ordering::Equal => a.hit.title().cmp(&b.hit.title()),
		ordering => ordering,
	}
}
