use std::io;
use std::process::Child;

/// Test fixture for setting up an Xvfb server.
pub struct Xvfb {
	process: Option<Child>,
	display: String,
}

impl Xvfb {
	/// Start new Xvfb server with sequential display number.
	#[cfg(unix)]
	pub fn new() -> Result<Self, io::Error> {
		use std::process::Command;
		use std::sync::atomic::{AtomicUsize, Ordering};

		static DISPLAY_COUNTER: AtomicUsize = AtomicUsize::new(42);

		let display = format!(":{}", DISPLAY_COUNTER.fetch_add(1, Ordering::SeqCst));

		let process = Command::new("Xvfb").arg(&display).arg("-ac").spawn()?;

		Ok(Self {
			process: Some(process),
			display,
		})
	}

	/// Return placeholder
	#[cfg(windows)]
	pub fn new() -> Result<Self, io::Error> {
		// TODO: somehow check if windows has a graphical session available?
		// Is it even possible to have a windows system with no GUI at all?

		Ok(Self {
			process: None,
			display: String::default(),
		})
	}

	pub fn display(&self) -> &str {
		self.display.as_ref()
	}
}

impl Drop for Xvfb {
	fn drop(&mut self) {
		if let Some(mut process) = self.process.take() {
			process.kill().expect("failed to kill xvfb process");
		}
	}
}
