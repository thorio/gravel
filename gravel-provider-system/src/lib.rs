//! System provider.
//! Provides system commands such as shutdown, log out or exiting gravel.

use anyhow::Result;
use gravel_ffi::prelude::*;
use serde::Deserialize;
use std::env;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

struct SystemProvider {
	hits: StaticHitCache,
}

#[gravel_provider("system")]
impl Provider for SystemProvider {
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		let config = config.get::<Config>(DEFAULT_CONFIG);
		let hits = [
			context_hit(config.exit, |ctx| ctx.exit()),
			context_hit(config.reload, |ctx| ctx.restart()),
			shell_hit(config.lock, implementation::lock),
			shell_hit(config.logout, implementation::logout),
			shell_hit(config.restart, implementation::restart),
			shell_hit(config.shutdown, implementation::shutdown),
			shell_hit(config.sleep, implementation::sleep),
		];

		Self {
			hits: StaticHitCache::new(hits),
		}
	}

	fn query(&self, _query: &str) -> ProviderResult {
		ProviderResult::from_cached(self.hits.get())
	}
}

fn context_hit(
	config: CommandConfig,
	action: impl Fn(RefDynHitActionContext<'_>) + Send + Sync + 'static,
) -> SimpleHit {
	SimpleHit::new(config.title, config.subtitle, move |_, ctx| action(ctx))
}

fn shell_hit(config: ShellCommandConfig, action: impl Fn(&str) -> Result<()> + Send + Sync + 'static) -> SimpleHit {
	SimpleHit::new(config.title, config.subtitle, move |hit, context| {
		action(&config.command_linux)
			.inspect_err(|e| log::error!("unable to perform system operation {}: {e}", hit.title()))
			.ok();

		context.hide_frontend();
	})
}

#[derive(Deserialize, Debug)]
struct Config {
	pub exit: CommandConfig,
	pub reload: CommandConfig,
	pub lock: ShellCommandConfig,
	pub logout: ShellCommandConfig,
	pub restart: ShellCommandConfig,
	pub shutdown: ShellCommandConfig,
	pub sleep: ShellCommandConfig,
}

#[derive(Deserialize, Debug)]
struct CommandConfig {
	pub title: String,
	pub subtitle: String,
}

#[derive(Deserialize, Debug)]
struct ShellCommandConfig {
	pub title: String,
	pub subtitle: String,
	pub command_linux: String,
}
