//! gravel's default frontend, based on fltk.

pub use implementation::DefaultFrontend;

mod builder;
mod constants;
mod implementation;
mod scroll;
mod scrollbar;
mod structs;

#[cfg_attr(target_os = "linux", path = "native/linux.rs")]
#[cfg_attr(windows, path = "native/windows.rs")]
mod native;
