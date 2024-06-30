use proc_macro::TokenStream as TokenStream1;
use syn::{parse_quote, File, ItemImpl};

mod declare;
mod util;

/// Automatically implements the necessary FFI-glue for plugins to work.  
/// The Argument is the plugin name, which is used to identify it.
///
/// Usage:
/// ```compile_fail
/// #[gravel_provider("my_provider")]
/// impl ProviderDef for MyProvider {
///     // ... //
/// }
/// ```
#[proc_macro_attribute]
pub fn gravel_provider(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
	util::wrap_syn(item, |impl_block: ItemImpl| {
		let provider_type = &impl_block.self_ty;

		let declaration = declare::provider(&util::get_name(attr)?, provider_type);

		let ast: File = parse_quote! {
			impl ::gravel_ffi::Provider for #provider_type {
				fn query(&self, query: ::abi_stable::std_types::RStr<'_>) -> ::gravel_ffi::ProviderResult {
					::gravel_ffi::ProviderDef::query(self, query.as_str())
				}
			}

			#declaration
			#impl_block
		};

		Ok(ast)
	})
}

/// Automatically implements the necessary FFI-glue for plugins to work.  
/// The Argument is the plugin name, which is used to identify it.
///
/// Usage:
/// ```compile_fail
/// #[gravel_frontend("my_frontend")]
/// impl FrontendDef for MyFrontend {
///     // ... //
/// }
/// ```
#[proc_macro_attribute]
pub fn gravel_frontend(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
	util::wrap_syn(item, |impl_block: ItemImpl| {
		let frontend_type = &impl_block.self_ty;

		let declaration = declare::frontend(&util::get_name(attr)?, frontend_type);

		let ast: File = parse_quote! {
			impl ::gravel_ffi::Frontend for #frontend_type {
				fn run(&mut self, receiver: RReceiver<FrontendMessage>) -> FrontendExitStatus {
					::gravel_ffi::FrontendDef::run(self, receiver)
				}
			}

			#declaration
			#impl_block
		};

		Ok(ast)
	})
}
