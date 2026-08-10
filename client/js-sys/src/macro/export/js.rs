use core::marker::PhantomData;

use super::Writer;
use crate::hazard::{FromJS, ReturnIntoJS};
use crate::r#macro::text::{JS_TEMPLATE_PLACEHOLDERS, js_template_placeholder};
use crate::r#macro::{
	WatSlot, from_js_wat_slots, js_export_output_template, js_from_has_conversion, js_from_prepare,
	js_from_templates, js_return_has_conversion, return_into_js_is_direct,
	return_into_js_is_result, return_into_js_wat_slots, validate_from_js, validate_return_into_js,
};

/// All target-dependent information needed to render one exported argument.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ExportInput {
	name: &'static str,
	ty: &'static ExportInputType,
}

#[derive(Clone, Copy)]
struct ExportInputType {
	slots: [WatSlot; 4],
	prepare: &'static str,
	templates: [&'static str; 4],
	has_conversion: bool,
}

/// All target-dependent information needed to render one exported result.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ExportOutput {
	slots: [WatSlot; 4],
	template: &'static str,
	direct: bool,
	result: bool,
	has_conversion: bool,
}

/// Selects the JavaScript export wrapper generated for a descriptor.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum ExportMode {
	Sync,
	Promising,
}

/// A semantic description of one JavaScript export.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ExportDescriptor {
	name: &'static str,
	inputs: &'static [ExportInput],
	output: Option<&'static ExportOutput>,
	mode: ExportMode,
}

struct ExportInputMetadata<T>(PhantomData<T>);

impl<T: FromJS> ExportInputMetadata<T> {
	const VALUE: ExportInputType = {
		validate_from_js::<T>();

		ExportInputType {
			slots: from_js_wat_slots::<T>(),
			prepare: js_from_prepare::<T>(),
			templates: js_from_templates::<T>(),
			has_conversion: js_from_has_conversion::<T>(),
		}
	};
}

struct ExportOutputMetadata<T>(PhantomData<T>);

impl<T: ReturnIntoJS> ExportOutputMetadata<T> {
	const VALUE: ExportOutput = {
		validate_return_into_js::<T>();

		ExportOutput {
			slots: return_into_js_wat_slots::<T>(),
			template: js_export_output_template::<T>(),
			direct: return_into_js_is_direct::<T>(),
			result: return_into_js_is_result::<T>(),
			has_conversion: js_return_has_conversion::<T>(),
		}
	};
}

/// Builds the descriptor for one exported argument.
#[doc(hidden)]
#[must_use]
pub const fn export_input<T: FromJS>(name: &'static str) -> ExportInput {
	ExportInput {
		name,
		ty: &ExportInputMetadata::<T>::VALUE,
	}
}

/// Returns the descriptor for an exported result.
#[doc(hidden)]
#[must_use]
pub const fn export_output<T: ReturnIntoJS>() -> &'static ExportOutput {
	&ExportOutputMetadata::<T>::VALUE
}

impl ExportDescriptor {
	/// Creates an export descriptor.
	#[doc(hidden)]
	#[must_use]
	pub const fn new(
		name: &'static str,
		inputs: &'static [ExportInput],
		output: Option<&'static ExportOutput>,
		mode: ExportMode,
	) -> Self {
		Self {
			name,
			inputs,
			output,
			mode,
		}
	}

	const fn is_passthrough(&self) -> bool {
		let mut input = 0;
		while input < self.inputs.len() {
			if self.inputs[input].ty.has_conversion {
				return false;
			}
			input += 1;
		}

		match self.output {
			Some(output) => !output.needs_postprocess(),
			None => true,
		}
	}

	const fn has_prepares(&self) -> bool {
		let mut input = 0;
		while input < self.inputs.len() {
			if !self.inputs[input].ty.prepare.is_empty() {
				return true;
			}
			input += 1;
		}

		false
	}

	const fn write<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		if self.is_passthrough() {
			self.write_raw(writer);
			return;
		}

