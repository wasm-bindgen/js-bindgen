mod js;

pub use js::*;

use super::writer::Writer;

// WAT shim generation.

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_direct {
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {
		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_imports!(
				slots = [
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
				],
				extras = [],
			),
			"\n(import \"env\" \"raw\" (func $raw (@sym (name \"",
			$raw,
			"\"))",
			$($crate::r#macro::wat_input!(export raw_param; $input),)*
			"))\n",
			"(func $export (@sym (name \"",
			$export,
			"\"))",
			$($crate::r#macro::wat_input!(export params; $par, $input),)*
			$crate::r#macro::wat_locals!(
				slots = [
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
				],
				extras = [],
			),
			"\n",
			$($crate::r#macro::wat_input!(export gets; $par, $input),)*
			"  call $raw (@reloc)\n",
			")"
		)
	};
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const SLOT: $crate::r#macro::WatSlot =
			$crate::r#macro::return_into_js_wat_slots::<$output>()[0];

		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_imports!(
				slots = [
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			"\n(import \"env\" \"raw\" (func $raw (@sym (name \"",
			$raw,
			"\"))",
			$($crate::r#macro::wat_input!(export raw_param; $input),)*
			" (result ",
			SLOT.abi,
			")))\n",
			"(func $export (@sym (name \"",
			$export,
			"\"))",
			$($crate::r#macro::wat_input!(export params; $par, $input),)*
			" (result ",
			SLOT.boundary,
			")",
			$crate::r#macro::wat_locals!(
				slots = [
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			"\n",
			$($crate::r#macro::wat_input!(export gets; $par, $input),)*
			"  call $raw (@reloc)",
			$crate::r#macro::wat_conv_prefix(SLOT.conv),
			SLOT.conv,
			"\n)"
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_indirect {
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const POINTER: &::core::primitive::str = $crate::r#macro::wat_pointer_type();
		const SIZE: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_frame_size::<$output>()
		);
		const RESULT_TYPES: &::core::primitive::str =
			$crate::r#macro::wat_slots!(
				types,
				$crate::r#macro::return_into_js_wat_slots::<$output>(),
				boundary,
			);

		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_imports!(
				slots = [
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			"\n(import \"env\" \"raw\" (func $raw (@sym (name \"",
			$raw,
			"\")) (param ",
			POINTER,
			")",
			$($crate::r#macro::wat_input!(export raw_param; $input),)*
			"))\n",
			"(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut ",
			POINTER,
			")))\n",
			"(func $export (@sym (name \"",
			$export,
			"\"))",
			$($crate::r#macro::wat_input!(export params; $par, $input),)*
			" (result ",
			RESULT_TYPES,
			")\n",
			"  (local $retptr ",
			POINTER,
			")",
			$crate::r#macro::wat_locals!(
				slots = [
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			"\n",
			"  global.get $__stack_pointer\n  ",
			POINTER,
			".const ",
			SIZE,
			"\n  ",
			POINTER,
			".sub\n  local.tee $retptr\n  global.set $__stack_pointer\n",
			"  local.get $retptr\n",
			$($crate::r#macro::wat_input!(export gets; $par, $input),)*
			"  call $raw (@reloc)\n",
			$crate::r#macro::wat_slots!(
				loads,
				$output,
				$crate::r#macro::return_into_js_wat_slots::<$output>(),
			),
			"  local.get $retptr\n  ",
			POINTER,
			".const ",
			SIZE,
			"\n  ",
			POINTER,
			".add\n  global.set $__stack_pointer\n)"
		)
	}};
}

/// Generates the complete WAT shim for one Rust export.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_export {
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		$($crate::r#macro::validate_from_js::<$input>();)*

		$crate::r#macro::wat_export_direct!($raw, $export, ($(($par, $input)),*))
	}};
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		$($crate::r#macro::validate_from_js::<$input>();)*
		$crate::r#macro::validate_return_into_js::<$output>();

		if $crate::r#macro::return_into_js_is_direct::<$output>() {
			$crate::r#macro::wat_export_direct!(
				$raw,
				$export,
				($(($par, $input)),*),
				$output,
			)
		} else {
			$crate::r#macro::wat_export_indirect!(
				$raw,
				$export,
				($(($par, $input)),*),
				$output,
			)
		}
	}};
}

/// Generates the complete JavaScript wrapper for one Rust export.
#[doc(hidden)]
#[macro_export]
macro_rules! js_export {
	($export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		const INPUTS: &[$crate::r#macro::ExportInput] = &[
			$($crate::r#macro::export_input::<$input>($par),)*
		];
		const DESCRIPTOR: $crate::r#macro::ExportDescriptor =
			$crate::r#macro::ExportDescriptor::new(
				$export,
				INPUTS,
				::core::option::Option::None,
				$crate::r#macro::ExportMode::Sync,
			);
		const LEN: ::core::primitive::usize =
			$crate::r#macro::export_js_len(&DESCRIPTOR);
		const VALUE: [::core::primitive::u8; LEN] =
			$crate::r#macro::render_export_js::<LEN>(&DESCRIPTOR);

		// SAFETY: Rendering only concatenates and substitutes valid strings.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
	($export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const INPUTS: &[$crate::r#macro::ExportInput] = &[
			$($crate::r#macro::export_input::<$input>($par),)*
		];
		const DESCRIPTOR: $crate::r#macro::ExportDescriptor =
			$crate::r#macro::ExportDescriptor::new(
				$export,
				INPUTS,
				::core::option::Option::Some($crate::r#macro::export_output::<$output>()),
				$crate::r#macro::ExportMode::Sync,
			);
		const LEN: ::core::primitive::usize =
			$crate::r#macro::export_js_len(&DESCRIPTOR);
		const VALUE: [::core::primitive::u8; LEN] =
			$crate::r#macro::render_export_js::<LEN>(&DESCRIPTOR);

		// SAFETY: Rendering only concatenates and substitutes valid strings.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
}

/// Generates a JavaScript wrapper for a Wasm export marked as `promising`.
#[doc(hidden)]
#[macro_export]
macro_rules! js_export_promising {
	($export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		const INPUTS: &[$crate::r#macro::ExportInput] = &[
			$($crate::r#macro::export_input::<$input>($par),)*
		];
		const DESCRIPTOR: $crate::r#macro::ExportDescriptor =
			$crate::r#macro::ExportDescriptor::new(
				$export,
				INPUTS,
				::core::option::Option::None,
				$crate::r#macro::ExportMode::Promising,
			);
		const LEN: ::core::primitive::usize =
			$crate::r#macro::export_js_len(&DESCRIPTOR);
		const VALUE: [::core::primitive::u8; LEN] =
			$crate::r#macro::render_export_js::<LEN>(&DESCRIPTOR);

		// SAFETY: Rendering only concatenates and substitutes valid strings.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
	($export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const INPUTS: &[$crate::r#macro::ExportInput] = &[
			$($crate::r#macro::export_input::<$input>($par),)*
		];
		const DESCRIPTOR: $crate::r#macro::ExportDescriptor =
			$crate::r#macro::ExportDescriptor::new(
				$export,
				INPUTS,
				::core::option::Option::Some($crate::r#macro::export_output::<$output>()),
				$crate::r#macro::ExportMode::Promising,
			);
		const LEN: ::core::primitive::usize =
			$crate::r#macro::export_js_len(&DESCRIPTOR);
		const VALUE: [::core::primitive::u8; LEN] =
			$crate::r#macro::render_export_js::<LEN>(&DESCRIPTOR);

		// SAFETY: Rendering only concatenates and substitutes valid strings.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
}
