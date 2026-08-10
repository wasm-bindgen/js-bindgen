use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::DerefMut;
use std::string::ToString;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
	Attribute, Error, FnArg, ForeignItemFn, GenericParam, Generics, Ident, LitStr, Pat, PatIdent,
	PatType, Path, Receiver, Result, ReturnType, Signature, Token, TraitBoundModifier, Type,
	TypeParamBound, TypePath, TypeReference, WherePredicate, parse_quote,
};

use crate::hygiene::Hygiene;

mod js;
mod options;

use js::{ForeignItem, FunctionBinding};
use options::{BindingKind, ConstructorOwner, FunctionOptions};

/// Backend-independent description of one JavaScript import.
pub(crate) struct FunctionImport {
	pub(crate) cfg_attrs: Vec<Attribute>,
	pub(crate) module: LitStr,
	pub(crate) name: LitStr,
	pub(crate) inputs: Vec<FunctionImportInput>,
	pub(crate) output_type: Option<Type>,
	pub(crate) binding: Option<FunctionBinding>,
	pub(crate) suspending: bool,
	pub(crate) macro_path: Path,
}

pub(crate) struct FunctionImportInput {
	pub(crate) ty: Type,
}

struct FunctionPlan {
	inputs: Vec<InputArg>,
	output_ty: Option<Type>,
	output_abi_override: Option<Type>,
	impl_generic_params: TokenStream,
	binding: ForeignItem,
	suspending: bool,
}

struct InputArg {
	abi_type: Type,
	rust_name: Ident,
	slot_names: [Ident; 4],
	uses_abi_override: bool,
}

impl InputArg {
	fn new(index: usize, abi_type: Type, rust_name: Ident, uses_abi_override: bool) -> Self {
		let span = Span::mixed_site();
		let base = format!("arg{index}");
		let slot_name = |slot| Ident::new(&format!("{base}_{slot}"), span);

		Self {
			abi_type,
			rust_name,
			slot_names: [slot_name(0), slot_name(1), slot_name(2), slot_name(3)],
			uses_abi_override,
		}
	}
}

