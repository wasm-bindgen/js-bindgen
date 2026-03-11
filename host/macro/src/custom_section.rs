use std::borrow::Cow;
use std::iter;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
#[cfg(test)]
use proc_macro2 as proc_macro;

use crate::path;

pub struct CustomSection {
	named_values: Vec<NamedValue>,
	tuple_values: Vec<TupleValue>,
	values: Vec<Value>,
}

struct NamedValue {
	name: String,
	kind: Vec<NamedValueKind>,
}

enum NamedValueKind {
	Const {
		cfg: Option<[TokenTree; 2]>,
		expr: Vec<TokenTree>,
	},
	Interpolate {
		cfg: Option<[TokenTree; 2]>,
		expr: Vec<TokenTree>,
	},
}

struct TupleValue {
	cfg: Option<[TokenTree; 2]>,
	expr: Vec<TokenTree>,
}

struct Value {
	cfg: Option<[TokenTree; 2]>,
	kind: ValueKind,
}

#[derive(Clone)]
enum ValueKind {
	Bytes(Vec<u8>),
	Const(Vec<TokenTree>),
	Interpolate(Vec<TokenTree>),
	Named(String),
	TupleCount,
	TupleA(usize),
	TupleB(usize),
}

impl CustomSection {
	pub fn new() -> Self {
		Self {
			named_values: Vec::new(),
			tuple_values: Vec::new(),
			values: Vec::new(),
		}
	}

	pub fn bytes_value(&mut self, cfg: Option<[TokenTree; 2]>, bytes: Vec<u8>) {
		self.values.push(Value {
			cfg,
			kind: ValueKind::Bytes(bytes),
		});
	}

	pub fn const_value(&mut self, cfg: Option<[TokenTree; 2]>, expr: Vec<TokenTree>) {
		self.values.push(Value {
			cfg,
			kind: ValueKind::Const(expr),
		});
	}

	pub fn interpolate_value(&mut self, cfg: Option<[TokenTree; 2]>, expr: Vec<TokenTree>) {
		self.values.push(Value {
			cfg,
			kind: ValueKind::Interpolate(expr),
		});
	}

	pub fn tuple_count(&mut self) {
		self.values.push(Value {
			cfg: None,
			kind: ValueKind::TupleCount,
		});
	}

	pub fn tuple_value(&mut self, cfg: Option<[TokenTree; 2]>, expr: Vec<TokenTree>) {
		let index = self.tuple_values.len();
		self.tuple_values.push(TupleValue { cfg, expr });
		self.values.push(Value {
			cfg: None,
			kind: ValueKind::TupleA(index),
		});
		self.values.push(Value {
			cfg: None,
			kind: ValueKind::TupleB(index),
		});
	}

	pub fn named_value(&mut self, cfg: Option<[TokenTree; 2]>, name: String) {
		self.values.push(Value {
			cfg,
			kind: ValueKind::Named(name),
		});
	}

	pub fn named_const(&mut self, name: String, cfg: Option<[TokenTree; 2]>, expr: Vec<TokenTree>) {
		let named_arg = if let Some(arg) = self.named_values.iter_mut().find(|arg| arg.name == name)
		{
			arg
		} else {
			self.named_values.push(NamedValue {
				name,
				kind: Vec::new(),
			});
			self.named_values.last_mut().unwrap()
		};

		named_arg.kind.push(NamedValueKind::Const { cfg, expr });
	}

	pub fn named_interpolate(
		&mut self,
		name: String,
		cfg: Option<[TokenTree; 2]>,
		expr: Vec<TokenTree>,
	) {
		let named_arg = if let Some(arg) = self.named_values.iter_mut().find(|arg| arg.name == name)
		{
			arg
		} else {
			self.named_values.push(NamedValue {
				name,
				kind: Vec::new(),
			});
			self.named_values.last_mut().unwrap()
		};

		named_arg
			.kind
			.push(NamedValueKind::Interpolate { cfg, expr });
	}

