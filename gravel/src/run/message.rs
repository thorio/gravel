use crate::cli::Args;
use anyhow::{bail, Result};
use gravel_core::config::ConfigManager;
use gravel_core::ipc::client;
use gravel_core::ipc::{self, ClientMessage};

pub fn show(config: &ConfigManager, args: &Args) -> Result<()> {
	message(config, args, ClientMessage::Show, true)
}

pub fn hide(config: &ConfigManager, args: &Args) -> Result<()> {
	message(config, args, ClientMessage::Hide, false)
}

fn message(config: &ConfigManager, args: &Args, msg: ClientMessage, autostart: bool) -> Result<()> {
	let ipc_config = &config.root().ipc;
	if !ipc_config.enabled {
		bail!("ipc is disabled in the config");
	}

	let mut conn = client::connect(ipc::get_name(ipc_config))?;

	if !client::ping(&mut conn)? {
		if !autostart {
			bail!("gravel is not running");
		}

		super::start(config, args)?;

		return Ok(());
	}

	client::query(&mut conn, msg)?;
	Ok(())
}
