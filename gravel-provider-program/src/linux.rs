use crate::Config;
use glob::glob;
use gravel_core::*;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

/// Expands the path globs and returns hit representations of all
/// desktop entries it finds.
pub(crate) fn get_programs(config: &Config) -> Vec<Box<dyn Hit>> {
	let mut hits = Vec::new() as Vec<Box<dyn Hit>>;

	for path in config.paths_linux.iter() {
		let expanded_path = shellexpand::env(path).unwrap();
		for result in glob(&expanded_path).expect("Failed to read glob pattern") {
			if result.is_err() {
				continue;
			}

			let hit = get_program(result.unwrap());
			if hit.is_err() {
				continue;
			}

			hits.push(Box::new(hit.unwrap()));
		}
	}

	hits
}

/// Parses a desktop entry and returns a [`SimpleHit`] that represents it.
fn get_program(path: PathBuf) -> Result<SimpleHit<ExtraData>, Box<dyn Error>> {
	let filename = path.file_name().unwrap().to_str().unwrap();

	let entry = freedesktop_entry_parser::parse_entry(&path)?;
	let section = entry.section("Desktop Entry");
	let name = section.attr("Name").unwrap_or(filename);

	let hit_data = HitData::new(name, path.to_str().unwrap());
	let hit = SimpleHit::new_extra(hit_data, ExtraData::new(filename), run_program);

	Ok(hit)
}

struct ExtraData {
	pub desktop_file: String,
}

impl ExtraData {
	pub fn new(desktop_file: &str) -> Self {
		ExtraData {
			desktop_file: desktop_file.to_owned(),
		}
	}
}

/// Runs the given entry using gtk-launch.
fn run_program(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	Command::new("gtk-launch")
		.arg(&hit.get_extra_data().desktop_file)
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
