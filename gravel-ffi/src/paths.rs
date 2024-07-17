//! Defines useful functions for working with common paths.

use std::{env, iter::once, path::PathBuf};

const XDG_DATA_HOME: &str = "XDG_DATA_HOME";
const XDG_DATA_DIRS: &str = "XDG_DATA_DIRS";

/// Returns the user's home directory.
///
/// # Panics
/// When `$HOME/$USERPROFILE` are not set.
pub fn home() -> PathBuf {
	#[cfg(unix)]
	let home = env::var("HOME").expect("$HOME must always be set");

	#[cfg(windows)]
	let home = env::var("USERPROFILE").expect("$USERPROFILE must always be set");

	PathBuf::from(home)
}

/// Returns the value of `$XDG_CONFIG_HOME` or the default value.
pub fn xdg_config_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_CONFIG_HOME") {
		return path.into();
	}

	home().join(".config")
}

/// Returns the value of `$XDG_DATA_HOME` or the default value.
pub fn xdg_data_home() -> PathBuf {
	if let Ok(path) = env::var(XDG_DATA_HOME) {
		return path.into();
	}

	home().join(".local/share")
}

/// Returns the value of `$XDG_DATA_DIRS` or the default value.
pub fn xdg_data_dirs() -> Vec<PathBuf> {
	if let Ok(path) = env::var(XDG_DATA_DIRS) {
		return path.split(':').map(PathBuf::from).collect();
	}

	#[cfg(unix)]
	return vec![PathBuf::from("/usr/local/share/"), PathBuf::from("/usr/share/")];

	#[cfg(windows)]
	return vec![];
}

/// Returns [`xdg_data_home`] plus [`xdg_data_dirs`], postfixed with `path`.
pub fn xdg_data_globs(path: &str) -> impl Iterator<Item = PathBuf> + '_ {
	once(xdg_data_home()).chain(xdg_data_dirs()).map(move |mut p| {
		p.push(path);
		p
	})
}

/// Returns the value of `$XDG_STATE_HOME` or the default value.
pub fn xdg_state_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_STATE_HOME") {
		return path.into();
	}

	home().join(".local/state")
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	pub fn should_return_globs() {
		// this is fine for now because there's only one test that deals with env vars
		env::set_var(XDG_DATA_HOME, "/home/user/.xdg/share");
		env::set_var(XDG_DATA_DIRS, "/share:/data");

		let paths = xdg_data_globs("test/*").collect::<Vec<_>>();

		assert_eq!(
			paths,
			vec![
				PathBuf::from("/home/user/.xdg/share/test/*"),
				PathBuf::from("/share/test/*"),
				PathBuf::from("/data/test/*")
			]
		);
	}
}
