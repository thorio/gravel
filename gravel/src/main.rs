//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

use anyhow::{Context, Result};
use cli::Command;

mod cli;
mod init;
mod run;

#[cfg(windows)]
pub mod windows_console;

fn main() {
	init::panic();

	#[cfg(windows)]
	windows_console::attach();

	// unwrap so we hit color_eyre's panic handler
	#[expect(clippy::unwrap_used)]
	run().unwrap();

	#[cfg(windows)]
	windows_console::detach();

	log::debug!("exiting");
}

fn run() -> Result<()> {
	let args = cli::parse();
	init::logging(args.logging).context("failed to set up logging")?;
	let config = init::config();

	match args.command.unwrap_or(Command::Daemon) {
		Command::Daemon => run::daemon(&config),
	}
}

#[cfg(test)]
mod clippy_shut_up {
	// this has to be put *somewhere* so clippy doesn't complain that the crates are unused
	// (even though they're used in the integration tests)
	use gravel_test_utils as _;
	use test_bin as _;
}
