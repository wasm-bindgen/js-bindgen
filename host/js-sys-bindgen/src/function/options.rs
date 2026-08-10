use syn::{Attribute, Error, Ident, LitStr, Path, Result, Type};

/// Options read from `#[js_sys(...)]` on a foreign function declaration.
pub(super) struct FunctionOptions {
	/// Overrides the JavaScript function or constructor name.
	pub(super) js_name: Option<String>,
	/// Makes the foreign function a static member of this Rust type.
	pub(super) static_of: Option<Path>,
	/// Spreads the foreign function's last argument at the JavaScript call
	/// site.
	pub(super) variadic: bool,
	/// The validated JavaScript implementation and operation.
	pub(super) binding: BindingKind,
	/// Uses this concrete type to describe the JavaScript return conversion.
	pub(super) return_abi: Option<Type>,
	/// Allows a Promise returned by the JavaScript implementation to suspend
	/// the current Wasm stack.
	pub(super) suspending: bool,
}

/// The mutually exclusive JavaScript binding selected by the attributes.
pub(super) enum BindingKind {
	Call,
	Constructor,
	Getter(String),
	Setter(String),
	IndexingGetter,
	IndexingSetter,
	IndexingDeleter,
	Embed(String),
	Import,
}

impl BindingKind {
	pub(super) fn is_external(&self) -> bool {
		matches!(self, Self::Embed(_) | Self::Import)
	}
}

#[derive(Default)]
struct RawFunctionOptions {
	js_name: Option<String>,
	static_of: Option<Path>,
	variadic: bool,
	constructor: bool,
	getter: Option<String>,
	setter: Option<String>,
	indexing_getter: bool,
	indexing_setter: bool,
	indexing_deleter: bool,
	embed: Option<String>,
	import: bool,
	return_abi: Option<Type>,
	suspending: bool,
}

