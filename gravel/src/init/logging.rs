use crate::cli::LogArgs;
use anyhow::Result;
use chrono::Local;
use fern::{Dispatch, FormatCallback};
use file_rotate::{ContentLimit, FileRotate, compression::Compression, suffix::AppendCount};
use gravel_core::paths;
use log::{LevelFilter, Log, Record};
use std::fmt::Arguments;
use std::io::Write;
use std::path::Path;

pub fn logging(args: LogArgs) -> Result<()> {
	let mut dispatch = Dispatch::new().level(args.verbosity.log_level_filter());

	if !args.no_stderr_log {
		dispatch = chain_stderr(dispatch);
	}

	let log_path = &args.log_file.unwrap_or_else(paths::log_path);
	if log_path.to_str() != Some("off") {
		dispatch = chain_file(dispatch, log_path);
	}

	dispatch.apply()?;

	log::trace!("hello world");

	Ok(())
}

fn chain_stderr(dispatch: Dispatch) -> Dispatch {
	let mut stderrlog = stderrlog::new();
	stderrlog.verbosity(LevelFilter::max());

	dispatch.chain(Box::new(stderrlog) as Box<dyn Log>)
}

fn chain_file(dispatch: Dispatch, path: &Path) -> Dispatch {
	fn format_line(out: FormatCallback<'_>, message: &Arguments<'_>, record: &Record<'_>) {
		let level = record.level();
		let target = record.target();
		let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S");
		out.finish(format_args!("{level:<5} {timestamp} [{target}] {message}"));
	}

	let rotate = FileRotate::new(
		path,
		AppendCount::new(2),
		ContentLimit::Lines(2000),
		Compression::None,
		None,
	);

	let file_dispatch = Dispatch::new()
		.format(format_line)
		.chain(Box::new(rotate) as Box<dyn Write + Send>);

	dispatch.chain(file_dispatch)
}
