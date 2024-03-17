use anyhow::{anyhow, Result};
use std::ffi::CString;
use winapi::shared::ntdef::{LPCSTR, NULL};
use winapi::shared::windef::HWND;
use winapi::um::shellapi::ShellExecuteA;

/// Passes the given string to ShellExecute.
pub fn run_command(cmd: &str) -> Result<()> {
	log::debug!("running command with shell_execute '{cmd}'");

	shell_execute(cmd).map_err(|_| anyhow!(""))
}

fn shell_execute(cmd: &str) -> Result<()> {
	let cmd = CString::new(cmd).unwrap();

	let result = unsafe {
		ShellExecuteA(
			NULL as HWND,
			NULL as LPCSTR,
			cmd.as_ptr(),
			NULL as LPCSTR,
			NULL as LPCSTR,
			1,
		) as i32
	};

	if result <= 32 {
		return Err(anyhow!("unhandled winapi error during ShellExecute: {result}"));
	}

	Ok(())
}
