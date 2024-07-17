use color_eyre::config::{HookBuilder, PanicHook};
use std::panic::{set_hook, PanicInfo};

pub fn panic() {
	#[allow(clippy::print_stderr)]
	set_hook(Box::new(move |panic_info| {
		log_panic(panic_info);
		eprintln!("{}", get_eyre().panic_report(panic_info));
	}));
}

fn get_eyre() -> PanicHook {
	let (eyre_panic, _) = HookBuilder::default()
		.issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
		.into_hooks();

	eyre_panic
}

fn log_panic(panic_info: &PanicInfo<'_>) {
	let payload = panic_info
		.payload()
		.downcast_ref::<String>()
		.map(String::as_str)
		.or_else(|| panic_info.payload().downcast_ref::<&str>().cloned())
		.unwrap_or("<non-string panic payload>");

	let location = panic_info
		.location()
		.map_or_else(|| String::from("awd"), |l| format!("{}:{}", l.file(), l.line()));

	log::error!("panicked at {location}: {payload}");
	log::logger().flush();
}

#[cfg(test)]
mod test {
	use super::*;
	use rstest::rstest;

	#[rstest]
	fn eyre_hook_init() {
		get_eyre();
	}
}