impl FunctionOptions {
	pub(super) fn parse(attrs: &mut Vec<Attribute>, rust_name: &Ident) -> Result<Self> {
		let mut options = RawFunctionOptions::default();

		for attr in attrs.extract_if(.., |attr| attr.path().is_ident("js_sys")) {
			attr.parse_nested_meta(|meta| {
				if meta.path.is_ident("js_name") {
					let name = meta.value()?.parse::<LitStr>()?.value();

					if options.js_name.replace(name).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("static_of") {
					let owner = meta.value()?.parse()?;

					if options.static_of.replace(owner).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("variadic") {
					parse_flag(&meta, "variadic", &mut options.variadic)
				} else if meta.path.is_ident("constructor") {
					parse_flag(&meta, "constructor", &mut options.constructor)
				} else if meta.path.is_ident("getter") {
					let name = if meta.input.is_empty() {
						rust_name.to_string()
					} else {
						meta.value()?.parse::<LitStr>()?.value()
					};

					if options.getter.replace(name).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("setter") {
					let name = if meta.input.is_empty() {
						infer_setter_property(rust_name)?
					} else {
						meta.value()?.parse::<LitStr>()?.value()
					};

					if options.setter.replace(name).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("indexing_getter") {
					parse_flag(&meta, "indexing_getter", &mut options.indexing_getter)
				} else if meta.path.is_ident("indexing_setter") {
					parse_flag(&meta, "indexing_setter", &mut options.indexing_setter)
				} else if meta.path.is_ident("indexing_deleter") {
					parse_flag(&meta, "indexing_deleter", &mut options.indexing_deleter)
				} else if meta.path.is_ident("js_embed") {
					let name = meta.value()?.parse::<LitStr>()?.value();

					if options.embed.replace(name).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("js_import") {
					parse_flag(&meta, "js_import", &mut options.import)
				} else if meta.path.is_ident("return_abi") {
					let ty = meta.value()?.parse()?;

					if options.return_abi.replace(ty).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("suspending") {
					parse_flag(&meta, "suspending", &mut options.suspending)
				} else {
					Err(meta.error("unsupported attribute"))
				}
			})?;
		}

		options.validate(rust_name)?;
		Ok(options.finish())
	}
}

impl RawFunctionOptions {
	fn validate(&self, rust_name: &Ident) -> Result<()> {
		let source_count = usize::from(self.import) + usize::from(self.embed.is_some());
		let operation_count = usize::from(self.constructor)
			+ usize::from(self.getter.is_some())
			+ usize::from(self.setter.is_some())
			+ usize::from(self.indexing_getter)
			+ usize::from(self.indexing_setter)
			+ usize::from(self.indexing_deleter);
		let has_property = self.getter.is_some() || self.setter.is_some();
		let has_indexing = self.indexing_getter || self.indexing_setter || self.indexing_deleter;
		let has_binding_options = self.js_name.is_some()
			|| self.static_of.is_some()
			|| operation_count != 0
			|| self.variadic;

		if source_count > 1 || source_count == 1 && has_binding_options {
			return Err(Error::new_spanned(
				rust_name,
				"`js_import` and `js_embed` cannot be combined with JavaScript binding options",
			));
		}
		if self.import && self.suspending {
			return Err(Error::new_spanned(
				rust_name,
				"`suspending` cannot be combined with `js_import`; provide a \
				 `WebAssembly.Suspending` import directly",
			));
		}
		if operation_count > 1 {
			return Err(Error::new_spanned(
				rust_name,
				"JavaScript operations are mutually exclusive",
			));
		}
		if self.constructor && self.static_of.is_some() {
			return Err(Error::new_spanned(
				rust_name,
				"`constructor` cannot be combined with `static_of`",
			));
		}
		if (has_property || has_indexing) && self.js_name.is_some() {
			return Err(Error::new_spanned(
				rust_name,
				"`js_name` cannot be combined with a property operation",
			));
		}
		if (has_property || has_indexing) && self.variadic {
			return Err(Error::new_spanned(
				rust_name,
				"`variadic` cannot be combined with a property operation",
			));
		}

		Ok(())
	}

	fn finish(self) -> FunctionOptions {
		let Self {
			js_name,
			static_of,
			variadic,
			constructor,
			getter,
			setter,
			indexing_getter,
			indexing_setter,
			indexing_deleter,
			embed,
			import,
			return_abi,
			suspending,
		} = self;
		let binding = if import {
			BindingKind::Import
		} else if let Some(embed) = embed {
			BindingKind::Embed(embed)
		} else if constructor {
			BindingKind::Constructor
		} else if let Some(getter) = getter {
			BindingKind::Getter(getter)
		} else if let Some(setter) = setter {
			BindingKind::Setter(setter)
		} else if indexing_getter {
			BindingKind::IndexingGetter
		} else if indexing_setter {
			BindingKind::IndexingSetter
		} else if indexing_deleter {
			BindingKind::IndexingDeleter
		} else {
			BindingKind::Call
		};

		FunctionOptions {
			js_name,
			static_of,
			variadic,
			binding,
			return_abi,
			suspending,
		}
	}
}

fn parse_flag(meta: &syn::meta::ParseNestedMeta<'_>, name: &str, value: &mut bool) -> Result<()> {
	if meta.input.peek(syn::Token![=]) || meta.input.peek(syn::token::Paren) {
		return Err(meta.error(format!("`{name}` supports no values")));
	}
	if *value {
		return Err(meta.error("duplicate attribute"));
	}

	*value = true;
	Ok(())
}

fn infer_setter_property(ident: &Ident) -> Result<String> {
	let name = ident.to_string();
	let Some(property) = name
		.strip_prefix("set_")
		.filter(|property| !property.is_empty())
	else {
		return Err(Error::new_spanned(
			ident,
			"`setter` cannot infer a field name; use `setter = \"field\"`",
		));
	};

	Ok(property.to_owned())
}
