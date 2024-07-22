use anyhow::{anyhow, Result};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::{error::Error, path::PathBuf};

pub fn cli() -> Args {
	Args::parse()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[command(flatten)]
	pub logging: LogArgs,

	/// Key.subkey=value pairs for dynamically patching the config
	#[clap(short, long, value_delimiter = ',', value_parser = parse_key_val::<String, String>)]
	pub config_override: Vec<(String, String)>,
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

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U)>
where
	T: std::str::FromStr,
	T::Err: Error + Send + Sync + 'static,
	U: std::str::FromStr,
	U::Err: Error + Send + Sync + 'static,
{
	let (key, value) = s
		.split_once('=')
		.ok_or_else(|| anyhow!("invalid KEY=value: no `=` found in `{s}`"))?;

	Ok((key.parse()?, value.parse()?))
}
