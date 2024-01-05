use crate::Config;
use gravel_core::*;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::Sender;

pub(crate) fn get_program_paths(config: &Config) -> Vec<String> {
	config.windows.shortcut_paths.iter().filter_map(expand_path).collect()
}

fn expand_path(path: &String) -> Option<String> {
	shellexpand::env(path)
		.map(|p| p.into_owned())
		.map_err(|err| log::error!("couldn't expand shortcut_path '{path}': {err}"))
		.ok()
}

/// Extracts an application's name from the filename of the link and
/// returns a [`SimpleHit`] that represents it.
pub fn get_program(path: &Path) -> Option<SimpleHit> {
	let name = path.file_stem()?.to_string_lossy();
	let path = path.to_str()?.to_owned();

	Some(SimpleHit::new(name, path.clone(), move |h, s| run_program(&path, h, s)))
}

/// Passes the link's path to explorer, which then launches the application.
fn run_program(link_path: &str, _: &SimpleHit, sender: &Sender<FrontendMessage>) {
	log::debug!("starting application '{link_path}'");

	Command::new("explorer")
		.arg(link_path)
		.spawn()
		.expect("running explorer should never fail");

	sender.send(FrontendMessage::Hide).ok();
}
