use fltk::{prelude::*, window::Window};

pub fn activate_window(window: &Window) {
	// this doesn't seem to be needed under linux, at least not in my xmonad setup
	// TODO: check if the window is correctly focused when using floating window managers
}
