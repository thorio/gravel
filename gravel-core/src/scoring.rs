use crate::provider::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;

pub const MAX_SCORE: u32 = u32::MAX;
pub const MIN_SCORE: u32 = u32::MIN + 1;
pub const NULL_SCORE: u32 = u32::MIN;

pub fn score_hits(query: &str, result: &mut QueryResult) {
	let matcher = SkimMatcherV2::default();

	for hit in result.hits.iter_mut() {
		let data = hit.get_data();
		if data.scored {
			continue;
		}

		let score = matcher
			.fuzzy_match(&data.title, query)
			.unwrap_or(NULL_SCORE as i64);

		hit.set_score(score as u32);
	}
}

pub fn trim_hits(result: &mut QueryResult) {
	result.hits.retain(|h| h.get_data().score != NULL_SCORE);
}

pub fn order_hits(result: &mut QueryResult) {
	result.hits.sort_by(compare_hits);
}

fn compare_hits(a: &Box<dyn Hit>, b: &Box<dyn Hit>) -> Ordering {
	b.get_data().score.partial_cmp(&a.get_data().score).unwrap()
}
