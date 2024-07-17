use abi_stable::std_types::{RArc, RStr};
use abi_stable::{sabi_trait, traits::IntoReprC};
use gravel_ffi::{ArcDynHit, Hit, RefDynHitActionContext};

pub static TEST_HITS_ARCHEOLOGISTS: [TestHit; 5] = [
	hit("Chert", "Let's sit together and watch the stars die."),
	hit("Esker", "Can't get enough of the moon? …I'm kidding."),
	hit("Riebeck", "Oh gosh, how was it? Was it amazing?"),
	hit("Gabbro", "Good to see you made it here in one piece."),
	hit("Feldspar", "Hey, hatchling, pull up a marshmallow stick!"),
];

pub static TEST_HITS_CHAPTERS: [TestHit; 7] = [
	hit("Forsaken City", "First Steps"),
	hit("Old Site", "Resurrections"),
	hit("Celestial Resort", "Checking In"),
	hit("Golden Ridge", "Anxiety"),
	hit("Mirror Temple", "Quiet and Falling"),
	hit("Reflection", "Starjump"),
	hit("The Summit", "Reach for the Summit"),
];

const fn hit(title: &'static str, subtitle: &'static str) -> TestHit {
	TestHit { title, subtitle }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestHit {
	title: &'static str,
	subtitle: &'static str,
}

impl Hit for TestHit {
	fn title(&self) -> RStr<'_> where {
		self.title.into_c()
	}

	fn subtitle(&self) -> RStr<'_> where {
		self.subtitle.into_c()
	}

	fn action(&self, _context: RefDynHitActionContext<'_>) {}
}

impl PartialEq<ArcDynHit> for TestHit {
	fn eq(&self, other: &ArcDynHit) -> bool {
		other.obj.downcast_as::<Self>().is_ok_and(|o| o == self)
	}
}

impl From<TestHit> for ArcDynHit {
	fn from(value: TestHit) -> Self {
		Self::from_ptr(RArc::new(value), sabi_trait::TD_CanDowncast)
	}
}
