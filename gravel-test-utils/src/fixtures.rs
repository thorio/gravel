use crate::data::{TEST_HITS_ARCHEOLOGISTS, TestHit};
use crate::xvfb::Xvfb;
use rstest::fixture;

#[fixture]
pub fn xvfb() -> Xvfb {
	Xvfb::new().expect("failed to setup xvfb")
}

#[fixture]
pub fn test_hits() -> &'static [TestHit] {
	&TEST_HITS_ARCHEOLOGISTS
}
