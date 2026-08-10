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
			// On an exported Rust function, `js_sys` only overrides the crate
			// path used by generated support code.
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
	let js_bindgen_path: Path = parse_quote!(#js_sys::js_bindgen);
	let macro_path: Path = parse_quote!(#js_sys::r#macro);
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
	let inputs = function.sig.inputs.iter().map(|input| {
		let FnArg::Typed(input) = input else {
			unreachable!();
		};
		input.ty.as_ref()
	});
	let ExportAbi {
		raw_inputs,
		join_inputs,
		arguments,
		codegen_inputs,
		required_embeds,
		raw_output,
		output_argument,
		..
	} = lower_abi(inputs, output_ty, &js_sys)?;

	let call = if function.sig.unsafety.is_some() {
		quote_spanned!(span=> unsafe { #ident(#(#arguments),*) })
	} else {
		quote_spanned!(span=> #ident(#(#arguments),*))
	};
	let raw_export_name = LitStr::new(&format!("__export_{export_name_value}"), ident.span());
	let raw_body = if output_ty.is_some() {
		quote_spanned! {span=>
			#(#join_inputs)*
			#macro_path::return_to_js(#call)
		}
	} else {
		quote_spanned! {span=>
			#(#join_inputs)*
			#call;
		}
	};

	Ok(quote_spanned! {span=>
		#function

		const _: () = {
			#[unsafe(export_name = #raw_export_name)]
			extern "C" fn export_raw(
				#(#raw_inputs),*
			) #raw_output {
				#raw_body
			}

			#js_bindgen_path::unsafe_global_wat! {
				"{}",
				interpolate #macro_path::wat_export!(
					#raw_export_name,
					#export_name,
					(#(#codegen_inputs),*)
					#output_argument,
				),
			}

			#js_bindgen_path::export_js! {
				module = #crate_name,
				name = #export_name,
				required_embeds = [
					#(#required_embeds),*
				],
				"{}",
				interpolate #macro_path::js_export!(
					#export_name,
					(#(#codegen_inputs),*)
					#output_argument,
				),
			}
		};
	})
}

pub(crate) struct ExportAbi {
	pub raw_types: Vec<TokenStream>,
	pub raw_inputs: Vec<TokenStream>,
	pub join_inputs: Vec<TokenStream>,
	pub arguments: Vec<syn::Ident>,
	pub codegen_inputs: Vec<TokenStream>,
	pub required_embeds: Vec<TokenStream>,
	pub raw_output: TokenStream,
	pub output_argument: TokenStream,
}

pub(crate) fn lower_abi<'a>(
	inputs: impl IntoIterator<Item = &'a Type>,
	output: Option<&Type>,
	js_sys: &Path,
) -> Result<ExportAbi, Error> {
	let mut raw_types = Vec::new();
	let mut raw_inputs = Vec::new();
	let mut join_inputs = Vec::new();
	let mut arguments = Vec::new();
	let mut codegen_inputs = Vec::new();
	let mut required_embeds = Vec::new();

	for (index, ty) in inputs.into_iter().enumerate() {
		let span = ty.span();
		let argument = format_ident!("arg{index}", span = span);
		let parameter = LitStr::new(&argument.to_string(), span);
		let reference = match ty {
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
			|| quote_spanned!(span=> #ty),
			|reference| {
				let ty = &reference.elem;
				quote_spanned! {span=>
					<#ty as #js_sys::hazard::RefFromJS>::Anchor
				}
			},
		);
		let mut slots = Vec::new();

		for slot in 1_usize..=4 {
			let slot_ident = format_ident!("arg{index}_{}", slot - 1, span = span);
			let slot_alias = format_ident!("FromJsSlot{slot}", span = span);
			let raw_type = quote_spanned!(span=> #js_sys::r#macro::#slot_alias<#js_ty>);

			raw_types.push(raw_type.clone());
			raw_inputs.push(quote_spanned!(span=> #slot_ident: #raw_type));
			slots.push(slot_ident);
		}

		if let Some(reference) = reference {
			let anchor = format_ident!("arg{index}_anchor", span = span);
			let ty = &reference.elem;

			join_inputs.push(quote_spanned! {span=>
				let #anchor = #js_sys::r#macro::join_from_js::<#js_ty>(#(#slots),*);
				let #argument = ::core::borrow::Borrow::<#ty>::borrow(&#anchor);
			});
		} else {
			join_inputs.push(quote_spanned! {span=>
				let #argument = #js_sys::r#macro::join_from_js::<#js_ty>(#(#slots),*);
			});
		}

		codegen_inputs.push(quote_spanned!(span=> (#parameter, #js_ty)));
		required_embeds.push(quote_spanned!(span=> #js_sys::r#macro::js_from_embed::<#js_ty>()));
		arguments.push(argument);
	}

	let (raw_output, output_argument) = output.map_or_else(
		|| (TokenStream::new(), TokenStream::new()),
		|output| {
			(
				quote_spanned! {output.span()=>
					-> #js_sys::hazard::WasmRet<
						<#output as #js_sys::hazard::ReturnIntoJS>::Abi
					>
				},
				quote_spanned!(output.span()=> , #output),
			)
		},
	);

	if let Some(output) = output {
		required_embeds
			.push(quote_spanned!(output.span()=> #js_sys::r#macro::js_return_embed::<#output>()));
	}

	Ok(ExportAbi {
		raw_types,
		raw_inputs,
		join_inputs,
		arguments,
		codegen_inputs,
		required_embeds,
		raw_output,
		output_argument,
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
