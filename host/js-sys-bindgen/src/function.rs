use std::fmt::Write;
use std::ops::Deref;
use std::str::FromStr;
use std::{iter, slice};

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::spanned::Spanned;
use syn::{
	Error, FnArg, ForeignItemFn, ItemFn, Pat, PatIdent, PatType, Result, ReturnType, Stmt,
	parse_quote_spanned,
};

use crate::Hygiene;

pub struct Function(pub ItemFn);

#[derive(Eq, PartialEq)]
pub enum FunctionJsOutput {
	Generate(Option<String>),
	Embed(String),
	Import,
}

impl Default for FunctionJsOutput {
	fn default() -> Self {
		Self::Generate(None)
	}
}

impl Function {
	pub fn new(
		hygiene: &mut Hygiene<'_>,
		js_output: &FunctionJsOutput,
		namespace: Option<&str>,
		crate_: &str,
		item: &ForeignItemFn,
	) -> Result<Self> {
		if let Some(constness) = item.sig.constness {
			return Err(Error::new_spanned(
				constness,
				"`const` functions are not supported",
			));
		}

		if let Some(asyncness) = item.sig.asyncness {
			return Err(Error::new_spanned(
				asyncness,
				"`async` functions are not supported",
			));
		}

		if let Some(variadic) = &item.sig.variadic {
			return Err(Error::new_spanned(
				variadic,
				"variadic functions are not supported",
			));
		}

		let span = item.span();
		let ForeignItemFn {
			attrs,
			vis,
			sig,
			semi_token: _,
		} = item;
		let ident = &item.sig.ident;
		let js_bindgen = hygiene.js_bindgen(attrs, span);
		let input = hygiene.input(attrs, span);
		let output = hygiene.output(attrs, span);

		let import_name = if let Some(namespace) = namespace {
			format!("{namespace}.{ident}")
		} else {
			ident.to_string()
		};
		let parms_placeholder: String = iter::repeat_n("{}", sig.inputs.len()).join(", ");
		let ret_placeholder = if let ReturnType::Default = sig.output {
			""
		} else {
			"{}"
		};
		let import_funcs_placeholder: String = iter::repeat_n(
			r#""{}","","#,
			sig.inputs.len() + usize::from(matches!(sig.output, ReturnType::Type(..))),
		)
		.collect();
		let foreign_name = format!("{crate_}.{import_name}");
		let asm_param_gets = (0..sig.inputs.len()).fold(String::new(), |mut output, index| {
			write!(output, r#""\tlocal.get {index}","\t{{}}","#).unwrap();
			output
		});
		let asm_ret_conv = if let ReturnType::Default = sig.output {
			""
		} else {
			r#""\t{}","#
		};

		let asm = TokenStream::from_str(&format!(
			r#"".import_module {crate_}.import.{import_name}, {crate_}",
			".import_name {crate_}.import.{import_name}, {import_name}",
			".functype {crate_}.import.{import_name} ({parms_placeholder}) -> ({ret_placeholder})",
			"",
			{import_funcs_placeholder}
			".globl {foreign_name}",
			"{foreign_name}:",
			"\t.functype {foreign_name} ({parms_placeholder}) -> ({ret_placeholder})",
			{asm_param_gets}
			"\tcall {crate_}.import.{import_name}",
			{asm_ret_conv}
			"\tend_function","#
		))
		.unwrap();

		let input_tys =
			sig.inputs
				.iter()
				.map(|arg| {
					if let FnArg::Typed(PatType { pat, ty, .. }) = arg
						&& let Pat::Ident(PatIdent {
							attrs,
							by_ref: None,
							mutability: None,
							ident: _,
							subpat: None,
						}) = pat.deref() && attrs.is_empty()
					{
						Ok(ty)
					} else {
						Err(Error::new_spanned(arg, "unsupported arguments found"))
					}
				})
				.collect::<Result<Vec<_>>>()?;
		let output_ty = match &sig.output {
			ReturnType::Default => &[],
			ReturnType::Type(_, ty) => slice::from_ref(ty.deref()),
		};

		let asm: Stmt = parse_quote_spanned! {span=>
			#js_bindgen::unsafe_embed_asm! {
				#asm
				#(interpolate <#input_tys as #input>::IMPORT_TYPE,)*
				#(interpolate <#output_ty as #output>::IMPORT_TYPE,)*
				#(interpolate <#input_tys as #input>::IMPORT_FUNC,)*
				#(interpolate <#output_ty as #output>::IMPORT_FUNC,)*
				#(interpolate <#input_tys as #input>::TYPE,)*
				#(interpolate <#output_ty as #output>::TYPE,)*
				#(interpolate <#input_tys as #input>::CONV,)*
				#(interpolate <#output_ty as #output>::CONV,)*
			}
		};

		let input_names: Vec<_> = sig
			.inputs
			.iter()
			.map(|arg| {
				if let FnArg::Typed(PatType { pat, .. }) = arg
					&& let Pat::Ident(PatIdent { ident, .. }) = pat.deref()
				{
					ident
				} else {
					unreachable!()
				}
			})
			.collect();

		let js: Stmt = 'js: {
			let js_call = match js_output {
				FunctionJsOutput::Generate(None) => format!("globalThis.{import_name}"),
				FunctionJsOutput::Generate(Some(name)) => {
					if let Some(namespace) = namespace {
						format!("globalThis.{namespace}.{name}")
					} else {
						format!("globalThis.{name}")
					}
				}
				FunctionJsOutput::Embed(name) => {
					format!("this.#jsEmbed.{crate_}['{name}']")
				}
				FunctionJsOutput::Import => {
					break 'js parse_quote_spanned! {span=>
						#js_bindgen::import_js!(name = #import_name, no_import);
					};
				}
			};

