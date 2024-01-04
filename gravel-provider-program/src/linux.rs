use crate::Config;
use gravel_core::paths::{get_xdg_data_dirs, get_xdg_data_home};
use gravel_core::*;
use std::iter::once;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

pub(crate) fn get_program_paths(_config: &Config) -> Vec<String> {
	once(get_xdg_data_home())
		.chain(get_xdg_data_dirs())
		.map(|mut p| {
			p.push("applications/*.desktop");
			p.to_string_lossy().into_owned()
		})
		.collect()
}

/// Parses a desktop entry and returns a [`SimpleHit`] that represents it.
pub fn get_program(path: &Path) -> Option<SimpleHit<()>> {
	let filename = path.file_name()?.to_str()?;

	let entry = freedesktop_entry_parser::parse_entry(path).ok()?;
	let section = entry.section("Desktop Entry");

	if let Some("true") = section.attr("NoDisplay") {
		return None;
	}

	let name = section.attr("Name").unwrap_or(filename);

	let filename = filename.to_owned();
	let hit = SimpleHit::new(name, path.to_string_lossy(), move |h, s| run_program(&filename, h, s));

	Some(hit)
}

/// Runs the given entry using gtk-launch.
fn run_program(desktop_file: &str, _: &SimpleHit<()>, sender: &Sender<FrontendMessage>) {
	Command::new("gtk-launch")
		.arg(desktop_file)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.expect("gtk-launch should be present");

	sender.send(FrontendMessage::Hide).ok();
}
