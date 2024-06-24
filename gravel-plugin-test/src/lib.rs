use abi_stable::external_types::crossbeam_channel::RSender;
use abi_stable::std_types::{RStr, RString};
use abi_stable::traits::IntoOwned;
use abi_stable::StableAbi;
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use gravel_ffi::{
	plugin, BoxDynProvider, FrontendMessage, GravelPluginLib, GravelPluginLibRef, HitExt, PluginConfigAdapter,
	PluginDefinition, Provider, ProviderExt, ProviderResult, SimpleHit,
};

#[export_root_module]
pub fn get_library() -> GravelPluginLibRef {
	GravelPluginLib { get_plugin }.leak_into_prefix()
}

#[sabi_extern_fn]
fn get_plugin() -> PluginDefinition {
	plugin("abc").with_provider(get_provider)
}

#[sabi_extern_fn]
fn get_provider(_config: &PluginConfigAdapter<'_>) -> BoxDynProvider {
	let val = RString::from("abcdef");
	let provider = WebsearchProvider { _some_value: val };

	provider.into_dyn()
}

#[repr(C)]
#[derive(StableAbi)]
pub struct WebsearchProvider {
	_some_value: RString,
}

impl Provider for WebsearchProvider {
	fn query(&self, query: RStr<'_>) -> ProviderResult {
		let query_owned = query.into_owned();

		let hit = SimpleHit::new(query, "abcdef", move |s| do_search(query_owned.as_rstr(), s)).with_score(0);

		ProviderResult::single(hit.into_dyn())
	}
}

fn do_search(_query: RStr<'_>, sender: &RSender<FrontendMessage>) {
	sender.send(FrontendMessage::Hide).ok();
}
