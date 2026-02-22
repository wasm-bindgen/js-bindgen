use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{
	Error, Expr, ExprLit, ItemFn, Lit, LitByteStr, Meta, MetaNameValue, Path, Result, ReturnType,
	meta, parse_quote,
};

enum TestAttribute {
	None,
	Present,
	WithText(String),
}

#[proc_macro_attribute]
pub fn test(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	test_internal(attr.into(), item.into())
		.unwrap_or_else(Error::into_compile_error)
		.into()
}

fn test_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
	let mut crate_: Option<Path> = None;

	meta::parser(|meta| {
		if meta.path.is_ident("js_sys_test") {
			if crate_.is_some() {
				Err(meta.error("duplicate attribute"))
			} else {
				crate_ = Some(meta.value()?.parse()?);
				Ok(())
			}
		} else {
			Err(meta.error("unsupported attribute"))
		}
	})
	.parse2(attr)?;

	let crate_ = crate_.unwrap_or_else(|| parse_quote!(::js_bindgen_test));

	let mut function: ItemFn = syn::parse2(item)?;
	let span = function.span();
	let mut ignore = TestAttribute::None;
	let mut should_panic = TestAttribute::None;

	for attr in function.attrs.extract_if(.., |attr| {
		attr.path().is_ident("ignore") || attr.path().is_ident("should_panic")
	}) {
		if attr.path().is_ident("ignore") {
			if !matches!(ignore, TestAttribute::None) {
				return Err(Error::new_spanned(
					attr,
					"only one `ignore` attribute supported",
				));
			}

			match attr.meta {
				Meta::Path(_) => ignore = TestAttribute::Present,
				Meta::NameValue(MetaNameValue {
					value: Expr::Lit(ExprLit {
						lit: Lit::Str(reason),
						..
					}),
					..
				}) => ignore = TestAttribute::WithText(reason.value()),
				meta => {
					return Err(Error::new_spanned(meta, "`ignore` syntax not supported"));
				}
			}
		} else if attr.path().is_ident("should_panic") {
			if !matches!(should_panic, TestAttribute::None) {
				return Err(Error::new_spanned(
					attr,
					"only one `should_panic` attribute supported",
				));
			}

			if let Meta::Path(_) = attr.meta {
				should_panic = TestAttribute::Present;
			} else {
				let value = if let Meta::NameValue(MetaNameValue { value, .. }) = &attr.meta {
					Some(Cow::Borrowed(value))
				} else if let Meta::List(list) = &attr.meta
					&& let MetaNameValue { path, value, .. } = list.parse_args()?
					&& path.is_ident("expected")
				{
					Some(Cow::Owned(value))
				} else {
					None
				};

				if let Some(Expr::Lit(ExprLit {
					lit: Lit::Str(expected),
					..
				})) = value.as_deref()
				{
					should_panic = TestAttribute::WithText(expected.value());
				} else {
					return Err(Error::new_spanned(
						attr.meta,
						"`should_panic` syntax not supported",
					));
				}
			}
		}
	}

	if let Some(constness) = function.sig.constness {
		return Err(Error::new_spanned(constness, "`const` test not supported"));
	}

	if let Some(asyncness) = function.sig.asyncness {
		return Err(Error::new_spanned(asyncness, "`async` test not supported"));
	}

	if !function.sig.inputs.is_empty() {
		return Err(Error::new_spanned(
			function.sig.inputs,
			"test with parameters not supported",
		));
	}

	if let ReturnType::Type(..) = function.sig.output {
		return Err(Error::new_spanned(
			function.sig.output,
			"test with return value not supported",
		));
	}

	let mut data = Vec::new();
	ignore.encode_into(&mut data);
	should_panic.encode_into(&mut data);
	let data_len = data.len();
	let data = LitByteStr::new(&data, span);
	let ident = &function.sig.ident;
	let foreign_test = quote! {
		::core::concat!(::core::module_path!(), "::", ::core::stringify!(#ident))
	};

	Ok(quote! {
		#function

		const _: () = {
			const DATA: [::core::primitive::u8; #data_len] = *#data;

			const TEST: &::core::primitive::str = #foreign_test;
			const TEST_LEN: ::core::primitive::usize = ::core::primitive::str::len(TEST);
			const TEST_PTR: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(TEST);
			const TEST_ARR: [::core::primitive::u8; TEST_LEN] = unsafe { *(TEST_PTR as *const _) };

			const LEN: ::core::primitive::u32 = (#data_len + TEST_LEN) as _;
			const LEN_ARR: [::core::primitive::u8; 4] = ::core::primitive::u32::to_le_bytes(LEN);

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; #data_len],
				[::core::primitive::u8; TEST_LEN],
			);

			#[unsafe(link_section = "js_bindgen.test")]
			static CUSTOM_SECTION: Layout = Layout(LEN_ARR, DATA, TEST_ARR);
		};

		const _: () = {
			#[unsafe(export_name = #foreign_test)]
			extern "C" fn test() {
				#crate_::set_panic_hook();
				#ident();
			}
		};
	})
}

impl TestAttribute {
	fn encode_into(self, buffer: &mut Vec<u8>) {
		match self {
			Self::None => buffer.push(0),
			Self::Present => buffer.push(1),
			Self::WithText(s) => {
				let len = u16::try_from(s.len()).unwrap().to_le_bytes();
				buffer.reserve(1 + len.len() + s.len());
				buffer.push(2);
				buffer.extend_from_slice(&len);
				buffer.append(&mut s.into_bytes());
			}
		}
	}
}
