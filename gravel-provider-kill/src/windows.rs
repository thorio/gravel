use winapi::shared::minwindef::DWORD;
use winapi::um::{handleapi, processthreadsapi, winnt, winnt::HANDLE};

pub type Pid = u32;

struct HandleWrapper {
	pub handle: HANDLE,
}

impl HandleWrapper {
	pub fn from(handle: HANDLE) -> HandleWrapper {
		HandleWrapper { handle }
	}
}

impl Drop for HandleWrapper {
	fn drop(&mut self) {
		unsafe { handleapi::CloseHandle(self.handle) };
	}
}

pub struct CannotKillProcess;

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
