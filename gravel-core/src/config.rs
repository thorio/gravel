//! For an explanation of the config, see `config.yml` in the crate's root.

use abi_stable::std_types::RString;
use gravel_ffi::prelude::*;
use nameof::name_of;
use serde::Deserialize;

pub const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../config.yml"));
pub const PROVIDERS: &str = name_of!(providers in RootConfig);
pub const FRONTEND: &str = name_of!(frontend in RootConfig);

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

impl From<&HotkeyAction> for FrontendMessage {
	fn from(value: &HotkeyAction) -> Self {
		match value {
			HotkeyAction::ShowHide => Self::ShowOrHide,
			HotkeyAction::Show => Self::Show,
			HotkeyAction::Hide => Self::Hide,
			HotkeyAction::ShowWith(query) => Self::ShowWithQuery(RString::from(query.as_str())),
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
