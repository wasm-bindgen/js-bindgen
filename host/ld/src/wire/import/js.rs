use std::fmt::Write;

use js_bindgen_wire::model::{
	DirectImportConversion, Embed, GlobalPath, Import, ImportBindingKind, ImportCatch,
	ImportErrorMode, ImportGroup, ImportOutputAbi, ImportRetptr, ImportWriter, JsCatch,
};

use crate::wire::JsBinding;
use crate::wire::js::{JsPath, Placeholder, render_template};

use super::input_name;

/// Renders every import which has a generated JavaScript binding.
pub(super) fn render<'a>(group: &ImportGroup<'a>) -> Vec<JsBinding<'a>> {
	let group_catch = match group.catch.as_ref() {
		Some(ImportCatch::JavaScript(catch)) => Some(catch),
		Some(ImportCatch::Wasm(_)) | None => None,
	};

	group
		.imports
		.iter()
		.filter_map(|import| {
			let binding = import.binding.as_ref()?;
			let catches = import
				.output
				.as_ref()
				.is_some_and(|output| output.error == ImportErrorMode::CatchInJavaScript);
			let catch = catches.then(|| {
				group_catch
					.expect("a JavaScript-catching Result import has no JavaScript catch metadata")
			});
			let mut embeds = binding.embeds.clone();
			if let Some(catch) = catch {
				embeds.extend(catch.embeds.iter().copied());
			}
			Some(JsBinding {
				module: import.module,
				name: import.name,
				js: render_function(
					import,
					render_operation(binding.kind, import.inputs.len()),
					catch,
				),
				embeds,
			})
		})
		.collect()
}

pub(super) fn render_closure_factory(
	import: &Import<'_>,
	helper: Embed<'_>,
	call_name: &str,
) -> String {
	let mut arguments = input_values(import.inputs.len());
	arguments.push(
		JsPath::new("this.#jsExports")
			.property(call_name)
			.into_string(),
	);
	let expression = format!("{}({})", embed_path(helper), arguments.join(", "));

	render_function(
		import,
		RenderedOperation {
			expression,
			direct_callable: None,
		},
		None,
	)
}

fn render_function(
	import: &Import<'_>,
	operation: RenderedOperation,
	catch: Option<&JsCatch<'_>>,
) -> String {
	debug_assert!(!import.suspending || catch.is_none());
	let output_needs_wrapper = import.output.as_ref().is_some_and(|output| {
		catch.is_some()
			|| match &output.abi {
				ImportOutputAbi::Direct { conversion, .. } => conversion.is_some(),
				ImportOutputAbi::Indirect { .. } => true,
			}
	});
	let needs_wrapper = import
		.inputs
		.iter()
		.any(|input| input.js_conversion.is_some())
		|| output_needs_wrapper;
	let await_output = import.suspending && output_needs_wrapper;
	let RenderedOperation {
		expression,
		direct_callable,
	} = operation;

	if !needs_wrapper && let Some(direct) = direct_callable {
		if import.suspending {
			return format!("new WebAssembly.Suspending({direct})");
		}
		return direct;
	}

	let PreparedInputs {
		parameters,
		conversions,
	} = prepare_inputs(import);
	let js = if needs_wrapper {
		let asynchronous = if await_output { "async " } else { "" };
		let body = if let Some(output) = import.output.as_ref() {
			match &output.abi {
				ImportOutputAbi::Direct { conversion, .. } => {
					render_direct_output(&expression, conversion.as_ref(), await_output, catch)
				}
				ImportOutputAbi::Indirect { retptr, writer } => {
					render_indirect_output(&expression, retptr, writer, await_output, catch)
				}
			}
		} else {
			let return_ = if import.suspending { "return " } else { "" };
			format!("    {return_}{expression}\n}}")
		};
		format!("{asynchronous}({parameters}) => {{\n{conversions}{body}")
	} else {
		format!("({parameters}) => {expression}")
	};

	if import.suspending {
		format!("new WebAssembly.Suspending({js})")
	} else {
		js
	}
}

struct RenderedOperation {
	expression: String,
	// Present only when the operation itself can be installed as the import.
	direct_callable: Option<String>,
}

