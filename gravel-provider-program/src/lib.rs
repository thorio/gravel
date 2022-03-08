//! Program provider.
//! Searches for applications and allows you to launch them.
//!
//! ### Linux
//! Searches for .desktop files in
//! - `/usr/share/applications`
//! - `/usr/local/share/applications`
//! - `$XDG_DATA_HOME/applications`
//!
//! Launches applications using gtk-launch.
//!
//! ### Windows
//! Searches for .lnk files in
//! - `%ProgramData%\Microsoft\Windows\Start Menu\Programs`
//! - `%APPDATA%\Microsoft\Windows\Start Menu\Programs`
//!
//! Launches applications using explorer.

use gravel_core::{config::*, plugin::*, *};
use serde::Deserialize;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("program").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(config: &PluginConfigAdapter) -> Box<dyn Provider> {
	let plugin_config = config.get::<Config>(DEFAULT_CONFIG);

	Box::new(ProgramProvider { config: plugin_config })
}

struct ProgramProvider {
	config: Config,
}

impl Provider for ProgramProvider {
	fn query(&self, _query: &str) -> QueryResult {
		let hits = implementation::get_programs(&self.config);

		QueryResult::new(hits)
	}
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Config {
	pub paths_linux: Vec<String>,
	pub paths_windows: Vec<String>,
}
