use std::fmt::Write;

use js_bindgen_wire::model::{
	DirectImportConversion, Import, ImportBinding, ImportCatch, ImportErrorMode, ImportGroup,
	ImportInput, ImportOutput, ImportOutputAbi, ImportRetptr, ImportWriter, JsCatch,
};

use crate::wire::JsBinding;
use crate::wire::js::{Placeholder, render_template};

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
				js: render_binding(import, binding, catch),
				embeds,
			})
		})
		.collect()
}

fn render_binding(
	import: &Import<'_>,
	binding: &ImportBinding<'_>,
	catch: Option<&JsCatch<'_>>,
) -> String {
	debug_assert!(!import.suspending || catch.is_none());
	let output_needs_wrapper = import
		.output
		.as_ref()
		.is_some_and(|output| output_needs_wrapper(output, catch));
	let needs_wrapper = import
		.inputs
		.iter()
		.any(|input| input.js_conversion.is_some())
		|| output_needs_wrapper;
	let await_output = import.suspending && output_needs_wrapper;
	let parameters = render_parameters(import);

	let js = if needs_wrapper {
		let conversions = render_input_conversions(&import.inputs);
		let asynchronous = if await_output { "async " } else { "" };
		let body = if let Some(output) = import.output.as_ref() {
			render_output(output, binding.call_expression, await_output, catch)
		} else {
			let return_ = if import.suspending { "return " } else { "" };
			format!("    {return_}{}\n}}", binding.call_expression)
		};
		format!("{asynchronous}({parameters}) => {{\n{conversions}{body}")
	} else if let Some(direct) = binding.direct_expression {
		direct.to_owned()
	} else {
		format!("({parameters}) => {}", binding.call_expression)
	};

	if import.suspending {
		format!("new WebAssembly.Suspending({js})")
	} else {
		js
	}
}

fn output_needs_wrapper(output: &ImportOutput<'_>, catch: Option<&JsCatch<'_>>) -> bool {
	catch.is_some()
		|| match output.abi {
			ImportOutputAbi::Direct { conversion, .. } => conversion.is_some(),
			ImportOutputAbi::Indirect { .. } => true,
		}
}

fn render_parameters(import: &Import<'_>) -> String {
	let mut parameters = Vec::new();
	if import
		.output
		.as_ref()
		.is_some_and(|output| !output.is_direct())
	{
		parameters.push("$retptr".to_owned());
	}
	parameters.extend(
		import
			.inputs
			.iter()
			.flat_map(|input| (0..input.slots.len()).map(|slot| format!("{}_{slot}", input.name))),
	);
	parameters.join(", ")
}

fn render_input_conversions(inputs: &[ImportInput<'_>]) -> String {
	inputs
		.iter()
		.filter_map(|input| {
			let template = input.js_conversion?;
			let declaration = if input.slots.is_empty() { "const " } else { "" };
			let expression = render_input_template(template, input.name);
			Some(format!(
				"    {declaration}{}_0 = {expression}\n",
				input.name
			))
		})
		.collect()
}

fn render_output(
	output: &ImportOutput<'_>,
	call: &str,
	await_output: bool,
	catch: Option<&JsCatch<'_>>,
) -> String {
	match &output.abi {
		ImportOutputAbi::Direct { conversion, .. } => {
			render_direct_output(call, conversion.as_ref(), await_output, catch)
		}
		ImportOutputAbi::Indirect { retptr, writer } => {
			render_indirect_output(call, retptr, writer, await_output, catch)
		}
	}
}

fn render_direct_output(
	call: &str,
	conversion: Option<&DirectImportConversion<'_>>,
	await_output: bool,
	catch: Option<&JsCatch<'_>>,
) -> String {
	let indent = if catch.is_some() { "        " } else { "    " };
	let call = if await_output {
		format!("await ({call})")
	} else {
		call.to_owned()
	};
	let mut js = if catch.is_some() {
		"    try {\n".to_owned()
	} else {
		String::new()
	};

	if let Some(conversion) = conversion {
		write!(js, "{indent}const $ret = {call}").expect("writing to a String cannot fail");
		js.push_str(&render_prepare(conversion.prepare, indent));
		write!(
			js,
			"\n{indent}return {}",
			render_result_template(conversion.expression),
		)
		.expect("writing to a String cannot fail");
	} else {
		write!(js, "{indent}return {call}").expect("writing to a String cannot fail");
	}

	js.push_str(catch.map_or("\n}", |catch| catch.direct));
	js
}

fn render_indirect_output(
	call: &str,
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
		writeln!(js, "    $retptr = {}", render_retptr_template(template))
			.expect("writing to a String cannot fail");
	}
	if catch.is_some() {
		js.push_str("    try {\n");
	}

	let call = if await_output {
		format!("await ({call})")
	} else {
		call.to_owned()
	};
	write!(js, "{indent}const $ret = {call}").expect("writing to a String cannot fail");

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

fn render_input_template(template: &str, name: &str) -> String {
	render_template(template, |rendered, placeholder| {
		if let Placeholder::Slot(slot) = placeholder {
			write!(rendered, "{name}_{slot}").expect("writing to a String cannot fail");
		}
	})
}

fn render_result_template(template: &str) -> String {
	render_template(template, |rendered, placeholder| match placeholder {
		Placeholder::Value => rendered.push_str("$ret"),
		Placeholder::Prepared => rendered.push_str("$prepared"),
		Placeholder::Slot(_) => {}
	})
}

fn render_retptr_template(template: &str) -> String {
	render_template(template, |rendered, placeholder| match placeholder {
		Placeholder::Slot(0) => rendered.push_str("$retptr"),
		Placeholder::Value => rendered.push_str("$value"),
		Placeholder::Prepared => rendered.push_str("$prepared"),
		Placeholder::Slot(slot) => {
			write!(rendered, "$slot{}", slot + 1).expect("writing to a String cannot fail");
		}
	})
}
