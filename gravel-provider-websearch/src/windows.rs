use std::io;
use std::process::{Child, Command};

/// Opens the given URL by passing it to `start`.
pub fn open_url(url: &str) -> io::Result<Child> {
	// opening the URL with explorer doesn't always escape correctly, so use cmd /c as a workaround
	Command::new("cmd").arg("/c").arg(format!("start {}", url)).spawn()
}
