//! gravel's process killer
//! Lists running processes on your system and will allow you to kill them.

use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use implementation::Pid;
use std::sync::mpsc::Sender;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("kill").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(_config: &PluginConfigAdapter) -> Box<dyn Provider> {
	Box::new(KillProvider {})
}

pub struct KillProvider {}

impl Provider for KillProvider {
	fn query(&self, _query: &str) -> QueryResult {
		QueryResult::new(implementation::query().unwrap())
	}
}

pub(crate) fn get_hit(name: &str, pid: Pid, cmdline: &str) -> Box<dyn Hit> {
	let title = format!("{} - {}", name, pid);
	let data = HitData::new(&title, cmdline);
	let extra = ExtraData { pid };

	Box::new(SimpleHit::new_extra(data, extra, do_kill))
}

struct ExtraData {
	pub pid: Pid,
}

fn do_kill(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	// We don't care if this fails
	implementation::kill_process(hit.get_extra_data().pid).unwrap_or(());

	sender
		.send(FrontendMessage::Hide)
		.expect("receiver should live for the lifetime of the program");
}
