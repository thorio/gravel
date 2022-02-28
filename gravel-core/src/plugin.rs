use crate::provider::Provider;

pub struct PluginRegistry {
	pub providers: Vec<Box<dyn Provider>>,
}

impl PluginRegistry {
	fn new() -> Self {
		PluginRegistry { providers: vec![] }
	}

	pub fn provider(&mut self, provider: Box<dyn Provider>) -> &mut Self {
		self.providers.push(provider);
		self
	}
}

/// placeholder
pub fn load_plugins() -> PluginRegistry {
	PluginRegistry::new()
}
