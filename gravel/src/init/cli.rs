use std::error::Error;

use clap::Parser;
use log::LevelFilter;

pub fn cli() -> Args {
	Args::parse()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	/// enable debug loglevel
	#[command(flatten)]
	pub verbosity: Verbosity,

	/// Key.subkey=value pairs for dynamically patching the config
	#[clap(short, long, value_delimiter = ',', value_parser = parse_key_val::<String, String>)]
	pub config_override: Vec<(String, String)>,
}

#[derive(clap::Args, Debug, Clone, Default)]
pub struct Verbosity {
	/// Increase logging verbosity
	#[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
    )]
	verbose: u8,

	/// Decrease logging verbosity
	#[arg(
        long,
        short = 'q',
        action = clap::ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
	quiet: u8,
}

impl Verbosity {
	pub fn verbosity(&self) -> i8 {
		// 1 == Warn
		1 - (self.quiet as i8) + (self.verbose as i8)
	}

	pub fn log_level(&self) -> LevelFilter {
		match self.verbosity() {
			i8::MIN..=-1 => LevelFilter::Off,
			0 => LevelFilter::Error,
			1 => LevelFilter::Warn,
			2 => LevelFilter::Info,
			3 => LevelFilter::Debug,
			4..=i8::MAX => LevelFilter::Trace,
		}
	}
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
	T: std::str::FromStr,
	T::Err: Error + Send + Sync + 'static,
	U: std::str::FromStr,
	U::Err: Error + Send + Sync + 'static,
{
	let pos = s
		.find('=')
		.ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
	Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
