use abi_stable::std_types::{RSlice, RString};
use abi_stable::StableAbi;
use figment::providers::{Format, Yaml};
use figment::Figment;
use serde::Deserialize;

// I'd really like to keep the app's figment and re-use it for plugins,
// but unfortunately it can't safely pass through FFI-boundaries.
pub fn create_figment(config_sources: &[ConfigLayer]) -> Figment {
	fn fold(figment: Figment, ConfigLayer(source, strategy): &ConfigLayer) -> Figment {
		let source = match source {
			ConfigSource::String(s) => Yaml::string(s.as_str()),
			ConfigSource::File(f) => Yaml::file(f.as_str()),
		};

		match strategy {
			MergeStrategy::Merge => figment.merge(source),
			MergeStrategy::AdMerge => figment.admerge(source),
		}
	}

	config_sources.iter().fold(Figment::new(), fold)
}

#[repr(C)]
#[derive(StableAbi)]
pub struct ConfigLayer(pub ConfigSource, pub MergeStrategy);

#[repr(u8)]
#[derive(StableAbi)]
pub enum MergeStrategy {
	Merge,
	AdMerge,
}

#[repr(u8)]
#[derive(StableAbi)]
pub enum ConfigSource {
	String(RString),
	File(RString),
}

/// Allows a plugin to deserialize its config without
/// knowing where in the main config it is.
#[repr(C)]
#[derive(StableAbi)]
pub struct PluginConfigAdapter<'a> {
	key: RString,
	config_sources: RSlice<'a, ConfigLayer>,
}

impl<'a> PluginConfigAdapter<'a> {
	pub fn from(key: RString, config_sources: RSlice<'a, ConfigLayer>) -> Self {
		Self { key, config_sources }
	}

	/// Build and deserialize the plugin's config into the given type.
	pub fn get<'de, T: Deserialize<'de>>(&self, default_config: &str) -> T {
		log::trace!("reading plugin config for {}", self.key);

		// layer the plugins' defaults under the provider's config section
		let figment = create_figment(&self.config_sources)
			.focus(&format!("{}.config", self.key))
			.join(Yaml::string(default_config));

		match figment.extract() {
			Ok(config) => config,
			Err(err) => {
				log::error!("error in plugin config {}: {err}", self.key);
				std::process::exit(1);
			}
		}
	}
}
