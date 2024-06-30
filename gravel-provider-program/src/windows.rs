use crate::Config;
use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_ffi::{FrontendMessage, SimpleHit};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_program_paths(config: &Config) -> impl Iterator<Item = PathBuf> + '_ {
	config.windows.shortcut_paths.iter().filter_map(expand_path)
}

fn expand_path(path: &String) -> Option<PathBuf> {
	shellexpand::env(path)
		.map(Cow::into_owned)
		.inspect_err(|e| log::error!("unable to expand shortcut_path '{path}': {e}"))
		.map(PathBuf::from)
		.ok()
}

/// Extracts an application's name from the filename of the link and
/// returns a [`SimpleHit`] that represents it.
pub fn get_program(path: &Path) -> Option<SimpleHit> {
	let name = path.file_stem()?.to_string_lossy();
	let path = path.to_str()?.to_owned();

	Some(SimpleHit::new(name, path.clone(), move |_h, s| run_program(&path, s)))
}

/// Passes the link's path to explorer, which then launches the application.
fn run_program(link_path: &str, sender: &RSender<FrontendMessage>) {
	log::debug!("starting application '{link_path}'");

	Command::new("explorer")
		.arg(link_path)
		.spawn()
		.expect("running explorer should never fail");

	sender.send(FrontendMessage::Hide).ok();
}
