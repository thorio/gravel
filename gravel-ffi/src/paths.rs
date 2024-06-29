use std::{env, iter::once, path::PathBuf};

pub fn home() -> PathBuf {
	#[cfg(unix)]
	let home = env::var("HOME").expect("$HOME must always be set");

	#[cfg(windows)]
	let home = env::var("USERPROFILE").expect("$USERPROFILE must always be set");

	PathBuf::from(home)
}

pub fn xdg_config_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_CONFIG_HOME") {
		return path.into();
	}

	home().join(".config")
}

pub fn xdg_data_dirs() -> Vec<PathBuf> {
	if let Ok(path) = env::var("XDG_DATA_DIRS") {
		return path.split(':').map(PathBuf::from).collect();
	}

	vec![PathBuf::from("/usr/local/share/"), PathBuf::from("/usr/share/")]
}

/// Returns XDG_DATA_HOME plus XDG_DATA_DIRS, postfixed with `path`.
pub fn xdg_data_globs(path: &str) -> impl Iterator<Item = PathBuf> + '_ {
	once(xdg_data_home()).chain(xdg_data_dirs()).map(move |mut p| {
		p.push(path);
		p
	})
}

pub fn xdg_data_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_DATA_HOME") {
		return path.into();
	}

	home().join(".local/share")
}

pub fn xdg_state_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_STATE_HOME") {
		return path.into();
	}

	home().join(".local/state")
}