pub(crate) fn expand(
	hygiene: &mut Hygiene<'_>,
	namespace: Option<&str>,
	crate_: &str,
	js_names: &HashMap<String, String>,
	item: ForeignItemFn,
) -> Result<(TokenStream, FunctionImport)> {
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
		mut attrs,
		vis,
		mut sig,
		..
	} = item;
	let cfg_attrs: Vec<_> = attrs
		.iter()
		.filter(|attr| {
			let path = attr.path();
			path.is_ident("cfg") || path.is_ident("cfg_attr")
		})
		.cloned()
		.collect();

	let options = FunctionOptions::parse(&mut attrs, &sig.ident)?;

	let plan = FunctionPlan::parse(
		hygiene, &mut sig, options, namespace, &cfg_attrs, js_names, span,
	)?;
	let import_name = plan.binding.import_name(namespace, &sig.ident);
	let link_name = format!("{crate_}.{import_name}");
	let macro_path = hygiene.r#macro(&cfg_attrs, span);
	let inputs = &plan.inputs;
	let output_ty = &plan.output_ty;
	let output_abi_override = &plan.output_abi_override;
	let foreign_ident = Ident::new("__import_", Span::mixed_site());
	let split_inputs = inputs.iter().map(|input| {
		let InputArg {
			abi_type,
			rust_name,
			slot_names: [slot1, slot2, slot3, slot4],
			uses_abi_override,
			..
		} = input;
		let split_input = if *uses_abi_override {
			quote_spanned!(span=> unsafe {
				#macro_path::split_input_as::<#abi_type>(#rust_name)
			})
		} else {
			quote_spanned!(span=> #macro_path::split_input::<#abi_type>(#rust_name))
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
				quote_spanned!(span=> #macro_path::InputSlot1<#ty>),
				quote_spanned!(span=> #macro_path::InputSlot2<#ty>),
				quote_spanned!(span=> #macro_path::InputSlot3<#ty>),
				quote_spanned!(span=> #macro_path::InputSlot4<#ty>),
			]
		})
		.collect();
	let output_abi_ty = output_abi_override.as_ref().or(output_ty.as_ref());
	let foreign_output = output_abi_ty.map_or_else(
		TokenStream::new,
		|ty| quote_spanned!(span=> -> #macro_path::OutputRet<#ty>),
	);

	let foreign_call = quote_spanned! {span=> {
		#(#split_inputs)*
		unsafe { #foreign_ident(#(#foreign_input_names),*) }
	}};
	let foreign_call = if let Some(output_abi_ty) = output_abi_override.as_ref() {
		let output_ty = output_ty.as_ref().expect("validated during parsing");

		quote_spanned!(span=> {
			let value = #foreign_call;
			unsafe { #macro_path::join_output_as::<#output_ty, #output_abi_ty>(value) }
		})
	} else if output_ty.is_some() {
		quote_spanned!(span=> #macro_path::join_output(#foreign_call))
	} else {
		foreign_call
	};

	let item_fn = quote_spanned! {span=>
		#(#attrs)*
		#vis #sig {
			unsafe extern "C" {
				#[link_name = #link_name]
				fn #foreign_ident(#(#foreign_input_names: #foreign_input_tys),*) #foreign_output;
			}

			#foreign_call
		}
	};

	let item = if let Some(owner) = plan.binding.owner() {
		let impl_generic_params = &plan.impl_generic_params;
		quote_spanned! {span=>
			impl #impl_generic_params #owner {
				#item_fn
			}
		}
	} else {
		item_fn
	};
	let import = plan.into_import_descriptor(macro_path, crate_, &import_name, cfg_attrs, span);

	Ok((item, import))
}

impl FunctionPlan {
	fn parse(
		hygiene: &mut Hygiene<'_>,
		sig: &mut Signature,
		options: FunctionOptions,
		namespace: Option<&str>,
		cfg_attrs: &[Attribute],
		js_names: &HashMap<String, String>,
		span: Span,
	) -> Result<Self> {
		let suspending = options.suspending;
		let external_implementation = options.binding.is_external();
		let output_abi_override = options.return_abi.clone();
		let (inputs, self_ty) =
			Self::parse_inputs(hygiene, sig, cfg_attrs, span, external_implementation)?;
		let binding = Self::resolve_binding(options, sig, self_ty, namespace, js_names, span)?;
		let output_ty = match &sig.output {
			ReturnType::Default => None,
			ReturnType::Type(_, ty) => Some(*ty.clone()),
		};
		if output_abi_override.is_some() && output_ty.is_none() {
			return Err(Error::new(span, "`return_abi` requires a return value"));
		}

		let impl_generic_params = Self::impl_generic_params(&binding, &mut sig.generics);

		Ok(Self {
			inputs,
			output_ty,
			output_abi_override,
			impl_generic_params,
			binding,
			suspending,
		})
	}

	fn parse_inputs(
		hygiene: &mut Hygiene<'_>,
		sig: &mut Signature,
		cfg_attrs: &[Attribute],
		span: Span,
		external_implementation: bool,
	) -> Result<(Vec<InputArg>, Option<Path>)> {
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
					let mut abi_override = None;

					// `#[js_sys(type = T)]` applies to this parameter only. It
					// overrides the `ABI` conversion while preserving the public
					// Rust signature.
					for attr in attrs.extract_if(.., |attr| attr.path().is_ident("js_sys")) {
						attr.parse_nested_meta(|meta| {
							if meta.path.is_ident("type") {
								meta.input.parse::<Token![=]>()?;

								if abi_override.replace(meta.input.parse::<Type>()?).is_some() {
									Err(meta.error("duplicate attribute"))
								} else {
									Ok(())
								}
							} else {
								Err(meta.error("unsupported attribute"))
							}
						})?;
					}

					let uses_abi_override = abi_override.is_some();
					let abi_type = abi_override.unwrap_or_else(|| *ty.clone());

					Ok(InputArg::new(
						index,
						abi_type,
						ident.clone(),
						uses_abi_override,
					))
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
					if external_implementation {
						return Err(Error::new_spanned(
							path,
							"`self` is not supported with `js_import` and `js_embed`",
						));
					}

					self_ty = Some(path.clone());
					let js_value = hygiene.js_value(cfg_attrs, span);
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

		Ok((inputs, self_ty))
	}

	fn resolve_binding(
		options: FunctionOptions,
		sig: &Signature,
		self_ty: Option<Path>,
		namespace: Option<&str>,
		js_names: &HashMap<String, String>,
		span: Span,
	) -> Result<ForeignItem> {
		let FunctionOptions {
			js_name,
			static_of,
			variadic,
			binding,
			return_abi: _,
			suspending: _,
		} = options;

		let binding = match binding {
			BindingKind::Import => return Ok(ForeignItem::Import),
			BindingKind::Embed(embed) => return Ok(ForeignItem::Embed(embed)),
			binding => binding,
		};

		let argument_count = sig.inputs.len() - usize::from(self_ty.is_some());

		if matches!(&binding, BindingKind::Constructor(_)) && self_ty.is_some() {
			return Err(Error::new(
				span,
				"`constructor` cannot be used with a `self` parameter",
			));
		}
		if self_ty.is_some() && static_of.is_some() {
			return Err(Error::new(
				span,
				"`static_of` cannot be used with a `self` parameter",
			));
		}
		if variadic && argument_count == 0 {
			return Err(Error::new(
				span,
				"`variadic` requires at least one argument",
			));
		}
		if matches!(
			&binding,
			BindingKind::IndexingGetter
				| BindingKind::IndexingSetter
				| BindingKind::IndexingDeleter
		) && self_ty.is_none()
		{
			return Err(Error::new(
				span,
				"indexing operations require a `self` parameter",
			));
		}

		match binding {
			BindingKind::Constructor(owner) => {
				// `constructor` applies to this foreign function. Its return type
				// selects the Rust `impl` owner unless one was supplied explicitly,
				// and JavaScript invokes it with `new`.
				let output = Self::constructor_output(&sig.output)?;
				let owner = match owner {
					ConstructorOwner::Infer => Self::constructor_type(output)?,
					ConstructorOwner::Explicit(owner) => owner,
				};
				let name = js_name.unwrap_or_else(|| Self::type_js_name(&owner, js_names));

				Ok(ForeignItem::constructor(owner, namespace, &name, variadic))
			}
			BindingKind::IndexingGetter => {
				if argument_count != 1 || !matches!(&sig.output, ReturnType::Type(..)) {
					return Err(Error::new(
						span,
						"`indexing_getter` requires one argument and a return value",
					));
				}

				Ok(ForeignItem::indexing_getter(
					self_ty.expect("validated above"),
				))
			}
			BindingKind::IndexingSetter => {
				if argument_count != 2 {
					return Err(Error::new(span, "`indexing_setter` requires two arguments"));
				}

				Ok(ForeignItem::indexing_setter(
					self_ty.expect("validated above"),
				))
			}
			BindingKind::IndexingDeleter => {
				if argument_count != 1 {
					return Err(Error::new(span, "`indexing_deleter` requires one argument"));
				}

				Ok(ForeignItem::indexing_deleter(
					self_ty.expect("validated above"),
				))
			}
			BindingKind::Getter(name) => {
				if argument_count != 0 || !matches!(&sig.output, ReturnType::Type(..)) {
					return Err(Error::new(
						span,
						"`getter` requires no arguments and a return value",
					));
				}
				let (owner, object, receiver) = Self::member_target(static_of, self_ty, js_names);

				Ok(ForeignItem::getter(
					owner,
					namespace,
					object.as_deref(),
					&name,
					receiver,
				))
			}
			BindingKind::Setter(name) => {
				if argument_count != 1 || !matches!(&sig.output, ReturnType::Default) {
					return Err(Error::new(
						span,
						"`setter` requires one argument and no return value",
					));
				}
				let (owner, object, receiver) = Self::member_target(static_of, self_ty, js_names);

				Ok(ForeignItem::setter(
					owner,
					namespace,
					object.as_deref(),
					&name,
					receiver,
				))
			}
			BindingKind::Call => {
				let name = js_name.unwrap_or_else(|| sig.ident.to_string());
				let (owner, object, receiver) = Self::member_target(static_of, self_ty, js_names);

				Ok(ForeignItem::call(
					owner,
					namespace,
					object.as_deref(),
					&name,
					receiver,
					variadic,
				))
			}
			BindingKind::Embed(_) | BindingKind::Import => {
				unreachable!("external bindings returned above")
			}
		}
	}

	fn member_target(
		static_of: Option<Path>,
		self_ty: Option<Path>,
		js_names: &HashMap<String, String>,
	) -> (Option<Path>, Option<String>, bool) {
		// `static_of` attaches the declaration to a type, an explicit `self`
		// parameter makes it an instance member, and neither means a global.
		if let Some(owner) = static_of {
			let type_name = Self::type_js_name(&owner, js_names);

			(Some(owner), Some(type_name), false)
		} else if let Some(owner) = self_ty {
			(Some(owner), None, true)
		} else {
			(None, None, false)
		}
	}

	fn constructor_output(output: &ReturnType) -> Result<&Type> {
		let ReturnType::Type(_, output) = output else {
			return Err(Error::new_spanned(
				output,
				"`constructor` requires a return type",
			));
		};

		Ok(output)
	}

	fn constructor_type(output: &Type) -> Result<Path> {
		let Type::Path(TypePath { qself: None, path }) = output else {
			return Err(Error::new_spanned(
				output,
				"`constructor` requires a path return type or an explicit owner",
			));
		};

		Ok(path.clone())
	}

	fn type_js_name(owner: &Path, js_names: &HashMap<String, String>) -> String {
		let rust_name = owner
			.segments
			.last()
			.expect("a type path always contains a segment")
			.ident
			.to_string();

		if owner.leading_colon.is_none() && owner.segments.len() == 1 {
			js_names.get(&rust_name).cloned().unwrap_or(rust_name)
		} else {
			rust_name
		}
	}

	// Extract type generics from signature that are part of `impl <type>`.
	fn impl_generic_params(binding: &ForeignItem, generics: &mut Generics) -> TokenStream {
		if let Some(owner) = binding.owner() {
			let params: Vec<_> = mem::take(&mut generics.params).into_iter().collect();
			let mut identifiers = HashMap::new();
			let mut lifetimes = HashMap::new();

			for (index, param) in params.iter().enumerate() {
				match param {
					GenericParam::Lifetime(param) => {
						lifetimes.insert(param.lifetime.ident.to_string(), index);
					}
					GenericParam::Type(param) => {
						identifiers.insert(param.ident.to_string(), index);
					}
					GenericParam::Const(param) => {
						identifiers.insert(param.ident.to_string(), index);
					}
				}
			}

			let mut impl_indices = HashSet::new();
			for segment in &owner.segments {
				Self::collect_generic_references(
					segment.arguments.to_token_stream(),
					&identifiers,
					&lifetimes,
					&mut impl_indices,
				);
			}

			let mut impl_generic_params = Vec::new();
			let mut fn_generic_params = Vec::new();
			let mut method_predicates = Vec::new();
			for (index, param) in params.into_iter().enumerate() {
				if impl_indices.contains(&index) {
					let (param, predicates) = Self::prepare_impl_generic(param);
					impl_generic_params.push(param);
					method_predicates.extend(predicates);
				} else {
					fn_generic_params.push(param);
				}
			}
			generics.params = fn_generic_params.into_iter().collect();
			if !method_predicates.is_empty() {
				generics
					.make_where_clause()
					.predicates
					.extend(method_predicates);
			}

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

	fn prepare_impl_generic(param: GenericParam) -> (GenericParam, Vec<WherePredicate>) {
		match param {
			GenericParam::Lifetime(mut param) => {
				let bounds = mem::take(&mut param.bounds);
				param.colon_token = None;
				let predicates = if bounds.is_empty() {
					Vec::new()
				} else {
					let lifetime = &param.lifetime;
					vec![parse_quote!(#lifetime: #bounds)]
				};

				(GenericParam::Lifetime(param), predicates)
			}
			GenericParam::Type(mut param) => {
				let bounds = mem::take(&mut param.bounds);
				let mut impl_bounds =
					syn::punctuated::Punctuated::<TypeParamBound, Token![+]>::new();
				let mut method_bounds =
					syn::punctuated::Punctuated::<TypeParamBound, Token![+]>::new();

				for bound in bounds {
					if matches!(
						&bound,
						TypeParamBound::Trait(bound)
							if matches!(bound.modifier, TraitBoundModifier::Maybe(_))
								&& bound.path.is_ident("Sized")
					) {
						impl_bounds.push(bound);
					} else {
						method_bounds.push(bound);
					}
				}

				param.colon_token =
					(!impl_bounds.is_empty()).then_some(Token![:](param.ident.span()));
				param.bounds = impl_bounds;
				param.eq_token = None;
				param.default = None;
				let predicates = if method_bounds.is_empty() {
					Vec::new()
				} else {
					let ident = &param.ident;
					vec![parse_quote!(#ident: #method_bounds)]
				};

				(GenericParam::Type(param), predicates)
			}
			GenericParam::Const(mut param) => {
				param.eq_token = None;
				param.default = None;

				(GenericParam::Const(param), Vec::new())
			}
		}
	}

	fn collect_generic_references(
		tokens: TokenStream,
		identifiers: &HashMap<String, usize>,
		lifetimes: &HashMap<String, usize>,
		references: &mut HashSet<usize>,
	) {
		let tokens: Vec<_> = tokens.into_iter().collect();
		let mut index = 0;

		while index < tokens.len() {
			match &tokens[index] {
				TokenTree::Group(group) => Self::collect_generic_references(
					group.stream(),
					identifiers,
					lifetimes,
					references,
				),
				TokenTree::Punct(punct)
					if punct.as_char() == '\''
						&& matches!(tokens.get(index + 1), Some(TokenTree::Ident(_))) =>
				{
					let Some(TokenTree::Ident(ident)) = tokens.get(index + 1) else {
						unreachable!("validated by the match guard");
					};
					if let Some(parameter) = lifetimes.get(&ident.to_string()) {
						references.insert(*parameter);
					}
					index += 1;
				}
				TokenTree::Ident(ident) => {
					let is_qualified_path_segment = index >= 2
						&& matches!(&tokens[index - 1], TokenTree::Punct(punct) if punct.as_char() == ':')
						&& matches!(&tokens[index - 2], TokenTree::Punct(punct) if punct.as_char() == ':');
					let is_member = index >= 1
						&& matches!(&tokens[index - 1], TokenTree::Punct(punct) if punct.as_char() == '.');
					if !is_qualified_path_segment
						&& !is_member && let Some(parameter) = identifiers.get(&ident.to_string())
					{
						references.insert(*parameter);
					}
				}
				TokenTree::Literal(_) | TokenTree::Punct(_) => (),
			}
			index += 1;
		}
	}

	fn into_import_descriptor(
		self,
		macro_path: Path,
		crate_: &str,
		import_name: &str,
		cfg_attrs: Vec<Attribute>,
		span: Span,
	) -> FunctionImport {
		let Self {
			inputs,
			output_ty,
			output_abi_override,
			binding,
			suspending,
			..
		} = self;
		let output_type = output_abi_override.or(output_ty);

		let binding = match binding {
			ForeignItem::Generate { binding, .. } => Some(binding),
			ForeignItem::Embed(name) => Some(FunctionBinding::CallEmbed {
				module: crate_.to_owned(),
				name,
			}),
			ForeignItem::Import => None,
		};
		FunctionImport {
			cfg_attrs,
			module: LitStr::new(crate_, span),
			name: LitStr::new(import_name, span),
			inputs: inputs
				.into_iter()
				.map(|input| FunctionImportInput { ty: input.abi_type })
				.collect(),
			output_type,
			binding,
			suspending,
			macro_path,
		}
	}
}
