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

use gravel_core::{plugin::*, *};

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("program").with_provider(|_config| Box::new(ProgramProvider::new()));

	registry.register(definition);
}

struct ProgramProvider {}

impl ProgramProvider {
	pub fn new() -> Self {
		ProgramProvider {}
	}
}

impl Provider for ProgramProvider {
	fn query(&self, _query: &str) -> QueryResult {
		let hits = implementation::get_programs();

		QueryResult::new(hits)
	}
}
