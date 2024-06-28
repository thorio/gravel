use std::{env, path::PathBuf};

pub fn home() -> PathBuf {
	#[cfg(unix)]
	let home = env::var("HOME").expect("$HOME should always be set");

	#[cfg(windows)]
	let home = env::var("USERPROFILE").expect("$USERPROFILE should always be set");

	PathBuf::from(home)
}

pub fn xdg_data_dirs() -> Vec<PathBuf> {
	if let Ok(path) = env::var("XDG_DATA_DIRS") {
		return path.split(':').map(PathBuf::from).collect();
	}

	vec![PathBuf::from("/usr/local/share/"), PathBuf::from("/usr/share/")]
}

pub fn xdg_data_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_DATA_HOME") {
		return path.into();
	}

	home().join(".local/share")
}
