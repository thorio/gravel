#![allow(unused_crate_dependencies, clippy::missing_panics_doc)]

use gravel_test_utils::xvfb::Xvfb;
use itertools::Itertools;
use rstest::{fixture, rstest};
use std::process::Command;
use std::time::Duration;
use test_bin::get_test_bin;

#[fixture]
fn xvfb() -> Xvfb {
	Xvfb::new().expect("failed to setup xvfb")
}

#[fixture]
fn bin() -> Command {
	get_test_bin("gravel")
}

/// Check if gravel starts up correctly.
///
/// It's making several assumptions and might break when everything is fine,
/// but it's still a decent indicator.
///
/// Note: configuration file `./config/config.yml` is used, which causes fltk to immediately exit.
#[rstest]
#[timeout(Duration::from_secs(10))]
pub fn run_gravel(mut bin: Command, xvfb: Xvfb) {
	let output = bin
		.env("GRAVEL_CONFIG_PATH", "tests/config")
		.env("DISPLAY", xvfb.display())
		.output()
		.expect("gravel broke");

	assert!(output.status.success());

	let stderr_string = String::from_utf8(output.stderr).expect("application must output valid UTF-8");

	// fltk frontend will warn once about the weird config we're using
	// any other warns or errors are bad
	assert_eq!(1, stderr_string.matches("WARN").collect_vec().len());
	assert_eq!(0, stderr_string.matches("ERROR").collect_vec().len());
}
