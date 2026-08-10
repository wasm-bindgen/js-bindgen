use std::env;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
	Error, Expr, Path, PathArguments, ReturnType, Token, TraitBound, TraitBoundModifier, Type,
	TypeParamBound, TypeTraitObject, parse_quote_spanned,
};
use xxhash_rust::xxh3::xxh3_128;

use crate::export::{ExportAbi, lower_abi};

mod keyword {
	syn::custom_keyword!(js_sys);
}

pub fn closure(input: TokenStream) -> Result<TokenStream, Error> {
	let crate_name = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
	let package_name = env::var("CARGO_PKG_NAME").expect("`CARGO_PKG_NAME` not found");
	let package_version = env::var("CARGO_PKG_VERSION").expect("`CARGO_PKG_VERSION` not found");

	closure_with(input, &crate_name, &package_name, &package_version)
}

pub(crate) fn closure_with(
	input: TokenStream,
	crate_name: &str,
	package_name: &str,
	package_version: &str,
) -> Result<TokenStream, Error> {
	let ClosureInput {
		js_sys,
		trait_object,
		expression,
	} = syn::parse2(input)?;
	let signature = Signature::parse(&trait_object)?;
	let span = trait_object.span();
	let js_sys = js_sys.unwrap_or_else(|| parse_quote_spanned!(span=> ::js_sys));
	let trait_syntax_hash = format!(
		"{:032x}",
		xxh3_128(trait_object.to_token_stream().to_string().as_bytes()),
	);
	let trait_syntax_hash = syn::LitStr::new(&trait_syntax_hash, span);
	// This name only correlates the Rust factory call with this closure's raw
	// linker import. The linker derives dispatcher identity from the semantic
	// Wire `ABI` rather than from source tokens.
	let factory_raw_symbol = quote_spanned! {span=>
		::core::concat!(
			#crate_name, ".",
			#package_name, "@", #package_version, ":",
			::core::module_path!(), ":",
			::core::line!(), ":", ::core::column!(), ":",
			#trait_syntax_hash,
		)
	};
	let wire_kind = match signature.kind {
		ClosureKind::Shared => {
			quote_spanned!(span=> #js_sys::wire::ClosureKind::Shared)
		}
		ClosureKind::Mutable => {
			quote_spanned!(span=> #js_sys::wire::ClosureKind::Mutable)
		}
		ClosureKind::Once => quote_spanned!(span=> #js_sys::wire::ClosureKind::Once),
	};
	let closure = signature.closure_type(&trait_object);
	let inputs: Vec<_> = signature.inputs.iter().collect();
	let output = signature.output.as_ref();
	let ExportAbi {
		raw_types,
		raw_inputs,
		join_inputs,
		arguments,
		mut wire_inputs,
		raw_output,
		wire_output,
	} = lower_abi(inputs.iter().copied(), output, &js_sys)?;
	wire_inputs.insert(
		0,
		quote_spanned! {span=>
			#js_sys::wire::wire_export_input::<::core::primitive::usize>()
		},
	);
	let closure_bound = if signature.kind == ClosureKind::Shared {
		if let Some(output) = output {
			quote_spanned!(span=> ::core::ops::Fn(#(#inputs),*) -> #output)
		} else {
			quote_spanned!(span=> ::core::ops::Fn(#(#inputs),*))
		}
	} else {
		if let Some(output) = output {
			quote_spanned!(span=> ::core::ops::FnMut(#(#inputs),*) -> #output)
		} else {
			quote_spanned!(span=> ::core::ops::FnMut(#(#inputs),*))
		}
	};
	let callback_call = if signature.kind == ClosureKind::Shared {
		quote_spanned! {span=>
			let callback = unsafe {
				&*#js_sys::ClosureHeader::callback::<F, CallShim>(pointer)
			};
			callback(#(#arguments),*)
		}
	} else {
		quote_spanned! {span=>
			let callback = unsafe {
				&mut *#js_sys::ClosureHeader::callback::<F, CallShim>(pointer)
			};
			callback(#(#arguments),*)
		}
	};
	let call_body = if output.is_some() {
		quote_spanned! {span=>
			#(#join_inputs)*
			#js_sys::wire::return_to_js({
				#callback_call
			})
		}
	} else {
		quote_spanned! {span=>
			#(#join_inputs)*
			#callback_call;
		}
	};
	let expression = if signature.kind == ClosureKind::Once {
		quote_spanned! {expression.span()=>
			{
				let mut callback = ::core::option::Option::Some(#expression);
				move |#(#arguments),*| {
					let callback = ::core::option::Option::take(&mut callback)
						.expect("FnOnce called more than once");
					callback(#(#arguments),*)
				}
			}
		}
	} else {
		quote_spanned!(expression.span()=> #expression)
	};

	Ok(quote_spanned! {span=>
		{
			type CallShim = unsafe extern "C" fn(
				*mut #js_sys::ClosureHeader,
				#(#raw_types),*
			) #raw_output;

			#[allow(clippy::undocumented_unsafe_blocks)]
			unsafe extern "C" fn call_raw<F>(
				pointer: *mut #js_sys::ClosureHeader,
				#(#raw_inputs),*
			) #raw_output
			where
				F: #closure_bound,
			{
				#call_body
			}

			fn allocate<F>(
				callback: F,
			) -> #js_sys::ClosureAllocation
			where
				F: #closure_bound + 'static,
			{
				#js_sys::ClosureAllocation::new(
					callback,
					call_raw::<F> as CallShim,
				)
			}

			const _: () = {
				const _WIRE: #js_sys::wire::Wire =
					#js_sys::wire::wire_closure(
						#factory_raw_symbol,
						#wire_kind,
						#js_sys::ClosureHeader::call_shim_offset::<CallShim>(),
						&[#(#wire_inputs),*],
						#wire_output,
					);
				const _LEN: ::core::primitive::usize = #js_sys::wire::wire_blob_len(&_WIRE);

				#[unsafe(link_section = "js_bindgen.wire")]
				static _WIRE_SECTION: #js_sys::wire::WireBlob<_LEN> =
					#js_sys::wire::WireBlob::new(&_WIRE);
			};

			let allocation = allocate(#expression);
			let value = {
				unsafe extern "C" {
					#[link_name = #factory_raw_symbol]
					fn __import_(
						arg0_0: #js_sys::wire::InputSlot1<::core::primitive::usize>,
						arg0_1: #js_sys::wire::InputSlot2<::core::primitive::usize>,
						arg0_2: #js_sys::wire::InputSlot3<::core::primitive::usize>,
						arg0_3: #js_sys::wire::InputSlot4<::core::primitive::usize>,
					) -> #js_sys::wire::OutputRet<#js_sys::JsValue>;
				}
				let (arg0_0, arg0_1, arg0_2, arg0_3) =
					#js_sys::wire::split_input::<::core::primitive::usize>(allocation.data());

				#js_sys::wire::join_output(unsafe {
					__import_(arg0_0, arg0_1, arg0_2, arg0_3)
				})
			};
			allocation.forget();
			// SAFETY: `value` is created by the matching closure factory above.
			unsafe { #js_sys::Closure::<#closure>::from_js_value(value) }
		}
	})
}

struct ClosureInput {
	js_sys: Option<Path>,
	trait_object: TypeTraitObject,
	expression: Expr,
}

impl Parse for ClosureInput {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		// The optional leading `js_sys = path` controls macro hygiene. It is not
		// part of the closure trait object or the captured expression.
		let js_sys = if input.peek(keyword::js_sys) && input.peek2(Token![=]) {
			input.parse::<keyword::js_sys>()?;
			input.parse::<Token![=]>()?;
			let path = input.parse()?;
			input.parse::<Token![,]>()?;
			Some(path)
		} else {
			None
		};
		let trait_object = input.parse()?;
		input.parse::<Token![,]>()?;
		let expression = input.parse()?;

		if input.is_empty() {
			Ok(Self {
				js_sys,
				trait_object,
				expression,
			})
		} else {
			Err(input.error("unexpected tokens after closure expression"))
		}
	}
}

struct Signature {
	kind: ClosureKind,
	inputs: Punctuated<Type, Token![,]>,
	output: Option<Type>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ClosureKind {
	Shared,
	Mutable,
	Once,
}

impl ClosureKind {
	fn parse(bound: &syn::Ident) -> Result<Self, Error> {
		match bound.to_string().as_str() {
			"Fn" => Ok(Self::Shared),
			"FnMut" => Ok(Self::Mutable),
			"FnOnce" => Ok(Self::Once),
			_ => Err(Error::new_spanned(
				bound,
				"expected `Fn`, `FnMut`, or `FnOnce`",
			)),
		}
	}
}

impl Signature {
	fn parse(trait_object: &TypeTraitObject) -> Result<Self, Error> {
		if trait_object.bounds.len() != 1 {
			return Err(Error::new_spanned(
				trait_object,
				"expected exactly one closure trait",
			));
		}

		let Some(TypeParamBound::Trait(TraitBound {
			paren_token: None,
			modifier: TraitBoundModifier::None,
			lifetimes: None,
			path,
		})) = trait_object.bounds.first()
		else {
			return Err(Error::new_spanned(
				trait_object,
				"expected `dyn Fn(...)`, `dyn FnMut(...)`, or `dyn FnOnce(...)`",
			));
		};
		let Some(segment) = path.segments.last() else {
			return Err(Error::new_spanned(path, "expected a closure trait"));
		};
		let kind = ClosureKind::parse(&segment.ident)?;

		let PathArguments::Parenthesized(arguments) = &segment.arguments else {
			return Err(Error::new_spanned(
				&segment.arguments,
				"expected parenthesized closure arguments",
			));
		};
		let output = match &arguments.output {
			ReturnType::Default => None,
			ReturnType::Type(_, output) => Some(*output.clone()),
		};

		Ok(Self {
			kind,
			inputs: arguments.inputs.clone(),
			output,
		})
	}

	fn closure_type(&self, trait_object: &TypeTraitObject) -> TypeTraitObject {
		let mut closure = trait_object.clone();
		if self.kind == ClosureKind::Once {
			let Some(TypeParamBound::Trait(bound)) = closure.bounds.first_mut() else {
				unreachable!("validated closure trait");
			};
			let Some(segment) = bound.path.segments.last_mut() else {
				unreachable!("validated closure trait path");
			};
			segment.ident = format_ident!("FnMut", span = segment.ident.span());
		}
		closure
	}
}
