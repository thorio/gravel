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
	fn query(&self, _query: &str) -> ProviderResult {
		ProviderResult::new(implementation::query().unwrap())
	}
}

pub(crate) fn get_hit(name: &str, pid: Pid, cmdline: &str) -> Box<dyn Hit> {
	let title = format!("{} - {}", name, pid);

	let hit = SimpleHit::new(title, cmdline, move |_, s| do_kill(s, pid));
	Box::new(hit)
}

fn do_kill(sender: &Sender<FrontendMessage>, pid: Pid) {
	// We don't care if this fails
	implementation::kill_process(pid).unwrap_or(());

	sender
		.send(FrontendMessage::Hide)
		.expect("receiver should live for the lifetime of the program");
}
