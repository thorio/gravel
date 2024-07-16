use std::fmt::{self, Display, Formatter};
use std::time::Instant;

pub struct Stopwatch {
	begin: Instant,
}

impl Stopwatch {
	pub fn start() -> Self {
		Self { begin: Instant::now() }
	}
}

impl Display for Stopwatch {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// convert micros -> millis so we get decimal values
		let micros = self.begin.elapsed().as_micros() as f32;
		let millis = micros / 1000.0;
		write!(f, "{millis}ms")
	}
}
