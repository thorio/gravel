//! Program provider.
//! Searches for applications and allows you to launch them.
//!
//! ### Linux
//! Searches for .desktop files in `$XDG_DATA_DIRS` and `$XDG_DATA_HOME`
//!
//! Launches applications using gtk-launch.
//!
//! ### Windows
//! Searches for .lnk files in
//! - `%ProgramData%\Microsoft\Windows\Start Menu\Programs`
//! - `%APPDATA%\Microsoft\Windows\Start Menu\Programs`
//!
//! Launches applications using explorer.

use glob::{glob, Paths};
use gravel_ffi::prelude::*;
use itertools::Itertools;
use serde::Deserialize;
use std::path::PathBuf;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

struct ProgramProvider {
	program_paths: Vec<PathBuf>,
}

#[gravel_provider("program")]
impl ProviderDef for ProgramProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		let config = config.get::<Config>(DEFAULT_CONFIG);

		let program_paths = implementation::get_program_paths(&config).collect_vec();
		log::debug!("determined program paths: {program_paths:?}");

		Self { program_paths }
	}

	fn query(&self, _query: &str) -> ProviderResult {
		let hits = self
			.program_paths
			.iter()
			.filter_map(expand_glob)
			.flatten()
			.filter_map(Result::ok)
			// TODO: follow desktop file specification on deduplication
			.unique_by(|p| p.file_name().map(ToOwned::to_owned))
			.filter_map(|p| implementation::get_program(&p));

		ProviderResult::new(hits)
	}
}

fn expand_glob(pattern: &PathBuf) -> Option<Paths> {
	glob(&pattern.to_string_lossy())
		.inspect_err(|e| log::error!("unable to expand glob {pattern:?}: {e}"))
		.ok()
}

#[derive(Deserialize, Debug)]
struct Config {
	#[cfg(windows)]
	pub windows: WindowsConfig,
}

#[cfg(windows)]
#[derive(Deserialize, Debug)]
struct WindowsConfig {
	shortcut_paths: Vec<String>,
}
