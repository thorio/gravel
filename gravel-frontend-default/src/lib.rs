//! gravel's default frontend, based on fltk.

use gravel_core::plugin::*;
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
	let definition = PluginDefinition::new("default").with_frontend(|engine| Box::new(DefaultFrontend::new(engine)));

	registry.register(definition);
}
