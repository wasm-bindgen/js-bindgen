use syn::{Ident, Path};

/// The final JavaScript binding selected for a foreign function.
pub(super) enum ForeignItem {
	Generate {
		/// Rust type receiving the generated method, if this is not a free
		/// function.
		owner: Option<Path>,
		/// Function reference usable without a wrapper.
		direct: Option<String>,
		/// Call expression used when argument or result conversion requires a
		/// wrapper.
		call: String,
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
		path: &str,
		receiver: bool,
		variadic: bool,
		namespace: Option<&str>,
		inputs: &[String],
	) -> Self {
		let arguments = Self::arguments(inputs, receiver, variadic);
		let call = format!("{path}({arguments})");

		// Only a bare global function can be passed directly. Calls through a
		// `namespace`, instance, or static member need a wrapper to preserve their
		// receiver; `variadic` calls need one to emit the spread expression.
		let direct = (namespace.is_none() && owner.is_none() && !variadic).then(|| path.to_owned());

		Self::Generate {
			owner,
			direct,
			call,
		}
	}

	pub(super) fn constructor(owner: Path, path: &str, variadic: bool, inputs: &[String]) -> Self {
		let arguments = Self::arguments(inputs, false, variadic);
		let call = format!("new {path}({arguments})");

		Self::Generate {
			owner: Some(owner),
			direct: None,
			call,
		}
	}

	pub(super) fn getter(owner: Option<Path>, path: String) -> Self {
		Self::expression(owner, path)
	}

	pub(super) fn setter(
		owner: Option<Path>,
		path: &str,
		receiver: bool,
		inputs: &[String],
	) -> Self {
		let arguments = Self::arguments(inputs, receiver, false);
		let call = format!("{path} = {arguments}");

		Self::expression(owner, call)
	}

	pub(super) fn indexing_getter(owner: Path, inputs: &[String]) -> Self {
		let call = format!("{}[{}]", inputs[0], inputs[1]);

		Self::expression(Some(owner), call)
	}

	pub(super) fn indexing_setter(owner: Path, inputs: &[String]) -> Self {
		let call = format!("{}[{}] = {}", inputs[0], inputs[1], inputs[2]);

		Self::expression(Some(owner), call)
	}

	pub(super) fn indexing_deleter(owner: Path, inputs: &[String]) -> Self {
		let call = format!("delete {}[{}]", inputs[0], inputs[1]);

		Self::expression(Some(owner), call)
	}

	pub(super) fn global_path(namespace: Option<&str>, name: &str) -> String {
		if let Some(namespace) = namespace {
			format!("globalThis.{namespace}.{name}")
		} else {
			format!("globalThis.{name}")
		}
	}

	fn owner_name(&self) -> &Ident {
		self.owner()
			.and_then(|owner| owner.segments.last())
			.map(|segment| &segment.ident)
			.expect("static and instance bindings always have an owner")
	}

	fn expression(owner: Option<Path>, expression: String) -> Self {
		Self::Generate {
			owner,
			direct: None,
			call: expression,
		}
	}

	fn arguments(inputs: &[String], receiver: bool, variadic: bool) -> String {
		let inputs = if receiver { &inputs[1..] } else { inputs };

		if variadic {
			let (last, inputs) = inputs
				.split_last()
				.expect("variadic bindings always have an argument");

			if inputs.is_empty() {
				format!("...{last}")
			} else {
				format!("{}, ...{last}", inputs.join(", "))
			}
		} else {
			inputs.join(", ")
		}
	}
}
