use anyhow::Result;
use nix::sys::signal::{kill, Signal};

pub type Pid = u32;

pub fn kill_process(pid: Pid) -> Result<()> {
	let pid = nix::unistd::Pid::from_raw(pid as i32);

	kill(pid, Signal::SIGKILL)?;
	Ok(())
}
