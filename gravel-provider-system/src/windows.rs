use crate::SubcommandConfig;
use winapi::um::{powrprof, winuser};

pub(crate) fn lock(_config: &SubcommandConfig) {
	unsafe {
		winuser::LockWorkStation();
	}
}

pub(crate) fn logout(_config: &SubcommandConfig) {
	system_shutdown::logout().ok();
}

pub(crate) fn restart(_config: &SubcommandConfig) {
	system_shutdown::reboot().ok();
}

pub(crate) fn shutdown(_config: &SubcommandConfig) {
	system_shutdown::shutdown().ok();
}

pub(crate) fn sleep(_config: &SubcommandConfig) {
	unsafe {
		powrprof::SetSuspendState(0, 0, 0);
	}
}
