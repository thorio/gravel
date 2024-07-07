use crate::frontend::BoxDynFrontend;
use crate::provider::BoxDynProvider;
use crate::{config::PluginConfigAdapter, BoxDynFrontendContext};
use abi_stable::{std_types::RString, StableAbi};

pub type ProviderFactory = extern "C" fn(&PluginConfigAdapter<'_>) -> BoxDynProvider;
pub type FrontendFactory = extern "C" fn(BoxDynFrontendContext, &PluginConfigAdapter<'_>) -> BoxDynFrontend;

/// Factory functions for the different plugin types.
#[repr(u8)]
#[derive(StableAbi, Debug)]
pub enum PluginFactory {
	// abi_stable doesn't let me use the type aliases ;_;
	Provider(extern "C" fn(&PluginConfigAdapter<'_>) -> BoxDynProvider),
	Frontend(extern "C" fn(BoxDynFrontendContext, &PluginConfigAdapter<'_>) -> BoxDynFrontend),
}

impl PluginFactory {
	/// Returns the provider factory or [`None`].
	pub fn provider(&self) -> Option<ProviderFactory> {
		if let Self::Provider(factory) = self {
			return Some(*factory);
		}

		None
	}

	/// Returns the frontend factory or [`None`].
	pub fn frontend(&self) -> Option<FrontendFactory> {
		if let Self::Frontend(factory) = self {
			return Some(*factory);
		}

		None
	}
}

/// Holds metadata and a factory function for a plugin.
#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct PluginDefinition {
	pub meta: PluginMetadata,
	pub factory: PluginFactory,
}

/// Holds metadata about a plugin.
#[repr(C)]
#[derive(StableAbi, Debug)]
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
