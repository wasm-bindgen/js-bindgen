mod js;
mod wat;

use core::marker::PhantomData;

use super::writer::Writer;
use super::{
	WatSlot, into_js_wat_slots, js_input_template, js_output_has_conversion, js_output_prepare,
	js_output_sret, js_output_templates, js_result_catch, js_result_try, return_from_js_is_direct,
	return_from_js_wat_slots, validate_into_js, validate_return_from_js, wat_result_catch,
	wat_result_default, wat_result_imports, wat_result_locals, wat_result_try,
};
use crate::hazard::{IntoJS, ReturnFromJS, Sret};

const JS_RETPTR_CONV: &str = crate::js_template!(
	js_input_template::<crate::util::PtrMut<()>>(),
	slots = ["$retptr", "", "", ""],
);

/// All target-dependent information needed to render one imported argument.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ImportInput {
	name: &'static str,
	ty: &'static ImportInputType,
}

#[derive(Clone, Copy)]
struct ImportInputType {
	slots: [WatSlot; 4],
	js_template: &'static str,
	has_js_conversion: bool,
	wat_capacity: WatInputCapacity,
}

/// All target-dependent information needed to render one imported result.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ImportOutput {
	direct: bool,
	slots: [WatSlot; 4],
	pointer: WatSlot,
	has_js_conversion: bool,
	js_prepare: &'static str,
	js_templates: [&'static str; 4],
	js_sret: Option<Sret>,
	js_try: &'static str,
	js_catch: &'static str,
	wat_result_imports: &'static str,
	wat_result_locals: &'static str,
	wat_result_try: &'static str,
	wat_result_catch: &'static str,
	wat_result_default: &'static str,
	wat_capacity: usize,
}

/// JavaScript-specific parts of an import descriptor.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ImportJs {
	pub direct_wrapper: bool,
	pub direct_call: &'static str,
	pub indirect_call: &'static str,
	pub required_embeds: &'static [(&'static str, &'static str)],
}

/// A semantic description of one JavaScript import.
///
/// Rendering is deliberately centralized in ordinary `const fn`s. Generated
/// bindings construct this value once instead of expanding a tree of string
/// concatenation macros and materializing every intermediate fragment.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ImportDescriptor {
	module: &'static str,
	import: &'static str,
	shim: &'static str,
	inputs: &'static [ImportInput],
	output: Option<&'static ImportOutput>,
	js: Option<ImportJs>,
	suspending: bool,
	wat_capacity: usize,
	js_capacity: usize,
}

/// One framed sequence of length-prefixed custom-section records.
///
/// `capacity` lets multiple padded fragments be concatenated in one custom
/// section. `used` excludes the zero-filled tail of `records`.
#[doc(hidden)]
#[repr(C)]
pub struct ImportSection<const CAPACITY: usize> {
	capacity: [u8; 4],
	used: [u8; 4],
	records: [u8; CAPACITY],
}

struct ImportInputMetadata<T>(PhantomData<T>);

#[derive(Clone, Copy)]
struct WatInputCapacity {
	fixed: usize,
	name_uses: usize,
}

impl<T: IntoJS> ImportInputMetadata<T> {
	const VALUE: ImportInputType = {
		validate_into_js::<T>();
		let slots = into_js_wat_slots::<T>();

		ImportInputType {
			slots,
			js_template: js_input_template::<T>(),
			has_js_conversion: T::JS_CONV.is_some(),
			wat_capacity: wat::input_capacity(&slots),
		}
	};
}

struct ImportOutputMetadata<T>(PhantomData<T>);

