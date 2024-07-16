//! For an explanation of the config, see `config.yml` in the crate's root.

use abi_stable::{std_types::RString, traits::IntoReprC};
use gravel_ffi::config::__private_api::{create_figment, ConfigLayer};
use gravel_ffi::{FrontendMessage, PluginConfigAdapter};
use nameof::name_of;
use serde::Deserialize;

pub const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../config.yml"));

pub struct ConfigManager {
	root: Config,
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

	pub fn root(&self) -> &Config {
		&self.root
	}

	pub fn adapt_provider(&self, index: usize) -> PluginConfigAdapter<'_> {
		self.adapt(format!("{}.{index}", name_of!(providers in Config)))
	}

	pub fn adapt_frontend(&self) -> PluginConfigAdapter<'_> {
		self.adapt(name_of!(frontend in Config))
	}

	fn adapt(&self, key: impl Into<RString>) -> PluginConfigAdapter<'_> {
		PluginConfigAdapter::from(key.into(), self.config_sources.as_slice().into_c())
	}
}

#[derive(Debug, Deserialize)]
pub struct Config {
	pub single_instance: Option<String>,
	pub external_plugins: ExternalPlugins,
	pub hotkeys: Vec<Hotkey>,
	pub frontend: Frontend,
	pub providers: Vec<Provider>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalPlugins {
	Disabled,
	All,
	Whitelist(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct Hotkey {
	pub binding: String,
	pub action: HotkeyAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotkeyAction {
	// TODO: remove aliases on next breaking
	#[serde(alias = "ShowHide")]
	ShowHide,
	#[serde(alias = "Show")]
	Show,
	#[serde(alias = "Hide")]
	Hide,
	#[serde(alias = "ShowWith")]
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
pub struct Frontend {
	pub plugin: String,
}

#[derive(Debug, Deserialize)]
pub struct Provider {
	pub plugin: String,
	pub keyword: Option<String>,
	// Technically expected here but is deserialized differently, see [`gravel_ffi::PluginConfigAdapter`]
	//pub config: Any,
}
