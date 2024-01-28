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

use std::{path::PathBuf, sync::Arc};

use glob::{glob, Paths};
use gravel_core::{config::*, plugin::*, *};
use itertools::Itertools;
use serde::Deserialize;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = plugin("program").with_provider(Box::new(get_provider));

	registry.register(definition);
}

fn get_provider(config_adapter: &PluginConfigAdapter) -> Box<dyn Provider> {
	let config = config_adapter.get::<Config>(DEFAULT_CONFIG);

	let program_paths = implementation::get_program_paths(&config);
	log::debug!("determined program paths: {program_paths:?}");

	Box::new(ProgramProvider { program_paths })
}

struct ProgramProvider {
	program_paths: Vec<String>,
}

impl Provider for ProgramProvider {
	fn query(&self, _query: &str) -> ProviderResult {
		let hits = get_programs(&self.program_paths);

		ProviderResult::new(hits)
	}
}

/// Expands the path globs and returns hit representations of all programs it finds
pub(crate) fn get_programs(paths: &[String]) -> Vec<Arc<dyn Hit>> {
	paths
		.iter()
		.filter_map(expand_glob)
		.flatten()
		.filter_map(Result::ok)
		.unique_by(|p| p.file_name().map(ToOwned::to_owned))
		.filter_map(get_hit)
		.collect()
}

pub fn expand_glob(pattern: &String) -> Option<Paths> {
	glob(pattern)
		.map_err(|err| log::error!("couldn't expand glob '{pattern}': {err}"))
		.ok()
}

fn get_hit(path: PathBuf) -> Option<Arc<dyn Hit>> {
	Some(Arc::new(implementation::get_program(&path)?))
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
