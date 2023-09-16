//! gravel's process killer
//!
//! TODO!

use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use implementation::Pid;
use serde::Deserialize;
use std::sync::mpsc::Sender;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));

pub fn register_plugins(registry: &mut PluginRegistry) {
	let definition = PluginDefinition::new("kill").with_provider(get_provider);

	registry.register(definition);
}

fn get_provider(config: &PluginConfigAdapter) -> Box<dyn Provider> {
	let plugin_config = config.get::<Config>(DEFAULT_CONFIG);

	Box::new(KillProvider::new(plugin_config))
}

#[allow(unused)]
pub struct KillProvider {
	config: Config,
}

impl KillProvider {
	fn new(config: Config) -> Self {
		Self { config }
	}
}

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

#[derive(Deserialize, Debug)]
struct Config {
	//
}
