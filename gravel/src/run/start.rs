use crate::cli::Args;
use anyhow::{Context, Result};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use gravel_core::config::ConfigManager;
use log::LevelFilter;
use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio};

pub fn start(_config: &ConfigManager, args: &Args) -> Result<()> {
	log::debug!("attempting to start the gravel daemon");

	let executable = env::current_exe().context("failed to get gravel executable")?;

	Command::new(executable)
		.args(format_args(args))
		.arg("daemon")
		// explicitly prevent stream inheritance
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()?;

	Ok(())
}

fn format_args(args: &Args) -> Vec<OsString> {
	let logging = &args.logging;

	// since the daemon runs with stderr attached to /dev/null
	// when started like this, skip stderr logging entirely
	let mut buf = vec![OsString::from("--no-stderr-log")];

	if let Some(verbosity) = format_verbosity(&logging.verbosity) {
		buf.push(OsString::from(verbosity));
	}

	if let Some(log_file) = &logging.log_file {
		buf.push(OsString::from("--log-file"));
		buf.push(log_file.clone().into_os_string());
	}

	buf
}

fn format_verbosity(verbosity: &Verbosity<InfoLevel>) -> Option<&'static str> {
	match verbosity.log_level_filter() {
		LevelFilter::Off => Some("-qqq"),
		LevelFilter::Error => Some("-qq"),
		LevelFilter::Warn => Some("-q"),
		LevelFilter::Info => None,
		LevelFilter::Debug => Some("-v"),
		LevelFilter::Trace => Some("-vv"),
	}
}
