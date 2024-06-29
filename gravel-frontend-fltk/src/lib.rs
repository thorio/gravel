//! gravel's default frontend, based on fltk.

use abi_stable::sabi_extern_fn;
use gravel_ffi::prelude::*;
use ui::FltkFrontend;

mod builder;
mod config;
mod scroll;
mod scrollbar;
mod structs;
mod ui;

#[cfg_attr(target_os = "linux", path = "native/linux.rs")]
#[cfg_attr(windows, path = "native/windows.rs")]
mod native;

#[cfg(not(feature = "no-root"))]
#[abi_stable::export_root_module]
pub fn get_library() -> PluginLibRef {
	use abi_stable::prefix_type::PrefixTypeTrait;
	PluginLib { plugin: get_plugin }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_plugin() -> PluginDefinition {
	PluginMetadata::new("fltk").with_frontend(FltkFrontend::create)
}
