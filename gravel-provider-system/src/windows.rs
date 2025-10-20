// Needs to be Result to maintain same signature as linux implementation
#![expect(clippy::unnecessary_wraps)]

use anyhow::Result;
use winapi::um::{powrprof, winuser};

pub fn lock(_command_linux: &str) -> Result<()> {
	unsafe {
		winuser::LockWorkStation();
	}

	Ok(())
}

pub fn logout(_command_linux: &str) -> Result<()> {
	system_shutdown::logout()?;
	Ok(())
}

pub fn restart(_command_linux: &str) -> Result<()> {
	system_shutdown::reboot()?;
	Ok(())
}

pub fn shutdown(_command_linux: &str) -> Result<()> {
	system_shutdown::shutdown()?;
	Ok(())
}

pub fn sleep(_command_linux: &str) -> Result<()> {
	unsafe {
		powrprof::SetSuspendState(0, 0, 0);
	}

	Ok(())
}
