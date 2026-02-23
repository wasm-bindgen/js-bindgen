use std::borrow::Cow;
use std::fmt::Write;
use std::ops::Deref;
use std::str::FromStr;
use std::{iter, mem, slice};

use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
	Attribute, Error, FnArg, ForeignItemFn, GenericArgument, GenericParam, Generics, Ident, Item,
	ItemFn, ItemImpl, Pat, PatIdent, PatType, Path, PathArguments, Receiver, Result, ReturnType,
	Stmt, Token, Type, TypePath, TypeReference, parse_quote, parse_quote_spanned,
};

use crate::Hygiene;

pub enum Function {
	Fn(ItemFn),
	Impl(ItemImpl),
}

#[derive(Eq, PartialEq)]
pub enum FunctionJsOutput {
	Generate {
		js_name: Option<String>,
		property: bool,
	},
	Embed(String),
	Import,
}

struct State<'a, 'h> {
	crate_: &'a str,
	hygiene: &'a mut Hygiene<'h>,
	namespace: Option<&'a str>,
	js_bindgen: Path,
	input: Path,
	output: Path,
	outer_attrs: &'a [Attribute],
	import_name: String,
	foreign_name: String,
	input_tys: Vec<Cow<'a, Box<Type>>>,
	output_ty: &'a [Type],
	extern_input_names: Vec<Cow<'a, Ident>>,
	intern_input_names: Vec<Cow<'a, Ident>>,
	impl_generic_params: TokenStream,
	r#type: OutputType<'a>,
	span: Span,
}

enum OutputType<'p> {
	Generate {
		js_name: Option<String>,
		member: Option<Member<'p>>,
	},
	Embed(String),
	Import,
}

struct Member<'p> {
	self_ty: &'p Path,
	r#type: MemberType,
}

enum MemberType {
	Method,
	Getter,
	Setter,
}

impl Function {
	pub fn new(
		hygiene: &mut Hygiene<'_>,
		js_output: FunctionJsOutput,
		namespace: Option<&str>,
		crate_: &str,
		item: ForeignItemFn,
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
			mut sig,
			..
		} = item;

		let mut state = State::parse(
			crate_,
			js_output,
			namespace,
			hygiene,
			&attrs,
			&mut sig.generics,
			&sig.ident,
			&sig.inputs,
			&sig.output,
			span,
		)?;
		let asm = state.asm();
		let js = state.js();
		let State {
			input,
			output,
			foreign_name,
			input_tys,
			output_ty,
			extern_input_names,
			intern_input_names,
			impl_generic_params,
			r#type,
			..
		} = state;
		let ident = &sig.ident;

		let mut foreign_call =
			quote_spanned!(span=> unsafe { #ident(#(#input::into_raw(#intern_input_names)),*) });
		if output_ty.is_empty() {
			foreign_call.extend(quote_spanned!(span=> ;));
		} else {
			foreign_call = quote_spanned! (span=> #output::from_raw(#foreign_call));
		}

		let item_fn = parse_quote_spanned! {span=>
			#(#attrs)*
			#vis #sig {
				#asm

				#js

				unsafe extern "C" {
					#[link_name = #foreign_name]
					fn #ident(#(#extern_input_names: <#input_tys as #input>::Type),*) #( -> <#output_ty as #output>::Type)*;
				}

				#foreign_call
			}
		};

		if let Some(Member { self_ty, .. }) = r#type.member() {
			Ok(Self::Impl(parse_quote_spanned! {span=>
				impl #impl_generic_params #self_ty {
					#item_fn
				}
			}))
		} else {
			Ok(Self::Fn(item_fn))
		}
	}
}

impl From<Function> for Item {
	fn from(value: Function) -> Self {
		match value {
			Function::Fn(item) => item.into(),
			Function::Impl(item) => item.into(),
		}
	}
}

impl ToTokens for Function {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Fn(item) => item.to_tokens(tokens),
			Self::Impl(item) => item.to_tokens(tokens),
		}
	}
}

impl Default for FunctionJsOutput {
	fn default() -> Self {
		Self::Generate {
			js_name: None,
			property: false,
		}
	}
}