		match self.mode {
			ExportMode::Sync => self.write_sync(writer),
			ExportMode::Promising => self.write_promising(writer),
		}
	}

	const fn write_sync<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		writer.write_byte(b'(');
		self.write_parameters(writer);
		writer.write_str(") => {\n");
		self.write_prepares(writer, "    ");

		if let Some(output) = self.output {
			writer.write_str("    const ret = ");
			self.write_raw_call(writer);
			writer.write_byte(b'\n');
			output.write_result_throw(writer, "    ");
			writer.write_str("    return ");
			output.write_expression(writer);
			writer.write_str("\n}");
		} else {
			writer.write_str("    ");
			self.write_raw_call(writer);
			writer.write_str("\n}");
		}
	}

	const fn write_promising<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		writer.write_str("(() => {\n    const $promising = ");
		self.write_raw(writer);
		writer.write_str("\n    return (");
		self.write_parameters(writer);

		if self.has_prepares() {
			writer.write_str(") => {\n");
			self.write_prepares(writer, "        ");
			writer.write_str("        return $promising(");
			self.write_arguments(writer);
			writer.write_byte(b')');
			self.write_promising_then(writer);
			writer.write_str("\n    }\n})()");
		} else {
			writer.write_str(") => $promising(");
			self.write_arguments(writer);
			writer.write_byte(b')');
			self.write_promising_then(writer);
			writer.write_str("\n})()");
		}
	}

	const fn write_promising_then<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		let Some(output) = self.output else {
			return;
		};
		if !output.needs_postprocess() {
			return;
		}

		writer.write_str(".then(ret => {\n");
		output.write_result_throw(writer, "        ");
		writer.write_str("        return ");
		output.write_expression(writer);
		writer.write_str("\n    })");
	}

	const fn write_raw<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		if matches!(self.mode, ExportMode::Promising) {
			writer.write_str("WebAssembly.promising(");
		}

		writer.write_str("wasmExports['");
		writer.write_str(self.name);
		writer.write_str("']");

		if matches!(self.mode, ExportMode::Promising) {
			writer.write_byte(b')');
		}
	}

	const fn write_raw_call<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		self.write_raw(writer);
		writer.write_byte(b'(');
		self.write_arguments(writer);
		writer.write_byte(b')');
	}

	const fn write_parameters<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		let mut input = 0;
		while input < self.inputs.len() {
			if input != 0 {
				writer.write_str(", ");
			}
			writer.write_str(self.inputs[input].name);
			input += 1;
		}
	}

	const fn write_arguments<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		let mut wrote_argument = false;
		let mut input = 0;

		while input < self.inputs.len() {
			let descriptor = &self.inputs[input];
			let mut slot = 0;

			while slot < descriptor.ty.slots.len() {
				if !descriptor.ty.slots[slot].abi.is_empty() {
					if wrote_argument {
						writer.write_str(", ");
					}
					write_template(
						writer,
						descriptor.ty.templates[slot],
						TemplateValues::Input(descriptor.name),
					);
					wrote_argument = true;
				}
				slot += 1;
			}

			input += 1;
		}
	}

	const fn write_prepares<const LEN: usize>(&self, writer: &mut Writer<LEN>, indent: &str) {
		let mut input = 0;
		while input < self.inputs.len() {
			let descriptor = &self.inputs[input];
			if !descriptor.ty.prepare.is_empty() {
				writer.write_str(indent);
				writer.write_str("const ");
				writer.write_str(descriptor.name);
				writer.write_str("$prepared = ");
				write_template(
					writer,
					descriptor.ty.prepare,
					TemplateValues::Value(descriptor.name),
				);
				writer.write_byte(b'\n');
			}
			input += 1;
		}
	}
}

impl ExportOutput {
	const fn needs_postprocess(&self) -> bool {
		self.has_conversion || self.result
	}

	const fn write_expression<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		write_template(
			writer,
			self.template,
			TemplateValues::Output {
				direct: self.direct,
				result: self.result,
			},
		);
	}

	const fn write_result_throw<const LEN: usize>(&self, writer: &mut Writer<LEN>, indent: &str) {
		if !self.result {
			return;
		}

		let discriminant = if self.slots[0].abi.is_empty() {
			0
		} else if self.slots[1].abi.is_empty() {
			1
		} else {
			2
		};

		writer.write_str(indent);
		writer.write_str("if (");
		write_ret_index(writer, discriminant);
		writer.write_str(" !== 0) throw ");
		write_ret_index(writer, discriminant + 1);
		writer.write_byte(b'\n');
	}
}

/// Returns the exact byte length produced by [`render_export_js`].
#[doc(hidden)]
#[must_use]
pub const fn export_js_len(descriptor: &ExportDescriptor) -> usize {
	let mut writer = Writer::<0>::new();
	descriptor.write(&mut writer);
	writer.len()
}

/// Renders one JavaScript export into an exact-size byte array.
#[doc(hidden)]
#[must_use]
pub const fn render_export_js<const LEN: usize>(descriptor: &ExportDescriptor) -> [u8; LEN] {
	let mut writer = Writer::<LEN>::new();
	descriptor.write(&mut writer);
	assert!(writer.len() == LEN);
	writer.finish_padded()
}

#[derive(Clone, Copy)]
enum TemplateValues<'a> {
	Input(&'a str),
	Value(&'a str),
	Output { direct: bool, result: bool },
}

const fn write_template<const LEN: usize>(
	writer: &mut Writer<LEN>,
	template: &str,
	values: TemplateValues<'_>,
) {
	let bytes = template.as_bytes();
	let mut input = 0;

	while input < bytes.len() {
		let placeholder = js_template_placeholder(bytes, input);

		if placeholder == 0 {
			match values {
				TemplateValues::Input(value) | TemplateValues::Value(value) => {
					writer.write_str(value);
				}
				TemplateValues::Output { .. } => {}
			}
			input += JS_TEMPLATE_PLACEHOLDERS[placeholder].len();
		} else if placeholder == 1 {
			if let TemplateValues::Input(name) = values {
				writer.write_str(name);
				writer.write_str("$prepared");
			}
			input += JS_TEMPLATE_PLACEHOLDERS[placeholder].len();
		} else if placeholder < JS_TEMPLATE_PLACEHOLDERS.len() {
			if let TemplateValues::Output { direct, result } = values {
				write_output_slot(writer, direct, result, placeholder - 2);
			}
			input += JS_TEMPLATE_PLACEHOLDERS[placeholder].len();
		} else {
			let start = input;
			input += 1;
			while input < bytes.len() && bytes[input] != b'$' {
				input += 1;
			}
			writer.write_str_range(template, start, input);
		}
	}
}

const fn write_output_slot<const LEN: usize>(
	writer: &mut Writer<LEN>,
	direct: bool,
	result: bool,
	slot: usize,
) {
	if result {
		if slot < 2 {
			write_ret_index(writer, slot);
		}
	} else if direct {
		if slot == 0 {
			writer.write_str("ret");
		}
	} else {
		write_ret_index(writer, slot);
	}
}

const fn write_ret_index<const LEN: usize>(writer: &mut Writer<LEN>, index: usize) {
	assert!(index < 4);
	writer.write_str("ret[");
	writer.write_byte(b"0123"[index]);
	writer.write_byte(b']');
}
