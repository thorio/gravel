use abi_stable::std_types::{RString, RVec};
use gravel_core::config::DEFAULT_CONFIG;
use gravel_core::paths::config_dir;
use gravel_ffi::prelude::*;
use std::env::consts;

/// Reads and deserializes the configuration from multiple sources:
/// - baked-in default config (config.yml in crate root)
/// - user config file in `$XDG_CONFIG_HOME/gravel/config.yml`
/// - platform-specific user config file in e.g.
///   `$XDG_CONFIG_HOME/gravel/platform/linux.yml`
/// - host-specific user config file in e.g.
///   `$XDG_CONFIG_HOME/gravel/host/yourhostname.yml`
///
/// Each layer can override the values of the previous layers.
pub fn config() -> ConfigManager {
	log::trace!("loading config");

	ConfigManager::new(sources())
}

/// Initializes up the [`ConfigBuilder`] with all sources.
fn sources() -> RVec<ConfigLayer> {
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
	.into()
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
