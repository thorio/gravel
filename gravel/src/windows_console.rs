//! Handles attaching and detaching a console if running under windows subsystem
//! I hate this.

use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};

// Attempts to attach the parent processes console and fails silently if the parent has no console
pub fn attach() {
	log::trace!("attempting to attach console");
	unsafe {
		AttachConsole(ATTACH_PARENT_PROCESS);
	}
}

// Attempts to detach the console and fails silently if the parent has no console
pub fn detach() {
	log::trace!("attempting to free console");
	unsafe {
		FreeConsole();
	}
}
