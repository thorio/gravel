use self::{engine::engine, frontend::frontend, hotkeys::hotkeys, ipc::ipc, plugins::plugins};
use abi_stable::external_types::crossbeam_channel;
use anyhow::Result;
use gravel_core::ipc::client;
use gravel_core::{config::ConfigManager, ipc::server::ServerHandle, plugin::PluginRegistry};
use gravel_core::{Core, CoreMessage, FrontendCtx};
use gravel_ffi::{BoxDynFrontend, FrontendMessageNe};

mod config;
mod engine;
mod frontend;
mod hotkeys;
mod ipc;
mod logging;
mod panic;
mod plugins;

pub use self::{config::config, logging::logging, panic::panic};

pub fn is_duplicate_instance(name: impl Into<String>) -> Result<bool> {
	let mut conn = client::connect(name)?;

	Ok(client::ping(&mut conn)?)
}

pub fn init(config: &ConfigManager) -> (Core, Option<ServerHandle>, BoxDynFrontend) {
	let registry = plugins(&config.root().external_plugins);

	let (core, frontend_ctx, ipc) = init_core(&registry, config);

	let frontend = frontend(&registry, frontend_ctx, config);

	(core, ipc, frontend)
}

fn init_core(registry: &PluginRegistry, config: &ConfigManager) -> (Core, FrontendCtx, Option<ServerHandle>) {
	let (frontend_send, frontend_recv) = crossbeam_channel::bounded::<FrontendMessageNe>(16);
	let (core_send, core_recv) = crossbeam_channel::bounded::<CoreMessage>(16);

	let ipc = ipc(&config.root().ipc, core_send.clone());
	let engine = engine(core_send.clone(), registry, config);
	let runner = Core::new(engine, frontend_send.clone(), core_recv);
	let frontend_ctx = FrontendCtx::new(frontend_recv, core_send.clone());
	hotkeys(&config.root().hotkeys, core_send);

	(runner, frontend_ctx, ipc)
}
