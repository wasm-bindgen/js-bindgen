use js_sys_bindgen::syn::Error;
use proc_macro::TokenStream;

#[proc_macro]
pub fn closure(input: TokenStream) -> TokenStream {
	js_sys_bindgen::closure(input.into())
		.unwrap_or_else(Error::into_compile_error)
		.into()
}

#[proc_macro_attribute]
pub fn js_sys(attr: TokenStream, item: TokenStream) -> TokenStream {
	js_sys_bindgen::r#macro(attr.into(), item.into())
		.unwrap_or_else(|e| e)
		.into()
}
