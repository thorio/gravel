use anyhow::Result;
use winapi::um::{powrprof, winuser};

pub(crate) fn lock(_command_linux: &str) -> Result<()> {
	unsafe {
		winuser::LockWorkStation();
	}

	Ok(())
}

pub(crate) fn logout(_command_linux: &str) -> Result<()> {
	system_shutdown::logout()?;
	Ok(())
}

pub(crate) fn restart(_command_linux: &str) -> Result<()> {
	system_shutdown::reboot()?;
	Ok(())
}

pub(crate) fn shutdown(_command_linux: &str) -> Result<()> {
	system_shutdown::shutdown()?;
	Ok(())
}

pub(crate) fn sleep(_command_linux: &str) -> Result<()> {
	unsafe {
		powrprof::SetSuspendState(0, 0, 0);
	}

	Ok(())
}
