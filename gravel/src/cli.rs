use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

pub fn parse() -> Args {
	Args::parse()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[command(subcommand)]
	pub command: Option<Command>,

	#[command(flatten)]
	pub logging: LogArgs,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	/// Start the daemon without spawning a new process [default]
	#[allow(rustdoc::broken_intra_doc_links)] // [] used verbatim in clap help text
	Daemon,
}

#[derive(clap::Args, Debug)]
pub struct LogArgs {
	#[command(flatten)]
	pub verbosity: Verbosity<InfoLevel>,

	/// Disable stderr logging
	#[arg(long)]
	pub no_stderr_log: bool,

	/// Set to "off" to disable [default: "$XDG_STATE_DIR/gravel/current.log"]
	#[allow(rustdoc::broken_intra_doc_links)]
	#[arg(long)]
	pub log_file: Option<PathBuf>,
}
