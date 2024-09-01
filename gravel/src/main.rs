//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

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

	run();

	#[cfg(windows)]
	windows_console::detach();

	log::debug!("exiting");
}

fn run() {
	let args = cli::parse();
	init::logging(&args.logging).expect("failed to set up logging");
	let config = init::config();

	let result = match args.command.as_ref().unwrap_or(&Command::Daemon) {
		Command::Daemon => run::daemon(&config),
		Command::Show => run::show(&config, &args),
		Command::Hide => run::hide(&config, &args),
	};

	result.inspect_err(|e| log::error!("{e}")).ok();
}

#[cfg(test)]
mod clippy_shut_up {
	// this has to be put *somewhere* so clippy doesn't complain that the crates are unused
	// (even though they're used in the integration tests)
	use gravel_test_utils as _;
	use test_bin as _;
}
