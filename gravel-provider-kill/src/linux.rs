use std::sync::Arc;

use anyhow::Result;
use gravel_core::Hit;
use itertools::Itertools;
use nix::sys::signal::{kill, Signal};
use procfs::process::Process;

pub type Pid = i32;

pub fn kill_process(pid: Pid) -> Result<()> {
	let pid = nix::unistd::Pid::from_raw(pid);

	kill(pid, Signal::SIGKILL)?;
	Ok(())
}

pub fn query() -> Result<Vec<Arc<dyn Hit>>> {
	let hits = procfs::process::all_processes()?
		.filter_map(Result::ok)
		.filter_map(|p| get_hit(&p).ok())
		.collect_vec();

	Ok(hits)
}

fn get_hit(process: &Process) -> Result<Arc<dyn Hit>> {
	let args = process.cmdline()?;
	let cmdline = args.join(" ");
	let name = get_cmdline_binary(&args)
		.or_else(|| get_exe_binary(process))
		.or_else(|| get_command_name(process))
		.unwrap_or(String::from("unknown process"));

	Ok(super::get_hit(&name, process.pid, &cmdline))
}

fn get_cmdline_binary(args: &[String]) -> Option<String> {
	args.first()?
		.split(' ')
		.next()?
		.split('/')
		.last()
		.map(ToOwned::to_owned)
}

fn get_exe_binary(process: &Process) -> Option<String> {
	let exe = process.exe().ok()?;

	exe.file_name().map(|s| s.to_string_lossy().into_owned())
}

fn get_command_name(process: &Process) -> Option<String> {
	process.stat().ok().map(|s| format!("[{}]", s.comm))
}