impl<T: ReturnFromJS> ImportOutputMetadata<T> {
	const VALUE: ImportOutput = {
		validate_return_from_js::<T>();
		let slots = return_from_js_wat_slots::<T>();
		let direct = return_from_js_is_direct::<T>();
		let mut output = ImportOutput {
			direct,
			slots,
			pointer: into_js_wat_slots::<crate::util::PtrMut<()>>()[0],
			has_js_conversion: js_output_has_conversion::<T>(),
			js_prepare: js_output_prepare::<T>(),
			js_templates: js_output_templates::<T>(),
			js_sret: js_output_sret::<T>(),
			js_try: js_result_try::<T>(),
			js_catch: js_result_catch::<T>(direct),
			wat_result_imports: wat_result_imports::<T>(),
			wat_result_locals: wat_result_locals::<T>(),
			wat_result_try: wat_result_try::<T>(),
			wat_result_catch: wat_result_catch::<T>(),
			wat_result_default: wat_result_default::<T>(),
			wat_capacity: 0,
		};

		output.wat_capacity = wat::output_capacity(&output);
		output
	};
}

#[doc(hidden)]
#[must_use]
pub const fn import_input<T: IntoJS>(name: &'static str) -> ImportInput {
	ImportInput {
		name,
		ty: &ImportInputMetadata::<T>::VALUE,
	}
}

#[doc(hidden)]
#[must_use]
pub const fn import_output<T: ReturnFromJS>() -> &'static ImportOutput {
	&ImportOutputMetadata::<T>::VALUE
}

impl ImportDescriptor {
	#[doc(hidden)]
	#[must_use]
	pub const fn new(
		module: &'static str,
		import: &'static str,
		shim: &'static str,
		inputs: &'static [ImportInput],
		output: Option<&'static ImportOutput>,
		js: Option<ImportJs>,
	) -> Self {
		Self::build(module, import, shim, inputs, output, js, false)
	}

	#[doc(hidden)]
	#[must_use]
	pub const fn new_suspending(
		module: &'static str,
		import: &'static str,
		shim: &'static str,
		inputs: &'static [ImportInput],
		output: Option<&'static ImportOutput>,
		js: Option<ImportJs>,
	) -> Self {
		assert!(
			js.is_some(),
			"suspending imports require a generated JavaScript binding",
		);
		if let Some(output) = output {
			assert!(
				!catches_result_in_js_from_output(output),
				"suspending Result imports require the Wasm exception-handling target feature",
			);
		}
		Self::build(module, import, shim, inputs, output, js, true)
	}

	const fn build(
		module: &'static str,
		import: &'static str,
		shim: &'static str,
		inputs: &'static [ImportInput],
		output: Option<&'static ImportOutput>,
		js: Option<ImportJs>,
		suspending: bool,
	) -> Self {
		let mut descriptor = Self {
			module,
			import,
			shim,
			inputs,
			output,
			js,
			suspending,
			wat_capacity: 0,
			js_capacity: 0,
		};

		descriptor.wat_capacity = wat::descriptor_capacity(&descriptor);
		descriptor.js_capacity = js::descriptor_capacity(&descriptor);
		descriptor
	}

	#[must_use]
	const fn needs_js_shim(&self) -> bool {
		let mut input = 0;

		while input < self.inputs.len() {
			if self.inputs[input].ty.has_js_conversion {
				return true;
			}
			input += 1;
		}

		match self.output {
			Some(output) => output.has_js_conversion || catches_result_in_js_from_output(output),
			None => false,
		}
	}

	const fn awaits_suspending_output(&self) -> bool {
		if !self.suspending {
			return false;
		}

		match self.output {
			Some(output) => output.has_js_conversion || catches_result_in_js_from_output(output),
			None => false,
		}
	}
}

/// Returns a safe upper bound for [`import_wat`].
///
/// This follows the rendering logic without scanning declaration contents, so
/// it is suitable for sizing a padded, single-pass section.
#[doc(hidden)]
#[must_use]
pub const fn import_wat_capacity(imports: &[ImportDescriptor]) -> usize {
	let mut capacity = Capacity::new();
	let mut import = 0;

	if !imports.is_empty() {
		capacity.add(4);
		capacity.add(imports.len() - 1);
	}

	while import < imports.len() {
		capacity.add(imports[import].wat_capacity);
		import += 1;
	}

	capacity.get()
}

