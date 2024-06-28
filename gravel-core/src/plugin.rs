use gravel_ffi::prelude::*;
use std::collections::HashMap;

/// Facilitates registering and finding plugins.
#[derive(Default)]
pub struct PluginRegistry {
	plugins: HashMap<String, PluginDefinition>,
}

impl PluginRegistry {
	/// Registers the plugin.
	///
	/// If the plugin is incorrectly defined or another plugin with identical
	/// name and type is already registered, an error is logged and the plugin
	/// is skipped.
	pub fn register(&mut self, plugin: PluginDefinition) -> &mut Self {
		let name = plugin.meta.name.as_str();

		if self.plugins.contains_key(name) {
			log::warn!("attempted to register duplicate plugin '{}', skipping", name);
			return self;
		}

		self.plugins.insert(name.to_owned(), plugin);
		self
	}

	#[must_use]
	pub fn get(&self, name: &str) -> Option<&PluginDefinition> {
		self.plugins.get(name)
	}
}
