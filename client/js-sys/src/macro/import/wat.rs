use super::{Capacity, ImportDescriptor, ImportOutput, WatInputCapacity, Writer};
use crate::r#macro::WatSlot;

pub(super) const fn descriptor_capacity(descriptor: &ImportDescriptor) -> usize {
	let mut capacity = Capacity::new();

	capacity.add_str("(import \"");
	capacity.add_str(descriptor.module);
	capacity.add_str("\" \"");
	capacity.add_str(descriptor.import);
	capacity.add_str("\" (func $");
	add_import_name_capacity(&mut capacity, descriptor);
	capacity.add_str(" (@sym (name \"");
	add_import_name_capacity(&mut capacity, descriptor);
	capacity.add_str("\"))");

	if !input_types_are_empty(descriptor.inputs) {
		capacity.add_str(" (param ");
		capacity.add(descriptor.inputs.len() - 1);
		capacity.add(1);
	}

	capacity.add_str("))");
	capacity.add_str("\n(func $");
	capacity.add_str(descriptor.shim);
	capacity.add_str(" (@sym)");

	let mut input = 0;
	while input < descriptor.inputs.len() {
		let argument = &descriptor.inputs[input];
		capacity.add(argument.ty.wat_capacity.fixed);
		capacity.add_repeated_str(argument.name, argument.ty.wat_capacity.name_uses);
		input += 1;
	}

	if let Some(output) = descriptor.output {
		capacity.add(output.wat_capacity);
	}

	capacity.add_str("\n  call $");
	add_import_name_capacity(&mut capacity, descriptor);
	capacity.add_str(" (@reloc)");
	capacity.add_str("\n)");
	capacity.get()
}

pub(super) const fn input_capacity(slots: &[WatSlot; 4]) -> WatInputCapacity {
	let mut capacity = Capacity::new();
	let mut name_uses = 0;
	let mut slot = 0;
	let mut wrote_type = false;
	let mut wrote_get = false;

	while slot < slots.len() {
		let descriptor = &slots[slot];

		if !descriptor.boundary.is_empty() {
			if wrote_type {
				capacity.add(1);
			}
			capacity.add_str(descriptor.boundary);
			wrote_type = true;
		}

		if !descriptor.abi.is_empty() {
			capacity.add_str(" (param $");
			capacity.add_str(slot_suffix(slot));
			capacity.add_str(descriptor.abi);
			capacity.add(1);
			name_uses += 1;

			if wrote_get {
				capacity.add(1);
			}
			capacity.add_str("  local.get $");
			capacity.add_str(get_suffix(slot));
			add_conversion_capacity(&mut capacity, descriptor.conv);
			name_uses += 1;
			wrote_get = true;
		}

		capacity.add_wat_lines(descriptor.imports);
		capacity.add_wat_lines(descriptor.locals);
		slot += 1;
	}

	// `write_wat` starts every input's local-get sequence on a new line,
	// including a zero-slot input.
	capacity.add(1);
	WatInputCapacity {
		fixed: capacity.get(),
		name_uses,
	}
}

pub(super) const fn output_capacity(output: &ImportOutput) -> usize {
	let mut capacity = Capacity::new();

	if output.direct {
		capacity.add_str(" (result ");
		capacity.add_str(output.slots[0].boundary);
		capacity.add(1);
		capacity.add_str(" (result ");
		capacity.add_str(output.slots[0].abi);
		capacity.add(1);
		capacity.add_wat_lines(output.slots[0].imports);
		capacity.add_wat_lines(output.slots[0].locals);

		if !output.slots[0].conv.is_empty() {
			capacity.add_str("\n  ");
			capacity.add_str(output.slots[0].conv);
		}
	} else {
		capacity.add_str(" (param $retptr ");
		capacity.add_str(output.pointer.boundary);
		capacity.add(1);
		capacity.add_str(" (param $retptr ");
		capacity.add_str(output.pointer.abi);
		capacity.add(1);
		capacity.add_wat_lines(output.pointer.locals);
		capacity.add_str("\n  local.get $retptr");
		add_conversion_capacity(&mut capacity, output.pointer.conv);
	}

	capacity.add_wat_lines(output.wat_result_imports);
	capacity.add_wat_lines(output.wat_result_locals);
	capacity.add_str(output.wat_result_try);
	capacity.add_str(output.wat_result_catch);
	capacity.add_str(output.wat_result_default);
	capacity.get()
}

const fn add_import_name_capacity(capacity: &mut Capacity, descriptor: &ImportDescriptor) {
	capacity.add_str(descriptor.module);
	capacity.add_str(".import.");
	capacity.add_str(descriptor.import);
}

