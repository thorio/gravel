//! gravel's process killer
//!
//! TODO!

use gravel_core::{config::PluginConfigAdapter, plugin::*, scoring::MAX_SCORE, *};
use serde::Deserialize;
use std::sync::mpsc::Sender;
use sysinfo::{Process, ProcessExt, System, SystemExt};

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
	fn query(&self, query: &str) -> QueryResult {
		let mut sys = System::new();
		sys.refresh_processes();

		let processes = sys.processes();

		let mut hits = vec![];
		for (_pid, process) in processes {
			hits.push(get_hit(process));
		}

		// let data =
		// 	HitData::new(&format!("killall {}", query), "kills all processes listed below").with_score(MAX_SCORE);
		// let extra = ExtraData { pids: vec![] };

		// let hit = Box::new(SimpleHit::new_extra(data, extra, do_kill));
		QueryResult::new(hits)
	}
}

fn get_hit(process: &Process) -> Box<dyn Hit> {
	let title = format!("{} - {}", process.name(), process.pid());
	let data = HitData::new(&title, process.exe().to_str().unwrap()).with_score(MAX_SCORE);
	let extra = ExtraData { pids: vec![] };

	Box::new(SimpleHit::new_extra(data, extra, do_kill))
}

struct ExtraData {
	pub pids: Vec<u32>,
}

fn do_kill(hit: &SimpleHit<ExtraData>, sender: &Sender<FrontendMessage>) {
	println!("{:?}", hit.get_extra_data().pids);
	sender.send(FrontendMessage::Hide).unwrap();
}

#[derive(Deserialize, Debug)]
struct Config {
	//
}
