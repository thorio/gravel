//! gravel's process killer
//!
//! TODO!

use gravel_core::{config::PluginConfigAdapter, plugin::*, *};
use serde::Deserialize;
use std::sync::mpsc::Sender;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

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

pub struct KillProvider {
	config: Config,
}

impl KillProvider {
	fn new(config: Config) -> Self {
		KillProvider { config }
	}
}

impl Provider for KillProvider {
	fn query(&self, _query: &str) -> QueryResult {
		let mut sys = System::new();
		sys.refresh_processes();

		let processes = sys.processes();

		let mut hits = vec![];
		for process in processes.values() {
			hits.push(get_hit(process));
		}

		// let data =
		// 	HitData::new(&format!("killall {}", query), "kills all processes listed below").with_score(MAX_SCORE);
		// let extra = ExtraData { pid: Pid::from(1) };

		// let hit = Box::new(SimpleHit::new_extra(data, extra, do_kill));

		// hits.push(hit);
		QueryResult::new(hits)
	}
}

fn get_hit(process: &Process) -> Box<dyn Hit> {
	let title = format!("{} - {}", process.name(), process.pid());
	let data = HitData::new(&title, process.exe().to_str().unwrap());
	let extra = ExtraData { pid: process.pid() };

	Box::new(SimpleHit::new_extra(data, extra, do_kill))
}

struct ExtraData {
	pub pid: Pid,
}

fn do_kill(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	let pid = hit.get_extra_data().pid;

	let mut sys = System::new();
	sys.refresh_process(pid);

	if let Some(process) = sys.process(pid) {
		process.kill();
	}

	sender
		.send(FrontendMessage::Hide)
		.expect("unable to send frontend message");
}

#[derive(Deserialize, Debug)]
struct Config {
	//
}
