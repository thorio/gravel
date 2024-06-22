//! gravel's default frontend, based on fltk.

use gravel_core::config::PluginConfigAdapter;
use gravel_core::plugin::{plugin, PluginRegistry};
use gravel_core::{Frontend, QueryEngine};
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

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = plugin("fltk").with_frontend(Box::new(get_frontend));

	registry.register(definition);
}

fn get_frontend(engine: QueryEngine, config: &PluginConfigAdapter<'_>) -> Box<dyn Frontend> {
	Box::new(FltkFrontend::new(engine, config::get(config)))
}
