use winapi::um::{powrprof, winuser};

pub(crate) fn lock(_command_linux: &str) {
	unsafe {
		winuser::LockWorkStation();
	}
}

pub(crate) fn logout(_command_linux: &str) {
	system_shutdown::logout().ok();
}

pub(crate) fn restart(_command_linux: &str) {
	system_shutdown::reboot().ok();
}

pub(crate) fn shutdown(_command_linux: &str) {
	system_shutdown::shutdown().ok();
}

pub(crate) fn sleep(_command_linux: &str) {
	unsafe {
		powrprof::SetSuspendState(0, 0, 0);
	}
}