#[doc(hidden)]
#[must_use]
pub const fn import_wat<const CAPACITY: usize>(
	imports: &[ImportDescriptor],
) -> ImportSection<CAPACITY> {
	let mut writer = Writer::<CAPACITY>::new();
	write_wat(&mut writer, imports);

	ImportSection::new(writer)
}

/// Returns a safe upper bound for [`import_js`].
///
/// This follows the rendering logic without scanning template contents, so it
/// is suitable for sizing a padded, single-pass section.
#[doc(hidden)]
#[must_use]
pub const fn import_js_capacity(imports: &[ImportDescriptor]) -> usize {
	let mut capacity = Capacity::new();
	let mut import = 0;

	while import < imports.len() {
		if imports[import].js.is_some() {
			capacity.add(4);
			capacity.add(imports[import].js_capacity);
		}
		import += 1;
	}

	capacity.get()
}

#[doc(hidden)]
#[must_use]
pub const fn import_js<const CAPACITY: usize>(
	imports: &[ImportDescriptor],
) -> ImportSection<CAPACITY> {
	let mut writer = Writer::<CAPACITY>::new();
	write_js(&mut writer, imports);

	ImportSection::new(writer)
}

const fn write_wat<const LEN: usize>(writer: &mut Writer<LEN>, imports: &[ImportDescriptor]) {
	if imports.is_empty() {
		return;
	}

	let header = writer.len();
	writer.write_u32(0);
	let start = writer.len();
	let mut import = 0;

	while import < imports.len() {
		if import != 0 {
			writer.write_byte(b'\n');
		}

		imports[import].write_wat_boundary_import(writer);
		import += 1;
	}

	wat::write_wat_support_imports(writer, imports);

	import = 0;
	while import < imports.len() {
		imports[import].write_wat_shim(writer);
		import += 1;
	}

	let record_len = writer.len() - start;
	writer.set_u32(header, record_len);
}

const fn write_js<const LEN: usize>(writer: &mut Writer<LEN>, imports: &[ImportDescriptor]) {
	let mut import = 0;

	while import < imports.len() {
		if imports[import].js.is_some() {
			let header = writer.len();
			writer.write_u32(0);
			let start = writer.len();
			imports[import].write_js_record(writer);
			let record_len = writer.len() - start;
			writer.set_u32(header, record_len);
		}
		import += 1;
	}
}

#[must_use]
const fn catches_result_in_js_from_output(output: &ImportOutput) -> bool {
	!output.js_try.is_empty()
}

pub(super) struct Capacity(usize);

impl Capacity {
	pub const fn new() -> Self {
		Self(0)
	}

	pub const fn add(&mut self, additional: usize) {
		self.0 = match self.0.checked_add(additional) {
			Some(capacity) => capacity,
			None => panic!("import section capacity overflows usize"),
		};
	}

	pub const fn add_str(&mut self, value: &str) {
		self.add(value.len());
	}

	pub const fn add_repeated_str(&mut self, value: &str, repetitions: usize) {
		let Some(additional) = value.len().checked_mul(repetitions) else {
			panic!("import section capacity overflows usize");
		};
		self.add(additional);
	}

	pub const fn add_wat_lines(&mut self, value: &str) {
		if !value.is_empty() {
			// The separators already present in `value` become the leading
			// newlines of all but its first line.
			self.add(value.len());
			self.add(1);
		}
	}

	pub const fn get(self) -> usize {
		self.0
	}
}

impl<const CAPACITY: usize> ImportSection<CAPACITY> {
	const fn new(writer: Writer<CAPACITY>) -> Self {
		assert!(CAPACITY <= u32::MAX as usize);
		let used = writer.len();
		assert!(used <= u32::MAX as usize);

		Self {
			capacity: u32_bytes(CAPACITY),
			used: u32_bytes(used),
			records: writer.finish_padded(),
		}
	}
}

const fn u32_bytes(value: usize) -> [u8; 4] {
	assert!(value <= u32::MAX as usize);
	let bytes = value.to_le_bytes();

	[bytes[0], bytes[1], bytes[2], bytes[3]]
}
