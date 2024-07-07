//! gravel's command runner
//!
//! Always returns a hit with the minimum score that, when selected,
//! runs the command with the system shell.

use gravel_ffi::prelude::*;
use serde::Deserialize;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub struct ExecProvider {
	config: Config,
}

#[gravel_provider("exec")]
impl Provider for ExecProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		Self {
			config: config.get(DEFAULT_CONFIG),
		}
	}

	fn query(&self, query: &str) -> ProviderResult {
		let hit = SimpleHit::new(query, &*self.config.subtitle, run_command).with_score(MIN_SCORE);

		ProviderResult::single(hit)
	}
}

fn run_command(hit: &SimpleHit, context: RefDynHitActionContext<'_>) {
	implementation::run_command(hit.title().as_str())
		.inspect_err(|e| log::error!("{e}"))
		.ok();

	context.hide_frontend();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub subtitle: String,
}
