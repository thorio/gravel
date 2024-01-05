use ::config::{builder::DefaultState, Config, ConfigBuilder, File, FileFormat};
use gravel_core::config::{ConfigManager, DEFAULT_CONFIG};
use gravel_core::paths::get_gravel_config_dir;
use std::path::PathBuf;

/// Reads and deserializes the configuration from multiple sources:
/// - baked-in default config (config.yml in crate root)
/// - user config file in `$XDG_CONFIG_HOME/gravel/user.yml`
///
/// Each layer can override the values of the previous layers.
pub fn config() -> ConfigManager {
	log::trace!("loading config");

	let config = match get_builder().build() {
		Ok(config) => config,
		Err(err) => {
			log::error!("config: {err}");
			std::process::exit(1);
		}
	};

	ConfigManager::new(config)
}

/// Initializes up the [`ConfigBuilder`] with all sources.
fn get_builder() -> ConfigBuilder<DefaultState> {
	let user_config_path = get_user_config_path();
	log::debug!("reading config from {user_config_path:?}");

	Config::builder()
		.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
		.add_source(File::from(user_config_path).required(false))
}

fn get_user_config_path() -> PathBuf {
	get_gravel_config_dir().join("config.yml")
}
