//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

use abi_stable::external_types::crossbeam_channel;
use anyhow::{Context, Result};
use gravel_core::performance::Stopwatch;
use gravel_ffi::{FrontendExitStatus, FrontendMessageNe};
use std::{env, path::Path};

mod init;

// unwrap instead of returning a Result so we hit color_eyre's panic handler
#[allow(clippy::unwrap_used)]
fn main() {
	init::panic();

	#[cfg(windows)]
	init::windows_console::attach();

	run().unwrap();

	#[cfg(windows)]
	init::windows_console::detach();

	log::debug!("exiting");
}

fn run() -> Result<()> {
	let stopwatch = Stopwatch::start();

	let executable = env::current_exe()?;

	let args = init::cli();
	init::logging(args.logging).context("logger error")?;

	let config = init::config();

	let single_instance = init::single_instance(config.root().single_instance.as_deref());

	let registry = init::plugins(&config.root().external_plugins);

	let (sender, receiver) = crossbeam_channel::bounded::<FrontendMessageNe>(8);
	let engine = init::engine(sender.clone(), &registry, &config);
	let mut frontend = init::frontend(&registry, engine, &config);

	init::hotkeys(&config.root().hotkeys, sender);

	log::info!("initialization complete, took {stopwatch}");
	log::trace!("starting frontend");
	let exit_status = frontend
		.run(receiver)
		.into_enum()
		.expect("plugin must not be newer than application");

	drop(single_instance);

	match exit_status {
		FrontendExitStatus::Exit => Ok(()),
		FrontendExitStatus::Restart => restart(&executable),
	}
}

fn restart(executable: &Path) -> Result<()> {
	log::debug!("attempting to restart gravel");

	#[cfg(unix)]
	anyhow::bail!(exec::execvp(executable, env::args()));

	#[cfg(not(unix))]
	{
		// Windows doesn't like the first arg being the binary path
		let args = env::args().skip(1);

		std::process::Command::new(executable).args(args).spawn()?;

		Ok(())
	}
}
