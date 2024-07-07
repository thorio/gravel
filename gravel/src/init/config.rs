use abi_stable::std_types::RString;
use gravel_core::config::{ConfigManager, DEFAULT_CONFIG};
use gravel_core::paths::config_dir;
use gravel_ffi::config::__private_api::{ConfigLayer, ConfigSource, MergeStrategy};
use std::env::consts;

/// Reads and deserializes the configuration from multiple sources, see [`sources`].
///
/// Each layer can override the values of the previous layers.
pub fn config() -> ConfigManager {
	log::trace!("loading config");

	ConfigManager::new(sources())
}

/// Returns configuration layers:
/// - Baked-in default config (config.yml in crate root)
/// - User config file in `$XDG_CONFIG_HOME/gravel/config.yml`
/// - Platform-specific user config file in e.g.
///   `$XDG_CONFIG_HOME/gravel/platform/linux.yml`
/// - Host-specific user config file in e.g.
///   `$XDG_CONFIG_HOME/gravel/host/yourhostname.yml`
fn sources() -> Vec<ConfigLayer> {
	let user_config_dir = config_dir();
	let user_config_path = user_config_dir.join("config.yml");
	let platform_config_path = user_config_dir.join(format!("platform/{}.yml", consts::OS));
	let host_config_path = user_config_dir.join(format!("host/{}.yml", hostname()));

	log::debug!("reading configs from {user_config_path:?}; {platform_config_path:?}; {host_config_path:?}");

	let user_config_path = RString::from(user_config_path.to_string_lossy());
	let platform_config_path = RString::from(platform_config_path.to_string_lossy());
	let host_config_path = RString::from(host_config_path.to_string_lossy());

	use {ConfigSource as C, MergeStrategy as S};
	vec![
		ConfigLayer(C::String(RString::from(DEFAULT_CONFIG)), S::Merge),
		ConfigLayer(C::File(user_config_path), S::Merge),
		ConfigLayer(C::File(platform_config_path), S::AdMerge),
		ConfigLayer(C::File(host_config_path), S::AdMerge),
	]
}

fn hostname() -> String {
	match hostname::get() {
		Ok(h) => h.to_string_lossy().into_owned(),
		Err(e) => {
			log::warn!("unable to get hostname, falling back to 'default'. error: {e}");
			String::from("default")
		}
	}
}
