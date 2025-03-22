use color_eyre::config::{HookBuilder, PanicHook};
use std::panic::{PanicHookInfo, set_hook};

const ISSUE_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new");

pub fn panic() {
	#[allow(clippy::print_stderr)]
	set_hook(Box::new(move |panic_info| {
		log_panic(panic_info);
		eprintln!("{}", get_eyre().panic_report(panic_info));
	}));
}

fn get_eyre() -> PanicHook {
	let (eyre_panic, _) = HookBuilder::default().issue_url(ISSUE_URL).into_hooks();

	eyre_panic
}

fn log_panic(panic_info: &PanicHookInfo<'_>) {
	let payload = panic_info
		.payload()
		.downcast_ref::<String>()
		.map(String::as_str)
		.or_else(|| panic_info.payload().downcast_ref::<&str>().cloned())
		.map(|p| p.replace('\r', "\\r").replace('\n', "\\n"))
		.unwrap_or_else(|| String::from("<non-string panic payload>"));

	let location = panic_info.location().map_or_else(
		|| String::from("<unknown location>"),
		|l| format!("{}:{}", l.file(), l.line()),
	);

	log::error!("panicked at {location}: {payload}, please open an issue at {ISSUE_URL}.",);
	log::logger().flush();
}

#[cfg(test)]
mod test {
	use super::*;
	use rstest::rstest;

	#[rstest]
	fn eyre_hook_init() {
		// make sure this doesn't panic
		get_eyre();
	}
}
