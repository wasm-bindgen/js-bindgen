use std::collections::HashMap;
use std::mem;
use std::ops::DerefMut;
use std::string::ToString;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
	Attribute, Error, FnArg, ForeignItemFn, GenericArgument, GenericParam, Generics, Ident, LitStr,
	Pat, PatIdent, PatType, Path, PathArguments, Receiver, Result, ReturnType, Signature, Token,
	Type, TypePath, TypeReference, parse_quote,
};

use crate::hygiene::Hygiene;

mod js;
mod options;

use js::ForeignItem;
use options::{BindingKind, FunctionOptions};

/// Backend-independent description of one JavaScript import.
pub(crate) struct FunctionImport {
	pub(crate) cfg_attrs: Vec<Attribute>,
	pub(crate) module: LitStr,
	pub(crate) name: LitStr,
	pub(crate) input_names: Vec<LitStr>,
	pub(crate) input_types: Vec<Type>,
	pub(crate) output_type: Option<Type>,
	pub(crate) binding: Option<FunctionBinding>,
	pub(crate) suspending: bool,
	pub(crate) kind: FunctionImportKind,
	pub(crate) macro_path: Path,
}

#[derive(Clone, Copy)]
pub(crate) enum FunctionImportKind {
	Normal,
	ClosureFactory,
}

/// JavaScript binding data shared by the flat and direct Wire emitters.
pub(crate) struct FunctionBinding {
	pub(crate) direct: Option<String>,
	pub(crate) call: String,
	pub(crate) required_embeds: Vec<TokenStream>,
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
	descriptor_name: syn::LitStr,
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
			descriptor_name: syn::LitStr::new(&base, span),
			slot_names: [slot_name(0), slot_name(1), slot_name(2), slot_name(3)],
			uses_abi_override,
		}
	}
}

fn join_input_slots(inputs: &[InputArg]) -> String {
	let mut inputs = inputs.iter();
	let Some(first) = inputs.next() else {
		return String::new();
	};
	let mut output = first.slot_names[0].to_string();

	for input in inputs {
		output.push_str(", ");
		output.push_str(&input.slot_names[0].to_string());
	}

	output
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
	let import = plan.import_descriptor(&macro_path, crate_, &import_name, &cfg_attrs, span);
	let FunctionPlan {
		inputs,
		output_ty,
		output_abi_override,
		impl_generic_params,
		binding,
		..
	} = plan;
	let ident = &sig.ident;
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
		unsafe { #ident(#(#foreign_input_names),*) }
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
		quote_spanned!(span=> #foreign_call;)
	};

	let item_fn = quote_spanned! {span=>
		#(#attrs)*
		#vis #sig {
			unsafe extern "C" {
				#[link_name = #link_name]
				fn #ident(#(#foreign_input_names: #foreign_input_tys),*) #foreign_output;
			}

			#foreign_call
		}
	};

	let item = if let Some(owner) = binding.owner() {
		quote_spanned! {span=>
			impl #impl_generic_params #owner {
				#item_fn
			}
		}
	} else {
		item_fn
	};

	Ok((item, import))
}

