#![allow(single_use_lifetimes)]

use abi_stable::std_types::{RString, RVec};
use abi_stable::StableAbi;
use figment::providers::{Format, Yaml};
use figment::Figment;
use serde::Deserialize;

#[repr(C)]
#[derive(StableAbi)]
pub struct ConfigManager {
	config: RVec<ConfigLayer>,
}

impl ConfigManager {
	pub fn new(placeholder: impl Into<RVec<ConfigLayer>>) -> Self {
		Self {
			config: placeholder.into(),
		}
	}

	// I'd really like to keep the figment and re-use it for plugins,
	// but unfortunately it can't pass through FFI-boundaries.
	pub(crate) fn figment(&self) -> Figment {
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

		self.config.iter().fold(Figment::new(), fold)
	}

	pub fn root<'a, T: Deserialize<'a>>(&self) -> T {
		match self.figment().extract() {
			Ok(root) => root,
			Err(err) => {
				log::error!("config: {err}");
				std::process::exit(1);
			}
		}
	}

	pub fn adapt(&self, key: impl Into<RString>) -> PluginConfigAdapter<'_> {
		PluginConfigAdapter {
			key: key.into(),
			manager: self,
		}
	}
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
	manager: &'a ConfigManager,
}

impl PluginConfigAdapter<'_> {
	/// Build and deserialize the plugin's config into the given type.
	pub fn get<'de, T: Deserialize<'de>>(&self, default_config: &str) -> T {
		log::trace!("reading plugin config for {}", self.key);

		// layer the plugins' defaults under the provider's config section
		let figment = self
			.manager
			.figment()
			.focus(&format!("{}.config", self.key))
			.join(Yaml::string(default_config));

		match figment.extract() {
			Ok(config) => config,
			Err(err) => {
				log::error!("plugin config {}: {err}", self.key);
				std::process::exit(1);
			}
		}
	}
}
