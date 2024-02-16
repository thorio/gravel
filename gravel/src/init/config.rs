use figment::providers::{Format, Yaml};
use figment::Figment;
use gravel_core::config::{ConfigManager, DEFAULT_CONFIG};
use gravel_core::paths::get_gravel_config_dir;
use std::env::consts;

/// Reads and deserializes the configuration from multiple sources:
/// - baked-in default config (config.yml in crate root)
/// - user config file in `$XDG_CONFIG_HOME/gravel/user.yml`
/// - platform-specific user config file in e.g.
///   `$XDG_CONFIG_HOME/gravel/platform/linux.yml`
///
/// Each layer can override the values of the previous layers.
pub fn config() -> ConfigManager {
	log::trace!("loading config");

	let figment = get_figment();

	ConfigManager::new(figment)
}

/// Initializes up the [`ConfigBuilder`] with all sources.
fn get_figment() -> Figment {
	let user_config_dir = get_gravel_config_dir();
	let user_config_path = user_config_dir.join("config.yml");
	let platform_config_path = user_config_dir.join(format!("platform/{}.yml", consts::OS));

	log::debug!("reading configs from {user_config_path:?}; {platform_config_path:?}");

	Figment::new()
		.merge(Yaml::string(DEFAULT_CONFIG))
		.merge(Yaml::file(user_config_path))
		.admerge(Yaml::file(platform_config_path))
}
