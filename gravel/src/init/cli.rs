use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

pub fn cli() -> Args {
	Args::parse()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[command(flatten)]
	pub logging: LogArgs,
}

#[derive(clap::Args, Debug)]
pub struct LogArgs {
	#[command(flatten)]
	pub verbosity: Verbosity<InfoLevel>,

	/// Disable stderr logging
	#[arg(long)]
	pub no_stderr_log: bool,

	/// Defaults to "$XDG_STATE_DIR/gravel/current.log"; Set to "off" to disable
	#[arg(long)]
	pub log_file: Option<PathBuf>,
}
