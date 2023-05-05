use crate::{config::PluginConfigAdapter, Frontend, Provider, QueryEngine};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PluginType {
	Provider,
	Frontend,
}

type ProviderFactory = dyn Fn(&PluginConfigAdapter) -> Box<dyn Provider>;
type FrontendFactory = dyn Fn(QueryEngine, &PluginConfigAdapter) -> Box<dyn Frontend>;

/// Holds metadata about a frontend or provider, as well as
/// a way to instantiate them.
pub struct PluginDefinition {
	pub name: String,
	pub plugin_type: PluginType,
	provider: Option<Box<ProviderFactory>>,
	frontend: Option<Box<FrontendFactory>>,
	has_plugin: bool,
}

impl PluginDefinition {
	/// Creates a new instance with the given name.
	///
	/// The plugin *must* be further defined using the `with_` functions,
	/// otherwise the definition is invalid.
	pub fn new(name: &str) -> Self {
		Self {
			name: String::from(name),
			plugin_type: PluginType::Provider,
			provider: None,
			frontend: None,
			has_plugin: false,
		}
	}

	/// Assigns a [`Provider`] to the definition.
	///
	/// Panics the definition has already been assigned a plugin.
	pub fn with_provider(mut self, get_fn: impl Fn(&PluginConfigAdapter) -> Box<dyn Provider> + 'static) -> Self {
		assert!(!self.has_plugin, "cannot assign multiple plugin types");

		self.provider = Some(Box::new(get_fn));
		self.plugin_type = PluginType::Provider;
		self.has_plugin = true;
		self
	}

	/// Assigns a [`Frontend`] to the definition.
	///
	/// Panics the definition has already been assigned a plugin.
	pub fn with_frontend(
		mut self,
		get_fn: impl Fn(QueryEngine, &PluginConfigAdapter) -> Box<dyn Frontend> + 'static,
	) -> Self {
		assert!(!self.has_plugin, "cannot assign multiple plugin types");

		self.frontend = Some(Box::new(get_fn));
		self.plugin_type = PluginType::Frontend;
		self.has_plugin = true;
		self
	}

	/// Attempts to instantiate a [`Provider`].
	pub fn get_provider(&self, config: &PluginConfigAdapter) -> Option<Box<dyn Provider>> {
		let get_fn = self.provider.as_ref()?;
		Some(get_fn(config))
	}

	/// Attempts to instantiate a [`Frontend`].
	pub fn get_frontend(&self, engine: QueryEngine, config: &PluginConfigAdapter) -> Option<Box<dyn Frontend>> {
		let get_fn = self.frontend.as_ref()?;
		Some(get_fn(engine, config))
	}

	/// Returns if the definition has been assigned a plugin.
	pub fn has_plugin(&self) -> bool {
		self.has_plugin
	}
}

/// Facilitates registering and finding plugins.
#[derive(Default)]
pub struct PluginRegistry {
	plugins: Vec<PluginDefinition>,
}

impl PluginRegistry {
	/// Registers the plugin.
	///
	/// If the plugin is incorrectly defined or another plugin with indentical
	/// name and type is already registered, an error is logged and the plugin
	/// is skipped.
	pub fn register(&mut self, plugin: PluginDefinition) -> &mut Self {
		if !plugin.has_plugin() {
			println!("malformed plugin {}, skipping", plugin.name);
			return self;
		}

		if self.find_plugin(&plugin.name, plugin.plugin_type).is_some() {
			println!("duplicate {:?} \"{}\", skipping", plugin.plugin_type, plugin.name);
			return self;
		}

		self.plugins.push(plugin);
		self
	}

	/// Attempts to retrieve a provider plugin with the given name.
	pub fn get_provider(&self, name: &str) -> Option<&PluginDefinition> {
		self.find_plugin(name, PluginType::Provider)
	}

	/// Attempts to retrieve a frontend plugin with the given name.
	pub fn get_frontend(&self, name: &str) -> Option<&PluginDefinition> {
		self.find_plugin(name, PluginType::Frontend)
	}

	fn find_plugin(&self, name: &str, plugin_type: PluginType) -> Option<&PluginDefinition> {
		let plugin = self
			.plugins
			.iter()
			.find(|plugin| plugin.plugin_type == plugin_type && plugin.name == name)?;

		Some(plugin)
	}
}
