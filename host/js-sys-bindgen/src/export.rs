use std::env;

use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::ext::IdentExt;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{Error, FnArg, ItemFn, LitStr, Path, ReturnType, Type, meta, parse_quote};

pub(crate) fn r#macro(
	attr: TokenStream,
	function: &ItemFn,
	crate_: Option<&str>,
) -> Result<TokenStream, Error> {
	let mut js_sys: Option<Path> = None;

	meta::parser(|meta| {
		if meta.path.is_ident("js_sys") {
			if js_sys.is_some() {
				Err(meta.error("duplicate `js_sys` argument"))
			} else {
				js_sys = Some(meta.value()?.parse()?);
				Ok(())
			}
		} else {
			Err(meta.error("unsupported attribute"))
		}
	})
	.parse2(attr)?;

	validate(function)?;

	let span = function.span();
	let js_sys: Path = js_sys.unwrap_or_else(|| parse_quote!(::js_sys));
	let js_bindgen: Path = parse_quote!(#js_sys::js_bindgen);
	let r#macro: Path = parse_quote!(#js_sys::r#macro);
	let ident = &function.sig.ident;
	let export_name_value = ident.unraw().to_string();
	let export_name = LitStr::new(&export_name_value, ident.span());
	let crate_name = crate_.map_or_else(
		|| env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found"),
		str::to_owned,
	);
	let crate_name = LitStr::new(&crate_name, span);
	let output_ty = match &function.sig.output {
		ReturnType::Type(_, ty) if matches!(ty.as_ref(), Type::Tuple(tuple) if tuple.elems.is_empty()) => {
			None
		}
		ReturnType::Type(_, ty) => Some(ty.as_ref()),
		ReturnType::Default => None,
	};
	let mut raw_inputs = Vec::new();
	let mut join_inputs = Vec::new();
	let mut arguments = Vec::new();
	let mut codegen_inputs = Vec::new();
	let mut required_embeds = Vec::new();

	for (index, input) in function.sig.inputs.iter().enumerate() {
		let FnArg::Typed(input) = input else {
			unreachable!();
		};
		let ty = &input.ty;
		let argument = format_ident!("arg{index}", span = input.span());
		let parameter = LitStr::new(&argument.to_string(), input.span());
		let reference = match ty.as_ref() {
			Type::Reference(reference) if reference.mutability.is_some() => {
				return Err(Error::new_spanned(
					reference,
					"mutable references are not supported",
				));
			}
			Type::Reference(reference) => Some(reference),
			_ => None,
		};
		let js_ty = reference.map_or_else(
			|| quote_spanned!(input.span()=> #ty),
			|reference| {
				let ty = &reference.elem;
				quote_spanned! {input.span()=>
					<#ty as #js_sys::hazard::RefFromJS>::Anchor
				}
			},
		);
		let mut slots = Vec::new();

		for slot in 1_usize..=4 {
			let slot_ident = format_ident!("arg{index}_{}", slot - 1, span = input.span());
			let slot_alias = format_ident!("OutputSlot{slot}", span = input.span());

			raw_inputs.push(quote_spanned! {input.span()=>
				#slot_ident: #r#macro::#slot_alias<#js_ty>
			});
			slots.push(slot_ident);
		}

		if let Some(reference) = reference {
			let anchor = format_ident!("arg{index}_anchor", span = input.span());
			let ty = &reference.elem;

			join_inputs.push(quote_spanned! {input.span()=>
				let #anchor = #r#macro::join_from_js::<#js_ty>(#(#slots),*);
				let #argument = ::core::borrow::Borrow::<#ty>::borrow(&#anchor);
			});
		} else {
			join_inputs.push(quote_spanned! {input.span()=>
				let #argument = #r#macro::join_from_js::<#js_ty>(#(#slots),*);
			});
		}

		codegen_inputs.push(quote_spanned! {input.span()=> (#parameter, #js_ty) });
		required_embeds.push(quote_spanned!(input.span()=> #r#macro::js_output_embed::<#js_ty>()));
		arguments.push(argument);
	}

	let call = if function.sig.unsafety.is_some() {
		quote_spanned!(span=> unsafe { #ident(#(#arguments),*) })
	} else {
		quote_spanned!(span=> #ident(#(#arguments),*))
	};
	let raw_name = LitStr::new(&format!("__export_{export_name_value}"), ident.span());
	let (raw_output, output_argument) = if let Some(output_ty) = output_ty {
		(
			quote_spanned! {output_ty.span()=>
				-> #js_sys::hazard::WasmRet<
					<#output_ty as #js_sys::hazard::ReturnIntoJS>::Abi
				>
			},
			quote_spanned!(output_ty.span()=> , #output_ty),
		)
	} else {
		(TokenStream::new(), TokenStream::new())
	};
	let raw_body = if output_ty.is_some() {
		quote_spanned! {span=>
			#(#join_inputs)*
			#r#macro::return_to_js(#call)
		}
	} else {
		quote_spanned! {span=>
			#(#join_inputs)*
			#call;
		}
	};
	if let Some(output_ty) = output_ty {
		required_embeds
			.push(quote_spanned!(output_ty.span()=> #r#macro::js_return_embed::<#output_ty>()));
	}

	Ok(quote_spanned! {span=>
		#function

		const _: () = {
			#[unsafe(export_name = #raw_name)]
			extern "C" fn export_raw(
				#(#raw_inputs),*
			) #raw_output {
				#raw_body
			}

			#js_bindgen::unsafe_global_wat! {
				"{}",
				interpolate #r#macro::wat_export!(
					#raw_name,
					#export_name,
					(#(#codegen_inputs),*)
					#output_argument,
				),
			}

			#js_bindgen::export_js! {
				module = #crate_name,
				name = #export_name,
				required_embeds = [
					#(#required_embeds),*
				],
				"{}",
				interpolate #r#macro::js_export!(
					#export_name,
					(#(#codegen_inputs),*)
					#output_argument,
				),
			}
		};
	})
}

fn validate(function: &ItemFn) -> Result<(), Error> {
	let sig = &function.sig;

	if let ReturnType::Type(_, ty) = &sig.output
		&& matches!(ty.as_ref(), Type::Reference(_))
	{
		return Err(Error::new_spanned(ty, "cannot return a borrowed reference"));
	}

	if let Some(constness) = sig.constness {
		return Err(Error::new_spanned(
			constness,
			"`const` functions are not supported",
		));
	}

	if let Some(asyncness) = sig.asyncness {
		return Err(Error::new_spanned(
			asyncness,
			"`async` functions are not supported",
		));
	}

	if let Some(abi) = &sig.abi {
		return Err(Error::new_spanned(
			abi,
			"explicit function ABIs are not supported",
		));
	}

	if !sig.generics.params.is_empty() || sig.generics.where_clause.is_some() {
		return Err(Error::new_spanned(
			&sig.generics,
			"generic functions are not supported",
		));
	}

	if let Some(variadic) = &sig.variadic {
		return Err(Error::new_spanned(
			variadic,
			"variadic functions are not supported",
		));
	}

	for input in &sig.inputs {
		if let FnArg::Receiver(receiver) = input {
			return Err(Error::new_spanned(receiver, "methods are not supported"));
		}
	}

	Ok(())
}
