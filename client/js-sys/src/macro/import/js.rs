use super::{Capacity, ImportDescriptor, ImportInput, ImportJs, ImportOutput, Writer};

pub(super) const fn descriptor_capacity(descriptor: &ImportDescriptor) -> usize {
	let Some(js) = descriptor.js else {
		return 0;
	};
	let mut capacity = Capacity::new();

	capacity.add(2);
	capacity.add_str(descriptor.module);
	capacity.add(2);
	capacity.add_str(descriptor.import);
	capacity.add(1);

	let mut embed = 0;
	while embed < js.required_embeds.len() {
		let (module, name) = js.required_embeds[embed];

		capacity.add(2);
		capacity.add_str(module);
		capacity.add(2);
		capacity.add_str(name);
		embed += 1;
	}

	let wrapped = descriptor.needs_js_shim();

	if wrapped {
		capacity.add(1);

		if let Some(output) = descriptor.output
			&& !output.direct
		{
			capacity.add_str("$retptr");

			if !descriptor.inputs.is_empty() {
				capacity.add_str(", ");
			}
		}

		add_input_parameters_capacity(&mut capacity, descriptor.inputs);
		capacity.add_str(") => {\n");
	} else if js.direct_wrapper {
		capacity.add(1);
		add_input_parameters_capacity(&mut capacity, descriptor.inputs);
		capacity.add_str(") => ");
	}

	let mut input = 0;
	while input < descriptor.inputs.len() {
		add_input_conversion_capacity(&mut capacity, &descriptor.inputs[input]);
		input += 1;
	}

	match descriptor.output {
		Some(output) => add_output_capacity(&mut capacity, output, js, wrapped),
		None => {
			if wrapped {
				capacity.add_str(js.indirect_call);
				capacity.add_str("\n}");
			} else {
				capacity.add_str(js.direct_call);
			}
		}
	}

	capacity.get()
}

const fn add_input_parameters_capacity(capacity: &mut Capacity, inputs: &[ImportInput]) {
	let mut input = 0;

	while input < inputs.len() {
		if input != 0 {
			capacity.add_str(", ");
		}

		let descriptor = &inputs[input];
		add_slot_name_capacity(capacity, descriptor.name);

		let mut slot = 1;
		while slot < descriptor.ty.slots.len() {
			if !descriptor.ty.slots[slot].abi.is_empty() {
				capacity.add_str(", ");
				add_slot_name_capacity(capacity, descriptor.name);
			}
			slot += 1;
		}

		input += 1;
	}
}

const fn add_input_conversion_capacity(capacity: &mut Capacity, input: &ImportInput) {
	if !input.ty.has_js_conversion {
		return;
	}

	capacity.add_str("    ");
	add_slot_name_capacity(capacity, input.name);
	capacity.add_str(" = ");
	add_template_capacity(capacity, input.ty.js_template, "", Some(input.name));
	capacity.add(1);
}

const fn add_output_capacity(
	capacity: &mut Capacity,
	output: &ImportOutput,
	js: ImportJs,
	wrapped: bool,
) {
	let convert_direct = output.direct && output.has_js_conversion;
	let catches_result = !output.js_try.is_empty();
	let call = if wrapped {
		js.indirect_call
	} else {
		js.direct_call
	};
	let indent = if catches_result { "        " } else { "    " };

	capacity.add_str(output.js_try);

	if convert_direct {
		capacity.add_str(indent);
		capacity.add_str("const $ret = ");
	} else if output.direct {
		if catches_result {
			capacity.add_str("        return ");
		} else if wrapped {
			capacity.add_str("    return ");
		}
	} else {
		capacity.add_str(indent);
		capacity.add_str("const $ret = ");
	}

	if output.direct && !convert_direct {
		add_template_capacity(capacity, output.js_templates[0], call, None);
	} else {
		capacity.add_str(call);
	}

	if convert_direct {
		capacity.add(1);
		capacity.add_str(indent);
		capacity.add_str("return ");
		add_template_capacity(capacity, output.js_templates[0], "$ret", None);
	}

	if !output.direct {
		capacity.add(1);
		capacity.add_str(indent);
		capacity.add_str(output.js_sret);
		capacity.add(1);
		add_template_capacity(capacity, output.js_templates[0], "$ret", None);

		let mut slot = 1;
		while slot < output.js_templates.len() {
			capacity.add_str(", ");
			add_template_capacity(capacity, output.js_templates[slot], "$ret", None);
			slot += 1;
		}

		capacity.add_str(", $retptr)");
	}

	if catches_result {
		capacity.add_str(output.js_catch);
	} else if wrapped {
		capacity.add_str("\n}");
	}
}

