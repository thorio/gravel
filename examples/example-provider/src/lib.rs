//! Simplistic example of a gravel provider.

use gravel_ffi::prelude::*;
use serde::Deserialize;

// Default configs are defined as yml and loaded at runtime.
//
// Use `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.yml"));`
// to read it from a file at compile time.
const DEFAULT_CONFIG: &str = "number: 2";

// The provider can define any state, as long as it's immutable.
struct ExampleProvider {
	_number: u32,
}

// This macro generates the FFI loader code to make this library work as a plugin.
// The name of the plugin should be unique.
#[gravel_provider("example")]
impl Provider for ExampleProvider {
	// Creates a new instance of the provider.
	// There might be several with different configs!
	fn new(config: &PluginConfigAdapter<'_>) -> Self {
		// Plug in the default config, and the adapter will fetch
		// the config for this provider from the user's main config.
		let config = config.get::<Config>(DEFAULT_CONFIG);

		log::info!("configuration number was: {}", config.number);

		Self { _number: config.number }
	}

	// This function is called every time the user types a letter,
	// so it must be very quick to execute.
	fn query(&self, query: &str) -> ProviderResult {
		let subtitle = format!("you were searching for {query}?");

		// Plugins can define their own hit type, but for most anything
		// the default SimpleHit combined with a closure does the trick.
		let hit = SimpleHit::new("example hit", subtitle, |hit, context| {
			// When the user actually selects this hit in the frontend,
			// this closure is called.

			log::info!("hit action was called: title '{}'", hit.title);

			// Sends a message to the frontend and asks it to hide.
			// If this isn't called, the frontend will just stay open!
			context.hide_frontend();
		});

		// Wrap the hit in a ProviderResult and return it.
		ProviderResult::single(hit)
	}
}

// The config struct must be deserializable.
#[derive(Deserialize)]
struct Config {
	pub number: u32,
}
