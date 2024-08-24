use crate::init::init;
use anyhow::{Context, Result};
use gravel_core::config::ConfigManager;
use gravel_core::timed;
use gravel_ffi::{FrontendExitStatus, FrontendExitStatusNe};
use log::Level;
use std::path::{Path, PathBuf};
use std::{env, process::Command};

pub fn daemon(config: &ConfigManager) -> Result<()> {
	let (core, ipc, mut frontend) = timed!(Level::Info, "initialization took", { init(config) });

	log::trace!("starting frontend");

	// do this now so it doesn't break when the executable is replaced later
	let executable = env::current_exe().context("failed to get gravel executable");

	core.spawn();
	let exit_status = frontend.run();

	drop(ipc);

	on_exit(exit_status, executable)
}

fn on_exit(status: FrontendExitStatusNe, executable: Result<PathBuf>) -> Result<()> {
	let status = status.into_enum().expect("plugin must not be newer than application");

	match status {
		FrontendExitStatus::Exit => Ok(()),
		FrontendExitStatus::Restart => restart(&executable?),
	}
}

fn restart(executable: &Path) -> Result<()> {
	#[cfg(unix)]
	use std::os::unix::process::CommandExt;

	log::debug!("attempting to restart gravel");

	// skip arg0
	let args = env::args().skip(1);

	#[cfg(unix)]
	let res = Err(Command::new(executable).args(args).exec());

	#[cfg(not(unix))]
	let res = Command::new(executable).args(args).spawn().map(drop);

	res.context("failed to execute gravel binary")
}
