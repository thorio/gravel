//! gravel's command runner
//!
//! Always returns a hit with the minimum score that, when selected,
//! runs the command with the system shell.

use abi_stable::{external_types::crossbeam_channel::RSender, sabi_extern_fn, std_types::RStr};
use gravel_ffi::prelude::*;
use serde::Deserialize;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn get_plugin() -> PluginDefinition {
	PluginMetadata::new("exec").with_provider(get_provider)
}

#[sabi_extern_fn]
fn get_provider(config: &PluginConfigAdapter<'_>) -> BoxDynProvider {
	ExecProvider {
		config: config.get(DEFAULT_CONFIG),
	}
	.into_dyn()
}

pub struct ExecProvider {
	config: Config,
}

impl Provider for ExecProvider {
	fn query(&self, query: RStr<'_>) -> ProviderResult {
		let hit = SimpleHit::new(query, &*self.config.subtitle, run_command).with_score(MIN_SCORE);

		ProviderResult::single(hit.into_dyn())
	}
}

fn run_command(hit: &SimpleHit, sender: &RSender<FrontendMessage>) {
	if let Err(err) = implementation::run_command(hit.title().as_str()) {
		log::error!("{err}");
	}

	sender.send(FrontendMessage::Hide).ok();
}

#[derive(Deserialize, Debug)]
struct Config {
	pub subtitle: String,
}
