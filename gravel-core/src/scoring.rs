use crate::provider::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;

pub const MAX_SCORE: u32 = u32::MAX;
pub const MIN_SCORE: u32 = u32::MIN + 1;
pub const NULL_SCORE: u32 = u32::MIN;

/// Assigns each hit a score based on how closely its title matches the query.
pub fn score_hits(query: &str, result: &mut QueryResult) {
	let matcher = SkimMatcherV2::default();

	for hit in result.hits.iter_mut() {
		let data = hit.get_data();
		if data.scored {
			continue;
		}

		let score = matcher.fuzzy_match(&data.title, query).unwrap_or(NULL_SCORE as i64);

		hit.set_score(score as u32);
	}
}

/// Discards any hits that were assigned a [`NULL_SCORE`].
/// This happens when the title doesn't match the query at all.
pub fn trim_hits(result: &mut QueryResult) {
	result.hits.retain(|h| h.get_data().score != NULL_SCORE);
}

/// Orders the hits by their scores, highest to lowest.
/// Hits with equal scores are ordered by their title, alphabetically.
pub fn order_hits(result: &mut QueryResult) {
	result.hits.sort_by(compare_hits);
}

// Passing the box directly is easier since we're using sort_by
#[allow(clippy::borrowed_box)]
fn compare_hits(a: &Box<dyn Hit>, b: &Box<dyn Hit>) -> Ordering {
	let data_a = a.get_data();
	let data_b = b.get_data();

	let ordering = match data_a.score == data_b.score {
		true => data_a.title.partial_cmp(&data_b.title),
		false => data_b.score.partial_cmp(&data_a.score),
	};

	ordering.unwrap_or(Ordering::Equal)
}
