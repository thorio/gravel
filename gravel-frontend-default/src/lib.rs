//! gravel's default frontend, based on fltk.

use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use implementation::DefaultFrontend;

mod builder;
mod constants;
mod implementation;
mod scroll;
mod scrollbar;
mod structs;

#[cfg_attr(target_os = "linux", path = "native/linux.rs")]
#[cfg_attr(windows, path = "native/windows.rs")]
mod native;

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("default").with_frontend(get_frontend);

	registry.register(definition);
}

fn get_frontend(engine: QueryEngine, _config: &PluginConfigAdapter) -> Box<dyn Frontend> {
	Box::new(DefaultFrontend::new(engine))
}
