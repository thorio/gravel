use ::config::{builder::DefaultState, Config, ConfigBuilder, File, FileFormat};
use gravel_core::config::ConfigManager;
use std::{env, path::PathBuf};

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const ENV_GRAVEL_CONFIG_PATH: &str = "GRAVEL_CONFIG_PATH";

/// Reads and deserializes the configuration from multiple sources:
/// - baked-in default config (config.yml in crate root)
/// - user config file in `$XDG_CONFIG_HOME/gravel/user.yml`
///
/// Each layer can override the values of the previous layers.
pub fn config() -> ConfigManager {
	// TODO: error handling
	let config = get_builder().build().unwrap();

	ConfigManager::new(config)
}

/// Initializes up the [`ConfigBuilder`] with all sources.
fn get_builder() -> ConfigBuilder<DefaultState> {
	Config::builder()
		.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
		.add_source(File::from(get_user_config_path()).required(false))
}

fn get_user_config_path() -> PathBuf {
	get_user_config_dir().join("config.yml")
}

fn get_user_config_dir() -> PathBuf {
	if let Ok(path) = env::var(ENV_GRAVEL_CONFIG_PATH) {
		return path.into();
	}

	get_xdg_config_home().join(APP_NAME)
}

fn get_xdg_config_home() -> PathBuf {
	if let Ok(path) = env::var("XDG_CONFIG_HOME") {
		return path.into();
	}

	#[cfg(target_os = "linux")]
	return get_home().join(".config");

	#[cfg(windows)]
	return get_appdata_roaming();
}

#[cfg(target_os = "linux")]
fn get_home() -> PathBuf {
	let home = env::var("HOME").expect("$HOME is undefined");

	PathBuf::from(home)
}

#[cfg(windows)]
fn get_appdata_roaming() -> PathBuf {
	let roaming = env::var("APPDATA").expect("$APPDATA is undefined");

	PathBuf::from(roaming)
}
