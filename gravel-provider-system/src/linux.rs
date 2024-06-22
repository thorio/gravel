use anyhow::Result;
use std::process::Command;

pub fn lock(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub fn logout(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub fn restart(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub fn shutdown(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

pub fn sleep(command_linux: &str) -> Result<()> {
	shell_run(command_linux)
}

fn shell_run(cmd: &str) -> Result<()> {
	Command::new("/usr/bin/env").arg("bash").arg("-c").arg(cmd).spawn()?;

	Ok(())
}
