#![allow(unused_crate_dependencies, clippy::missing_panics_doc)]

use abi_stable::std_types::{ROption, RSlice, RString};
use abi_stable::{library::RootModule, sabi_trait, traits::IntoReprC};
use gravel_ffi::{logging::NoOpLogTarget, HitActionContext, PluginConfigAdapter, PluginLibRef, RefDynHitActionContext};
use mockall::mock;
use std::path::PathBuf;

mock! {
	HitActionContext {}
	impl HitActionContext for HitActionContext {
		fn hide_frontend(&self);
		fn refresh_frontend(&self);
		fn exit(&self);
		fn restart(&self);
	}
}

impl<'a> From<&'a MockHitActionContext> for RefDynHitActionContext<'a> {
	fn from(value: &'a MockHitActionContext) -> Self {
		Self::from_ptr(value, sabi_trait::TD_Opaque)
	}
}

/// This is a smoke test to check for breaking changes in the plugin interface
#[cfg_attr(unix, test)]
pub fn use_provider() {
	// abi_stable checks if the types are compatible
	let lib = PluginLibRef::load_from_file(&PathBuf::from("tests/data/libexample_provider.so"))
		.expect("plugin must be loadable");

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
