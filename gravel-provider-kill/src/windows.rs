use gravel_core::Hit;
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};
use winapi::shared::minwindef::DWORD;
use winapi::um::{handleapi, processthreadsapi, winnt, winnt::HANDLE};

pub type Pid = u32;

struct HandleWrapper {
	pub handle: HANDLE,
}

impl HandleWrapper {
	pub fn from(handle: HANDLE) -> Self {
		Self { handle }
	}
}

impl Drop for HandleWrapper {
	fn drop(&mut self) {
		unsafe { handleapi::CloseHandle(self.handle) };
	}
}

pub struct CannotKillProcess;

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

fn open_process(desired_access: DWORD, pid: Pid) -> Option<HandleWrapper> {
	let handle = unsafe { processthreadsapi::OpenProcess(desired_access, 0, pid) };

	if handle == 0 as HANDLE {
		return None;
	}

	Some(HandleWrapper::from(handle))
}

pub fn kill_process(pid: Pid) -> Result<(), CannotKillProcess> {
	let handle = open_process(winnt::PROCESS_TERMINATE, pid).ok_or(CannotKillProcess)?;

	if unsafe { processthreadsapi::TerminateProcess(handle.handle, 1) } != 0 {
		return Err(CannotKillProcess);
	}

	Ok(())
}
