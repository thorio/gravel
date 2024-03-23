//! gravel's bin crate.
//! Reads the config, loads plugins and initializes features.

// Without this, windows will open an additional console window for the application
#![windows_subsystem = "windows"]

use gravel_core::{performance::Stopwatch, *};
use std::{env, path::Path, sync::mpsc};

mod init;

fn main() {
	color_eyre::install().unwrap();

	#[cfg(windows)]
	init::windows_console::attach();

	run();

	#[cfg(windows)]
	init::windows_console::detach();

	log::trace!("exiting");
}

fn run() {
	let stopwatch = Stopwatch::start();

	let executable = env::current_exe().unwrap();

	let args = init::cli();
	init::logging(args.verbosity.log_level());

	let config = init::config();

	let single_instance = init::single_instance(config.root.single_instance.as_deref());

	let registry = init::plugins();

	let (sender, receiver) = mpsc::channel::<FrontendMessage>();
	let engine = init::engine(sender.clone(), &registry, &config);
	let mut frontend = init::frontend(&registry, engine, &config);

	init::hotkeys(&config.root.hotkeys, sender);

	log::trace!("initialization complete, took {stopwatch}");
	log::trace!("starting frontend");
	let exit_status = frontend.run(receiver);

	drop(single_instance);

	match exit_status {
		FrontendExitStatus::Exit => (),
		FrontendExitStatus::Restart => restart(&executable),
	};
}

fn restart(executable: &Path) {
	log::debug!("attempting to restart gravel");

	#[cfg(unix)]
	panic!("{:?}", exec::execvp(executable, env::args()));

	#[cfg(not(unix))]
	{
		// Windows doesn't like the first arg being the binary path
		let args = env::args().skip(1);

		std::process::Command::new(executable).args(args).spawn().unwrap();
	}
}
