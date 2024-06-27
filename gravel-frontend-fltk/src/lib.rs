//! gravel's default frontend, based on fltk.

use abi_stable::sabi_extern_fn;
use gravel_ffi::{plugin, BoxDynFrontend, BoxDynQueryEngine, FrontendExt, PluginConfigAdapter, PluginDefinition};
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

pub fn get_plugin() -> PluginDefinition {
	plugin("fltk").with_frontend(get_frontend)
}

#[sabi_extern_fn]
fn get_frontend(engine: BoxDynQueryEngine, config: &PluginConfigAdapter<'_>) -> BoxDynFrontend {
	FltkFrontend::new(engine, config::get(config)).into_dyn()
}
