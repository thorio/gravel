use glob::glob;
use gravel_core::{frontend::ControlMessage, provider::*};
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::Sender;

static PATHS: &[&str] = &[
	"/usr/share/applications/*.desktop",
	"/usr/local/share/applications/*.desktop",
	"${XDG_DATA_HOMA:-$HOME/.local/share}/applications/*.desktop",
];

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

fn run_program(hit: &SimpleHit<ExtraData>, sender: &Sender<ControlMessage>) {
	gravel_util::process::run_freedesktop(&hit.get_extra_data().desktop_file)
		.expect("failed to run application");

	sender.send(ControlMessage::Hide).unwrap();
}
