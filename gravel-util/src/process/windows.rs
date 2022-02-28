use std::io;
use std::process::{Child, Command};

pub fn open_url(url: &str) -> io::Result<Child> {
	// opening the URL with explorer doesn't always escape correctly, so use cmd /c as a workaround
	Command::new("cmd")
		.arg("/c")
		.arg(format!("start {}", url))
		.spawn()
}

pub fn run_lnk(path: &str) -> io::Result<Child> {
	Command::new("explorer").arg(path).spawn()
}
