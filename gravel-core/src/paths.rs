use gravel_ffi::paths;
use lazy_static::lazy_static;
use std::env::{self, consts::DLL_EXTENSION};
use std::path::PathBuf;

const APP_NAME: &str = "gravel";

pub fn config_dir() -> PathBuf {
	if let Ok(path) = env::var("GRAVEL_CONFIG_PATH") {
		return path.into();
	}

	paths::xdg_config_home().join(APP_NAME)
}

pub fn log_path() -> PathBuf {
	let mut path = paths::xdg_state_home();
	path.push(APP_NAME);
	path.push(APP_NAME);
	path.set_extension("log");

	path
}

pub fn plugin_globs() -> impl Iterator<Item = PathBuf> {
	lazy_static! {
		static ref PLUGIN_DIR: String = format!("{APP_NAME}/plugins/*.{DLL_EXTENSION}");
	}

	paths::xdg_data_globs(&PLUGIN_DIR)
}
