use anyhow::Result;
use gravel_core::Hit;
use itertools::Itertools;
use std::sync::Arc;
use sysinfo::{Process, System};
use thiserror::Error;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi;
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

#[derive(Error, Debug)]
pub enum KillError {
	#[error("winapi error: {0}")]
	NativeError(u32),
}

pub fn query() -> Result<Vec<Arc<dyn Hit>>> {
	// TODO: sysinfo crate loads a lot of unnecessary data into memory,
	// replace with native calls (or a crate that does streaming)
	let mut sys = System::new();
	sys.refresh_processes();

	let hits = sys
		.processes()
		.iter()
		.map(|(pid, process)| get_hit(pid, process))
		.collect_vec();

	Ok(hits)
}

fn get_hit(pid: &sysinfo::Pid, process: &Process) -> Arc<dyn Hit> {
	let cmdline = process.cmd().join(" ");

	super::get_hit(process.name(), pid.as_u32(), &cmdline)
}

fn open_process(desired_access: DWORD, pid: Pid) -> Result<HandleWrapper, KillError> {
	let handle = unsafe { processthreadsapi::OpenProcess(desired_access, 0, pid) };

	if handle == 0 as HANDLE {
		return Err(get_last_error());
	}

	Ok(HandleWrapper::from(handle))
}

fn get_last_error() -> KillError {
	let errno = unsafe { errhandlingapi::GetLastError() };

	KillError::NativeError(errno)
}

pub fn kill_process(pid: Pid) -> Result<(), KillError> {
	let handle = open_process(winnt::PROCESS_TERMINATE, pid)?;

	if unsafe { processthreadsapi::TerminateProcess(handle.handle, 1) } != 0 {
		return Err(get_last_error());
	}

	Ok(())
}
