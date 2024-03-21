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
		let micros = self.begin.elapsed().as_micros() as f32;
		let millis = micros / 1000.0;
		write!(f, "{millis}ms")
	}
}
