use abi_stable::std_types::{RSlice, RString};
use abi_stable::StableAbi;
use figment::providers::{Format, Yaml};
use serde::Deserialize;

/// Abstracts config deserialization from plugins, allowing differing
/// configuration for different plugin instances.
#[repr(C)]
#[derive(StableAbi)]
pub struct PluginConfigAdapter<'a> {
	key: RString,
	config_sources: RSlice<'a, __private_api::ConfigLayer>,
}

impl<'a> PluginConfigAdapter<'a> {
	/// Used internally by gravel, do not use in plugins.
	#[doc(hidden)]
	pub fn from(key: RString, config_sources: RSlice<'a, __private_api::ConfigLayer>) -> Self {
		Self { key, config_sources }
	}

	/// Build and deserialize the plugin's config into the given type.
	#[must_use]
	pub fn get<'de, T: Deserialize<'de>>(&self, default_config: &str) -> T {
		log::trace!("reading plugin config for {}", self.key);

		// layer the plugins' defaults under the provider's config section
		let figment = __private_api::create_figment(&self.config_sources)
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

/// Used internally by gravel, do not use in plugins.
#[doc(hidden)]
pub mod __private_api {
	use abi_stable::std_types::RString;
	use abi_stable::StableAbi;
	use figment::providers::{Format, Yaml};
	use figment::Figment;

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
	#[derive(StableAbi, Debug)]
	pub struct ConfigLayer(pub ConfigSource, pub MergeStrategy);

	#[repr(u8)]
	#[derive(StableAbi, Debug)]
	pub enum MergeStrategy {
		Merge,
		AdMerge,
	}

	#[repr(u8)]
	#[derive(StableAbi, Debug)]
	pub enum ConfigSource {
		String(RString), // TODO: refactor with RStr<'static>
		File(RString),
	}
}
