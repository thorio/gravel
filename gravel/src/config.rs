//! For an explanation of the config, see `config.yml` in the crate's root.

use serde::Deserialize;

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
	pub query: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum HotkeyAction {
	ShowHide,
	Show,
	Hide,
	ShowWith,
}

#[derive(Debug, Deserialize)]
pub struct FrontendConfig {
	pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
	pub name: String,
	pub keyword: Option<String>,
}
