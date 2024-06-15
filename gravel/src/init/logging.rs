use super::cli::LogArgs;
use anyhow::Result;
use chrono::Local;
use fern::{Dispatch, FormatCallback};
use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
use gravel_core::paths::get_gravel_log_path;
use log::{Log, Record};
use std::fmt::Arguments;
use std::io::Write;
use std::path::Path;

pub fn logging(args: LogArgs) -> Result<()> {
	let mut dispatch = Dispatch::new().level(args.verbosity.log_level_filter());

	if !args.no_stderr_log {
		dispatch = chain_stderr(dispatch);
	}

	let log_path = &args.log_file.unwrap_or(get_gravel_log_path());
	if !matches!(log_path.to_str(), Some("off")) {
		dispatch = chain_file(dispatch, log_path)?;
	}

	dispatch.apply()?;

	log::trace!("hello world");

	Ok(())
}

fn chain_stderr(dispatch: Dispatch) -> Dispatch {
	let mut stderrlog = stderrlog::new();
	stderrlog.verbosity(4);

	dispatch.chain(Box::new(stderrlog) as Box<dyn Log>)
}

fn chain_file(dispatch: Dispatch, path: &Path) -> Result<Dispatch> {
	fn format_line(out: FormatCallback, message: &Arguments, record: &Record) {
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
		#[cfg(unix)]
		None,
	);

	let file_dispatch = Dispatch::new()
		.format(format_line)
		.chain(Box::new(rotate) as Box<dyn Write + Send>);

	Ok(dispatch.chain(file_dispatch))
}
