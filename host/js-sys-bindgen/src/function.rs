use std::mem;
use std::ops::DerefMut;
use std::string::ToString;

use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
	Attribute, Error, FnArg, ForeignItemFn, GenericArgument, GenericParam, Generics, Ident, Item,
	ItemFn, ItemImpl, Pat, PatIdent, PatType, Path, PathArguments, Receiver, Result, ReturnType,
	Signature, Stmt, Token, Type, TypePath, TypeReference, parse_quote, parse_quote_spanned,
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

struct State<'a> {
	crate_: &'a str,
	namespace: Option<&'a str>,
	js_bindgen: Path,
	r#macro: Path,
	import_name: String,
	foreign_name: String,
	inputs: Vec<InputArg>,
	output_ty: Option<Type>,
	impl_generic_params: TokenStream,
	r#type: OutputType,
	span: Span,
}

struct InputArg {
	abi_type: Type,
	rust_name: Ident,
	wat_name: syn::LitStr,
	slot_names: [Ident; 4],
	type_override: bool,
}

enum OutputType {
	Generate {
		js_name: Option<String>,
		member: Option<Member>,
	},
	Embed(String),
	Import,
}

struct Member {
	self_ty: Path,
	r#type: MemberType,
}

enum MemberType {
	Method,
	Getter,
	Setter,
}

impl InputArg {
	fn new(index: usize, abi_type: Type, rust_name: Ident, type_override: bool) -> Self {
		let span = Span::mixed_site();
		let base = format!("arg{index}");
		let slot_name = |slot| Ident::new(&format!("{base}_{slot}"), span);

		Self {
			abi_type,
			rust_name,
			wat_name: syn::LitStr::new(&base, span),
			slot_names: [slot_name(0), slot_name(1), slot_name(2), slot_name(3)],
			type_override,
		}
	}
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

