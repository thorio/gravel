use syn::{parse_quote, File, Ident, Type};

pub fn provider(plugin_name: &str, impl_type: &Type) -> File {
	let plugin = plugin(plugin_name, parse_quote! { with_provider });

	parse_quote! {
		#plugin

		#[::abi_stable::sabi_extern_fn]
		pub fn __gravel_plugin_get(config: &::gravel_ffi::PluginConfigAdapter<'_>) -> ::gravel_ffi::BoxDynProvider {
			let value = <#impl_type as ::gravel_ffi::Provider>::new(config);
			::gravel_ffi::BoxDynProvider::from_value(value, ::abi_stable::sabi_trait::TD_Opaque)
		}
	}
}

pub fn frontend(plugin_name: &str, impl_type: &Type) -> File {
	let plugin = plugin(plugin_name, parse_quote! { with_frontend });

	parse_quote! {
		#plugin

		#[::abi_stable::sabi_extern_fn]
		pub fn __gravel_plugin_get(engine: ::gravel_ffi::BoxDynFrontendContext, config: &::gravel_ffi::PluginConfigAdapter<'_>) -> ::gravel_ffi::BoxDynFrontend {
			let value = <#impl_type as ::gravel_ffi::Frontend>::new(engine, config);
			::gravel_ffi::BoxDynFrontend::from_value(value, ::abi_stable::sabi_trait::TD_Opaque)
		}
	}
}

fn plugin(plugin_name: &str, with_fn_name: Ident) -> File {
	parse_quote! {
		// omitting the root_module allows more than one plugin to be statically linked
		#[cfg(not(feature = "no-root"))]
		#[::abi_stable::export_root_module]
		pub fn __gravel_plugin_root() -> ::gravel_ffi::PluginLibRef {
			let plugin = ::gravel_ffi::PluginLib { plugin: __gravel_plugin };
			::abi_stable::prefix_type::PrefixTypeTrait::leak_into_prefix(plugin)
		}

		#[::abi_stable::sabi_extern_fn]
		pub fn __gravel_plugin(log_target: ::gravel_ffi::logging::BoxDynLogTarget) -> ::abi_stable::std_types::RVec<::gravel_ffi::PluginDefinition> {
			::gravel_ffi::logging::ForwardLogger::register(log_target, env!("CARGO_PKG_NAME"));
			::abi_stable::traits::IntoReprC::into_c(vec![__gravel_plugin_inner()])
		}

		pub fn __gravel_plugin_inner() -> ::gravel_ffi::PluginDefinition {
			::gravel_ffi::PluginMetadata::new(#plugin_name).#with_fn_name(__gravel_plugin_get)
		}
	}
}
