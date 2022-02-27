use fltk::{prelude::*, window::Window};
use winapi::{shared::windef::HWND, um::winuser};

pub fn activate_window(window: &Window) {
	let handle = window.raw_handle();

	unsafe {
		winuser::SetForegroundWindow(handle as HWND);
	}
}
