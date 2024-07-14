use crate::Config;
use core::fmt::Debug;
use gravel_ffi::prelude::*;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{borrow::Cow, process::Command};

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

	let hit = SimpleHit::new(name, path.clone(), |hit, context| {
		run_explorer(hit.subtitle().into_rust(), context);
	})
	.with_secondary(move |hit, context| {
		let Some(parent) = parent_dir(hit.subtitle()) else {
			log::error!("unable to get parent dir for '{}'", hit.subtitle());
			return;
		};

		run_explorer(parent, context);
	});

	Some(hit)
}

fn parent_dir(path: RStr<'_>) -> Option<&Path> {
	let path: &Path = path.into_rust().as_ref();

	path.parent()
}

/// Passes the link's path to explorer, which then launches the application.
fn run_explorer(link_path: impl AsRef<OsStr> + Debug, context: RefDynHitActionContext<'_>) {
	log::debug!("starting explorer with {link_path:?}");

	Command::new("explorer")
		.arg(link_path)
		.spawn()
		.expect("running explorer should never fail");

	context.hide_frontend();
}
