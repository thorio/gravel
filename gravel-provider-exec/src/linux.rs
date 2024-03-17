use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};

const SHELL: &str = "sh";

/// Passes the given string to a new bash process.
pub fn run_command(cmd: &str) -> Result<()> {
	log::debug!("running command in {SHELL} '{cmd}'");

	Command::new(SHELL)
		.arg("-c")
		.arg(cmd)
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.map(drop)
		.map_err(|e| anyhow!("error invoking {SHELL}: {e}"))
}
