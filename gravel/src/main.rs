//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

use abi_stable::external_types::crossbeam_channel;
use anyhow::{Context, Result};
use gravel_core::{performance::Stopwatch, Core, CoreMessage};
use gravel_ffi::{FrontendExitStatus, FrontendExitStatusNe, FrontendMessageNe};
use std::{env, path::Path, process::Command};

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

	// do this first so it doesn't break when the executable is replaced later
	let executable = env::current_exe()?;

	let args = init::cli();
	init::logging(args.logging).context("logger error")?;

	let config = init::config();
	let single_instance = init::single_instance(config.root().single_instance.as_deref());
	let registry = init::plugins(&config.root().external_plugins);

	let (frontend_send, frontend_recv) = crossbeam_channel::bounded::<FrontendMessageNe>(16);
	let (core_send, core_recv) = crossbeam_channel::bounded::<CoreMessage>(16);

	let engine = init::engine(core_send.clone(), &registry, &config);
	let runner = Core::new(engine, frontend_send.clone(), core_recv);

	init::hotkeys(&config.root().hotkeys, frontend_send);

	let mut frontend = init::frontend(&registry, runner, &config);

	log::info!("initialization took {stopwatch}");
	log::trace!("starting frontend");
	let exit_status = frontend.run(frontend_recv);

	drop(single_instance);

	on_exit(exit_status, &executable)
}

fn on_exit(status: FrontendExitStatusNe, executable: &Path) -> Result<()> {
	let status = status.into_enum().expect("plugin must not be newer than application");

	match status {
		FrontendExitStatus::Exit => Ok(()),
		FrontendExitStatus::Restart => restart(executable),
	}
}

fn restart(executable: &Path) -> Result<()> {
	#[cfg(unix)]
	use std::os::unix::process::CommandExt;

	log::debug!("attempting to restart gravel");

	// skip arg0
	let args = env::args().skip(1);

	#[cfg(unix)]
	anyhow::bail!(Command::new(executable).args(args).exec());

	#[cfg(not(unix))]
	Ok(Command::new(executable).args(args).spawn().map(drop)?)
}

#[cfg(test)]
mod clippy_shut_up {
	// this has to be put *somewhere* so clippy doesn't complain that the crates are unused
	// (even though they're used in the integration tests)
	use gravel_test_utils as _;
	use test_bin as _;
}
