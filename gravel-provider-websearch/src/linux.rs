use anyhow::Result;
use std::process::{Command, Stdio};

/// Opens the given URL using xdg-open, explicitly detaching all streams.
pub fn open_url(url: &str) -> Result<()> {
	Command::new("xdg-open")
		.arg(url)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()?;

	Ok(())
}
