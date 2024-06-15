use std::{env, path::PathBuf};

const APP_NAME: &str = "gravel";

fn get_home() -> PathBuf {
	#[cfg(unix)]
	let home = env::var("HOME").expect("$HOME should always be set");

	#[cfg(windows)]
	let home = env::var("USERPROFILE").expect("$USERPROFILE should always be set");

	PathBuf::from(home)
}

pub fn get_gravel_config_dir() -> PathBuf {
	if let Ok(path) = env::var("GRAVEL_CONFIG_PATH") {
		return path.into();
	}

	get_xdg_config_home().join(APP_NAME)
}

pub fn get_gravel_log_path() -> PathBuf {
	get_xdg_state_home().join(APP_NAME).join("gravel.log")
}

fn get_xdg_config_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_CONFIG_HOME") {
		return path.into();
	}

	get_home().join(".config")
}

fn get_xdg_state_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_STATE_HOME") {
		return path.into();
	}

	get_home().join(".local/state")
}

pub fn get_xdg_data_dirs() -> Vec<PathBuf> {
	if let Ok(path) = env::var("XDG_DATA_DIRS") {
		return path.split(':').map(PathBuf::from).collect();
	}

	vec![PathBuf::from("/usr/local/share/"), PathBuf::from("/usr/share/")]
}

pub fn get_xdg_data_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_DATA_HOME") {
		return path.into();
	}

	get_home().join(".local/share")
}
