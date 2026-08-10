use syn::{Attribute, Error, Ident, LitStr, Path, Result};

/// Options read from `#[js_sys(...)]` on a foreign function declaration.
#[derive(Default)]
pub(super) struct FunctionOptions {
	/// Overrides the JavaScript function or constructor name.
	pub(super) js_name: Option<String>,
	/// Makes the foreign function a static member of this Rust type.
	pub(super) static_of: Option<Path>,
	/// Spreads the foreign function's last argument at the JavaScript call
	/// site.
	pub(super) variadic: bool,
	/// Generates a JavaScript `new` expression for the foreign function.
	pub(super) constructor: bool,
	/// Reads this JavaScript property instead of calling a function.
	pub(super) getter: Option<String>,
	/// Writes this JavaScript property instead of calling a function.
	pub(super) setter: Option<String>,
	/// Uses a named JavaScript implementation embedded by the current crate.
	pub(super) embed: Option<String>,
	/// Leaves the JavaScript implementation to the import object.
	pub(super) import: bool,
}

impl FunctionOptions {
	pub(super) fn parse(attrs: &mut Vec<Attribute>, rust_name: &Ident) -> Result<Self> {
		let mut options = Self::default();

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
				} else if meta.path.is_ident("js_embed") {
					let name = meta.value()?.parse::<LitStr>()?.value();

					if options.embed.replace(name).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("js_import") {
					parse_flag(&meta, "js_import", &mut options.import)
				} else {
					Err(meta.error("unsupported attribute"))
				}
			})?;
		}

		options.validate(rust_name)?;
		Ok(options)
	}

	fn validate(&self, rust_name: &Ident) -> Result<()> {
		let source_count = usize::from(self.import) + usize::from(self.embed.is_some());
		let operation_count = usize::from(self.constructor)
			+ usize::from(self.getter.is_some())
			+ usize::from(self.setter.is_some());
		let has_property = self.getter.is_some() || self.setter.is_some();
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
		if operation_count > 1 {
			return Err(Error::new_spanned(
				rust_name,
				"`constructor`, `getter`, and `setter` are mutually exclusive",
			));
		}
		if self.constructor && self.static_of.is_some() {
			return Err(Error::new_spanned(
				rust_name,
				"`constructor` cannot be combined with `static_of`",
			));
		}
		if has_property && self.js_name.is_some() {
			return Err(Error::new_spanned(
				rust_name,
				"`js_name` cannot be combined with `getter` or `setter`; specify the field on the \
				 property operation",
			));
		}
		if has_property && self.variadic {
			return Err(Error::new_spanned(
				rust_name,
				"`variadic` cannot be combined with `getter` or `setter`",
			));
		}

		Ok(())
	}
}

fn parse_flag(meta: &syn::meta::ParseNestedMeta<'_>, name: &str, value: &mut bool) -> Result<()> {
	if !meta.input.is_empty() {
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
