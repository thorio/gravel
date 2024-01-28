//! gravel's default frontend, based on fltk.

use crate::config::get_config;
use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use implementation::FltkFrontend;

mod builder;
mod config;
mod implementation;
mod scroll;
mod scrollbar;
mod structs;

#[cfg_attr(target_os = "linux", path = "native/linux.rs")]
#[cfg_attr(windows, path = "native/windows.rs")]
mod native;

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = plugin("fltk").with_frontend(Box::new(get_frontend));

	registry.register(definition);
}

fn get_frontend(engine: QueryEngine, config: &PluginConfigAdapter) -> Box<dyn Frontend> {
	Box::new(FltkFrontend::new(engine, get_config(config)))
}
