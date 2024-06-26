use abi_stable::traits::IntoReprRust;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gravel_ffi::{ArcDynHit, ScoredHit};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;

pub const MAX_SCORE: u32 = u32::MAX;
pub const MIN_SCORE: u32 = u32::MIN;

lazy_static! {
	static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

/// Like [`get_scored_hits`], but skips the actual scoring step, defaulting to 0
pub fn get_unscored_hits(hits: impl IntoIterator<Item = ArcDynHit>) -> Vec<ScoredHit> {
	hits.into_iter()
		.map(|hit| {
			let score = hit.get_override_score().unwrap_or(0);
			ScoredHit::from(hit, score)
		})
		.sorted_by(compare_hits)
		.collect()
}

/// Assigns each hit a score based on how closely its title matches the query,
/// discards non-matching hits and orders them highest to lowest.
pub fn get_scored_hits(hits: Vec<ArcDynHit>, query: &str) -> Vec<ScoredHit> {
	hits.into_iter()
		.filter_map(|h| get_scored_hit(h, query))
		.sorted_by(compare_hits)
		.collect()
}

fn get_scored_hit(hit: ArcDynHit, query: &str) -> Option<ScoredHit> {
	let override_score = hit.get_override_score().into_rust();

	let score = override_score.or_else(|| get_score(&hit, query))?;

	Some(ScoredHit::from(hit, score))
}

fn get_score(hit: &ArcDynHit, query: &str) -> Option<u32> {
	let title = hit.get_title().into_rust();
	MATCHER.fuzzy_match(title, query).map(|s| s as u32)
}

fn compare_hits(a: &ScoredHit, b: &ScoredHit) -> Ordering {
	match b.score.cmp(&a.score) {
		Ordering::Equal => a.hit.get_title().cmp(&b.hit.get_title()),
		ordering => ordering,
	}
}
