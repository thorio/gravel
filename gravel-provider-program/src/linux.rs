use glob::glob;
use gravel_core::*;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;

static PATHS: &[&str] = &[
	"/usr/share/applications/*.desktop",
	"/usr/local/share/applications/*.desktop",
	"${XDG_DATA_HOME:-$HOME/.local/share}/applications/*.desktop",
];

/// Expands the [`PATH`] globs and returns hit representations of all
/// desktop entries it finds.
pub fn get_programs() -> Vec<Box<dyn Hit>> {
	let mut hits = Vec::new() as Vec<Box<dyn Hit>>;

	for path in PATHS {
		let expanded_path = shellexpand::env(path).unwrap();
		for result in glob(&expanded_path).expect("Failed to read glob pattern") {
			if !result.is_ok() {
				continue;
			}

			let hit = get_program(result.unwrap());
			if !hit.is_ok() {
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

/// Runs the given entry using gtk-launch, explicitly detaching all streams.
fn run_program(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	Command::new("gtk-launch")
		.arg(&hit.get_extra_data().desktop_file)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.expect("failed to run application");

	sender.send(FrontendMessage::Hide).unwrap();
}