	fn def_values(&self) -> impl Iterator<Item = DefValue<'_>> {
		self.named_values
			.iter()
			.flat_map(|value| {
				value.kind.iter().map(|kind| match kind {
					NamedValueKind::Const { cfg, expr } => DefValue {
						cfg_1: cfg.as_ref(),
						cfg_2: None,
						name: Cow::Borrowed(&value.name),
						kind: DefValueKind::Const(expr),
					},
					NamedValueKind::Interpolate { cfg, expr } => DefValue {
						cfg_1: cfg.as_ref(),
						cfg_2: None,
						name: Cow::Borrowed(&value.name),
						kind: DefValueKind::Interpolate(Cow::Borrowed(expr)),
					},
				})
			})
			.chain(
				self.tuple_values
					.iter()
					.enumerate()
					.map(|(index, value)| DefValue {
						cfg_1: value.cfg.as_ref(),
						cfg_2: None,
						name: Cow::Owned(index.to_string()),
						kind: DefValueKind::TupleDef(&value.expr),
					}),
			)
			.chain(
				self.values
					.iter()
					.filter(|value| {
						!matches!(value.kind, ValueKind::TupleCount | ValueKind::Named(_))
					})
					.enumerate()
					.filter_map(|(index, value)| {
						let name = Cow::Owned(index.to_string());

						match &value.kind {
							ValueKind::Bytes(bytes) => Some(DefValue {
								cfg_1: value.cfg.as_ref(),
								cfg_2: None,
								name,
								kind: DefValueKind::Bytes(bytes),
							}),
							ValueKind::Const(expr) => Some(DefValue {
								cfg_1: value.cfg.as_ref(),
								cfg_2: None,
								name,
								kind: DefValueKind::Const(expr),
							}),
							ValueKind::Interpolate(expr) => Some(DefValue {
								cfg_1: value.cfg.as_ref(),
								cfg_2: None,
								name,
								kind: DefValueKind::Interpolate(Cow::Borrowed(expr)),
							}),
							ValueKind::TupleA(tuple_index) => Some(DefValue {
								cfg_1: self.tuple_values[*tuple_index].cfg.as_ref(),
								cfg_2: None,
								name,
								kind: DefValueKind::TupleValue(Cow::Owned(
									[
										ident(&format!("TUPLE_{tuple_index}")),
										Punct::new('.', Spacing::Alone).into(),
										Literal::usize_unsuffixed(0).into(),
									]
									.into_iter()
									.collect(),
								)),
							}),
							ValueKind::TupleB(tuple_index) => Some(DefValue {
								cfg_1: self.tuple_values[*tuple_index].cfg.as_ref(),
								cfg_2: None,
								name,
								kind: DefValueKind::TupleValue(Cow::Owned(
									[
										ident(&format!("TUPLE_{tuple_index}")),
										Punct::new('.', Spacing::Alone).into(),
										Literal::usize_unsuffixed(1).into(),
									]
									.into_iter()
									.collect(),
								)),
							}),
							ValueKind::TupleCount | ValueKind::Named(_) => unreachable!(),
						}
					}),
			)
	}

	fn flattened_values(&self) -> impl Iterator<Item = FlattenedValue<'_>> {
		let mut index = 0;

		self.values.iter().flat_map(move |value| {
			let result = match &value.kind {
				ValueKind::Bytes(bytes) => vec![FlattenedValue {
					cfg_1: value.cfg.as_ref(),
					cfg_2: None,
					name: Cow::Owned(index.to_string()),
					kind: FlattenedValueKind::Bytes(bytes),
				}],
				ValueKind::Const(_) => vec![FlattenedValue {
					cfg_1: value.cfg.as_ref(),
					cfg_2: None,
					name: Cow::Owned(index.to_string()),
					kind: FlattenedValueKind::Const,
				}],
				ValueKind::Interpolate(_) => vec![FlattenedValue {
					cfg_1: value.cfg.as_ref(),
					cfg_2: None,
					name: Cow::Owned(index.to_string()),
					kind: FlattenedValueKind::Interpolate,
				}],
				ValueKind::Named(name) => {
					let named = self
						.named_values
						.iter()
						.find(|value| &value.name == name)
						.unwrap();

					named
						.kind
						.iter()
						.map(|kind| match kind {
							NamedValueKind::Const { cfg, .. } => FlattenedValue {
								cfg_1: cfg.as_ref(),
								cfg_2: value.cfg.as_ref(),
								name: Cow::Borrowed(name),
								kind: FlattenedValueKind::Const,
							},
							NamedValueKind::Interpolate { cfg, .. } => FlattenedValue {
								cfg_1: cfg.as_ref(),
								cfg_2: value.cfg.as_ref(),
								name: Cow::Borrowed(name),
								kind: FlattenedValueKind::Interpolate,
							},
						})
						.collect()
				}
				ValueKind::TupleCount => vec![FlattenedValue {
					cfg_1: value.cfg.as_ref(),
					cfg_2: None,
					name: Cow::Borrowed(""),
					kind: FlattenedValueKind::TupleCount,
				}],
				ValueKind::TupleA(tuple_index) | ValueKind::TupleB(tuple_index) => {
					vec![FlattenedValue {
						cfg_1: self.tuple_values[*tuple_index].cfg.as_ref(),
						cfg_2: None,
						name: Cow::Owned(index.to_string()),
						kind: FlattenedValueKind::TupleValue,
					}]
				}
			};

			if !matches!(value.kind, ValueKind::Named(_) | ValueKind::TupleCount) {
				index += 1;
			}

			result
		})
	}

	/// For every byte value we insert:
	/// ```"not rust"
	/// const ARR_<name>: [u8; <argument>.len()] = *<argument>;
	/// ```
	///
	/// For every `const` value we insert:
	/// ```"not rust"
	/// const LEN_<name>: usize = ::js_bindgen::r#macro::ConstInteger(VAL_<name>).__jbg_len();
	/// const ARR_<name>: [u8; LEN_<name>] = ::js_bindgen::r#macro::ConstInteger(VAL_<name>).__jbg_to_le_bytes::<LEN_<name>>();
	/// ```
	///
	/// For every `interpolate` value we insert:
	/// ```"not rust"
	/// const VAL_<name>: &str = <argument>;
	/// const LEN_<name>: usize = str::len(VAL_<name>);
	/// const PTR_<name>: *const u8 = str::as_ptr(VAL_<name>);
	/// const ARR_<name>: [u8; LEN_<name>] = unsafe { *(PTR_<name> as *const _) };
	/// ```
	///
	/// Values with length prefix additionally get:
	/// ```"not rust"
	/// const VAL_<name>_LEN: [u8; 2] = usize::to_le_bytes(LEN_<name>);
	/// ```
	///
	/// For every tuple we insert:
	/// ```"not rust"
	/// const TUPLE_<index>: (&str, &str) = <expr>;
	/// ```
	fn output_values(&self) -> impl Iterator<Item = TokenTree> {
		let span = Span::mixed_site();

		self.def_values().flat_map(move |value| match &value.kind {
			DefValueKind::Bytes(bytes) => {
				// ```
				// const ARR_<name>: [u8; <argument>.len()] = *<argument>;
				// ```
				value
					.cfg_iter()
					.chain(r#const(
						&format!("ARR_{}", value.name),
						iter::once(group(
							Delimiter::Bracket,
							path(["core", "primitive", "u8"], span).chain([
								Punct::new(';', Spacing::Alone).into(),
								Literal::usize_unsuffixed(bytes.len()).into(),
							]),
						)),
						[
							Punct::new('*', Spacing::Alone).into(),
							Literal::byte_string(bytes).into(),
						],
					))
					.collect::<Vec<_>>()
			}
			DefValueKind::Const(const_) => {
				let len_name = format!("LEN_{}", value.name);

				// ```
				// const LEN_<name>: usize = ::js_bindgen::r#macro::ConstInteger(VAL_<name>).__jbg_len();
				// ```
				value
					.cfg_iter()
					.chain(r#const(
						&len_name,
						path(["core", "primitive", "usize"], span),
						path(["js_bindgen", "r#macro", "ConstInteger"], span).chain([
							group(Delimiter::Parenthesis, const_.iter().cloned()),
							Punct::new('.', Spacing::Alone).into(),
							ident("__jbg_len"),
							group(Delimiter::Parenthesis, iter::empty()),
						]),
					))
					// ```
					// const ARR_<name>: [u8; LEN_<name>] = ::js_bindgen::r#macro::ConstInteger(VAL_<name>).__jbg_to_le_bytes::<LEN_<index>>();
					// ```
					.chain(value.cfg_iter())
					.chain(r#const(
						&format!("ARR_{}", value.name),
						iter::once(group(
							Delimiter::Bracket,
							path(["core", "primitive", "u8"], span)
								.chain([Punct::new(';', Spacing::Alone).into(), ident(&len_name)]),
						)),
						path(["js_bindgen", "r#macro", "ConstInteger"], span).chain([
							group(Delimiter::Parenthesis, const_.iter().cloned()),
							Punct::new('.', Spacing::Alone).into(),
							ident("__jbg_to_le_bytes"),
							Punct::new(':', Spacing::Joint).into(),
							Punct::new(':', Spacing::Alone).into(),
							Punct::new('<', Spacing::Alone).into(),
							ident(&len_name),
							Punct::new('>', Spacing::Alone).into(),
							group(Delimiter::Parenthesis, iter::empty()),
						]),
					))
					.collect()
			}
			DefValueKind::Interpolate(interpolate) | DefValueKind::TupleValue(interpolate) => {
				let value_name = format!("VAL_{}", value.name);
				let value_ident = ident(&value_name);
				let len_name = format!("LEN_{}", value.name);
				let ptr_name = format!("PTR_{}", value.name);

				// ```
				// const VAL_<name>: &str = <name>;
				// ```
				value
					.cfg_iter()
					.chain(r#const(
						&value_name,
						[Punct::new('&', Spacing::Alone).into()]
							.into_iter()
							.chain(path(["core", "primitive", "str"], span)),
						interpolate.iter().cloned(),
					))
					// ```
					// const LEN_<name>: usize = str::len(VAL_<name>);
					// ```
					.chain(value.cfg_iter())
					.chain(r#const(
						&len_name,
						path(["core", "primitive", "usize"], span),
						path(["core", "primitive", "str", "len"], span).chain(iter::once(group(
							Delimiter::Parenthesis,
							iter::once(value_ident.clone()),
						))),
					))
					// ```
					// const PTR_<name>: *const u8 = str::as_ptr(VAL_<name>);
					// ```
					.chain(value.cfg_iter())
					.chain(r#const(
						&ptr_name,
						[Punct::new('*', Spacing::Alone).into(), ident("const")]
							.into_iter()
							.chain(path(["core", "primitive", "u8"], span)),
						path(["core", "primitive", "str", "as_ptr"], span).chain(iter::once(
							group(Delimiter::Parenthesis, iter::once(value_ident)),
						)),
					))
					// ```
					// const ARR_<index>: [u8; LEN_<name>] = unsafe { *(PTR_<name> as *const _) };
					// ```
					.chain(value.cfg_iter())
					.chain(r#const(
						&format!("ARR_{}", value.name),
						iter::once(group(
							Delimiter::Bracket,
							path(["core", "primitive", "u8"], span)
								.chain([Punct::new(';', Spacing::Alone).into(), ident(&len_name)]),
						)),
						[
							ident("unsafe"),
							group(
								Delimiter::Brace,
								[
									Punct::new('*', Spacing::Alone).into(),
									group(
										Delimiter::Parenthesis,
										[
											ident(&ptr_name),
											ident("as"),
											Punct::new('*', Spacing::Alone).into(),
											ident("const"),
											ident("_"),
										],
									),
								],
							),
						],
					))
					.chain(
						matches!(value.kind, DefValueKind::TupleValue(_))
							.then(|| {
								// ```
								// const VAL_<name>_LEN: [u8; 2] = u16::to_le_bytes(LEN_<name> as u16);
								// ```
								value.cfg_iter().chain(r#const(
									&format!("VAL_{}_LEN", value.name),
									iter::once(group(
										Delimiter::Bracket,
										path(["core", "primitive", "u8"], span).chain([
											Punct::new(';', Spacing::Alone).into(),
											Literal::usize_unsuffixed(2).into(),
										]),
									)),
									path(["core", "primitive", "u16", "to_le_bytes"], span).chain(
										iter::once(group(
											Delimiter::Parenthesis,
											[ident(&len_name), ident("as"), ident("u16")],
										)),
									),
								))
							})
							.into_iter()
							.flatten(),
					)
					.collect::<Vec<_>>()
			}
			DefValueKind::TupleDef(expr) => {
				// ```
				// const TUPLE_<index>: (&str, &str) = <expr>;
				// ```
				value
					.cfg_iter()
					.chain(r#const(
						&format!("TUPLE_{}", value.name),
						iter::once(group(
							Delimiter::Parenthesis,
							iter::once(Punct::new('&', Spacing::Alone).into())
								.chain(path(["core", "primitive", "str"], span))
								.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
								.chain(iter::once(Punct::new('&', Spacing::Alone).into()))
								.chain(path(["core", "primitive", "str"], span)),
						)),
						expr.iter().cloned(),
					))
					.collect::<Vec<_>>()
			}
		})
	}

	/// ```"not rust"
	/// const LEN: u32 = {
	/// 	let mut len = 0;
	/// 	#(len += LEN_<name>;)*
	/// 	len as _
	/// };
	/// ```
	fn output_len(&self) -> impl Iterator<Item = TokenTree> {
		let span = Span::mixed_site();

		r#const(
			"LEN",
			path(["core", "primitive", "u32"], span),
			[group(
				Delimiter::Brace,
				[
					ident("let"),
					ident("mut"),
					ident("len"),
					Punct::new('=', Spacing::Alone).into(),
					Literal::usize_unsuffixed(0).into(),
					Punct::new(';', Spacing::Alone).into(),
				]
				.into_iter()
				.chain(self.flattened_values().flat_map(|value| {
					let expr = [
						ident("len"),
						Punct::new('+', Spacing::Joint).into(),
						Punct::new('=', Spacing::Alone).into(),
						match value.kind {
							FlattenedValueKind::Bytes(bytes) => {
								Literal::usize_unsuffixed(bytes.len()).into()
							}
							FlattenedValueKind::Const
							| FlattenedValueKind::Interpolate
							| FlattenedValueKind::TupleValue => ident(&format!("LEN_{}", value.name)),
							FlattenedValueKind::TupleCount => Literal::usize_unsuffixed(1).into(),
						},
						Punct::new(';', Spacing::Alone).into(),
					]
					.into_iter()
					.chain(
						matches!(value.kind, FlattenedValueKind::TupleValue)
							.then(|| {
								[
									ident("len"),
									Punct::new('+', Spacing::Joint).into(),
									Punct::new('=', Spacing::Alone).into(),
									Literal::usize_unsuffixed(2).into(),
									Punct::new(';', Spacing::Alone).into(),
								]
							})
							.into_iter()
							.flatten(),
					);

					if value.cfg_1.is_none() && value.cfg_2.is_none() {
						expr.collect::<Vec<_>>()
					} else {
						value
							.cfg_iter()
							.chain(iter::once(group(Delimiter::Brace, expr)))
							.collect()
					}
				}))
				.chain([ident("len"), ident("as"), ident("_")]),
			)],
		)
	}

	/// ```"not rust"
	/// const TUPLE_COUNT: u8 = {
	/// 	let mut len = 0;
	/// 	#(len += 1;)*
	/// 	len
	/// };
	/// ```
	fn output_tuple_count(&self) -> impl Iterator<Item = TokenTree> {
		(self
			.values
			.iter()
			.any(|value| matches!(value.kind, ValueKind::TupleCount)))
		.then(|| {
			let span = Span::mixed_site();
			let init = self.tuple_values.iter().fold(0, |mut count, value| {
				if value.cfg.is_none() {
					count += 1;
				}

				count
			});

			r#const(
				"TUPLE_COUNT",
				path(["core", "primitive", "u8"], span),
				[group(
					Delimiter::Brace,
					[
						ident("let"),
						ident("mut"),
						ident("len"),
						Punct::new('=', Spacing::Alone).into(),
						Literal::u8_unsuffixed(init).into(),
						Punct::new(';', Spacing::Alone).into(),
					]
					.into_iter()
					.chain(
						self.tuple_values
							.iter()
							.filter_map(|value| value.cfg.as_ref())
							.flat_map(|cfg| {
								cfg.iter().cloned().chain(iter::once(group(
									Delimiter::Brace,
									[
										ident("len"),
										Punct::new('+', Spacing::Joint).into(),
										Punct::new('=', Spacing::Alone).into(),
										Literal::u8_unsuffixed(1).into(),
										Punct::new(';', Spacing::Alone).into(),
									],
								)))
							}),
					)
					.chain(iter::once(ident("len"))),
				)],
			)
		})
		.into_iter()
		.flatten()
	}

	/// ```"not rust"
	/// #[repr(C)]
	/// struct Layout([u8; 4], #([u8; LEN_<name>]),*);
	/// ```
	fn output_layout(&self) -> impl Iterator<Item = TokenTree> {
		let span = Span::mixed_site();

		// ```
		// [u8; 4], #([u8; LEN_<name>]),*
		// ```
		let tys = [
			group(
				Delimiter::Bracket,
				path(["core", "primitive", "u8"], span).chain([
					Punct::new(';', Spacing::Alone).into(),
					Literal::usize_unsuffixed(4).into(),
				]),
			),
			Punct::new(',', Spacing::Alone).into(),
		]
		.into_iter()
		.chain(self.flattened_values().flat_map(move |value| {
			value
				.cfg_iter()
				.chain([
					group(
						Delimiter::Bracket,
						path(["core", "primitive", "u8"], span).chain([
							Punct::new(';', Spacing::Alone).into(),
							match value.kind {
								FlattenedValueKind::Bytes(bytes) => {
									Literal::usize_unsuffixed(bytes.len()).into()
								}
								FlattenedValueKind::Const | FlattenedValueKind::Interpolate => {
									ident(&format!("LEN_{}", value.name))
								}
								FlattenedValueKind::TupleValue => {
									Literal::usize_unsuffixed(2).into()
								}
								FlattenedValueKind::TupleCount => {
									Literal::usize_unsuffixed(1).into()
								}
							},
						]),
					),
					Punct::new(',', Spacing::Alone).into(),
				])
				.chain(
					matches!(value.kind, FlattenedValueKind::TupleValue)
						.then(|| {
							value.cfg_iter().chain([
								group(
									Delimiter::Bracket,
									path(["core", "primitive", "u8"], span).chain([
										Punct::new(';', Spacing::Alone).into(),
										ident(&format!("LEN_{}", value.name)),
									]),
								),
								Punct::new(',', Spacing::Alone).into(),
							])
						})
						.into_iter()
						.flatten(),
				)
		}));

		// ```
		// #[repr(C)]
		// struct Layout(...);
		// ```
		[
			Punct::new('#', Spacing::Alone).into(),
			group(
				Delimiter::Bracket,
				[
					ident("repr"),
					group(Delimiter::Parenthesis, iter::once(ident("C"))),
				],
			),
			ident("struct"),
			ident("Layout"),
			group(Delimiter::Parenthesis, tys),
			Punct::new(';', Spacing::Alone).into(),
		]
		.into_iter()
	}

	/// ```"not rust"
	/// #[link_section = name]
	/// static CUSTOM_SECTION: Layout = Layout(...(u32::to_le_bytes(LEN), #(ARR_<name>),*));
	/// ```
	fn output_custom_section(&self, name: &str) -> impl Iterator<Item = TokenTree> {
		let span = Span::mixed_site();

		// ```
		// #[link_section = name]
		// ```
		let link_section = [
			Punct::new('#', Spacing::Alone).into(),
			group(
				Delimiter::Bracket,
				[
					ident("unsafe"),
					group(
						Delimiter::Parenthesis,
						[
							ident("link_section"),
							Punct::new('=', Spacing::Alone).into(),
							Literal::string(name).into(),
						],
					),
				],
			),
		];

		// ```
		// (u32::to_le_bytes(LEN), #(ARR_<name>),*)
		// ```
		let values = group(
			Delimiter::Parenthesis,
			path(["core", "primitive", "u32", "to_le_bytes"], span)
				.chain([
					group(Delimiter::Parenthesis, iter::once(ident("LEN"))),
					Punct::new(',', Spacing::Alone).into(),
				])
				.chain(self.flattened_values().flat_map(move |value| {
					matches!(value.kind, FlattenedValueKind::TupleValue)
						.then(|| {
							value.cfg_iter().chain([
								ident(&format!("VAL_{}_LEN", value.name)),
								Punct::new(',', Spacing::Alone).into(),
							])
						})
						.into_iter()
						.flatten()
						.chain(value.cfg_iter().chain([
							if let FlattenedValueKind::TupleCount = value.kind {
								ident("TUPLE_COUNT")
							} else {
								ident(&format!("ARR_{}", value.name))
							},
							Punct::new(',', Spacing::Alone).into(),
						]))
				})),
		);

		// ```
		// static CUSTOM_SECTION: Layout = Layout(...);
		// ```
		let custom_section = [
			ident("static"),
			ident("CUSTOM_SECTION"),
			Punct::new(':', Spacing::Alone).into(),
			ident("Layout"),
			Punct::new('=', Spacing::Alone).into(),
			ident("Layout"),
			values,
			Punct::new(';', Spacing::Alone).into(),
		];

		link_section.into_iter().chain(custom_section)
	}

	/// ```"not rust"
	/// const _: () = {
	/// 	const LEN: u32 = {
	/// 		let mut len: usize = 0;
	/// 		#(len += LEN_<index>;)*
	/// 		len as _
	/// 	};
	///
	/// 	#[repr(C)]
	/// 	struct Layout([u8; 4], #([u8; LEN_<index>]),*);
	///
	/// 	#[link_section = name]
	/// 	static CUSTOM_SECTION: Layout = Layout(u32::to_le_bytes(LEN), #(ARR_<index>),*);
	/// };
	/// ```
	pub fn output(self, name: &str) -> TokenStream {
		r#const(
			"_",
			iter::once(group(Delimiter::Parenthesis, iter::empty())),
			iter::once(group(
				Delimiter::Brace,
				self.output_values()
					.chain(self.output_len())
					.chain(self.output_tuple_count())
					.chain(self.output_layout())
					.chain(self.output_custom_section(name)),
			)),
		)
		.collect()
	}
}

