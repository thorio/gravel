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
}

#[derive(clap::Args, Debug, Clone, Default)]
pub struct Verbosity {
	#[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
        help = "Increase logging verbosity",
    )]
	verbose: u8,

	#[arg(
        long,
        short = 'q',
        action = clap::ArgAction::Count,
        global = true,
        help = "Decrease logging verbosity",
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
