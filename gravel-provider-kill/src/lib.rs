//! gravel's process killer
//! Lists running processes on your system and will allow you to kill them.

use abi_stable::{external_types::crossbeam_channel::RSender, sabi_extern_fn, std_types::RStr};
use gravel_ffi::{
	plugin, BoxDynProvider, FrontendMessage, HitExt, PluginConfigAdapter, PluginDefinition, Provider, ProviderExt,
	ProviderResult, SimpleHit,
};
use implementation::Pid;
use itertools::Itertools;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

pub fn get_plugin() -> PluginDefinition {
	plugin("kill").with_provider(get_provider)
}

#[sabi_extern_fn]
fn get_provider(_config: &PluginConfigAdapter<'_>) -> BoxDynProvider {
	KillProvider {}.into_dyn()
}

pub struct KillProvider;

impl Provider for KillProvider {
	fn query(&self, _query: RStr<'_>) -> ProviderResult {
		let hits = match implementation::query() {
			Ok(hits) => hits,
			Err(err) => {
				log::error!("couldn't query running processes: {err}");
				return ProviderResult::empty();
			}
		};

		let hits = hits.map(HitExt::into_dyn).collect_vec();
		ProviderResult::new(hits)
	}
}

pub(crate) fn get_hit(name: &str, pid: Pid, cmdline: &str) -> SimpleHit {
	let title = format!("{name} - {pid}");

	SimpleHit::new(title, cmdline, move |s| do_kill(s, pid))
}

fn do_kill(sender: &RSender<FrontendMessage>, pid: Pid) {
	log::debug!("attempting to kill PID {pid}");

	if let Err(err) = implementation::kill_process(pid) {
		log::error!("killing PID {pid} failed: {err}");
	}

	sender.send(FrontendMessage::Refresh).ok();
}
