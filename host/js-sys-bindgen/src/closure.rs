use std::env;

use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
	Error, Expr, Path, PathArguments, ReturnType, Token, TraitBound, TraitBoundModifier, Type,
	TypeParamBound, TypeTraitObject, parse_quote_spanned,
};

mod keyword {
	syn::custom_keyword!(js_sys);
}

pub fn closure(input: TokenStream, id: usize) -> Result<TokenStream, Error> {
	let crate_name = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
	closure_with_crate_name(input, id, &crate_name)
}

fn closure_with_crate_name(
	input: TokenStream,
	id: usize,
	crate_name: &str,
) -> Result<TokenStream, Error> {
	let ClosureInput {
		js_sys,
		trait_object,
		expression,
	} = syn::parse2(input)?;
	let signature = Signature::parse(&trait_object)?;
	let span = trait_object.span();
	let js_sys = js_sys.unwrap_or_else(|| parse_quote_spanned!(span=> ::js_sys));
	let symbol_id = format!("{}_{}", crate_name.replace('-', "_"), id);
	let call_ident = format_ident!("closure_call_{symbol_id}", span = span);
	let factory_ident = format_ident!("closure_new_{symbol_id}", span = span);
	let factory_name = syn::LitStr::new(&format!("closure.new.{symbol_id}"), span);
	let factory_embed = syn::LitStr::new(signature.kind.factory_embed(), span);
	let closure = signature.closure_type(&trait_object);
	let factory_js = syn::LitStr::new(
		&format!(
			"(data) => this.#jsEmbed.js_sys['{}'](data, this.#jsExports['{call_ident}'])",
			signature.kind.factory_embed(),
		),
		span,
	);
	let inputs: Vec<_> = signature.inputs.iter().collect();
	let arguments: Vec<_> = inputs
		.iter()
		.enumerate()
		.map(|(index, ty)| format_ident!("arg{index}", span = ty.span()))
		.collect();
	let output = &signature.output;
	let output_decl = if signature.returns_unit {
		TokenStream::new()
	} else {
		quote_spanned!(output.span()=> -> #output)
	};
	let call_shim_type = format_ident!("ClosureCallShim{id}", span = span);
	let call_impl = format_ident!("closure_call_impl_{symbol_id}", span = span);
	let allocate = format_ident!("closure_alloc_{symbol_id}", span = span);
	let closure_bound = if signature.kind == ClosureKind::Shared {
		quote_spanned!(span=> ::core::ops::Fn(#(#inputs),*) -> #output)
	} else {
		quote_spanned!(span=> ::core::ops::FnMut(#(#inputs),*) -> #output)
	};
	let call_body = if signature.kind == ClosureKind::Shared {
		quote_spanned! {span=>
			let callback = unsafe {
				&*#js_sys::r#macro::ClosureHeader::callback::<F, #call_shim_type>(pointer)
			};
			callback(#(#arguments),*)
		}
	} else {
		quote_spanned! {span=>
			let callback = unsafe {
				&mut *#js_sys::r#macro::ClosureHeader::callback::<F, #call_shim_type>(pointer)
			};
			callback(#(#arguments),*)
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
			type #call_shim_type = unsafe fn(
				*mut #js_sys::r#macro::ClosureHeader,
				#(#inputs),*
			) #output_decl;

			#[allow(clippy::undocumented_unsafe_blocks)]
			unsafe fn #call_impl<F>(
				pointer: *mut #js_sys::r#macro::ClosureHeader,
				#(#arguments: #inputs),*
			) #output_decl
			where
				F: #closure_bound,
			{
				#call_body
			}

			fn #allocate<F>(
				callback: F,
			) -> #js_sys::r#macro::ClosureAllocation
			where
				F: #closure_bound + 'static,
			{
				#js_sys::r#macro::ClosureAllocation::new(
					callback,
					#call_impl::<F> as #call_shim_type,
				)
			}

			#[#js_sys::js_sys(js_sys = #js_sys)]
			#[allow(clippy::undocumented_unsafe_blocks)]
			fn #call_ident(
				data: ::core::primitive::usize,
				#(#arguments: #inputs),*
			) #output_decl {
				let pointer = ::core::ptr::with_exposed_provenance_mut(data);
				let call_shim = unsafe {
					#js_sys::r#macro::ClosureHeader::call_shim::<#call_shim_type>(pointer)
				};
				unsafe { call_shim(pointer.cast(), #(#arguments),*) }
			}

			#js_sys::js_bindgen::embed_js! {
				module = #crate_name,
				name = #factory_name,
				required_embeds = [("js_sys", #factory_embed)],
				#factory_js,
			}

			#[#js_sys::js_sys(js_sys = #js_sys)]
			extern "js-sys" {
				#[js_sys(js_embed = #factory_name)]
				fn #factory_ident(
					data: ::core::primitive::usize,
				) -> #js_sys::JsValue;
			}

			let allocation = #allocate(#expression);
			let value = #factory_ident(allocation.data());
			allocation.forget();
			#js_sys::Closure::<#closure>::from_js_value(value)
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
	output: Type,
	returns_unit: bool,
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
		let output: Type = match &arguments.output {
			ReturnType::Default => parse_quote_spanned!(trait_object.span()=> ()),
			ReturnType::Type(_, output) => *output.clone(),
		};
		let returns_unit = matches!(&output, Type::Tuple(tuple) if tuple.elems.is_empty());

		Ok(Self {
			kind,
			inputs: arguments.inputs.clone(),
			output,
			returns_unit,
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
