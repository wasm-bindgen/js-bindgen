use std::sync::atomic::{AtomicUsize, Ordering};

use js_sys_bindgen::syn::Error;
use proc_macro::TokenStream;

static CLOSURE_ID: AtomicUsize = AtomicUsize::new(0);

#[proc_macro]
pub fn closure(input: TokenStream) -> TokenStream {
	js_sys_bindgen::closure(input.into(), CLOSURE_ID.fetch_add(1, Ordering::Relaxed))
		.unwrap_or_else(Error::into_compile_error)
		.into()
}

#[proc_macro_attribute]
pub fn js_sys(attr: TokenStream, item: TokenStream) -> TokenStream {
	js_sys_bindgen::r#macro(attr.into(), item.into(), None)
		.unwrap_or_else(|e| e)
		.into()
}
