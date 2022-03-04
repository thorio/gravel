use std::io;
use std::process::{Child, Command, Stdio};

/// Opens the given URL using xdg-open, explicitly detaching all streams.
pub fn open_url(url: &str) -> io::Result<Child> {
	Command::new("xdg-open")
		.arg(url)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
}
