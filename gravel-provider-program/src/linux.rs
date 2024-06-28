use crate::Config;
use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_ffi::{FrontendMessage, SimpleHit};
use std::env;
use std::iter::once;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// TODO: temporary, move these
fn get_xdg_data_dirs() -> Vec<PathBuf> {
	if let Ok(path) = env::var("XDG_DATA_DIRS") {
		return path.split(':').map(PathBuf::from).collect();
	}

	vec![PathBuf::from("/usr/local/share/"), PathBuf::from("/usr/share/")]
}

fn get_home() -> PathBuf {
	let home = env::var("HOME").expect("$HOME should always be set");

	PathBuf::from(home)
}

fn get_xdg_data_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_DATA_HOME") {
		return path.into();
	}

	get_home().join(".local/share")
}

pub fn get_program_paths(_config: &Config) -> Vec<String> {
	once(get_xdg_data_home())
		.chain(get_xdg_data_dirs())
		.map(|mut p| {
			p.push("applications/*.desktop");
			p.to_string_lossy().into_owned()
		})
		.collect()
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
	let hit = SimpleHit::new(name, path.to_string_lossy(), move |s| run_program(&filename, s));

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