const fn add_conversion_capacity(capacity: &mut Capacity, conversion: &str) {
	if !conversion.is_empty() {
		capacity.add_str("\n  ");
		capacity.add_str(conversion);
	}
}

pub(super) const fn write_wat_support_imports<const LEN: usize>(
	writer: &mut Writer<LEN>,
	descriptors: &[ImportDescriptor],
) {
	let mut descriptor_index = 0;
	let mut source_index = 0;

	while descriptor_index < descriptors.len() {
		let descriptor = &descriptors[descriptor_index];
		let source_count = descriptor.unique_source_count();

		while source_index < source_count && descriptor.unique_source(true, source_index).is_empty()
		{
			source_index += 1;
		}

		if source_index < source_count {
			break;
		}

		descriptor_index += 1;
		source_index = 0;
	}

	if descriptor_index == descriptors.len() {
		return;
	}

	let mut seen = [SeenLine::EMPTY; 32];
	let mut seen_len = 0;

	while descriptor_index < descriptors.len() {
		let descriptor = &descriptors[descriptor_index];
		let source_count = descriptor.unique_source_count();

		while source_index < source_count {
			let source = descriptor.unique_source(true, source_index);
			let bytes = source.as_bytes();
			let mut line_start = 0;

			while line_start < bytes.len() {
				let line_end = line_end(source, line_start);

				if line_end != line_start {
					let mut was_seen = false;
					let mut seen_index = 0;

					while seen_index < seen_len {
						let candidate = seen[seen_index];

						if lines_equal(
							source,
							line_start,
							line_end,
							candidate.source,
							candidate.start,
							candidate.end,
						) {
							was_seen = true;
							break;
						}

						seen_index += 1;
					}

					if !was_seen && seen_len == seen.len() {
						was_seen = previous_line_was_seen(
							descriptors,
							descriptor_index,
							source_index,
							line_start,
							line_end,
						);
					}

					if !was_seen {
						writer.write_byte(b'\n');
						writer.write_str_range(source, line_start, line_end);

						if seen_len < seen.len() {
							seen[seen_len] = SeenLine {
								source,
								start: line_start,
								end: line_end,
							};
							seen_len += 1;
						}
					}
				}

				line_start = line_end + 1;
			}

			source_index += 1;
		}

		descriptor_index += 1;
		source_index = 0;
	}
}

const fn previous_line_was_seen(
	descriptors: &[ImportDescriptor],
	descriptor_index: usize,
	source_index: usize,
	line_start: usize,
	current_line_end: usize,
) -> bool {
	let value = descriptors[descriptor_index].unique_source(true, source_index);
	let mut candidate_descriptor_index = 0;

	while candidate_descriptor_index <= descriptor_index {
		let descriptor = &descriptors[candidate_descriptor_index];
		let source_count = if candidate_descriptor_index == descriptor_index {
			source_index + 1
		} else {
			descriptor.unique_source_count()
		};
		let mut candidate_source_index = 0;

		while candidate_source_index < source_count {
			let candidate = descriptor.unique_source(true, candidate_source_index);
			let limit = if candidate_descriptor_index == descriptor_index
				&& candidate_source_index == source_index
			{
				line_start
			} else {
				candidate.len()
			};
			let mut candidate_start = 0;

			while candidate_start < limit {
				let candidate_end = line_end(candidate, candidate_start);

				if candidate_end != candidate_start
					&& lines_equal(
						value,
						line_start,
						current_line_end,
						candidate,
						candidate_start,
						candidate_end,
					) {
					return true;
				}

				candidate_start = candidate_end + 1;
			}

			candidate_source_index += 1;
		}

		candidate_descriptor_index += 1;
	}

	false
}

impl ImportDescriptor {
	pub(super) const fn write_wat_boundary_import<const LEN: usize>(
		&self,
		writer: &mut Writer<LEN>,
	) {
		writer.write_str("(import \"");
		writer.write_str(self.module);
		writer.write_str("\" \"");
		writer.write_str(self.import);
		writer.write_str("\" (func $");
		self.write_import_name(writer);
		writer.write_str(" (@sym (name \"");
		self.write_import_name(writer);
		writer.write_str("\"))");

		if let Some(output) = self.output
			&& !output.direct
		{
			writer.write_str(" (param $retptr ");
			writer.write_str(output.pointer.boundary);
			writer.write_byte(b')');
		}

		if !input_types_are_empty(self.inputs) {
			writer.write_str(" (param ");
			write_input_types(writer, self.inputs);
			writer.write_byte(b')');
		}

		if let Some(output) = self.output
			&& output.direct
		{
			writer.write_str(" (result ");
			writer.write_str(output.slots[0].boundary);
			writer.write_byte(b')');
		}

		writer.write_str("))");
	}

