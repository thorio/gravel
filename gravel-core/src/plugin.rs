use crate::{config::PluginConfigAdapter, *};
use std::collections::HashMap;

pub type ProviderFactory = Box<dyn Fn(&PluginConfigAdapter) -> Box<dyn Provider>>;
pub type FrontendFactory = Box<dyn Fn(QueryEngine, &PluginConfigAdapter) -> Box<dyn Frontend>>;

pub enum PluginFactory {
	Provider(ProviderFactory),
	Frontend(FrontendFactory),
}

/// Holds metadata about a frontend or provider, as well as
/// a way to construct them.
pub struct PluginDefinition {
	pub meta: PluginMetadata,
	pub factory: PluginFactory,
}

pub struct PluginMetadata {
	pub name: String,
}

impl PluginMetadata {
	#[must_use]
	pub fn with_provider(self, factory: ProviderFactory) -> PluginDefinition {
		PluginDefinition {
			meta: self,
			factory: PluginFactory::Provider(factory),
		}
	}

	#[must_use]
	pub fn with_frontend(self, factory: FrontendFactory) -> PluginDefinition {
		PluginDefinition {
			meta: self,
			factory: PluginFactory::Frontend(factory),
		}
	}
}

#[must_use]
pub fn plugin(name: impl Into<String>) -> PluginMetadata {
	PluginMetadata { name: name.into() }
}

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
		let name = &plugin.meta.name;

		if self.plugins.contains_key(name) {
			log::warn!("attempted to register duplicate plugin '{}', skipping", name);
			return self;
		}

		self.plugins.insert(name.clone(), plugin);
		self
	}

	pub fn get_plugin(&self, name: &str) -> Option<&PluginDefinition> {
		self.plugins.get(name)
	}
}
