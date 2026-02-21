use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn js_sys(attr: TokenStream, item: TokenStream) -> TokenStream {
	js_sys_bindgen::r#macro(attr.into(), item.into(), None)
		.unwrap_or_else(|e| e)
		.into()
}
