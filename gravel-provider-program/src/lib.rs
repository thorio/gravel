use gravel_core::provider::*;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod implementation;

pub struct ProgramProvider {}

impl ProgramProvider {
	pub fn new() -> Self {
		ProgramProvider {}
	}
}

impl Provider for ProgramProvider {
	fn query(&self, _query: &str) -> QueryResult {
		let hits = implementation::get_programs();

		QueryResult::new(hits)
	}
}