		let state = State::parse(
			crate_, js_output, namespace, hygiene, &attrs, &mut sig, span,
		)?;
		let wat = state.wat();
		let js = state.js();
		let State {
			r#macro,
			foreign_name,
			inputs,
			output_ty,
			impl_generic_params,
			r#type,
			..
		} = state;
		let ident = &sig.ident;
		let split_inputs = inputs.iter().map(|input| {
			let InputArg {
				abi_type,
				rust_name,
				slot_names: [slot1, slot2, slot3, slot4],
				type_override,
				..
			} = input;
			let split_input = if *type_override {
				quote_spanned!(span=> unsafe {
					#r#macro::split_input_as::<#abi_type>(#rust_name)
				})
			} else {
				quote_spanned!(span=> #r#macro::split_input::<#abi_type>(#rust_name))
			};

			quote_spanned! {span=>
				let (#slot1, #slot2, #slot3, #slot4) = #split_input;
			}
		});
		let foreign_input_names: Vec<_> = inputs.iter().flat_map(|arg| &arg.slot_names).collect();
		let foreign_input_tys: Vec<_> = inputs
			.iter()
			.flat_map(|arg| {
				let ty = &arg.abi_type;

				[
					quote_spanned!(span=> #r#macro::InputSlot1<#ty>),
					quote_spanned!(span=> #r#macro::InputSlot2<#ty>),
					quote_spanned!(span=> #r#macro::InputSlot3<#ty>),
					quote_spanned!(span=> #r#macro::InputSlot4<#ty>),
				]
			})
			.collect();
		let foreign_output = output_ty.as_ref().map_or_else(
			TokenStream::new,
			|ty| quote_spanned!(span=> -> #r#macro::OutputRet<#ty>),
		);

		let foreign_call = quote_spanned! {span=> {
			#(#split_inputs)*
			unsafe { #ident(#(#foreign_input_names),*) }
		}};
		let foreign_call = if output_ty.is_some() {
			quote_spanned!(span=> #r#macro::join_output(#foreign_call))
		} else {
			quote_spanned!(span=> #foreign_call;)
		};

		let item_fn = parse_quote_spanned! {span=>
			#(#attrs)*
			#vis #sig {
				#wat

				#js

				unsafe extern "C" {
					#[link_name = #foreign_name]
					fn #ident(#(#foreign_input_names: #foreign_input_tys),*) #foreign_output;
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

impl<'a> State<'a> {
	fn parse(
		crate_: &'a str,
		js_output: FunctionJsOutput,
		namespace: Option<&'a str>,
		hygiene: &'a mut Hygiene<'_>,
		outer_attrs: &'a [Attribute],
		sig: &mut Signature,
		span: Span,
	) -> Result<Self> {
		let import_name = if let Some(namespace) = namespace {
			format!("{namespace}.{}", sig.ident)
		} else {
			sig.ident.to_string()
		};
		let foreign_name = format!("{crate_}.{import_name}");

		let mut self_ty = None;

		let inputs = sig
			.inputs
			.iter_mut()
			.enumerate()
			.map(|(index, arg)| {
				if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = arg
					&& let Pat::Ident(PatIdent {
						attrs: inner_attrs,
						by_ref: None,
						mutability: None,
						ident,
						subpat: None,
					}) = pat.deref_mut()
					&& inner_attrs.is_empty()
				{
					let mut r#type = None;

					if let Some(attr) = attrs
						.extract_if(.., |attr| attr.path().is_ident("js_sys"))
						.next()
					{
						attr.parse_nested_meta(|meta| {
							if meta.path.is_ident("type") {
								meta.input.parse::<Token![=]>()?;

								if r#type.replace(meta.input.parse::<Type>()?).is_some() {
									Err(meta.error("duplicate attribute"))
								} else {
									Ok(())
								}
							} else {
								Err(meta.error("unsupported attribute"))
							}
						})?;
					}

					let type_override = r#type.is_some();
					let r#type = r#type.unwrap_or_else(|| *ty.clone());

					Ok(InputArg::new(index, r#type, ident.clone(), type_override))
				} else if let FnArg::Receiver(Receiver {
					attrs,
					reference: None,
					mutability: None,
					self_token,
					colon_token: Some(_),
					ty,
				}) = arg && attrs.is_empty()
					&& let Type::Reference(TypeReference {
						and_token,
						lifetime: None,
						mutability: None,
						elem,
					}) = ty.deref_mut()
					&& let Type::Path(TypePath { qself: None, path }) = elem.deref_mut()
				{
					if !matches!(&js_output, FunctionJsOutput::Generate { .. }) {
						return Err(Error::new_spanned(
							path,
							"`self` is not supported with `js_import` and `js_embed`",
						));
					}

					self_ty = Some(path.clone());
					let js_value = hygiene.js_value(outer_attrs, span);
					Ok(InputArg::new(
						index,
						parse_quote! { #and_token #js_value },
						(*self_token).into(),
						true,
					))
				} else {
					Err(Error::new_spanned(arg, "unsupported arguments found"))
				}
			})
			.collect::<Result<Vec<_>>>()?;

		let r#type = match js_output {
			FunctionJsOutput::Generate { js_name, property } => {
				let member = if let Some(self_ty) = self_ty {
					let r#type = if property {
						match (sig.inputs.len(), &sig.output) {
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

		let output_ty = match &sig.output {
			ReturnType::Default => None,
			ReturnType::Type(_, ty) => Some(*ty.clone()),
		};

		let impl_generic_params = Self::impl_generic_params(&r#type, &mut sig.generics);

		let js_bindgen = hygiene.js_bindgen(outer_attrs, span);
		let r#macro = hygiene.r#macro(outer_attrs, span);

		Ok(Self {
			crate_,
			namespace,
			js_bindgen,
			r#macro,
			import_name,
			foreign_name,
			inputs,
			output_ty,
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
									) if &param.lifetime == arg => {
										return true;
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

	fn wat(&self) -> Stmt {
		let Self {
			crate_,
			js_bindgen,
			r#macro,
			import_name,
			foreign_name,
			inputs,
			output_ty,
			span,
			..
		} = self;
		let inputs = inputs.iter().map(|input| {
			let name = &input.wat_name;
			let ty = &input.abi_type;

			quote_spanned!(*span=> (#name, #ty))
		});
		let output = output_ty.iter();

		parse_quote_spanned! {*span=>
			#js_bindgen::unsafe_global_wat! {
				"{}",
				interpolate #r#macro::wat_import!(
					module = #crate_,
					import = #import_name,
					adapter = #foreign_name,
					inputs = [#(#inputs),*],
					#(output = #output,)*
				),
			}
		}
	}

	fn js(&self) -> Option<Stmt> {
		let Self {
			crate_,
			js_bindgen,
			r#macro,
			import_name,
			inputs,
			output_ty,
			r#type,
			span,
			..
		} = self;
		let input_tys: Vec<_> = inputs.iter().map(|input| &input.abi_type).collect();
		let input_names: Vec<_> = inputs.iter().map(|input| &input.wat_name).collect();
		let input_value_names: Vec<_> = inputs
			.iter()
			.map(|input| input.slot_names[0].to_string())
			.collect();
		let output_tys: Vec<_> = output_ty.iter().collect();

		let js_path = match r#type {
			OutputType::Generate { js_name, member } => {
				let base = if member.is_some() {
					input_value_names[0].as_str()
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

		let mut unique_inputs = Vec::new();

		for &ty in &input_tys {
			if !unique_inputs.contains(&ty) {
				unique_inputs.push(ty);
			}
		}

		let mut required_embeds = Vec::new();

		if let OutputType::Embed(name) = &r#type {
			required_embeds.push(quote_spanned!(*span=> (#crate_, #name)));
		}

		for ty in &unique_inputs {
			required_embeds.push(quote_spanned!(*span=> #r#macro::js_input_embed::<#ty>()));
		}

		for &ty in &output_tys {
			required_embeds.push(quote_spanned!(*span=> #r#macro::js_output_embed::<#ty>()));
			required_embeds.push(quote_spanned!(*span=> #r#macro::js_result_embed::<#ty>()));
		}

		let required_embeds = if required_embeds.is_empty() {
			[].as_slice()
		} else {
			&[quote_spanned!(*span=> required_embeds = [#(#required_embeds),*])]
		};

		let input_names_joined = input_value_names.iter().join(", ");
		let call_input_names_joined = if r#type.member().is_some() {
			input_value_names.iter().skip(1).join(", ")
		} else {
			input_names_joined.clone()
		};
		let js_inputs: Vec<_> = input_names
			.iter()
			.zip(input_tys.iter())
			.map(|(name, ty)| quote_spanned!(*span=> (#name, #ty)))
			.collect();
		let direct_fn_open = if r#type.member().is_none() {
			quote_spanned!(*span=> "")
		} else {
			quote_spanned!(*span=>
				#r#macro::js_function!("(", ") => ", #(#js_inputs),*)
			)
		};
		let direct_js_call = if let Some(member) = r#type.member() {
			match member.r#type {
				MemberType::Method => format!("{js_path}({call_input_names_joined})"),
				MemberType::Getter => js_path.clone(),
				MemberType::Setter => format!("{js_path} = {call_input_names_joined}"),
			}
		} else {
			js_path.clone()
		};
		let indirect_js_call = if r#type.member().is_some() {
			direct_js_call.clone()
		} else {
			format!("{js_path}({input_names_joined})")
		};
		let output = output_ty.iter();

		Some(parse_quote_spanned! {*span=>
			#js_bindgen::import_js! {
				module = #crate_,
				name = #import_name,
				#(#required_embeds,)*
				"{}",
				interpolate #r#macro::js_import!(
					direct_open = #direct_fn_open,
					direct_call = #direct_js_call,
					indirect_call = #indirect_js_call,
					inputs = [#(#js_inputs),*],
					#(output = #output,)*
				),
			}
		})
	}
}

impl OutputType {
	fn member(&self) -> Option<&Member> {
		if let Self::Generate { member, .. } = self {
			member.as_ref()
		} else {
			None
		}
	}
}
