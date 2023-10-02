use crate::Config;
use glob::glob;
use gravel_core::*;
use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{mpsc::Sender, Arc};

/// Expands the path globs and returns hit representations of all
/// symlinks it finds.
pub(crate) fn get_programs(config: &Config) -> Vec<Arc<dyn Hit>> {
	let mut hits = vec![] as Vec<Arc<dyn Hit>>;

	for path in &config.windows.shortcut_paths {
		let expanded_path = shellexpand::env(path).unwrap();
		fun_name(expanded_path, &mut hits);
	}

	hits
}

fn fun_name(expanded_path: Cow<str>, hits: &mut Vec<Arc<dyn Hit>>) {
	for result in glob(&expanded_path).expect("Failed to read glob pattern") {
		if result.is_err() {
			continue;
		}

		let hit = get_program(result.unwrap());
		hits.push(Arc::new(hit));
	}
}

/// Extracts an application's name from the filename of the link and
/// returns a [`SimpleHit`] that represents it.
fn get_program(path: PathBuf) -> SimpleHit<ExtraData> {
	let name = path.file_stem().unwrap().to_str().unwrap();
	let path_str = path.to_str().unwrap();
	SimpleHit::new_with_data(name, path_str, ExtraData::new(path_str), run_program)
}

struct ExtraData {
	pub link_file: String,
}

impl ExtraData {
	pub fn new(link_file: &str) -> Self {
		Self {
			link_file: link_file.to_owned(),
		}
	}
}

/// Passes the link's path to explorer, which then launches the application.
fn run_program(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	Command::new("explorer")
		.arg(&hit.get_data().link_file)
		.spawn()
		.expect("failed to run application");

	sender
		.send(FrontendMessage::Hide)
		.expect("failed to send frontend message");
}
