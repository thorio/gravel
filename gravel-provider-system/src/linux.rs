use crate::SubcommandConfig;
use std::process::Command;

pub(crate) fn lock(config: &SubcommandConfig) {
	shell_run(&config.command_linux);
}

pub(crate) fn logout(config: &SubcommandConfig) {
	shell_run(&config.command_linux);
}

pub(crate) fn restart(config: &SubcommandConfig) {
	shell_run(&config.command_linux);
}

pub(crate) fn shutdown(config: &SubcommandConfig) {
	shell_run(&config.command_linux);
}

pub(crate) fn sleep(config: &SubcommandConfig) {
	shell_run(&config.command_linux);
}

fn shell_run(cmd: &str) {
	Command::new("/usr/bin/env")
		.arg("bash")
		.arg("-c")
		.arg(cmd)
		.spawn()
		.expect("failed to run command");
}
