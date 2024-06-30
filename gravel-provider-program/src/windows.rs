use crate::Config;
use gravel_ffi::{BoxDynHitActionContext, SimpleHit};
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

	Some(SimpleHit::new(name, path.clone(), move |_, ctx| {
		run_program(&path, ctx);
	}))
}

/// Passes the link's path to explorer, which then launches the application.
fn run_program(link_path: &str, context: &BoxDynHitActionContext) {
	log::debug!("starting application '{link_path}'");

	Command::new("explorer")
		.arg(link_path)
		.spawn()
		.expect("running explorer should never fail");

	context.hide_frontend();
}
