use abi_stable::external_types::crossbeam_channel::RSender;
use gravel_core::ipc::server::{Server, ServerHandle};
use gravel_core::{config, ipc::get_name, CoreMessage};

/// Attempts to spawn the IPC server.
/// Returns [`None`] if the server was not spawned.
pub fn ipc(config: &config::Ipc, sender: RSender<CoreMessage>) -> Option<ServerHandle> {
	Server::start(get_name(config), sender)
		.map(Server::spawn)
		.inspect_err(|e| log::error!("unable to spawn ipc server: {e}"))
		.ok()
}
