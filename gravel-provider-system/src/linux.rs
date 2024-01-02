use anyhow::Result;
use std::process::Command;

pub(crate) fn lock(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub(crate) fn logout(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub(crate) fn restart(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub(crate) fn shutdown(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub(crate) fn sleep(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

fn shell_run(cmd: &str) -> Result<()> {
	Command::new("/usr/bin/env").arg("bash").arg("-c").arg(cmd).spawn()?;

	Ok(())
}
