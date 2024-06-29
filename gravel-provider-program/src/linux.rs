use crate::Config;
use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_ffi::{paths, prelude::*};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn get_program_paths(_config: &Config) -> impl Iterator<Item = PathBuf> {
	paths::xdg_data_globs("applications/*.desktop")
}

/// Parses a desktop entry and returns a [`SimpleHit`] that represents it.
pub fn get_program(path: &Path) -> Option<SimpleHit> {
	let filename = path.file_name()?.to_str()?;

	let entry = freedesktop_entry_parser::parse_entry(path)
		.inspect_err(|e| log::trace!("couldn't parse desktop entry {path:?}, ignoring: {e}"))
		.ok()?;

	let section = entry.section("Desktop Entry");

	if section.attr("NoDisplay") == Some("true") {
		return None;
	}

	let name = section.attr("Name").unwrap_or(filename);

	let filename = filename.to_owned();
	let hit = SimpleHit::new(name, path.to_string_lossy(), move |_h, s| run_program(&filename, s));

	Some(hit)
}

/// Runs the given entry using gtk-launch.
fn run_program(desktop_file: &str, sender: &RSender<FrontendMessage>) {
	log::debug!("starting application '{desktop_file}'");

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
