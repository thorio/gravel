use anyhow::Result;
use gravel_core::Hit;
use nix::sys::signal::{kill, Signal};
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};

pub type Pid = u32;

pub fn kill_process(pid: Pid) -> Result<()> {
	let pid = nix::unistd::Pid::from_raw(pid as i32);

	kill(pid, Signal::SIGKILL)?;
	Ok(())
}

pub fn query() -> Vec<Box<dyn Hit>> {
	// TODO: sysinfo crate loads a lot of unnecessary data into memory, replace with native calls
	let mut sys = System::new();
	sys.refresh_processes();

	let processes = sys.processes();

	let mut hits = vec![];
	for process in processes.values() {
		hits.push(get_hit(process));
	}

	hits
}

fn get_hit(process: &Process) -> Box<dyn Hit> {
	let cmdline = process.cmd().join(" ");

	super::get_hit(process.name(), process.pid().as_u32(), &cmdline)
}
