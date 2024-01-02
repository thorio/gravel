use crate::Config;
use glob::glob;
use gravel_core::paths::{get_xdg_data_dirs, get_xdg_data_home};
use gravel_core::*;
use itertools::Itertools;
use std::iter::once;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::sync::Arc;

/// Expands the path globs and returns hit representations of all
/// desktop entries it finds.
pub(crate) fn get_programs(_config: &Config) -> Vec<Arc<dyn Hit>> {
	get_application_paths()
		.into_iter()
		.flat_map(get_desktop_entries)
		.unique_by(|p| p.file_name().map(ToOwned::to_owned))
		.filter_map(get_hit)
		.collect()
}

fn get_hit(result: PathBuf) -> Option<Arc<dyn Hit>> {
	let path = result;
	let hit = get_program(&path)?;

	Some(Arc::new(hit))
}

fn get_desktop_entries(mut path: PathBuf) -> impl Iterator<Item = PathBuf> {
	path.push("*.desktop");

	let pattern = path.to_string_lossy();

	// TODO: error handling, unify windows/linux logic
	glob(&pattern)
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
fn get_program(path: &Path) -> Option<SimpleHit<ExtraData>> {
	let filename = path.file_name()?.to_str()?;

	let entry = freedesktop_entry_parser::parse_entry(path).ok()?;
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
		.arg(&*hit.get_data().desktop_file)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.expect("gtk-launch should be present");

	sender.send(FrontendMessage::Hide).ok();
}
