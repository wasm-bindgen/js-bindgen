use std::env;

use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::ext::IdentExt;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{Error, Expr, FnArg, ItemFn, LitStr, Path, ReturnType, Type, meta, parse_quote};

pub(crate) fn r#macro(
	attr: TokenStream,
	function: &ItemFn,
	crate_: Option<&str>,
) -> Result<TokenStream, Error> {
	let mut js_sys: Option<Path> = None;
	let mut js_name: Option<Expr> = None;
	let mut promising = false;

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
		} else if meta.path.is_ident("js_name") {
			if js_name.is_some() {
				Err(meta.error("duplicate `js_name` argument"))
			} else {
				js_name = Some(meta.value()?.parse()?);
				Ok(())
			}
		} else if meta.path.is_ident("promising") {
			if meta.input.peek(syn::Token![=]) || meta.input.peek(syn::token::Paren) {
				Err(meta.error("`promising` supports no values"))
			} else if promising {
				Err(meta.error("duplicate `promising` argument"))
			} else {
				promising = true;
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
	let macro_path: Path = parse_quote!(#js_sys::wire);
	let ident = &function.sig.ident;
	let export_name = js_name.map_or_else(
		|| {
			let name = LitStr::new(&ident.unraw().to_string(), ident.span());
			quote_spanned!(ident.span()=> #name)
		},
		|name| quote_spanned!(name.span()=> #name),
	);
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
		wire_inputs,
		raw_output,
		wire_output,
		..
	} = lower_abi(inputs, output_ty, &js_sys)?;

	let call = if function.sig.unsafety.is_some() {
		quote_spanned!(span=> unsafe { #ident(#(#arguments),*) })
	} else {
		quote_spanned!(span=> #ident(#(#arguments),*))
	};
	let raw_export_name = quote_spanned!(ident.span()=> ::core::concat!("__export_", #export_name));
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
	let descriptor_constructor = if promising {
		format_ident!("new_symbol_promising", span = span)
	} else {
		format_ident!("new_symbol", span = span)
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

			#[expect(dead_code, reason = "stored in a custom section")]
			pub const WIRE: #macro_path::Wire =
				#macro_path::Wire::exports(&[
					#macro_path::WireExport::#descriptor_constructor(
						#crate_name,
						#export_name,
						#raw_export_name,
						&[#(#wire_inputs),*],
						#wire_output,
					),
				]);
			#[expect(dead_code, reason = "stored in a custom section")]
			pub const LEN: ::core::primitive::usize = #macro_path::wire_blob_len(&WIRE);

			#[expect(dead_code, reason = "stored in a custom section")]
			#[unsafe(link_section = "js_bindgen.wire")]
			pub static WIRE_SECTION: #macro_path::WireBlob<LEN> =
				#macro_path::WireBlob::new(&WIRE);
		};
	})
}

pub(crate) struct ExportAbi {
	pub raw_types: Vec<TokenStream>,
	pub raw_inputs: Vec<TokenStream>,
	pub join_inputs: Vec<TokenStream>,
	pub arguments: Vec<syn::Ident>,
	pub wire_inputs: Vec<TokenStream>,
	pub raw_output: TokenStream,
	pub wire_output: TokenStream,
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
	let mut wire_inputs = Vec::new();

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
			let raw_type = quote_spanned!(span=> #js_sys::wire::#slot_alias<#js_ty>);

			raw_types.push(raw_type.clone());
			raw_inputs.push(quote_spanned!(span=> #slot_ident: #raw_type));
			slots.push(slot_ident);
		}

		if let Some(reference) = reference {
			let anchor = format_ident!("arg{index}_anchor", span = span);
			let ty = &reference.elem;

			join_inputs.push(quote_spanned! {span=>
				let #anchor = #js_sys::wire::join_from_js::<#js_ty>(#(#slots),*);
				let #argument = ::core::borrow::Borrow::<#ty>::borrow(&#anchor);
			});
		} else {
			join_inputs.push(quote_spanned! {span=>
				let #argument = #js_sys::wire::join_from_js::<#js_ty>(#(#slots),*);
			});
		}

		wire_inputs.push(quote_spanned! {span=>
			#js_sys::wire::wire_export_input::<#js_ty>(#parameter)
		});
		arguments.push(argument);
	}

	let (raw_output, wire_output) = output.map_or_else(
		|| {
			(
				TokenStream::new(),
				quote_spanned!(js_sys.span()=> ::core::option::Option::None),
			)
		},
		|output| {
			(
				quote_spanned! {output.span()=>
					-> #js_sys::hazard::WasmRet<
						<#output as #js_sys::hazard::ReturnIntoJS>::Abi
					>
				},
				quote_spanned! {output.span()=>
					::core::option::Option::Some(
						#js_sys::wire::wire_export_output::<#output>()
					)
				},
			)
		},
	);

	Ok(ExportAbi {
		raw_types,
		raw_inputs,
		join_inputs,
		arguments,
		wire_inputs,
		raw_output,
		wire_output,
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
