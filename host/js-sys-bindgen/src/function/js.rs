use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Ident, LitStr, Path};

/// A global JavaScript path referenced by an imported operation.
pub(crate) struct FunctionGlobalPath {
	namespace: Option<String>,
	object: Option<String>,
	name: String,
}

impl FunctionGlobalPath {
	fn new(namespace: Option<&str>, object: Option<&str>, name: &str) -> Self {
		Self {
			namespace: namespace.map(str::to_owned),
			object: object.map(str::to_owned),
			name: name.to_owned(),
		}
	}

	fn wire(&self, macro_path: &Path, span: Span) -> TokenStream {
		let namespace = self.namespace.as_ref().map_or_else(
			|| quote_spanned!(span=> ::core::option::Option::None),
			|namespace| {
				let namespace = LitStr::new(namespace, span);
				quote_spanned!(span=> ::core::option::Option::Some(#namespace))
			},
		);
		let object = self.object.as_ref().map_or_else(
			|| quote_spanned!(span=> ::core::option::Option::None),
			|object| {
				let object = LitStr::new(object, span);
				quote_spanned!(span=> ::core::option::Option::Some(#object))
			},
		);
		let name = LitStr::new(&self.name, span);

		quote_spanned! {span=>
			#macro_path::WireGlobalPath::new(#namespace, #object, #name)
		}
	}
}

/// Semantic JavaScript operation stored in an import Wire record.
pub(crate) enum FunctionBinding {
	CallGlobal {
		target: FunctionGlobalPath,
		variadic: bool,
	},
	CallMethod {
		name: String,
		variadic: bool,
	},
	Construct {
		target: FunctionGlobalPath,
		variadic: bool,
	},
	GetGlobal(FunctionGlobalPath),
	GetMember(String),
	SetGlobal(FunctionGlobalPath),
	SetMember(String),
	IndexGet,
	IndexSet,
	IndexDelete,
	CallEmbed {
		module: String,
		name: String,
	},
}

impl FunctionBinding {
	pub(crate) fn wire(&self, macro_path: &Path, span: Span) -> TokenStream {
		match self {
			Self::CallGlobal { target, variadic } => {
				let target = target.wire(macro_path, span);
				quote_spanned! {span=>
					#macro_path::WireImportBinding::CallGlobal {
						target: #target,
						variadic: #variadic,
					}
				}
			}
			Self::CallMethod { name, variadic } => {
				let name = LitStr::new(name, span);
				quote_spanned! {span=>
					#macro_path::WireImportBinding::CallMethod {
						name: #name,
						variadic: #variadic,
					}
				}
			}
			Self::Construct { target, variadic } => {
				let target = target.wire(macro_path, span);
				quote_spanned! {span=>
					#macro_path::WireImportBinding::Construct {
						target: #target,
						variadic: #variadic,
					}
				}
			}
			Self::GetGlobal(target) => {
				let target = target.wire(macro_path, span);
				quote_spanned!(span=> #macro_path::WireImportBinding::GetGlobal(#target))
			}
			Self::GetMember(name) => {
				let name = LitStr::new(name, span);
				quote_spanned!(span=> #macro_path::WireImportBinding::GetMember(#name))
			}
			Self::SetGlobal(target) => {
				let target = target.wire(macro_path, span);
				quote_spanned!(span=> #macro_path::WireImportBinding::SetGlobal(#target))
			}
			Self::SetMember(name) => {
				let name = LitStr::new(name, span);
				quote_spanned!(span=> #macro_path::WireImportBinding::SetMember(#name))
			}
			Self::IndexGet => {
				quote_spanned!(span=> #macro_path::WireImportBinding::IndexGet)
			}
			Self::IndexSet => {
				quote_spanned!(span=> #macro_path::WireImportBinding::IndexSet)
			}
			Self::IndexDelete => {
				quote_spanned!(span=> #macro_path::WireImportBinding::IndexDelete)
			}
			Self::CallEmbed { module, name } => {
				let module = LitStr::new(module, span);
				let name = LitStr::new(name, span);
				quote_spanned! {span=>
					#macro_path::WireImportBinding::CallEmbed(
						#macro_path::JsEmbed::new(#module, #name),
					)
				}
			}
		}
	}
}

/// The final JavaScript binding selected for a foreign function.
pub(super) enum ForeignItem {
	Generate {
		/// Rust type receiving the generated method, if this is not a free
		/// function.
		owner: Option<Path>,
		binding: FunctionBinding,
	},
	Embed(String),
	Import,
}

impl ForeignItem {
	pub(super) fn owner(&self) -> Option<&Path> {
		let Self::Generate { owner, .. } = self else {
			return None;
		};

		owner.as_ref()
	}

	pub(super) fn import_name(&self, namespace: Option<&str>, rust_name: &Ident) -> String {
		let name = if self.owner().is_some() {
			format!("{}.{}", self.owner_name(), rust_name)
		} else {
			rust_name.to_string()
		};

		if let Some(namespace) = namespace {
			format!("{namespace}.{name}")
		} else {
			name
		}
	}

	pub(super) fn call(
		owner: Option<Path>,
		namespace: Option<&str>,
		object: Option<&str>,
		name: &str,
		receiver: bool,
		variadic: bool,
	) -> Self {
		let binding = if receiver {
			FunctionBinding::CallMethod {
				name: name.to_owned(),
				variadic,
			}
		} else {
			FunctionBinding::CallGlobal {
				target: FunctionGlobalPath::new(namespace, object, name),
				variadic,
			}
		};

		Self::Generate { owner, binding }
	}

	pub(super) fn constructor(
		owner: Path,
		namespace: Option<&str>,
		name: &str,
		variadic: bool,
	) -> Self {
		Self::Generate {
			owner: Some(owner),
			binding: FunctionBinding::Construct {
				target: FunctionGlobalPath::new(namespace, None, name),
				variadic,
			},
		}
	}

	pub(super) fn getter(
		owner: Option<Path>,
		namespace: Option<&str>,
		object: Option<&str>,
		name: &str,
		receiver: bool,
	) -> Self {
		let binding = if receiver {
			FunctionBinding::GetMember(name.to_owned())
		} else {
			FunctionBinding::GetGlobal(FunctionGlobalPath::new(namespace, object, name))
		};

		Self::Generate { owner, binding }
	}

	pub(super) fn setter(
		owner: Option<Path>,
		namespace: Option<&str>,
		object: Option<&str>,
		name: &str,
		receiver: bool,
	) -> Self {
		let binding = if receiver {
			FunctionBinding::SetMember(name.to_owned())
		} else {
			FunctionBinding::SetGlobal(FunctionGlobalPath::new(namespace, object, name))
		};

		Self::Generate { owner, binding }
	}

	pub(super) fn indexing_getter(owner: Path) -> Self {
		Self::Generate {
			owner: Some(owner),
			binding: FunctionBinding::IndexGet,
		}
	}

	pub(super) fn indexing_setter(owner: Path) -> Self {
		Self::Generate {
			owner: Some(owner),
			binding: FunctionBinding::IndexSet,
		}
	}

	pub(super) fn indexing_deleter(owner: Path) -> Self {
		Self::Generate {
			owner: Some(owner),
			binding: FunctionBinding::IndexDelete,
		}
	}

	fn owner_name(&self) -> &Ident {
		self.owner()
			.and_then(|owner| owner.segments.last())
			.map(|segment| &segment.ident)
			.expect("static and instance bindings always have an owner")
	}
}