	pub(super) const fn write_wat_shim<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		writer.write_str("\n(func $");
		writer.write_str(self.shim);
		writer.write_str(" (@sym)");

		if let Some(output) = self.output
			&& !output.direct
		{
			writer.write_str(" (param $retptr ");
			writer.write_str(output.pointer.abi);
			writer.write_byte(b')');
		}

		let mut input = 0;
		while input < self.inputs.len() {
			write_input_params(
				writer,
				self.inputs[input].name,
				&self.inputs[input].ty.slots,
			);
			input += 1;
		}

		if let Some(output) = self.output
			&& output.direct
		{
			writer.write_str(" (result ");
			writer.write_str(output.slots[0].abi);
			writer.write_byte(b')');
		}

		self.write_unique_lines(writer, false);

		if let Some(output) = self.output {
			writer.write_str(output.wat_result_try);

			if !output.direct {
				writer.write_str("\n  local.get $retptr");
				write_conversion(writer, output.pointer.conv);
			}
		}

		input = 0;
		while input < self.inputs.len() {
			writer.write_byte(b'\n');
			write_input_gets(
				writer,
				self.inputs[input].name,
				&self.inputs[input].ty.slots,
			);
			input += 1;
		}

		writer.write_str("\n  call $");
		self.write_import_name(writer);
		writer.write_str(" (@reloc)");

		if let Some(output) = self.output {
			if output.direct && !output.slots[0].conv.is_empty() {
				writer.write_str("\n  ");
				writer.write_str(output.slots[0].conv);
			}

			writer.write_str(output.wat_result_catch);
			writer.write_str(output.wat_result_default);
		}

		writer.write_str("\n)");
	}

	const fn write_import_name<const LEN: usize>(&self, writer: &mut Writer<LEN>) {
		writer.write_str(self.module);
		writer.write_str(".import.");
		writer.write_str(self.import);
	}

	/// Writes the newline-delimited union used by the old `wat_imports!` and
	/// `wat_locals!` macros. A line is emitted only at its first occurrence,
	/// preserving both the original source order and its leading newline.
	const fn write_unique_lines<const LEN: usize>(&self, writer: &mut Writer<LEN>, imports: bool) {
		let source_count = self.unique_source_count();
		let mut source_index = 0;

		while source_index < source_count && self.unique_source(imports, source_index).is_empty() {
			source_index += 1;
		}

		if source_index == source_count {
			return;
		}

		let mut seen = [SeenLine::EMPTY; 8];
		let mut seen_len = 0;

		while source_index < source_count {
			let source = self.unique_source(imports, source_index);
			let bytes = source.as_bytes();
			let mut line_start = 0;

			while line_start < bytes.len() {
				let line_end = line_end(source, line_start);

				if line_end != line_start {
					let mut was_seen = false;
					let mut seen_index = 0;

					while seen_index < seen_len {
						let candidate = seen[seen_index];

						if lines_equal(
							source,
							line_start,
							line_end,
							candidate.source,
							candidate.start,
							candidate.end,
						) {
							was_seen = true;
							break;
						}

						seen_index += 1;
					}

					if !was_seen && seen_len == seen.len() {
						was_seen = self.line_was_seen(imports, source_index, line_start, line_end);
					}

					if !was_seen {
						writer.write_byte(b'\n');
						writer.write_str_range(source, line_start, line_end);

						if seen_len < seen.len() {
							seen[seen_len] = SeenLine {
								source,
								start: line_start,
								end: line_end,
							};
							seen_len += 1;
						}
					}
				}

				line_start = line_end + 1;
			}

			source_index += 1;
		}
	}

	const fn unique_source_count(&self) -> usize {
		self.inputs.len() * 4 + if self.output.is_some() { 2 } else { 0 }
	}

	const fn unique_source(&self, imports: bool, index: usize) -> &'static str {
		let input_sources = self.inputs.len() * 4;

		if index < input_sources {
			let slot = &self.inputs[index / 4].ty.slots[index % 4];
			return if imports { slot.imports } else { slot.locals };
		}

		let Some(output) = self.output else {
			return "";
		};

		match index - input_sources {
			0 => output_source(output, imports),
			1 => {
				if imports {
					output.wat_result_imports
				} else {
					output.wat_result_locals
				}
			}
			_ => "",
		}
	}

	const fn line_was_seen(
		&self,
		imports: bool,
		source_index: usize,
		line_start: usize,
		current_line_end: usize,
	) -> bool {
		let value = self.unique_source(imports, source_index);
		let mut candidate_source_index = 0;

		while candidate_source_index <= source_index {
			let candidate = self.unique_source(imports, candidate_source_index);
			let limit = if candidate_source_index == source_index {
				line_start
			} else {
				candidate.len()
			};
			let mut candidate_start = 0;

			while candidate_start < limit {
				let candidate_end = line_end(candidate, candidate_start);

				if candidate_end != candidate_start
					&& lines_equal(
						value,
						line_start,
						current_line_end,
						candidate,
						candidate_start,
						candidate_end,
					) {
					return true;
				}

				candidate_start = candidate_end + 1;
			}

			candidate_source_index += 1;
		}

		false
	}
}

