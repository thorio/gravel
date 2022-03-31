//! For an explanation of the config, see `config.yml` in the crate's root.

use ::config::{Config, File, FileFormat};
use serde::Deserialize;

/// Manages a [`Config`] and allows building a plugin's config.
pub struct ConfigManager {
	config: Config,
	pub root: RootConfig,
}

impl ConfigManager {
	pub fn new(config: Config) -> Self {
		// TODO: error handling
		let root = config.clone().try_deserialize().unwrap();

		ConfigManager { config, root }
	}

	pub fn get_plugin_adapter(&self, alias: &str) -> PluginConfigAdapter {
		PluginConfigAdapter {
			alias: alias.to_owned(),
			config: self,
		}
	}

	/// Build and deserialize the specified plugin config section.
	pub fn get_plugin_config<'de, T: Deserialize<'de>>(&self, alias: &str, default_config: &str) -> T {
		let processed_config = preprocess_plugin_config(default_config, alias);

		// layer the cached config over the plugins' defaults
		let builder = Config::builder()
			.add_source(File::from_str(&processed_config, FileFormat::Yaml))
			.add_source(self.config.clone());

		// TODO: error handling
		let key = format!("plugin_config.{}", alias);
		builder.build().unwrap().get(key.as_str()).unwrap()
	}
}

/// Allows a plugin to deserialize its config without knowing the config alias.
pub struct PluginConfigAdapter<'a> {
	alias: String,
	config: &'a ConfigManager,
}

impl<'a> PluginConfigAdapter<'a> {
	/// Build and deserialize the plugin's config into the given type.
	pub fn get<'de, T: Deserialize<'de>>(&self, default_config: &str) -> T {
		self.config.get_plugin_config(&self.alias, default_config)
	}

	/// Borrow the gravel's main config.
	pub fn get_root(&self) -> &RootConfig {
		&self.config.root
	}
}

/// Modifies the YAML to place it in the same "path" as the user's config.
fn preprocess_plugin_config(config: &str, alias: &str) -> String {
	// This entire function is incredibly sketchy, but I haven't found
	// a better alternative.

	// indent the entire config to place it two levels further down
	let indented = config
		.lines()
		.map(prepend_two_spaces)
		.collect::<Vec<String>>()
		.join("\n");

	// then prepend this to place it in the same config section as in the
	// user's config for this plugin
	let mut new_config = format!("plugin_config:\n {}:\n", alias);

	new_config.push_str(&indented);

	new_config
}

fn prepend_two_spaces(string: &str) -> String {
	let mut new = "  ".to_owned();
	new.push_str(string);

	new
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
	pub plugin: String,
	pub alias: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
	pub plugin: String,
	pub alias: Option<String>,
	pub keyword: Option<String>,
}
