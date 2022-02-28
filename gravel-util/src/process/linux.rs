use std::io;
use std::process::{Child, Command, Stdio};

pub fn open_url(url: &str) -> io::Result<Child> {
	Command::new("xdg-open")
		.arg(url)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
}

pub fn run_freedesktop(name: &str) -> io::Result<Child> {
	Command::new("gtk-launch")
		.arg(name)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
}
