use crate::{PluginDefinition, logging::BoxDynLogTarget};
use abi_stable::{StableAbi, library::RootModule, sabi_types::VersionStrings, std_types::RVec};
use abi_stable::{declare_root_module_statics, package_version_strings};

/// [`RootModule`] for a plugin. It is auto-implemented using the macros.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginLibRef)))]
pub struct PluginLib {
	#[sabi(last_prefix_field)]
	pub plugin: extern "C" fn(log_target: BoxDynLogTarget) -> RVec<PluginDefinition>,
}

impl RootModule for PluginLibRef {
	declare_root_module_statics! {PluginLibRef}

	const BASE_NAME: &'static str = "gravel_ffi";
	const NAME: &'static str = "gravel_ffi";
	const VERSION_STRINGS: VersionStrings = package_version_strings!();
}
