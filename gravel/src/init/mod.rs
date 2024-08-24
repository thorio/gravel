mod config;
mod engine;
mod frontend;
mod hotkeys;
mod logging;
mod panic;
mod plugins;
mod single_instance;

pub use self::config::config;
pub use self::single_instance::single_instance;
pub use engine::engine;
pub use frontend::frontend;
pub use hotkeys::hotkeys;
pub use logging::logging;
pub use panic::panic;
pub use plugins::plugins;

use ::single_instance::SingleInstance;
use abi_stable::external_types::crossbeam_channel;
use gravel_core::{config::ConfigManager, plugin::PluginRegistry, Core, CoreMessage, FrontendCtx};
use gravel_ffi::{BoxDynFrontend, FrontendMessageNe};

pub fn init(config: &ConfigManager) -> (Core, Option<SingleInstance>, BoxDynFrontend) {
	let registry = plugins(&config.root().external_plugins);

	let (core, frontend_ctx, ipc) = init_core(&registry, config);

	let frontend = frontend(&registry, frontend_ctx, config);

	(core, ipc, frontend)
}

fn init_core(registry: &PluginRegistry, config: &ConfigManager) -> (Core, FrontendCtx, Option<SingleInstance>) {
	let (frontend_send, frontend_recv) = crossbeam_channel::bounded::<FrontendMessageNe>(16);
	let (core_send, core_recv) = crossbeam_channel::bounded::<CoreMessage>(16);

	let ipc = single_instance(config.root().single_instance.as_deref());

	let engine = engine(core_send.clone(), registry, config);
	let runner = Core::new(engine, frontend_send.clone(), core_recv);
	let frontend_ctx = FrontendCtx::new(frontend_recv, core_send.clone());
	hotkeys(&config.root().hotkeys, core_send);

	(runner, frontend_ctx, ipc)
}
