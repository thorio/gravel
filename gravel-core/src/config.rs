//! For an explanation of the config, see `config.yml` in the crate's root.

use abi_stable::{std_types::RString, traits::IntoReprC};
use gravel_ffi::config::{create_figment, ConfigLayer};
use gravel_ffi::{FrontendMessage, PluginConfigAdapter};
use nameof::name_of;
use serde::Deserialize;

pub const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../config.yml"));

pub struct ConfigManager {
	root: RootConfig,
	config_sources: Vec<ConfigLayer>,
}

impl ConfigManager {
	pub fn new(config_sources: Vec<ConfigLayer>) -> Self {
		let root = match create_figment(&config_sources).extract() {
			Ok(root) => root,
			Err(err) => {
				log::error!("error in config: {err}");
				std::process::exit(1);
			}
		};

		Self { config_sources, root }
	}

	pub fn root(&self) -> &RootConfig {
		&self.root
	}

	pub fn adapt_provider(&self, index: usize) -> PluginConfigAdapter<'_> {
		self.adapt(format!("{}.{index}", name_of!(providers in RootConfig)))
	}

	pub fn adapt_frontend(&self) -> PluginConfigAdapter<'_> {
		self.adapt(name_of!(frontend in RootConfig))
	}

	fn adapt(&self, key: impl Into<RString>) -> PluginConfigAdapter<'_> {
		PluginConfigAdapter::from(key.into(), self.config_sources.as_slice().into_c())
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

impl From<HotkeyAction> for FrontendMessage {
	fn from(value: HotkeyAction) -> Self {
		match value {
			HotkeyAction::ShowHide => Self::ShowOrHide,
			HotkeyAction::Show => Self::Show,
			HotkeyAction::Hide => Self::Hide,
			HotkeyAction::ShowWith(query) => Self::ShowWithQuery(query.into_c()),
		}
	}
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
	// Technically expected here but is deserialized differently, see `gravel_ffi::PluginConfigAdapter`
	//pub config: Any,
}
