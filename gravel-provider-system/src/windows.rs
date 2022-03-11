use crate::SubcommandConfig;
use winapi::um::{powrprof, winuser};

pub(crate) fn lock(_config: &SubcommandConfig) {
	unsafe {
		winuser::LockWorkStation();
	}
}

pub(crate) fn logout(_config: &SubcommandConfig) {
	let _ = system_shutdown::logout();
}

pub(crate) fn restart(_config: &SubcommandConfig) {
	let _ = system_shutdown::reboot();
}

pub(crate) fn shutdown(_config: &SubcommandConfig) {
	let _ = system_shutdown::shutdown();
}

pub(crate) fn sleep(_config: &SubcommandConfig) {
	unsafe {
		powrprof::SetSuspendState(0, 0, 0);
	}
}
