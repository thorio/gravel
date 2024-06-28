use crate::{config::PluginConfigAdapter, BoxDynFrontend, BoxDynProvider, BoxDynQueryEngine};
use abi_stable::{std_types::RString, StableAbi};

pub type ProviderFactory = extern "C" fn(&PluginConfigAdapter<'_>) -> BoxDynProvider;
pub type FrontendFactory = extern "C" fn(BoxDynQueryEngine, &PluginConfigAdapter<'_>) -> BoxDynFrontend;

#[repr(u8)]
#[derive(StableAbi)]
pub enum PluginFactory {
	Provider(extern "C" fn(&PluginConfigAdapter<'_>) -> BoxDynProvider),
	Frontend(extern "C" fn(BoxDynQueryEngine, &PluginConfigAdapter<'_>) -> BoxDynFrontend),
}

impl PluginFactory {
	pub fn provider(&self) -> Option<ProviderFactory> {
		if let Self::Provider(factory) = self {
			return Some(*factory);
		}

		None
	}

	pub fn frontend(&self) -> Option<FrontendFactory> {
		if let Self::Frontend(factory) = self {
			return Some(*factory);
		}

		None
	}
}

/// Holds metadata about a frontend or provider, as well as
/// a way to construct them.
#[repr(C)]
#[derive(StableAbi)]
pub struct PluginDefinition {
	pub meta: PluginMetadata,
	pub factory: PluginFactory,
}

#[repr(C)]
#[derive(StableAbi)]
pub struct PluginMetadata {
	pub name: RString,
}

impl PluginMetadata {
	#[must_use]
	pub fn new(name: impl Into<RString>) -> Self {
		Self { name: name.into() }
	}

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
