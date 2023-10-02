use crate::provider::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;

pub const MAX_SCORE: u32 = u32::MAX;
pub const MIN_SCORE: u32 = u32::MIN;

lazy_static! {
	static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

pub struct ScoredHit {
	pub hit: Box<dyn Hit>,
	pub score: u32,
}

/// Assigns each hit a score based on how closely its title matches the query,
/// discards non-matching hits and orders them highest to lowest.
pub fn get_scored_hits(hits: Vec<Box<dyn Hit>>, query: &str) -> Vec<ScoredHit> {
	hits.into_iter()
		.filter_map(|h| get_scored_hit(h, query))
		.sorted_by(compare_hits)
		.collect()
}

fn get_scored_hit(hit: Box<dyn Hit>, query: &str) -> Option<ScoredHit> {
	let score = hit.get_override_score().or_else(|| get_score(&*hit, query))?;

	Some(ScoredHit { hit, score })
}

fn get_score(hit: &dyn Hit, query: &str) -> Option<u32> {
	MATCHER.fuzzy_match(hit.get_title(), query).map(|s| s as u32)
}

fn compare_hits(a: &ScoredHit, b: &ScoredHit) -> Ordering {
	match b.score.cmp(&a.score) {
		Ordering::Equal => a.hit.get_title().cmp(b.hit.get_title()),
		ordering => ordering,
	}
}
