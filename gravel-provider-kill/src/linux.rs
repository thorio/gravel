pub type Pid = u32;

pub struct CannotKillProcess;

pub fn kill_process(pid: Pid) -> Result<(), CannotKillProcess> {
	compile_error!("not implemented");
}
