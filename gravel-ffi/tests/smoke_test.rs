#![allow(unused_crate_dependencies, clippy::missing_panics_doc)]

use abi_stable::std_types::{ROption, RSlice, RString};
use abi_stable::traits::IntoReprC;
use gravel_core::plugin::load_library_from_path;
use gravel_ffi::PluginLibRef;
use gravel_ffi::{logging::NoOpLogTarget, PluginConfigAdapter};
use gravel_test_utils::mock::MockHitActionContext;
use std::path::PathBuf;

/// This is a smoke test to check for breaking changes in the plugin interface
#[cfg_attr(unix, test)]
pub fn load_plugin_use_provider() {
	// abi_stable checks if the types are compatible
	let lib = load_library_from_path::<PluginLibRef>(&PathBuf::from("tests/data/libexample_provider.so"))
		.expect("plugin library header must be loadable");

	// but just to be sure, let's do a test run of the provider
	let definitions = lib.plugin()(NoOpLogTarget::get());
	let definition = definitions.first().expect("must export at least one definition");

	let factory = definition.factory.provider();
	let factory = factory.expect("plugin must define provider factory");

	let adapter = PluginConfigAdapter::from(RString::from("noconfig"), RSlice::default());
	let provider = (factory)(&adapter);

	let result = provider.query("something".into_c());

	assert!(!result.hits.is_empty());

	let hit = &result.hits[0];

	assert_eq!("example hit", hit.title());
	assert_eq!("you were searching for something?", hit.subtitle());
	assert_eq!(ROption::RNone, hit.override_score());

	let mut context = MockHitActionContext::default();

	context.expect_hide_frontend().return_const(());

	hit.action((&context).into());
}
