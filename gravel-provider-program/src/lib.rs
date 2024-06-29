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

use std::path::PathBuf;

use abi_stable::{sabi_extern_fn, std_types::RStr};
use glob::{glob, Paths};
use gravel_ffi::prelude::*;
use itertools::Itertools;
use serde::Deserialize;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

#[cfg(not(feature = "no-root"))]
#[abi_stable::export_root_module]
pub fn get_library() -> PluginLibRef {
	use abi_stable::prefix_type::PrefixTypeTrait;
	PluginLib { plugin: get_plugin }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_plugin() -> PluginDefinition {
	PluginMetadata::new("program").with_provider(get_provider)
}

#[sabi_extern_fn]
fn get_provider(config_adapter: &PluginConfigAdapter<'_>) -> BoxDynProvider {
	let config = config_adapter.get::<Config>(DEFAULT_CONFIG);

	let program_paths = implementation::get_program_paths(&config).collect_vec();
	log::debug!("determined program paths: {program_paths:?}");

	ProgramProvider { program_paths }.into_dyn()
}

struct ProgramProvider {
	program_paths: Vec<PathBuf>,
}

impl Provider for ProgramProvider {
	fn query(&self, _query: RStr<'_>) -> ProviderResult {
		let hits = get_programs(&self.program_paths);

		ProviderResult::new(hits)
	}
}

/// Expands the path globs and returns hit representations of all programs it finds
pub(crate) fn get_programs(paths: &[PathBuf]) -> Vec<ArcDynHit> {
	paths
		.iter()
		.filter_map(expand_glob)
		.flatten()
		.filter_map(Result::ok)
		// TODO: follow desktop file specification on deduplication
		.unique_by(|p| p.file_name().map(ToOwned::to_owned))
		.filter_map(|p| implementation::get_program(&p))
		.map(HitExt::into_dyn)
		.collect()
}

pub fn expand_glob(pattern: &PathBuf) -> Option<Paths> {
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