pub(crate) fn expand_closure_factory(
	hygiene: &mut Hygiene<'_>,
	crate_: &str,
	item: ForeignItemFn,
) -> Result<(TokenStream, FunctionImport)> {
	let (function, mut import) = expand(hygiene, None, crate_, &HashMap::new(), item)?;
	import.kind = FunctionImportKind::ClosureFactory;
	Ok((function, import))
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
		let binding =
			Self::resolve_binding(options, sig, self_ty, namespace, js_names, &inputs, span)?;
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
		inputs: &[InputArg],
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

		let js_inputs: Vec<_> = inputs
			.iter()
			.map(|input| input.slot_names[0].to_string())
			.collect();
		let argument_count = sig.inputs.len() - usize::from(self_ty.is_some());

		if matches!(&binding, BindingKind::Constructor) && self_ty.is_some() {
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
			BindingKind::Constructor => {
				// `constructor` applies to this foreign function. Its return type
				// selects the Rust `impl` owner and JavaScript invokes it with `new`.
				let owner = Self::constructor_type(&sig.output)?;
				let name = js_name.unwrap_or_else(|| Self::type_js_name(&owner, js_names));
				let path = ForeignItem::global_path(namespace, &name);

				Ok(ForeignItem::constructor(owner, &path, variadic, &js_inputs))
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
					&js_inputs,
				))
			}
			BindingKind::IndexingSetter => {
				if argument_count != 2 {
					return Err(Error::new(span, "`indexing_setter` requires two arguments"));
				}

				Ok(ForeignItem::indexing_setter(
					self_ty.expect("validated above"),
					&js_inputs,
				))
			}
			BindingKind::IndexingDeleter => {
				if argument_count != 1 {
					return Err(Error::new(span, "`indexing_deleter` requires one argument"));
				}

				Ok(ForeignItem::indexing_deleter(
					self_ty.expect("validated above"),
					&js_inputs,
				))
			}
			BindingKind::Getter(name) => {
				if argument_count != 0 || !matches!(&sig.output, ReturnType::Type(..)) {
					return Err(Error::new(
						span,
						"`getter` requires no arguments and a return value",
					));
				}
				let (owner, path, _) =
					Self::member_path(static_of, self_ty, &name, namespace, js_names, &js_inputs);

				Ok(ForeignItem::getter(owner, path))
			}
			BindingKind::Setter(name) => {
				if argument_count != 1 || !matches!(&sig.output, ReturnType::Default) {
					return Err(Error::new(
						span,
						"`setter` requires one argument and no return value",
					));
				}
				let (owner, path, receiver) =
					Self::member_path(static_of, self_ty, &name, namespace, js_names, &js_inputs);

				Ok(ForeignItem::setter(owner, &path, receiver, &js_inputs))
			}
			BindingKind::Call => {
				let name = js_name.unwrap_or_else(|| sig.ident.to_string());
				let (owner, path, receiver) =
					Self::member_path(static_of, self_ty, &name, namespace, js_names, &js_inputs);

				Ok(ForeignItem::call(
					owner, &path, receiver, variadic, namespace, &js_inputs,
				))
			}
			BindingKind::Embed(_) | BindingKind::Import => {
				unreachable!("external bindings returned above")
			}
		}
	}

	fn member_path(
		static_of: Option<Path>,
		self_ty: Option<Path>,
		name: &str,
		namespace: Option<&str>,
		js_names: &HashMap<String, String>,
		inputs: &[String],
	) -> (Option<Path>, String, bool) {
		// `static_of` attaches the declaration to a type, an explicit `self`
		// parameter makes it an instance member, and neither means a global.
		if let Some(owner) = static_of {
			let type_name = Self::type_js_name(&owner, js_names);
			let path = ForeignItem::global_path(namespace, &format!("{type_name}.{name}"));

			(Some(owner), path, false)
		} else if let Some(owner) = self_ty {
			(Some(owner), format!("{}.{name}", inputs[0]), true)
		} else {
			(None, ForeignItem::global_path(namespace, name), false)
		}
	}

	fn constructor_type(output: &ReturnType) -> Result<Path> {
		let ReturnType::Type(_, output) = output else {
			return Err(Error::new_spanned(
				output,
				"`constructor` requires a return type",
			));
		};

		Self::constructor_type_from(output)
	}

	fn type_js_name(owner: &Path, js_names: &HashMap<String, String>) -> String {
		let rust_name = owner
			.segments
			.last()
			.expect("a type path always contains a segment")
			.ident
			.to_string();

		js_names.get(&rust_name).cloned().unwrap_or(rust_name)
	}

	fn constructor_type_from(output: &Type) -> Result<Path> {
		let Type::Path(TypePath { qself: None, path }) = output else {
			return Err(Error::new_spanned(
				output,
				"`constructor` requires a path return type",
			));
		};
		let segment = path
			.segments
			.last()
			.expect("a type path always contains a segment");

		if segment.ident == "Result"
			&& let PathArguments::AngleBracketed(arguments) = &segment.arguments
			&& let Some(GenericArgument::Type(output)) = arguments.args.first()
		{
			return Self::constructor_type_from(output);
		}

		Ok(path.clone())
	}

	// Extract type generics from signature that are part of `impl <type>`.
	fn impl_generic_params(binding: &ForeignItem, generics: &mut Generics) -> TokenStream {
		if let Some(owner) = binding.owner() {
			let mut fn_generic_params: Vec<_> =
				mem::take(&mut generics.params).into_iter().collect();

			let impl_generic_params: Vec<_> = fn_generic_params
				.extract_if(.., |param| {
					for path in &owner.segments {
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

	fn import_descriptor(
		&self,
		macro_path: &Path,
		crate_: &str,
		import_name: &str,
		cfg_attrs: &[Attribute],
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
		let output_abi_ty = output_abi_override.as_ref().or(output_ty.as_ref());
		let input_tys: Vec<_> = inputs.iter().map(|input| &input.abi_type).collect();

		let mut required_embeds = Vec::new();

		if let ForeignItem::Embed(name) = binding {
			required_embeds.push(quote_spanned!(span=>
				#macro_path::JsEmbed::new(#crate_, #name)
			));
		}

		let binding = match binding {
			ForeignItem::Generate {
				direct_wrapper,
				direct_call,
				indirect_call,
				..
			} => Some(FunctionBinding {
				direct: (!direct_wrapper).then(|| direct_call.clone()),
				call: indirect_call.clone(),
				required_embeds,
			}),
			ForeignItem::Embed(name) => {
				let path = format!("this.#jsEmbed.{crate_}['{name}']");
				let arguments = join_input_slots(inputs);
				let indirect_call = format!("{path}({arguments})");

				Some(FunctionBinding {
					direct: Some(path),
					call: indirect_call,
					required_embeds,
				})
			}
			ForeignItem::Import => None,
		};
		FunctionImport {
			cfg_attrs: cfg_attrs.to_vec(),
			module: LitStr::new(crate_, span),
			name: LitStr::new(import_name, span),
			input_names: inputs
				.iter()
				.map(|input| input.descriptor_name.clone())
				.collect(),
			input_types: input_tys.into_iter().cloned().collect(),
			output_type: output_abi_ty.cloned(),
			binding,
			suspending: *suspending,
			kind: FunctionImportKind::Normal,
			macro_path: macro_path.clone(),
		}
	}
}