fn render_operation(kind: ImportBindingKind<'_>, input_count: usize) -> RenderedOperation {
	let values = input_values(input_count);
	let (expression, direct_callable) = match kind {
		ImportBindingKind::CallGlobal { target, variadic } => {
			let is_direct = target.namespace.is_none() && target.object.is_none() && !variadic;
			let target = global_path(target);
			let arguments = render_arguments(&values, false, variadic);
			let direct = is_direct.then(|| target.to_string());
			(format!("{target}({arguments})"), direct)
		}
		ImportBindingKind::CallMethod { name, variadic } => {
			let arguments = render_arguments(&values, true, variadic);
			let method = JsPath::new(&values[0]).property(name);
			(format!("{method}({arguments})"), None)
		}
		ImportBindingKind::Construct { target, variadic } => {
			let target = global_path(target);
			let arguments = render_arguments(&values, false, variadic);
			(format!("new {target}({arguments})"), None)
		}
		ImportBindingKind::GetGlobal(target) => (global_path(target).into_string(), None),
		ImportBindingKind::GetMember(name) => {
			(JsPath::new(&values[0]).property(name).into_string(), None)
		}
		ImportBindingKind::SetGlobal(target) => {
			(format!("{} = {}", global_path(target), values[0]), None)
		}
		ImportBindingKind::SetMember(name) => (
			format!("{} = {}", JsPath::new(&values[0]).property(name), values[1]),
			None,
		),
		ImportBindingKind::IndexGet => (format!("{}[{}]", values[0], values[1]), None),
		ImportBindingKind::IndexSet => (
			format!("{}[{}] = {}", values[0], values[1], values[2]),
			None,
		),
		ImportBindingKind::IndexDelete => (format!("delete {}[{}]", values[0], values[1]), None),
		ImportBindingKind::CallEmbed(embed) => {
			let target = embed_path(embed);
			let arguments = values.join(", ");
			let direct = Some(target.to_string());
			(format!("{target}({arguments})"), direct)
		}
	};

	RenderedOperation {
		expression,
		direct_callable,
	}
}

fn global_path(target: GlobalPath<'_>) -> JsPath {
	let mut path = JsPath::new("globalThis");
	if let Some(namespace) = target.namespace {
		for component in namespace.split('.') {
			path = path.property(component);
		}
	}
	if let Some(object) = target.object {
		path = path.property(object);
	}
	path.property(target.name)
}

fn embed_path(embed: Embed<'_>) -> JsPath {
	JsPath::new("this.#jsEmbed")
		.property(embed.module)
		.property(embed.name)
}

fn input_values(count: usize) -> Vec<String> {
	(0..count).map(|index| format!("arg{index}_0")).collect()
}

fn render_arguments(inputs: &[String], receiver: bool, variadic: bool) -> String {
	let inputs = if receiver { &inputs[1..] } else { inputs };
	if !variadic {
		return inputs.join(", ");
	}

	let (last, inputs) = inputs
		.split_last()
		.expect("a variadic import must have an argument");
	if inputs.is_empty() {
		format!("...{last}")
	} else {
		format!("{}, ...{last}", inputs.join(", "))
	}
}

struct PreparedInputs {
	parameters: String,
	conversions: String,
}

fn prepare_inputs(import: &Import<'_>) -> PreparedInputs {
	let mut parameters = Vec::new();
	if import
		.output
		.as_ref()
		.is_some_and(|output| !output.is_direct())
	{
		parameters.push("$retptr".to_owned());
	}
	let mut conversions = String::new();
	for (index, input) in import.inputs.iter().enumerate() {
		let name = input_name(index);
		parameters.extend((0..input.slots.len()).map(|slot| format!("{name}_{slot}")));
		if let Some(template) = input.js_conversion {
			let declaration = if input.slots.is_empty() { "const " } else { "" };
			let expression = render_template(template, |rendered, placeholder| {
				if let Placeholder::Slot(slot) = placeholder {
					write!(rendered, "{name}_{slot}").expect("writing to a String cannot fail");
				}
			});
			writeln!(conversions, "    {declaration}{name}_0 = {expression}")
				.expect("writing to a String cannot fail");
		}
	}
	PreparedInputs {
		parameters: parameters.join(", "),
		conversions,
	}
}

fn render_direct_output(
	expression: &str,
	conversion: Option<&DirectImportConversion<'_>>,
	await_output: bool,
	catch: Option<&JsCatch<'_>>,
) -> String {
	let indent = if catch.is_some() { "        " } else { "    " };
	let expression = if await_output {
		format!("await ({expression})")
	} else {
		expression.to_owned()
	};
	let mut js = if catch.is_some() {
		"    try {\n".to_owned()
	} else {
		String::new()
	};

	if let Some(conversion) = conversion {
		write!(js, "{indent}const $ret = {expression}").expect("writing to a String cannot fail");
		js.push_str(&render_prepare(conversion.prepare, indent));
		write!(
			js,
			"\n{indent}return {}",
			render_result_template(conversion.expression),
		)
		.expect("writing to a String cannot fail");
	} else {
		write!(js, "{indent}return {expression}").expect("writing to a String cannot fail");
	}

	js.push_str(catch.map_or("\n}", |catch| catch.direct));
	js
}

