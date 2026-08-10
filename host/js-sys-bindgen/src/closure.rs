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
use crate::function;
use crate::hygiene::Hygiene;
use crate::r#macro::render_import_groups;

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
	// The package identity is part of the descriptor, so the hash is deterministic
	// and does not depend on macro expansion order or parallel compilation.
	let symbol_id = closure_symbol_hash(crate_name, package_name, package_version, &trait_object);
	let call_name_value = format!("closure_call_{symbol_id}");
	let call_name = syn::LitStr::new(&call_name_value, span);
	let factory_ident = format_ident!("closure_new_{symbol_id}", span = span);
	let factory_name = syn::LitStr::new(&format!("closure.new.{symbol_id}"), span);
	let factory_embed = syn::LitStr::new(signature.kind.factory_embed(), span);
	let closure = signature.closure_type(&trait_object);
	let factory_js = syn::LitStr::new(
		&format!(
			"(data) => this.#jsEmbed.js_sys['{}'](data, this.#jsExports['{call_name_value}'])",
			signature.kind.factory_embed(),
		),
		span,
	);
	let factory_item = parse_quote_spanned! {span=>
		#[js_sys(js_embed = #factory_name)]
		fn #factory_ident(
			data: ::core::primitive::usize,
		) -> #js_sys::JsValue;
	};
	let mut hygiene = Hygiene::Qualified {
		js_sys: Some(&js_sys),
	};
	let (factory_function, factory_import) =
		function::expand_closure_factory(&mut hygiene, crate_name, factory_item)?;
	let factory_wire = render_import_groups(vec![factory_import]);
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
			#js_sys::wire::wire_export_input::<::core::primitive::usize>("data")
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
					#js_sys::wire::Wire::exports(&[
						#js_sys::wire::wire_closure_export::<CallShim>(
							#crate_name,
							#call_name,
							&[#(#wire_inputs),*],
							#wire_output,
						),
					]);
				const _LEN: ::core::primitive::usize = #js_sys::wire::wire_blob_len(&_WIRE);

				#[unsafe(link_section = "js_bindgen.wire")]
				static _WIRE_SECTION: #js_sys::wire::WireBlob<_LEN> =
					#js_sys::wire::WireBlob::new(&_WIRE);
			};

			#js_sys::js_bindgen::embed_js! {
				module = #crate_name,
				name = #factory_name,
				required_embeds = [("js_sys", #factory_embed)],
				#factory_js,
			}

			#factory_function
			#factory_wire

			let allocation = allocate(#expression);
			let value = #factory_ident(allocation.data());
			allocation.forget();
			// SAFETY: `value` is created by the matching closure factory above.
			unsafe { #js_sys::Closure::<#closure>::from_js_value(value) }
		}
	})
}

fn closure_symbol_hash(
	crate_name: &str,
	package_name: &str,
	package_version: &str,
	trait_object: &TypeTraitObject,
) -> String {
	let mut descriptor = String::from("closure-v1\0");
	for value in [
		crate_name,
		package_name,
		package_version,
		&trait_object.to_token_stream().to_string(),
	] {
		descriptor.push_str(value);
		descriptor.push('\0');
	}

	format!("{:032x}", xxh3_128(descriptor.as_bytes()))
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

	fn factory_embed(self) -> &'static str {
		match self {
			Self::Shared => "closure.make",
			Self::Mutable => "closure.make_mut",
			Self::Once => "closure.make_once",
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
			ReturnType::Type(_, output) if matches!(&**output, Type::Tuple(tuple) if tuple.elems.is_empty()) => {
				None
			}
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
