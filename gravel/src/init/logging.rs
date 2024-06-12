use log::LevelFilter;

pub fn logging(level: LevelFilter) {
	stderrlog::new()
		.verbosity(level)
		.init()
		.expect("this must never be called twice");

	log::trace!("hello world");
}
