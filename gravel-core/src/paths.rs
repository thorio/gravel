use gravel_ffi::paths;
use std::{env, path::PathBuf};

const APP_NAME: &str = "gravel";

pub fn config_dir() -> PathBuf {
	if let Ok(path) = env::var("GRAVEL_CONFIG_PATH") {
		return path.into();
	}

	xdg_config_home().join(APP_NAME)
}

pub fn gravel_log_path() -> PathBuf {
	xdg_state_home().join(APP_NAME).join("gravel.log")
}

fn xdg_config_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_CONFIG_HOME") {
		return path.into();
	}

	paths::home().join(".config")
}

fn xdg_state_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_STATE_HOME") {
		return path.into();
	}

	paths::home().join(".local/state")
}
