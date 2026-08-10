use js_bindgen_wire::model::{Export, ExportOutput};

use crate::wire::JsBinding;
use crate::wire::js::{Placeholder, quote_string, render_template};

use super::input_names;

/// Renders one decoded Rust export or closure dispatcher.
pub(super) fn render<'a>(export: &Export<'a>) -> JsBinding<'a> {
	let function = format!("wasmExports[{}]", quote_string(export.name));
	let callable = if export.promising {
		format!("WebAssembly.promising({function})")
	} else {
		function
	};
	let passthrough = !export.inputs.iter().any(|input| input.conversion.is_some())
		&& export
			.output
			.as_ref()
			.is_none_or(|output| output.js_conversion().is_none() && output.result().is_none());
	let js = if passthrough {
		callable
	} else {
		let indent = if export.promising { "        " } else { "    " };
		let call = PreparedCall::new(export, callable, indent);
		if export.promising {
			render_promising(&call)
		} else {
			render_sync(&call)
		}
	};

	JsBinding {
		module: export.module,
		name: export.name,
		js,
		embeds: export.embeds.clone(),
	}
}

/// The wrapper signature and Rust `ABI` call assembled in one input pass.
struct PreparedCall<'export, 'wire> {
	export: &'export Export<'wire>,
	callable: String,
	parameters: String,
	arguments: String,
	prepares: String,
}

impl<'export, 'wire> PreparedCall<'export, 'wire> {
	fn new(export: &'export Export<'wire>, callable: String, indent: &str) -> Self {
		let names = input_names(&export.inputs);
		let parameters = names.join(", ");
		let mut arguments = String::new();
		let mut prepares = String::new();
		let mut wrote_argument = false;

		for (input, name) in export.inputs.iter().zip(&names) {
			if let Some(conversion) = input.conversion.as_ref() {
				for expression in &conversion.expressions {
					if wrote_argument {
						arguments.push_str(", ");
					}
					arguments.push_str(&render_template(expression, |rendered, placeholder| {
						match placeholder {
							Placeholder::Value => rendered.push_str(name),
							Placeholder::Prepared => {
								rendered.push_str(name);
								rendered.push_str("$prepared");
							}
							Placeholder::Slot(_) => {}
						}
					}));
					wrote_argument = true;
				}

				if let Some(prepare) = conversion.prepare.filter(|prepare| !prepare.is_empty()) {
					let prepare = render_template(prepare, |rendered, placeholder| {
						if placeholder == Placeholder::Value {
							rendered.push_str(name);
						}
					});
					prepares.push_str(indent);
					prepares.push_str("const ");
					prepares.push_str(name);
					prepares.push_str("$prepared = ");
					prepares.push_str(&prepare);
					prepares.push('\n');
				}
			} else if !input.slots.is_empty() {
				if wrote_argument {
					arguments.push_str(", ");
				}
				arguments.push_str(name);
				wrote_argument = true;
			}
		}

		Self {
			export,
			callable,
			parameters,
			arguments,
			prepares,
		}
	}
}

fn render_sync(call: &PreparedCall<'_, '_>) -> String {
	let invoke = format!("{}({})", call.callable, call.arguments);
	if let Some(output) = call.export.output.as_ref()
		&& !output.is_void()
	{
		format!(
			"({}) => {{\n{}    const ret = {invoke}\n{}\n}}",
			call.parameters,
			call.prepares,
			render_output(output, "    ")
		)
	} else {
		format!(
			"({}) => {{\n{}    {invoke}\n}}",
			call.parameters, call.prepares
		)
	}
}

fn render_promising(call: &PreparedCall<'_, '_>) -> String {
	let then = call
		.export
		.output
		.as_ref()
		.map_or_else(String::new, |output| {
			if output.js_conversion().is_none() && output.result().is_none() {
				String::new()
			} else {
				format!(
					".then(ret => {{\n{}\n    }})",
					render_output(output, "        ")
				)
			}
		});

	if call.prepares.is_empty() {
		format!(
			"(() => {{\n    const $promising = {}\n    return ({}) => \
			 $promising({}){then}\n}})()",
			call.callable, call.parameters, call.arguments
		)
	} else {
		format!(
			"(() => {{\n    const $promising = {}\n    return ({}) => \
			 {{\n{}        return $promising({}){then}\n    }}\n}})()",
			call.callable, call.parameters, call.prepares, call.arguments
		)
	}
}

fn render_output(output: &ExportOutput<'_>, indent: &str) -> String {
	let result = if let Some(result) = output.result() {
		format!(
			"{indent}if (ret[{}] !== 0) throw ret[{}]\n",
			result.discriminant, result.error
		)
	} else {
		String::new()
	};
	let expression = output.js_conversion().map_or_else(
		|| render_output_slot(output, 0),
		|template| {
			render_template(template, |rendered, placeholder| {
				if let Placeholder::Slot(slot) = placeholder {
					rendered.push_str(&render_output_slot(output, slot));
				}
			})
		},
	);
	format!("{result}{indent}return {expression}")
}

fn render_output_slot(output: &ExportOutput<'_>, slot: usize) -> String {
	if let Some(result) = output.result() {
		if slot < usize::from(result.discriminant) {
			return format!("ret[{slot}]");
		}
	} else if output.is_direct() && !output.is_void() {
		if slot == 0 {
			return "ret".to_owned();
		}
	} else {
		return format!("ret[{slot}]");
	}
	String::new()
}

#[cfg(test)]
mod tests {
	use js_bindgen_wire::PointerWidth;
	use js_bindgen_wire::model::{Callee, Export, ExportOutput};

	use super::render;

	#[test]
	fn void_passthrough() {
		let export = Export {
			module: "test",
			name: "unit",
			pointer_width: PointerWidth::Wasm32,
			inputs: Vec::new(),
			output: Some(ExportOutput::Direct {
				slot: None,
				js_conversion: None,
			}),
			embeds: Vec::new(),
			promising: false,
			callee: Callee::Symbol { name: "unit.raw" },
		};

		assert_eq!(render(&export).js, r#"wasmExports["unit"]"#);
	}
}