impl<'a, 'h> State<'a, 'h> {
	#[expect(clippy::too_many_arguments, reason = "TODO")]
	fn parse(
		crate_: &'a str,
		js_output: FunctionJsOutput,
		namespace: Option<&'a str>,
		hygiene: &'a mut Hygiene<'h>,
		outer_attrs: &'a [Attribute],
		generics: &mut Generics,
		ident: &Ident,
		inputs: &'a Punctuated<FnArg, Token![,]>,
		output: &'a ReturnType,
		span: Span,
	) -> Result<Self> {
		let import_name = if let Some(namespace) = namespace {
			format!("{namespace}.{ident}")
		} else {
			ident.to_string()
		};
		let foreign_name = format!("{crate_}.{import_name}");

		let mut self_ty = None;

		let input_tys = inputs
			.iter()
			.map(|arg| {
				if let FnArg::Typed(PatType { pat, ty, .. }) = arg
					&& let Pat::Ident(PatIdent {
						attrs,
						by_ref: None,
						mutability: None,
						ident: _,
						subpat: None,
					}) = pat.deref()
					&& attrs.is_empty()
				{
					Ok(Cow::Borrowed(ty))
				} else if let FnArg::Receiver(Receiver {
					attrs,
					reference: None,
					mutability: None,
					self_token: _,
					colon_token: Some(_),
					ty,
				}) = arg && attrs.is_empty()
					&& let Type::Reference(TypeReference {
						and_token,
						lifetime: None,
						mutability: None,
						elem,
					}) = ty.deref() && let Type::Path(TypePath { qself: None, path }) =
					elem.deref()
				{
					if !matches!(js_output, FunctionJsOutput::Generate { .. }) {
						return Err(Error::new_spanned(
							path,
							"`self` is not supported with `js_import` and `js_embed`",
						));
					}

					self_ty = Some(path);
					let js_value = hygiene.js_value(outer_attrs, span);
					Ok(Cow::Owned(parse_quote! { #and_token #js_value }))
				} else {
					Err(Error::new_spanned(arg, "unsupported arguments found"))
				}
			})
			.collect::<Result<Vec<_>>>()?;

		let r#type = match js_output {
			FunctionJsOutput::Generate { js_name, property } => {
				let member = if let Some(self_ty) = self_ty {
					let r#type = if property {
						match (inputs.len(), &output) {
							(1, ReturnType::Type(..)) => MemberType::Getter,
							(2, ReturnType::Default) => MemberType::Setter,
							_ => {
								return Err(Error::new(
									span,
									"`property` requires a getter or setter signature",
								));
							}
						}
					} else {
						MemberType::Method
					};

					Some(Member { self_ty, r#type })
				} else {
					if property {
						return Err(Error::new(span, "`property` requires `self` parameter"));
					}

					None
				};

				OutputType::Generate { js_name, member }
			}
			FunctionJsOutput::Embed(embed) => OutputType::Embed(embed),
			FunctionJsOutput::Import => OutputType::Import,
		};

		let output_ty = match &output {
			ReturnType::Default => &[],
			ReturnType::Type(_, ty) => slice::from_ref(ty.deref()),
		};

		let (extern_input_names, intern_input_names): (Vec<_>, Vec<_>) = inputs
			.iter()
			.map(|arg| {
				if let FnArg::Typed(PatType { pat, .. }) = arg
					&& let Pat::Ident(PatIdent { ident, .. }) = pat.deref()
				{
					(Cow::Borrowed(ident), Cow::Borrowed(ident))
				} else if let FnArg::Receiver(Receiver { self_token, .. }) = arg {
					(
						Cow::Owned(Ident::new("this", Span::mixed_site())),
						Cow::Owned((*self_token).into()),
					)
				} else {
					unreachable!()
				}
			})
			.collect();

		let impl_generic_params = Self::impl_generic_params(&r#type, generics);

		let js_bindgen = hygiene.js_bindgen(outer_attrs, span);
		let input = hygiene.input(outer_attrs, span);
		let output = hygiene.output(outer_attrs, span);

		Ok(Self {
			crate_,
			hygiene,
			namespace,
			js_bindgen,
			input,
			output,
			outer_attrs,
			import_name,
			foreign_name,
			input_tys,
			output_ty,
			extern_input_names,
			intern_input_names,
			impl_generic_params,
			r#type,
			span,
		})
	}

	// Extract type generics from signature that are part of `impl <type>`.
	fn impl_generic_params(r#type: &OutputType, generics: &mut Generics) -> TokenStream {
		if let Some(member) = r#type.member() {
			let mut fn_generic_params: Vec<_> =
				mem::take(&mut generics.params).into_iter().collect();

			let impl_generic_params: Vec<_> = fn_generic_params
				.extract_if(.., |param| {
					for path in &member.self_ty.segments {
						if let PathArguments::AngleBracketed(args) = &path.arguments {
							for arg in &args.args {
								match (&*param, arg) {
									(
										GenericParam::Lifetime(param),
										GenericArgument::Lifetime(arg),
									) => {
										if &param.lifetime == arg {
											return true;
										}
									}
									(
										GenericParam::Type(param),
										GenericArgument::Type(Type::Path(TypePath {
											qself: None,
											path,
										})),
									) => {
										if let Some(arg) = path.get_ident()
											&& &param.ident == arg
										{
											return true;
										}
									}
									_ => (),
								}
							}
						}
					}

					false
				})
				.collect();

			generics.params = fn_generic_params.into_iter().collect();

			if impl_generic_params.is_empty() {
				TokenStream::new()
			} else {
				let lt = generics.lt_token.unwrap();
				let gt = generics.gt_token.unwrap();

				quote!(#lt #(#impl_generic_params),* #gt)
			}
		} else {
			TokenStream::new()
		}
	}

	fn asm(&self) -> Stmt {
		let Self {
			crate_,
			js_bindgen,
			input,
			output,
			import_name,
			foreign_name,
			input_tys,
			output_ty,
			span,
			..
		} = self;

		let parms_placeholder: String = iter::repeat_n("{}", self.input_tys.len()).join(", ");
		let ret_placeholder = if self.output_ty.is_empty() { "" } else { "{}" };
		let import_funcs_placeholder: String =
			iter::repeat_n(r#""{}","","#, self.input_tys.len() + self.output_ty.len()).collect();
		let asm_param_gets = (0..self.input_tys.len()).fold(String::new(), |mut output, index| {
			write!(output, r#""\tlocal.get {index}","\t{{}}","#).unwrap();
			output
		});
		let asm_ret_conv = if self.output_ty.is_empty() {
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

		parse_quote_spanned! {*span=>
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
		}
	}

	fn js(&mut self) -> Option<Stmt> {
		let Self {
			crate_,
			hygiene,
			js_bindgen,
			input,
			import_name,
			outer_attrs,
			input_tys,
			output_ty,
			intern_input_names,
			r#type,
			span,
			..
		} = self;

		let js_path = match r#type {
			OutputType::Generate { js_name, member } => {
				let base = if member.is_some() {
					"self"
				} else {
					"globalThis"
				};

				if let Some(js_name) = js_name {
					if let Some(namespace) = self.namespace {
						format!("{base}.{namespace}.{js_name}")
					} else {
						format!("{base}.{js_name}")
					}
				} else {
					format!("{base}.{import_name}")
				}
			}
			OutputType::Embed(name) => {
				format!("this.#jsEmbed.{crate_}['{name}']")
			}
			OutputType::Import => return None,
		};

		let required_embed = if let OutputType::Embed(name) = &r#type {
			slice::from_ref(name)
		} else {
			&[]
		};

		if input_tys.is_empty() {
			return Some(parse_quote_spanned! {*span=>
				#js_bindgen::import_js!(
					name = #import_name,
					#(required_embeds = [#required_embed],)*
					#js_path
				);
			});
		}

		let placeholder: String = iter::once("{}")
			.chain(iter::repeat_n("{}{}{}", input_tys.len()))
			.chain(iter::once("{}"))
			.collect();

		let input_names_joined = intern_input_names.iter().join(", ");
		let call_input_names_joined = if r#type.member().is_some() {
			Cow::Owned(intern_input_names.iter().skip(1).join(", "))
		} else {
			Cow::Borrowed(&input_names_joined)
		};
		let js_arrow_open = format!("({input_names_joined}) => {{\n",);
		let input_conv = intern_input_names.iter().map(|arg| format!("\t{arg}"));
		let ret_call = if output_ty.is_empty() { "" } else { "return " };
		let direct_js_call = if let Some(member) = r#type.member() {
			match member.r#type {
				MemberType::Method => Cow::Owned(format!(
					"({input_names_joined}) => {js_path}({call_input_names_joined})"
				)),
				MemberType::Getter => Cow::Owned(format!("({input_names_joined}) => {js_path}")),
				MemberType::Setter => Cow::Owned(format!(
					"({input_names_joined}) => {js_path} = {call_input_names_joined}"
				)),
			}
		} else {
			Cow::Borrowed(&js_path)
		};
		let indirect_js_call = if let Some(member) = r#type.member() {
			match member.r#type {
				MemberType::Method => Cow::Owned(format!("{js_path}({call_input_names_joined})")),
				MemberType::Getter => Cow::Borrowed(&js_path),
				MemberType::Setter => Cow::Owned(format!("{js_path} = {call_input_names_joined}")),
			}
		} else {
			Cow::Owned(format!("{js_path}({input_names_joined})"))
		};
		let js_arrow_close = format!("\t{ret_call}{indirect_js_call}\n}}");
		let r#macro = hygiene.r#macro(outer_attrs, *span);

		Some(parse_quote_spanned! {*span=>
			#js_bindgen::import_js! {
				name = #import_name,
				#(required_embeds = [#required_embed],)*
				#placeholder,
				interpolate #r#macro::select(#direct_js_call, #js_arrow_open, [#(<#input_tys as #input>::JS_CONV),*]),
				#(
					interpolate #r#macro::select("", #input_conv, [<#input_tys as #input>::JS_CONV]),
					interpolate #r#macro::select("", <#input_tys as #input>::JS_CONV, [<#input_tys as #input>::JS_CONV]),
					interpolate #r#macro::select("", "\n", [<#input_tys as #input>::JS_CONV]),
				)*
				interpolate #r#macro::select("", #js_arrow_close, [#(<#input_tys as #input>::JS_CONV),*]),
			}
		})
	}
}

impl OutputType<'_> {
	fn member(&self) -> Option<&Member<'_>> {
		if let Self::Generate { member, .. } = self {
			member.as_ref()
		} else {
			None
		}
	}
}
