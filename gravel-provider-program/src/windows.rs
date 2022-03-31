use crate::Config;
use glob::glob;
use gravel_core::*;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::Sender;

/// Expands the path globs and returns hit representations of all
/// symlinks it finds.
pub(crate) fn get_programs(config: &Config) -> Vec<Box<dyn Hit>> {
	let mut hits = Vec::new() as Vec<Box<dyn Hit>>;

	for path in config.paths_windows.iter() {
		let expanded_path = shellexpand::env(path).unwrap();
		for result in glob(&expanded_path).expect("Failed to read glob pattern") {
			if result.is_err() {
				continue;
			}

			let hit = get_program(result.unwrap());
			hits.push(Box::new(hit));
		}
	}

	hits
}

/// Extracts an application's name from the filename of the link and
/// returns a [`SimpleHit`] that represents it.
fn get_program(path: PathBuf) -> SimpleHit<ExtraData> {
	let name = path.file_stem().unwrap().to_str().unwrap();
	let path_str = path.to_str().unwrap();
	let hit_data = HitData::new(name, path_str);

	SimpleHit::new_extra(hit_data, ExtraData::new(path_str), run_program)
}

struct ExtraData {
	pub link_file: String,
}

impl ExtraData {
	pub fn new(link_file: &str) -> Self {
		ExtraData {
			link_file: link_file.to_owned(),
		}
	}
}

/// Passes the link's path to explorer, which then launches the application.
fn run_program(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	Command::new("explorer")
		.arg(&hit.get_extra_data().link_file)
		.spawn()
		.expect("failed to run application");

	sender.send(FrontendMessage::Hide).unwrap();
}
