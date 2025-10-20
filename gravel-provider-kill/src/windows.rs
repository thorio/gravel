use anyhow::Result;
use gravel_ffi::SimpleHit;
use std::ffi::OsStr;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Process, ProcessesToUpdate, System};
use thiserror::Error;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi;
use winapi::um::{handleapi, processthreadsapi, winnt, winnt::HANDLE};

pub type Pid = u32;

struct Handle {
	pub inner: HANDLE,
}

impl Handle {
	pub fn from(handle: HANDLE) -> Self {
		Self { inner: handle }
	}
}

impl Drop for Handle {
	fn drop(&mut self) {
		unsafe { handleapi::CloseHandle(self.inner) };
	}
}

#[derive(Error, Debug)]
pub enum KillError {
	#[error("winapi error: {0}")]
	NativeError(u32),
}

// Needs to be Result to maintain same signature as linux implementation
#[expect(clippy::unnecessary_wraps)]
pub fn query() -> Result<impl Iterator<Item = SimpleHit>> {
	// TODO: sysinfo crate loads a lot of unnecessary data into memory,
	// replace with native calls (or a crate that does streaming)
	let mut sys = System::new();
	sys.refresh_processes(ProcessesToUpdate::All, true);

	let hits = sys
		.processes()
		.iter()
		.map(|(pid, process)| get_hit(*pid, process))
		.collect::<Vec<_>>()
		.into_iter();

	Ok(hits)
}

fn get_hit(pid: sysinfo::Pid, process: &Process) -> SimpleHit {
	let cmdline = process.cmd().join(OsStr::new(" "));
	let cmdline = cmdline.to_string_lossy();

	super::get_hit(&process.name().to_string_lossy(), pid.as_u32(), &cmdline)
}

fn open_process(desired_access: DWORD, pid: Pid) -> Result<Handle, KillError> {
	let handle = unsafe { processthreadsapi::OpenProcess(desired_access, 0, pid) };

	if handle == 0 as HANDLE {
		return Err(get_last_error());
	}

	Ok(Handle::from(handle))
}

fn get_last_error() -> KillError {
	let errno = unsafe { errhandlingapi::GetLastError() };

	KillError::NativeError(errno)
}

pub fn kill_process(pid: Pid) -> Result<(), KillError> {
	let handle = open_process(winnt::PROCESS_TERMINATE, pid)?;

	if unsafe { processthreadsapi::TerminateProcess(handle.inner, 1) } == 0 {
		return Err(get_last_error());
	}

	// give windows some extra time to actually kill the process
	// NOTE: using WaitForSingleObject doesn't work
	sleep(Duration::from_millis(100));

	Ok(())
}
