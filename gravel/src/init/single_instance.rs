use single_instance::SingleInstance;

/// Checks for duplicate instances with the given name.
/// If `name` is [`None`], does nothing.
pub fn single_instance(name: Option<&str>) {
	let Some(name) = name else {
		return;
	};

	log::debug!("initializing single-instance with key {name}");

	match SingleInstance::new(name) {
		Err(err) => {
			log::error!("unable to setup single-instance, error: {err}")
		}
		Ok(instance) if !instance.is_single() => {
			log::warn!("duplicate instance with name '{name}' detected, exiting");
			std::process::exit(1);
		}
		Ok(instance) => {
			// Leak the value to ensure it lives for the lifetime of the program.
			Box::leak(Box::new(instance));
		}
	};
}
