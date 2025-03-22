//! Contains procedural macros used to auto-implement FFI loader code.
//! See `gravel_ffi::Provider` and `gravel_ffi::Frontend` for usage instructions

use proc_macro::TokenStream as TokenStream1;
use syn::{File, ItemImpl, parse_quote};

mod declare;
mod util;

/// Automatically implements the necessary FFI-glue for plugins to work.
///
/// The Argument is the plugin name, which is used to identify it.  
/// See `gravel_ffi::Provider` for more detailed information.
#[proc_macro_attribute]
pub fn gravel_provider(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
	util::wrap_syn(item, |impl_block: ItemImpl| {
		let provider_type = &impl_block.self_ty;

		let declaration = declare::provider(&util::get_name(attr)?, provider_type);

		let ast: File = parse_quote! {
			impl ::gravel_ffi::ProviderInner for #provider_type {
				fn query(&self, query: ::abi_stable::std_types::RStr<'_>) -> ::gravel_ffi::ProviderResult {
					::gravel_ffi::Provider::query(self, query.as_str())
				}

				fn clear_caches(&self) {
					::gravel_ffi::Provider::clear_caches(self);
				}
			}

			#declaration
			#impl_block
		};

		Ok(ast)
	})
}

/// Automatically implements the necessary FFI-glue for plugins to work.
///
/// The Argument is the plugin name, which is used to identify it.  
/// See `gravel_ffi::Frontend` for more detailed information.
#[proc_macro_attribute]
pub fn gravel_frontend(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
	util::wrap_syn(item, |impl_block: ItemImpl| {
		let frontend_type = &impl_block.self_ty;

		let declaration = declare::frontend(&util::get_name(attr)?, frontend_type);

		let ast: File = parse_quote! {
			impl ::gravel_ffi::FrontendInner for #frontend_type {
				fn run(&mut self) -> ::gravel_ffi::FrontendExitStatusNe {
					let status = ::gravel_ffi::Frontend::run(self);
					::gravel_ffi::FrontendExitStatusNe::new(status)
				}
			}

			#declaration
			#impl_block
		};

		Ok(ast)
	})
}