const fn add_slot_name_capacity(capacity: &mut Capacity, name: &str) {
	capacity.add_str(name);
	capacity.add(2);
}

const fn add_template_capacity(
	capacity: &mut Capacity,
	template: &str,
	value: &str,
	slots: Option<&str>,
) {
	let mut maximum_replacement = value.len();

	if let Some(name) = slots {
		let mut slot_len = name.len();
		slot_len = match slot_len.checked_add(2) {
			Some(slot_len) => slot_len,
			None => panic!("import section capacity overflows usize"),
		};

		if slot_len > maximum_replacement {
			maximum_replacement = slot_len;
		}
	}

	// Literal bytes contribute at most `template.len()`. Every recognized
	// placeholder is six bytes long, so at most `len / 6` are replaced.
	capacity.add_str(template);
	let Some(replacements) =
		(template.len() / PLACEHOLDERS[0].len()).checked_mul(maximum_replacement)
	else {
		panic!("import section capacity overflows usize");
	};
	capacity.add(replacements);
}

impl ImportDescriptor {
	pub(super) const fn write_js_record<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		let Some(js) = self.js else {
			return;
		};

		writer.write_u16(self.module.len());
		writer.write_str(self.module);
		writer.write_u16(self.import.len());
		writer.write_str(self.import);

		assert!(js.required_embeds.len() <= u8::MAX as usize);
		writer.write_byte(js.required_embeds.len().to_le_bytes()[0]);

		let mut embed = 0;
		while embed < js.required_embeds.len() {
			let (module, name) = js.required_embeds[embed];

			writer.write_u16(module.len());
			writer.write_str(module);
			writer.write_u16(name.len());
			writer.write_str(name);
			embed += 1;
		}

		self.write_js(writer, js);
	}

	const fn write_js<const LEN: usize>(&self, writer: &mut Writer<LEN>, js: ImportJs) {
		let wrapped = self.needs_js_shim();

		if wrapped {
			writer.write_byte(b'(');

			if let Some(output) = self.output
				&& !output.direct
			{
				writer.write_str("$retptr");

				if !self.inputs.is_empty() {
					writer.write_str(", ");
				}
			}

			write_input_parameters(writer, self.inputs);
			writer.write_str(") => {\n");
		} else if js.direct_wrapper {
			writer.write_byte(b'(');
			write_input_parameters(writer, self.inputs);
			writer.write_str(") => ");
		}

		let mut input = 0;
		while input < self.inputs.len() {
			write_input_conversion(writer, &self.inputs[input]);
			input += 1;
		}

		match self.output {
			Some(output) => write_output(writer, output, js, wrapped),
			None => {
				if wrapped {
					writer.write_str(js.indirect_call);
					writer.write_str("\n}");
				} else {
					writer.write_str(js.direct_call);
				}
			}
		}
	}
}

const fn write_input_parameters<const LEN: usize>(
	writer: &mut Writer<LEN>,
	inputs: &[ImportInput],
) {
	let mut input = 0;

	while input < inputs.len() {
		if input != 0 {
			writer.write_str(", ");
		}

		write_input_parameter(writer, &inputs[input]);
		input += 1;
	}
}

const fn write_input_parameter<const LEN: usize>(writer: &mut Writer<LEN>, input: &ImportInput) {
	write_slot_name(writer, input.name, 0);

	let mut slot = 1;
	while slot < input.ty.slots.len() {
		if !input.ty.slots[slot].abi.is_empty() {
			writer.write_str(", ");
			write_slot_name(writer, input.name, slot);
		}

		slot += 1;
	}
}

const fn write_input_conversion<const LEN: usize>(writer: &mut Writer<LEN>, input: &ImportInput) {
	if !input.ty.has_js_conversion {
		return;
	}

	writer.write_str("    ");
	write_slot_name(writer, input.name, 0);
	writer.write_str(" = ");
	write_template(writer, input.ty.js_template, "", Some(input.name));
	writer.write_byte(b'\n');
}

