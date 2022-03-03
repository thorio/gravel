use config::{Config, File, FileFormat};
use serde::Deserialize;

static DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/default.yml"));

#[cfg(target_os = "linux")]
static CONFIG_PATH: &str = "${XDG_CONFIG_HOME:-$HOME/.config}/gravel";
#[cfg(windows)]
static CONFIG_PATH: &str = "${XDG_CONFIG_HOME:-$USERPROFILE/.config}/gravel";

pub fn config() -> RootConfig {
	let user_config_path = format!("{}/user.yml", shellexpand::env(CONFIG_PATH).unwrap());

	let mut config = Config::builder()
		.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
		.add_source(File::with_name(&user_config_path).required(false));

	// dev-only config layer, for example to use a different hotkey for the dev instance
	#[cfg(debug_assertions)]
	{
		let dev_config_path = format!("{}/dev.yml", shellexpand::env(CONFIG_PATH).unwrap());

		config = config.add_source(File::with_name(&dev_config_path).required(false));
	}

	config.build().unwrap().try_deserialize().unwrap()
}

#[derive(Debug, Deserialize)]
pub struct RootConfig {
	pub hotkeys: Vec<Hotkey>,
}

#[derive(Debug, Deserialize)]
pub struct Hotkey {
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
