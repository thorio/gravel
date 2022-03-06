use ::config::{builder::DefaultState, Config, ConfigBuilder, File, FileFormat};
use gravel_core::config::ConfigManager;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

#[cfg(target_os = "linux")]
const CONFIG_PATH: &str = "${XDG_CONFIG_HOME:-$HOME/.config}/gravel";
#[cfg(windows)]
const CONFIG_PATH: &str = "${XDG_CONFIG_HOME:-$USERPROFILE/.config}/gravel";

/// Reads and deserializes the configuration from multiple sources:
/// - baked-in default config (config.yml in crate root)
/// - user config file in `$XDG_CONFIG_HOME/gravel/user.yml`
/// - dev config file in `$XDG_CONFIG_HOME/gravel/dev.yml`, only when
///   compiling in debug mode.
///
/// Each layer can override the values of the previous layers.
pub fn config() -> ConfigManager {
	// TODO: error handling
	let config = get_builder().build().unwrap();

	ConfigManager::new(config)
}

/// Initializes up the [`ConfigBuilder`] with all sources.
fn get_builder() -> ConfigBuilder<DefaultState> {
	let user_config_path = format!("{}/user.yml", shellexpand::env(CONFIG_PATH).unwrap());

	#[allow(unused_mut)]
	let mut builder = Config::builder()
		.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
		.add_source(File::with_name(&user_config_path).required(false));

	// dev-only config layer, for example to use a different hotkey for the dev instance
	#[cfg(debug_assertions)]
	{
		let dev_config_path = format!("{}/dev.yml", shellexpand::env(CONFIG_PATH).unwrap());

		builder = builder.add_source(File::with_name(&dev_config_path).required(false));
	}

	builder
}
