use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn::{Error, LitStr, parse::Parse};

pub fn wrap_syn<P, R>(input: TokenStream1, f: impl FnOnce(P) -> Result<R, Error>) -> TokenStream1
where
	P: Parse,
	R: ToTokens,
{
	syn::parse::<P>(input)
		.and_then(f)
		.map(ToTokens::into_token_stream)
		.unwrap_or_else(|e| e.to_compile_error())
		.into()
}

pub fn get_name(attr: TokenStream1) -> Result<String, Error> {
	let lit = syn::parse::<LitStr>(attr)?;
	let name = lit.value();

	if name.is_empty() {
		return Err(Error::new_spanned(lit.token(), "plugin name must not be empty"));
	}

	Ok(name)
}
