use crate::init::{self, init};
use anyhow::{bail, Context, Result};
use gravel_core::{config::ConfigManager, ipc::get_name, timed};
use gravel_ffi::{FrontendExitStatus, FrontendExitStatusNe};
use log::Level;
use std::path::{Path, PathBuf};
use std::{env, process::Command};

pub fn daemon(config: &ConfigManager) -> Result<()> {
	check_ipc(config)?;

	let (core, ipc, mut frontend) = timed!(Level::Info, "initialization took", { init(config) });

	log::trace!("starting frontend");

	// do this now so it doesn't break when the executable is replaced later
	let executable = env::current_exe().context("failed to get gravel executable");

	core.spawn();
	let exit_status = frontend.run();

	if let Some(ipc) = ipc {
		ipc.quit();
	}

	on_exit(exit_status, executable)
}

fn check_ipc(config: &ConfigManager) -> Result<()> {
	let ipc_config = &config.root().ipc;
	if !ipc_config.enabled {
		log::trace!("ipc is disabled, skipping duplicate instance check");
		return Ok(());
	}

	let name = get_name(ipc_config);

	if init::is_duplicate_instance(&*name)? {
		bail!("duplicate instance with name '{name}' detected, exiting");
	}

	log::trace!("duplicate instance check passed");

	Ok(())
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