struct DefValue<'v> {
	cfg_1: Option<&'v [TokenTree; 2]>,
	cfg_2: Option<&'v [TokenTree; 2]>,
	name: Cow<'v, str>,
	kind: DefValueKind<'v>,
}

enum DefValueKind<'v> {
	Bytes(&'v [u8]),
	Const(&'v [TokenTree]),
	Interpolate(Cow<'v, [TokenTree]>),
	TupleDef(&'v [TokenTree]),
	TupleValue(Cow<'v, [TokenTree]>),
}

struct FlattenedValue<'v> {
	cfg_1: Option<&'v [TokenTree; 2]>,
	cfg_2: Option<&'v [TokenTree; 2]>,
	name: Cow<'v, str>,
	kind: FlattenedValueKind<'v>,
}

enum FlattenedValueKind<'v> {
	Bytes(&'v [u8]),
	Const,
	Interpolate,
	TupleCount,
	TupleValue,
}

impl<'v> DefValue<'v> {
	fn cfg_iter(&self) -> impl use<'v> + Iterator<Item = TokenTree> {
		self.cfg_1.into_iter().chain(self.cfg_2).flatten().cloned()
	}
}

impl<'v> FlattenedValue<'v> {
	fn cfg_iter(&self) -> impl use<'v> + Iterator<Item = TokenTree> {
		self.cfg_1.into_iter().chain(self.cfg_2).flatten().cloned()
	}
}

fn group(delimiter: Delimiter, inner: impl IntoIterator<Item = TokenTree>) -> TokenTree {
	Group::new(delimiter, inner.into_iter().collect()).into()
}

fn r#const<TY, VALUE>(
	name: &str,
	ty: TY,
	value: VALUE,
) -> impl use<TY, VALUE> + Iterator<Item = TokenTree>
where
	TY: IntoIterator<Item = TokenTree>,
	VALUE: IntoIterator<Item = TokenTree>,
{
	[
		ident("const"),
		ident(name),
		Punct::new(':', Spacing::Alone).into(),
	]
	.into_iter()
	.chain(ty)
	.chain(iter::once(Punct::new('=', Spacing::Alone).into()))
	.chain(value)
	.chain(iter::once(Punct::new(';', Spacing::Alone).into()))
}

fn ident(string: &str) -> TokenTree {
	Ident::new(string, Span::mixed_site()).into()
}
