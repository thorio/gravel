//! gravel's process killer
//! Lists running processes on your system and will allow you to kill them.

use abi_stable::{external_types::crossbeam_channel::RSender, sabi_extern_fn, std_types::RStr};
use gravel_ffi::prelude::*;
use implementation::Pid;
use itertools::Itertools;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

#[cfg(not(feature = "no-root"))]
#[abi_stable::export_root_module]
pub fn get_library() -> PluginLibRef {
	use abi_stable::prefix_type::PrefixTypeTrait;
	PluginLib { plugin: get_plugin }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_plugin() -> PluginDefinition {
	PluginMetadata::new("kill").with_provider(get_provider)
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
				log::error!("unable to query running processes: {err}");
				return ProviderResult::empty();
			}
		};

		let hits = hits.map(HitExt::into_dyn).collect_vec();
		ProviderResult::new(hits)
	}
}

pub(crate) fn get_hit(name: &str, pid: Pid, cmdline: &str) -> SimpleHit {
	let title = format!("{name} - {pid}");

	SimpleHit::new(title, cmdline, move |_h, s| do_kill(s, pid))
}

fn do_kill(sender: &RSender<FrontendMessage>, pid: Pid) {
	log::debug!("attempting to kill PID {pid}");

	implementation::kill_process(pid)
		.inspect_err(|e| log::error!("unable to kill PID {pid}: {e}"))
		.ok();

	sender.send(FrontendMessage::Refresh).ok();
}
