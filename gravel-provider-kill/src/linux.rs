use anyhow::Result;
use gravel_ffi::SimpleHit;
use nix::sys::signal::{kill, Signal};
use procfs::process::Process;

pub type Pid = i32;

pub fn kill_process(pid: Pid) -> Result<()> {
	let pid = nix::unistd::Pid::from_raw(pid);

	kill(pid, Signal::SIGKILL)?;
	Ok(())
}

pub fn query() -> Result<impl Iterator<Item = SimpleHit>> {
	let hits = procfs::process::all_processes()?
		.filter_map(Result::ok)
		.map(get_hit)
		.filter_map(Result::ok);

	Ok(hits)
}

fn get_hit(process: Process) -> Result<SimpleHit> {
	let args = process.cmdline()?;
	let cmdline = args.join(" ").replace('\n', "\\n");
	let name = get_cmdline_binary(&args)
		.or_else(|| get_exe_binary(&process))
		.or_else(|| get_command_name(&process))
		.unwrap_or_else(|| String::from("unknown process"));

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
	process.stat().map(|s| format!("[{}]", s.comm)).ok()
}
