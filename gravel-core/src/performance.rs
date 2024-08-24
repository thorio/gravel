use log::Level;
use std::fmt::{self, Arguments, Display, Formatter};
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

pub struct Timed {
	message: String,
	level: Level,
	stopwatch: Stopwatch,
}

impl Timed {
	pub fn new(level: Level, args: Arguments<'_>) -> Option<Self> {
		if level > log::STATIC_MAX_LEVEL || level > log::max_level() {
			return None;
		}

		Some(Self {
			message: args.to_string(),
			level,
			stopwatch: Stopwatch::start(),
		})
	}
}

impl Drop for Timed {
	fn drop(&mut self) {
		log::log!(self.level, "{} {}", self.message, self.stopwatch);
	}
}

#[macro_export]
macro_rules! timed {
	( ($($arg:tt)*), $body:block ) => { timed!(::log::Level::Trace, ::std::format_args!($($arg)*), $body) };
	( $msg:literal, $body:block ) => { timed!(::log::Level::Trace, ::std::format_args!($msg), $body) };
	( $level:expr, ($($arg:tt)*), $body:block ) => { timed!($level, ::std::format_args!($($arg)*), $body) };
	( $level:expr, $msg:literal, $body:block ) => { timed!($level, ::std::format_args!($msg), $body) };

	( $level:expr, $msg:expr, $body:block ) => {{
		let __time = $crate::performance::Timed::new($level, $msg);

		$body
	}};
}