const fn write_output<const LEN: usize>(
	writer: &mut Writer<LEN>,
	output: &ImportOutput,
	js: ImportJs,
	wrapped: bool,
) {
	let convert_direct = output.direct && output.has_js_conversion;
	let catches_result = !output.js_try.is_empty();
	let call = if wrapped {
		js.indirect_call
	} else {
		js.direct_call
	};
	let template_value = if output.direct && !convert_direct {
		call
	} else {
		"$ret"
	};
	let indent = if catches_result { "        " } else { "    " };

	writer.write_str(output.js_try);

	if convert_direct {
		writer.write_str(indent);
		writer.write_str("const $ret = ");
	} else if output.direct {
		if catches_result {
			writer.write_str("        return ");
		} else if wrapped {
			writer.write_str("    return ");
		}
	} else {
		writer.write_str(indent);
		writer.write_str("const $ret = ");
	}

	if output.direct && !convert_direct {
		write_template(writer, output.js_templates[0], template_value, None);
	} else {
		writer.write_str(call);
	}

	if convert_direct {
		writer.write_byte(b'\n');
		writer.write_str(indent);
		writer.write_str("return ");
		write_template(writer, output.js_templates[0], "$ret", None);
	}

	if !output.direct {
		writer.write_byte(b'\n');
		writer.write_str(indent);
		writer.write_str(output.js_sret);
		writer.write_byte(b'(');
		write_template(writer, output.js_templates[0], "$ret", None);

		let mut slot = 1;
		while slot < output.js_templates.len() {
			if template_len(output.js_templates[slot], "$ret", None) != 0 {
				writer.write_str(", ");
				write_template(writer, output.js_templates[slot], "$ret", None);
			}

			slot += 1;
		}

		writer.write_str(", $retptr)");
	}

	if catches_result {
		writer.write_str(output.js_catch);
	} else if wrapped {
		writer.write_str("\n}");
	}
}

const fn write_slot_name<const LEN: usize>(writer: &mut Writer<LEN>, name: &str, slot: usize) {
	assert!(slot < 4);
	writer.write_str(name);
	writer.write_byte(b'_');
	writer.write_byte(b"0123"[slot]);
}

const PLACEHOLDERS: [&[u8]; 5] = [b"$value", b"$slot1", b"$slot2", b"$slot3", b"$slot4"];

const fn write_template<const LEN: usize>(
	writer: &mut Writer<LEN>,
	template: &str,
	value: &str,
	slots: Option<&str>,
) {
	let bytes = template.as_bytes();
	let mut input = 0;

	while input < bytes.len() {
		let placeholder = template_placeholder(bytes, input);

		if placeholder == 0 {
			writer.write_str(value);
			input += PLACEHOLDERS[placeholder].len();
		} else if placeholder < PLACEHOLDERS.len() {
			if let Some(name) = slots {
				write_slot_name(writer, name, placeholder - 1);
			}

			input += PLACEHOLDERS[placeholder].len();
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

const fn template_len(template: &str, value: &str, slots: Option<&str>) -> usize {
	let bytes = template.as_bytes();
	let mut input = 0;
	let mut output = 0;

	while input < bytes.len() {
		let placeholder = template_placeholder(bytes, input);

		if placeholder == 0 {
			output += value.len();
			input += PLACEHOLDERS[placeholder].len();
		} else if placeholder < PLACEHOLDERS.len() {
			if let Some(name) = slots {
				output += name.len() + 2;
			}

			input += PLACEHOLDERS[placeholder].len();
		} else {
			let start = input;
			input += 1;

			while input < bytes.len() && bytes[input] != b'$' {
				input += 1;
			}

			output += input - start;
		}
	}

	output
}

const fn template_placeholder(template: &[u8], index: usize) -> usize {
	if template[index] != b'$' {
		return PLACEHOLDERS.len();
	}

	let mut placeholder = 0;
	while placeholder < PLACEHOLDERS.len() {
		let candidate = PLACEHOLDERS[placeholder];

		if index + candidate.len() <= template.len() {
			let mut byte = 0;
			let mut matches = true;

			while byte < candidate.len() {
				if template[index + byte] != candidate[byte] {
					matches = false;
					break;
				}

				byte += 1;
			}

			if matches {
				return placeholder;
			}
		}

		placeholder += 1;
	}

	PLACEHOLDERS.len()
}
