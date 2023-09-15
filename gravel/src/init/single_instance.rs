use lazy_static::*;
use single_instance::SingleInstance;
use std::sync::Mutex;

lazy_static! {
	static ref INSTANCE: Mutex<Option<SingleInstance>> = Mutex::new(None);
}

/// Checks for duplicate instances with the given name.
/// If `name` is [`None`], does nothing.
pub fn single_instance(name: Option<&String>) {
	if name.is_none() {
		// single-instance is disabled, do nothing
		return;
	}

	let instance = SingleInstance::new(name.as_ref().unwrap()).unwrap();

	if !instance.is_single() {
		println!("duplicate instance detected, exiting");
		std::process::exit(1);
	}

	// save the instance statically to avoid dropping it early.
	let mut mutex_value = INSTANCE.lock().unwrap();
	*mutex_value = Some(instance);
}
