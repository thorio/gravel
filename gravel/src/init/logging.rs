use log::LevelFilter;

pub fn logging(level: LevelFilter) {
	stderrlog::new()
		.timestamp(stderrlog::Timestamp::Off)
		.verbosity(level)
		.init()
		.expect("this must never be called twice");
}
