use crate::Config;
use glob::glob;
use gravel_core::paths::{get_xdg_data_dirs, get_xdg_data_home};
use gravel_core::*;
use itertools::Itertools;
use std::iter::once;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

/// Expands the path globs and returns hit representations of all
/// desktop entries it finds.
pub(crate) fn get_programs(_config: &Config) -> Vec<Box<dyn Hit>> {
	get_application_paths()
		.into_iter()
		.flat_map(get_desktop_entries)
		.unique_by(|p| p.file_name().map(|p| p.to_owned()))
		.filter_map(get_hit)
		.collect()
}

fn get_hit(result: PathBuf) -> Option<Box<dyn Hit>> {
	let path = result;
	let hit = get_program(path)?;

	Some(Box::new(hit))
}

fn get_desktop_entries(mut path: PathBuf) -> impl Iterator<Item = PathBuf> {
	path.push("*.desktop");

	let pattern = path.to_str().unwrap();

	glob(pattern)
		.expect("Failed to read glob pattern")
		.filter_map(Result::ok)
}

fn get_application_paths() -> Vec<PathBuf> {
	let data_dirs = once(get_xdg_data_home()).chain(get_xdg_data_dirs());

	data_dirs
		.map(|mut p| {
			p.push("applications");
			p
		})
		.collect()
}

/// Parses a desktop entry and returns a [`SimpleHit`] that represents it.
fn get_program(path: PathBuf) -> Option<SimpleHit<ExtraData>> {
	let filename = path.file_name()?.to_str()?;

	let entry = freedesktop_entry_parser::parse_entry(&path).ok()?;
	let section = entry.section("Desktop Entry");

	if let Some("true") = section.attr("NoDisplay") {
		return None;
	}

	let name = section.attr("Name").unwrap_or(filename);

	let data = ExtraData::new(filename);
	let hit = SimpleHit::new_with_data(name, path.to_str()?, data, run_program);

	Some(hit)
}

struct ExtraData {
	pub desktop_file: Box<str>,
}

impl ExtraData {
	pub fn new(desktop_file: impl Into<Box<str>>) -> Self {
		ExtraData {
			desktop_file: desktop_file.into(),
		}
	}
}

/// Runs the given entry using gtk-launch.
fn run_program(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	Command::new("gtk-launch")
		.arg(hit.get_data().desktop_file.deref())
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.expect("gtk-launch should be present");

	sender
		.send(FrontendMessage::Hide)
		.expect("receiver should live for the lifetime of the program");
}
