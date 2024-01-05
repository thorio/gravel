use std::{fmt::Display, time::Instant};

pub struct Stopwatch {
	begin: Instant,
}

impl Stopwatch {
	pub fn start() -> Self {
		Self { begin: Instant::now() }
	}
}

impl Display for Stopwatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let nanos = self.begin.elapsed().as_micros() as f32;
		let millis = nanos / 1000.0;
		write!(f, "{millis}ms")
	}
}