#[derive(Clone, Copy)]
struct SeenLine {
	source: &'static str,
	start: usize,
	end: usize,
}

impl SeenLine {
	const EMPTY: Self = Self {
		source: "",
		start: 0,
		end: 0,
	};
}

const fn output_source(output: &ImportOutput, imports: bool) -> &'static str {
	if imports {
		if output.direct {
			output.slots[0].imports
		} else {
			""
		}
	} else if output.direct {
		output.slots[0].locals
	} else {
		output.pointer.locals
	}
}

const fn input_types_are_empty(inputs: &[super::ImportInput]) -> bool {
	if inputs.is_empty() {
		return true;
	}

	// Input groups are separated by a space, so two or more arguments always
	// produce a non-empty fragment.
	if inputs.len() > 1 {
		return false;
	}

	let slots = &inputs[0].ty.slots;
	slots[0].boundary.is_empty()
		&& slots[1].boundary.is_empty()
		&& slots[2].boundary.is_empty()
		&& slots[3].boundary.is_empty()
}

const fn write_input_types<const LEN: usize>(
	writer: &mut Writer<LEN>,
	inputs: &[super::ImportInput],
) {
	let mut input = 0;

	while input < inputs.len() {
		if input != 0 {
			writer.write_byte(b' ');
		}
		write_slot_types(writer, &inputs[input].ty.slots);
		input += 1;
	}
}

const fn write_slot_types<const LEN: usize>(writer: &mut Writer<LEN>, slots: &[WatSlot; 4]) {
	let mut slot = 0;
	let mut wrote_type = false;

	while slot < slots.len() {
		let r#type = slots[slot].boundary;

		if !r#type.is_empty() {
			if wrote_type {
				writer.write_byte(b' ');
			}
			writer.write_str(r#type);
			wrote_type = true;
		}

		slot += 1;
	}
}

const fn write_input_params<const LEN: usize>(
	writer: &mut Writer<LEN>,
	name: &str,
	slots: &[WatSlot; 4],
) {
	let mut slot = 0;

	while slot < slots.len() {
		if !slots[slot].abi.is_empty() {
			writer.write_str(" (param $");
			writer.write_str(name);
			writer.write_str(slot_suffix(slot));
			writer.write_str(slots[slot].abi);
			writer.write_byte(b')');
		}

		slot += 1;
	}
}

const fn write_input_gets<const LEN: usize>(
	writer: &mut Writer<LEN>,
	name: &str,
	slots: &[WatSlot; 4],
) {
	let mut slot = 0;
	let mut wrote_get = false;

	while slot < slots.len() {
		if !slots[slot].abi.is_empty() {
			if wrote_get {
				writer.write_byte(b'\n');
			}

			writer.write_str("  local.get $");
			writer.write_str(name);
			writer.write_str(get_suffix(slot));
			write_conversion(writer, slots[slot].conv);
			wrote_get = true;
		}

		slot += 1;
	}
}

const fn write_conversion<const LEN: usize>(writer: &mut Writer<LEN>, conversion: &str) {
	if !conversion.is_empty() {
		writer.write_str("\n  ");
		writer.write_str(conversion);
	}
}

const fn slot_suffix(slot: usize) -> &'static str {
	match slot {
		0 => "_0 ",
		1 => "_1 ",
		2 => "_2 ",
		3 => "_3 ",
		_ => panic!("a Wasm ABI has exactly four slots"),
	}
}

const fn get_suffix(slot: usize) -> &'static str {
	match slot {
		0 => "_0",
		1 => "_1",
		2 => "_2",
		3 => "_3",
		_ => panic!("a Wasm ABI has exactly four slots"),
	}
}

const fn line_end(value: &str, start: usize) -> usize {
	let bytes = value.as_bytes();
	let mut end = start;

	while end < bytes.len() && bytes[end] != b'\n' {
		end += 1;
	}

	end
}

const fn lines_equal(
	left: &str,
	left_start: usize,
	left_end: usize,
	right: &str,
	right_start: usize,
	right_end: usize,
) -> bool {
	if left_end - left_start != right_end - right_start {
		return false;
	}

	let left = left.as_bytes();
	let right = right.as_bytes();
	let mut offset = 0;

	while left_start + offset < left_end {
		if left[left_start + offset] != right[right_start + offset] {
			return false;
		}
		offset += 1;
	}

	true
}
