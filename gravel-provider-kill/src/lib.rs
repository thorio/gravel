//! gravel's process killer
//! Lists running processes on your system and will allow you to kill them.

use gravel_ffi::prelude::*;
use implementation::Pid;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

pub struct KillProvider;

#[gravel_provider("kill")]
impl ProviderDef for KillProvider {
	fn new(_config: &PluginConfigAdapter<'_>) -> Self {
		Self {}
	}

	fn query(&self, _query: &str) -> ProviderResult {
		let hits = match implementation::query() {
			Ok(hits) => hits,
			Err(err) => {
				log::error!("unable to query running processes: {err}");
				return ProviderResult::empty();
			}
		};

		ProviderResult::new(hits)
	}
}

pub(crate) fn get_hit(name: &str, pid: Pid, cmdline: &str) -> SimpleHit {
	let title = format!("{name} - {pid}");

	SimpleHit::new(title, cmdline, move |_, ctx| do_kill(pid, ctx))
}

fn do_kill(pid: Pid, context: &BoxDynHitActionContext) {
	log::debug!("attempting to kill PID {pid}");

	implementation::kill_process(pid)
		.inspect_err(|e| log::error!("unable to kill PID {pid}: {e}"))
		.ok();

	context.refresh_frontend();
}
