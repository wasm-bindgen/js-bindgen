use js_bindgen_wire::model::{Export, ExportInput, ExportOutput};

use crate::wire::JsBinding;
use crate::wire::js::{Placeholder, render_template};

/// Renders one decoded Rust export or closure dispatcher.
pub(super) fn render<'a>(export: &Export<'a>) -> JsBinding<'a> {
	JsBinding {
		module: export.module,
		name: export.name,
		js: render_export(export),
		embeds: export.embeds.clone(),
	}
}

fn render_export(export: &Export<'_>) -> String {
	let function = format!("wasmExports['{}']", export.name);
	let callable = if export.promising {
		format!("WebAssembly.promising({function})")
	} else {
		function
	};

	if is_passthrough(export) {
		return callable;
	}

	let parameters = export
		.inputs
		.iter()
		.map(|input| input.name)
		.collect::<Vec<_>>()
		.join(", ");
	let arguments = render_arguments(&export.inputs);

	if export.promising {
		render_promising(export, &callable, &parameters, &arguments)
	} else {
		render_sync(export, &callable, &parameters, &arguments)
	}
}

fn render_sync(export: &Export<'_>, callable: &str, parameters: &str, arguments: &str) -> String {
	let prepares = render_prepares(&export.inputs, "    ");
	let call = format!("{callable}({arguments})");
	if let Some(output) = export.output.as_ref() {
		format!(
			"({parameters}) => {{\n{prepares}    const ret = {call}\n{}\n}}",
			render_output(output, "    ")
		)
	} else {
		format!("({parameters}) => {{\n{prepares}    {call}\n}}")
	}
}

fn render_promising(
	export: &Export<'_>,
	callable: &str,
	parameters: &str,
	arguments: &str,
) -> String {
	let prepares = render_prepares(&export.inputs, "        ");
	let then = export.output.as_ref().map_or_else(String::new, |output| {
		if output.js_conversion().is_none() && output.result().is_none() {
			String::new()
		} else {
			format!(
				".then(ret => {{\n{}\n    }})",
				render_output(output, "        ")
			)
		}
	});

	if prepares.is_empty() {
		format!(
			"(() => {{\n    const $promising = {callable}\n    return ({parameters}) => $promising({arguments}){then}\n}})()"
		)
	} else {
		format!(
			"(() => {{\n    const $promising = {callable}\n    return ({parameters}) => {{\n{prepares}        return $promising({arguments}){then}\n    }}\n}})()"
		)
	}
}

fn is_passthrough(export: &Export<'_>) -> bool {
	!export.inputs.iter().any(|input| input.conversion.is_some())
		&& export
			.output
			.as_ref()
			.is_none_or(|output| output.js_conversion().is_none() && output.result().is_none())
}

fn render_arguments(inputs: &[ExportInput<'_>]) -> String {
	let mut arguments = String::new();
	let mut wrote_argument = false;
	for input in inputs {
		if let Some(conversion) = input.conversion.as_ref() {
			for expression in &conversion.expressions {
				if wrote_argument {
					arguments.push_str(", ");
				}
				arguments.push_str(&render_input_template(expression, input.name));
				wrote_argument = true;
			}
		} else if !input.slots.is_empty() {
			if wrote_argument {
				arguments.push_str(", ");
			}
			arguments.push_str(input.name);
			wrote_argument = true;
		}
	}
	arguments
}

fn render_prepares(inputs: &[ExportInput<'_>], indent: &str) -> String {
	inputs
		.iter()
		.filter_map(|input| {
			let prepare = input
				.conversion
				.as_ref()
				.and_then(|conversion| conversion.prepare)
				.filter(|prepare| !prepare.is_empty())?;
			Some(format!(
				"{indent}const {}$prepared = {}\n",
				input.name,
				render_prepare_template(prepare, input.name)
			))
		})
		.collect()
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
		|template| render_output_template(template, output),
	);
	format!("{result}{indent}return {expression}")
}

fn render_input_template(template: &str, name: &str) -> String {
	render_template(template, |rendered, placeholder| match placeholder {
		Placeholder::Value => rendered.push_str(name),
		Placeholder::Prepared => {
			rendered.push_str(name);
			rendered.push_str("$prepared");
		}
		Placeholder::Slot(_) => {}
	})
}

fn render_prepare_template(template: &str, value: &str) -> String {
	render_template(template, |rendered, placeholder| {
		if placeholder == Placeholder::Value {
			rendered.push_str(value);
		}
	})
}

fn render_output_template(template: &str, output: &ExportOutput<'_>) -> String {
	render_template(template, |rendered, placeholder| {
		if let Placeholder::Slot(slot) = placeholder {
			rendered.push_str(&render_output_slot(output, slot));
		}
	})
}

fn render_output_slot(output: &ExportOutput<'_>, slot: usize) -> String {
	if let Some(result) = output.result() {
		if slot < usize::from(result.discriminant) {
			return render_ret(slot);
		}
	} else if output.is_direct() {
		if slot == 0 {
			return "ret".to_owned();
		}
	} else {
		return render_ret(slot);
	}
	String::new()
}

fn render_ret(index: usize) -> String {
	format!("ret[{index}]")
}