			let required_embed = if let FunctionJsOutput::Embed(name) = js_output {
				slice::from_ref(name)
			} else {
				&[]
			};

			if sig.inputs.is_empty() {
				break 'js parse_quote_spanned! {span=>
					#js_bindgen::import_js!(
						name = #import_name,
						#(required_embed = #required_embed,)*
						#js_call
					);
				};
			}

			let placeholder: String = iter::once("{}")
				.chain(iter::repeat_n("{}{}{}", sig.inputs.len()))
				.chain(iter::once("{}"))
				.collect();

			let input_names_joined = input_names.iter().join(", ");
			let js_arrow_open = format!("({input_names_joined}) => {{\n",);
			let input_conv = input_names.iter().map(|arg| format!("\t{arg}"));
			let ret_call = match sig.output {
				ReturnType::Default => "",
				ReturnType::Type(..) => "return ",
			};
			let js_arrow_close = format!("\t{ret_call}{js_call}({input_names_joined})\n}}");
			let r#macro = hygiene.r#macro(attrs, span);

			parse_quote_spanned! {span=>
				#js_bindgen::import_js! {
					name = #import_name,
					#(required_embed = #required_embed,)*
					#placeholder,
					interpolate #r#macro::select(#js_call, #js_arrow_open, [#(<#input_tys as #input>::JS_CONV),*]),
					#(
						interpolate #r#macro::select("", #input_conv, [<#input_tys as #input>::JS_CONV]),
						interpolate #r#macro::select("", <#input_tys as #input>::JS_CONV, [<#input_tys as #input>::JS_CONV]),
						interpolate #r#macro::select("", "\n", [<#input_tys as #input>::JS_CONV]),
					)*
					interpolate #r#macro::select("", #js_arrow_close, [#(<#input_tys as #input>::JS_CONV),*]),
				}
			}
		};

		let mut foreign_call =
			quote_spanned!(span=> unsafe { #ident(#(#input::into_raw(#input_names)),*) });
		match &sig.output {
			ReturnType::Default => foreign_call.extend(quote_spanned!(span=> ;)),
			ReturnType::Type(..) => {
				foreign_call = quote_spanned! (span=> #output::from_raw(#foreign_call));
			}
		}

		Ok(Self(parse_quote_spanned! {span=>
			#(#attrs)*
			#vis #sig {
				#asm

				#js

				unsafe extern "C" {
					#[link_name = #foreign_name]
					fn #ident(#(#input_names: <#input_tys as #input>::Type),*) #( -> <#output_ty as #output>::Type)*;
				}

				#foreign_call
			}
		}))
	}
}

impl ToTokens for Function {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens);
	}
}
