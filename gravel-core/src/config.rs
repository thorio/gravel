//! For an explanation of the config, see `config.yml` in the crate's root.

use figment::providers::{Format, Yaml};
use figment::Figment;
use nameof::name_of;
use serde::Deserialize;

pub const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../config.yml"));

/// Manages a [`Config`] and allows building a plugin's config.
pub struct ConfigManager {
	figment: Figment,
	pub root: RootConfig,
}

impl ConfigManager {
	pub fn new(figment: Figment) -> Self {
		match figment.extract() {
			Ok(root) => Self { figment, root },
			Err(err) => {
				log::error!("config: {err}");
				std::process::exit(1);
			}
		}
	}

	pub fn get_provider_adapter(&self, index: usize) -> PluginConfigAdapter {
		self.get_plugin_adapter(format!("{}.{index}", name_of!(providers in RootConfig)))
	}

	pub fn get_frontend_adapter(&self) -> PluginConfigAdapter {
		self.get_plugin_adapter(name_of!(frontend in RootConfig))
	}

	fn get_plugin_adapter(&self, key: impl Into<Box<str>>) -> PluginConfigAdapter {
		PluginConfigAdapter {
			key: key.into(),
			figment: &self.figment,
		}
	}
}

/// Allows a plugin to deserialize its config without
/// knowing where in the main config it is.
pub struct PluginConfigAdapter<'a> {
	key: Box<str>,
	figment: &'a Figment,
}

impl<'a> PluginConfigAdapter<'a> {
	/// Build and deserialize the plugin's config into the given type.
	pub fn get<'de, T: Deserialize<'de>>(&self, default_config: &str) -> T {
		log::trace!("reading plugin config for {}", self.key);

		// layer the plugins' defaults under the provider's config section
		let figment = self
			.figment
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

#[derive(Debug, Deserialize)]
pub struct RootConfig {
	pub single_instance: Option<String>,
	pub hotkeys: Vec<HotkeyConfig>,
	pub frontend: FrontendConfig,
	pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Deserialize)]
pub struct HotkeyConfig {
	pub binding: String,
	pub action: HotkeyAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum HotkeyAction {
	ShowHide,
	Show,
	Hide,
	ShowWith(String),
}

#[derive(Debug, Deserialize)]
pub struct FrontendConfig {
	pub plugin: String,
	pub alias: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
	pub plugin: String,
	pub keyword: Option<String>,
	// Technically expected here but is deserialized differently, see PluginConfigAdapter
	//pub config: Any,
}