fn render_indirect_output(
	expression: &str,
	retptr: &ImportRetptr<'_>,
	writer: &ImportWriter<'_>,
	await_output: bool,
	catch: Option<&JsCatch<'_>>,
) -> String {
	let indent = if catch.is_some() { "        " } else { "    " };
	let mut js = String::new();
	if let Some(template) = retptr.js_conversion
		&& !template.is_empty()
	{
		let expression = render_template(template, |rendered, placeholder| match placeholder {
			Placeholder::Slot(0) => rendered.push_str("$retptr"),
			Placeholder::Value => rendered.push_str("$value"),
			Placeholder::Prepared => rendered.push_str("$prepared"),
			Placeholder::Slot(slot) => {
				write!(rendered, "$slot{}", slot + 1).expect("writing to a String cannot fail");
			}
		});
		writeln!(js, "    $retptr = {expression}").expect("writing to a String cannot fail");
	}
	if catch.is_some() {
		js.push_str("    try {\n");
	}

	let expression = if await_output {
		format!("await ({expression})")
	} else {
		expression.to_owned()
	};
	write!(js, "{indent}const $ret = {expression}").expect("writing to a String cannot fail");

	match writer {
		ImportWriter::Slots {
			function,
			prepare,
			expressions,
		} => {
			js.push_str(&render_prepare(*prepare, indent));
			let mut arguments = expressions
				.iter()
				.map(|expression| render_result_template(expression))
				.filter(|expression| !expression.is_empty())
				.collect::<Vec<_>>();
			arguments.push("$retptr".to_owned());
			write!(js, "\n{indent}{function}({})", arguments.join(", "))
				.expect("writing to a String cannot fail");
		}
		ImportWriter::Value { function } => {
			write!(js, "\n{indent}{function}($ret, $retptr)")
				.expect("writing to a String cannot fail");
		}
	}

	js.push_str(catch.map_or("\n}", |catch| catch.indirect));
	js
}

fn render_prepare(prepare: Option<&str>, indent: &str) -> String {
	prepare
		.filter(|prepare| !prepare.is_empty())
		.map_or_else(String::new, |prepare| {
			format!(
				"\n{indent}const $prepared = {}",
				render_result_template(prepare),
			)
		})
}

fn render_result_template(template: &str) -> String {
	render_template(template, |rendered, placeholder| match placeholder {
		Placeholder::Value => rendered.push_str("$ret"),
		Placeholder::Prepared => rendered.push_str("$prepared"),
		Placeholder::Slot(_) => {}
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	const BARE: GlobalPath<'static> = GlobalPath {
		namespace: None,
		object: None,
		name: "identity-value",
	};

	#[test]
	fn renders_representative_call_shapes() {
		let cases = [
			(
				ImportBindingKind::CallGlobal {
					target: BARE,
					variadic: false,
				},
				1,
				"globalThis[\"identity-value\"](arg0_0)",
				Some("globalThis[\"identity-value\"]"),
			),
			(
				ImportBindingKind::CallMethod {
					name: "push-value",
					variadic: true,
				},
				3,
				"arg0_0[\"push-value\"](arg1_0, ...arg2_0)",
				None,
			),
			(
				ImportBindingKind::GetMember("data-value"),
				1,
				"arg0_0[\"data-value\"]",
				None,
			),
			(
				ImportBindingKind::SetMember("data-value"),
				2,
				"arg0_0[\"data-value\"] = arg1_0",
				None,
			),
			(
				ImportBindingKind::CallEmbed(Embed {
					module: "test",
					name: "map",
				}),
				1,
				"this.#jsEmbed[\"test\"][\"map\"](arg0_0)",
				Some("this.#jsEmbed[\"test\"][\"map\"]"),
			),
		];

		for (kind, input_count, expression, direct_callable) in cases {
			let rendered = render_operation(kind, input_count);
			assert_eq!(rendered.expression, expression);
			assert_eq!(rendered.direct_callable.as_deref(), direct_callable);
		}
	}

	#[test]
	fn renders_namespace_segments_and_single_properties() {
		let path = GlobalPath {
			namespace: Some("Temporal.Now"),
			object: Some("object.with.dot"),
			name: "method-name",
		};

		assert_eq!(
			global_path(path).into_string(),
			"globalThis[\"Temporal\"][\"Now\"][\"object.with.dot\"][\"method-name\"]",
		);
	}

	#[test]
	fn preserves_the_bare_global_direct_path() {
		let mut import = Import {
			module: "test",
			name: "identity",
			inputs: Vec::new(),
			output: None,
			binding: None,
			suspending: false,
		};
		let operation = || {
			render_operation(
				ImportBindingKind::CallGlobal {
					target: BARE,
					variadic: false,
				},
				0,
			)
		};

		assert_eq!(
			render_function(&import, operation(), None),
			"globalThis[\"identity-value\"]",
		);
		import.suspending = true;
		assert_eq!(
			render_function(&import, operation(), None),
			"new WebAssembly.Suspending(globalThis[\"identity-value\"])",
		);
	}
}
