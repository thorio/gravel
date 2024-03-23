use single_instance::SingleInstance;

/// Checks for duplicate instances with the given name.
/// If `name` is [`None`], does nothing.
pub fn single_instance(name: Option<&str>) -> Option<SingleInstance> {
	let Some(name) = name else {
		return None;
	};

	log::debug!("initializing single-instance with key {name}");

	match SingleInstance::new(name) {
		Err(err) => {
			log::error!("unable to setup single-instance, error: {err}");
			None
		}
		Ok(instance) if !instance.is_single() => {
			log::warn!("duplicate instance with name '{name}' detected, exiting");
			std::process::exit(1);
		}
		Ok(instance) => Some(instance),
	}
}
