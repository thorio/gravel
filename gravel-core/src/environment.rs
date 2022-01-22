use std::env::consts::OS;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Platform {
	Linux,
	Windows,
}

pub fn platform_is(platform: Platform) -> bool {
	get_platform() == platform
}

pub fn get_platform() -> Platform {
	match OS {
		"linux" => Platform::Linux,
		"windows" => Platform::Windows,
		_ => panic!("unsupported platform: {}", OS),
	}
}
