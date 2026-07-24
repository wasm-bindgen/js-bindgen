// WAT shim helpers.

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_imports {
	(($($input:ty),*) $(, $output:ty)? $(,)?) => {
		$crate::r#macro::wat_import_list!(
			$($crate::r#macro::from_js_wat_slots::<$input>()[0].import,)*
			$($crate::r#macro::from_js_wat_slots::<$input>()[1].import,)*
			$($crate::r#macro::from_js_wat_slots::<$input>()[2].import,)*
			$($crate::r#macro::from_js_wat_slots::<$input>()[3].import,)*
			$($crate::r#macro::return_into_js_wat_slots::<$output>()[0].import,)?
			$($crate::r#macro::return_into_js_wat_slots::<$output>()[1].import,)?
			$($crate::r#macro::return_into_js_wat_slots::<$output>()[2].import,)?
			$($crate::r#macro::return_into_js_wat_slots::<$output>()[3].import,)?
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_input_raw_types {
	($ty:ty $(,)?) => {
		$crate::r#macro::wat_slot_types!($crate::r#macro::from_js_wat_slots::<$ty>(), abi,)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_input_raw_param {
	($ty:ty $(,)?) => {{
		const TYPES: &::core::primitive::str =
			$crate::r#macro::wat_export_input_raw_types!($ty);

		$crate::r#macro::const_concat_if!(
			!TYPES.is_empty() => [" (param ", TYPES, ")"],
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_input_params {
	($par:literal, $ty:ty $(,)?) => {
		$crate::r#macro::wat_slot_params!(
			$par,
			$crate::r#macro::from_js_wat_slots::<$ty>(),
			boundary,
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_input_gets {
	($par:literal, $ty:ty $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] =
			$crate::r#macro::from_js_wat_slots::<$ty>();

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => ["  local.get $", $par, "_0", $crate::r#macro::wat_conv_prefix(SLOTS[0].conv), SLOTS[0].conv, "\n"],
			!SLOTS[1].abi.is_empty() => ["  local.get $", $par, "_1", $crate::r#macro::wat_conv_prefix(SLOTS[1].conv), SLOTS[1].conv, "\n"],
			!SLOTS[2].abi.is_empty() => ["  local.get $", $par, "_2", $crate::r#macro::wat_conv_prefix(SLOTS[2].conv), SLOTS[2].conv, "\n"],
			!SLOTS[3].abi.is_empty() => ["  local.get $", $par, "_3", $crate::r#macro::wat_conv_prefix(SLOTS[3].conv), SLOTS[3].conv, "\n"],
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_result_types {
	($ty:ty $(,)?) => {
		$crate::r#macro::wat_slot_types!(
			$crate::r#macro::return_into_js_wat_slots::<$ty>(),
			boundary,
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_result_loads {
	($ty:ty $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] =
			$crate::r#macro::return_into_js_wat_slots::<$ty>();
		const OFFSET_0: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 0>()
		);
		const OFFSET_1: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 1>()
		);
		const OFFSET_2: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 2>()
		);
		const OFFSET_3: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 3>()
		);

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[0].abi, ".load offset=", OFFSET_0, $crate::r#macro::wat_conv_prefix(SLOTS[0].conv), SLOTS[0].conv, "\n"],
			!SLOTS[1].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[1].abi, ".load offset=", OFFSET_1, $crate::r#macro::wat_conv_prefix(SLOTS[1].conv), SLOTS[1].conv, "\n"],
			!SLOTS[2].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[2].abi, ".load offset=", OFFSET_2, $crate::r#macro::wat_conv_prefix(SLOTS[2].conv), SLOTS[2].conv, "\n"],
			!SLOTS[3].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[3].abi, ".load offset=", OFFSET_3, $crate::r#macro::wat_conv_prefix(SLOTS[3].conv), SLOTS[3].conv, "\n"],
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_needs_shim {
	(($(($par:literal, $input:ty)),*) $(,)?) => {
		false $(|| $crate::r#macro::export_input_needs_wat_shim::<$input>())*
	};
	(($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {
		$crate::r#macro::wat_export_needs_shim!(($(($par, $input)),*))
			|| $crate::r#macro::export_output_needs_wat_shim::<$output>()
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_export_direct {
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {
		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_export_imports!(($($input),*)),
			"\n(import \"env\" \"raw\" (func $raw (@sym (name \"",
			$raw,
			"\"))",
			$($crate::r#macro::wat_export_input_raw_param!($input),)*
			"))\n",
			"(func $export (@sym (name \"",
			$export,
			"\"))",
			$($crate::r#macro::wat_export_input_params!($par, $input),)*
			"\n",
			$($crate::r#macro::wat_export_input_gets!($par, $input),)*
			"  call $raw (@reloc)\n",
			")"
		)
	};
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const SLOT: $crate::r#macro::WatSlot =
			$crate::r#macro::return_into_js_wat_slots::<$output>()[0];

		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_export_imports!(($($input),*), $output),
			"\n(import \"env\" \"raw\" (func $raw (@sym (name \"",
			$raw,
			"\"))",
			$($crate::r#macro::wat_export_input_raw_param!($input),)*
			" (result ",
			SLOT.abi,
			")))\n",
			"(func $export (@sym (name \"",
			$export,
			"\"))",
			$($crate::r#macro::wat_export_input_params!($par, $input),)*
			" (result ",
			SLOT.boundary,
			")\n",
			$($crate::r#macro::wat_export_input_gets!($par, $input),)*
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
			$crate::r#macro::wat_export_result_types!($output);

		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_export_imports!(($($input),*), $output),
			"\n(import \"env\" \"raw\" (func $raw (@sym (name \"",
			$raw,
			"\")) (param ",
			POINTER,
			")",
			$($crate::r#macro::wat_export_input_raw_param!($input),)*
			"))\n",
			"(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut ",
			POINTER,
			")))\n",
			"(func $export (@sym (name \"",
			$export,
			"\"))",
			$($crate::r#macro::wat_export_input_params!($par, $input),)*
			" (result ",
			RESULT_TYPES,
			")\n",
			"  (local $retptr ",
			POINTER,
			")\n",
			"  global.get $__stack_pointer\n  ",
			POINTER,
			".const ",
			SIZE,
			"\n  ",
			POINTER,
			".sub\n  local.tee $retptr\n  global.set $__stack_pointer\n",
			"  local.get $retptr\n",
			$($crate::r#macro::wat_export_input_gets!($par, $input),)*
			"  call $raw (@reloc)\n",
			$crate::r#macro::wat_export_result_loads!($output),
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

/// Generates the WAT shim for one Rust export, or an empty string when its
/// raw `ABI` already matches the JavaScript boundary.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_export {
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		$($crate::r#macro::validate_return_from_js::<$input>();)*

		if $crate::r#macro::wat_export_needs_shim!(($(($par, $input)),*)) {
			$crate::r#macro::wat_export_direct!($raw, $export, ($(($par, $input)),*))
		} else {
			""
		}
	}};
	($raw:expr, $export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		$($crate::r#macro::validate_return_from_js::<$input>();)*
		$crate::r#macro::validate_return_into_js::<$output>();

		if !$crate::r#macro::wat_export_needs_shim!(
			($(($par, $input)),*),
			$output,
		) {
			""
		} else if $crate::r#macro::return_into_js_is_direct::<$output>() {
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

// JavaScript wrapper helpers.

#[doc(hidden)]
#[macro_export]
macro_rules! js_export_input_arguments {
	($par:literal, $ty:ty $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] =
			$crate::r#macro::from_js_wat_slots::<$ty>();
		const TEMPLATES: [&::core::primitive::str; 4] =
			$crate::r#macro::js_output_templates::<$ty>();
		const VALUES: [&::core::primitive::str; 4] = [
			$crate::r#macro::js_template!(TEMPLATES[0], value = $par),
			$crate::r#macro::js_template!(TEMPLATES[1], value = $par),
			$crate::r#macro::js_template!(TEMPLATES[2], value = $par),
			$crate::r#macro::js_template!(TEMPLATES[3], value = $par),
		];

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => [VALUES[0]],
			!SLOTS[1].abi.is_empty() => [", ", VALUES[1]],
			!SLOTS[2].abi.is_empty() => [", ", VALUES[2]],
			!SLOTS[3].abi.is_empty() => [", ", VALUES[3]],
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_export_parameters {
	() => {
		""
	};
	(($par:literal, $ty:ty) $(,)?) => {
		$par
	};
	(($par:literal, $ty:ty), $(($rest_par:literal, $rest_ty:ty)),+ $(,)?) => {
		$crate::r#macro::const_concat!(
			$par,
			", ",
			$crate::r#macro::js_export_parameters!($(($rest_par, $rest_ty)),+)
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_export_arguments {
	() => {
		""
	};
	(($par:literal, $ty:ty) $(,)?) => {
		$crate::r#macro::js_export_input_arguments!($par, $ty)
	};
	(($par:literal, $ty:ty), $(($rest_par:literal, $rest_ty:ty)),+ $(,)?) => {{
		const FIRST: &::core::primitive::str =
			$crate::r#macro::js_export_input_arguments!($par, $ty);
		const REST: &::core::primitive::str =
			$crate::r#macro::js_export_arguments!($(($rest_par, $rest_ty)),+);

		$crate::r#macro::const_concat!(
			FIRST,
			$crate::r#macro::separator_between(FIRST, REST),
			REST
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_export_output_expression {
	($ty:ty $(,)?) => {{
		const DIRECT: ::core::primitive::bool = $crate::r#macro::return_into_js_is_direct::<$ty>();
		const RESULT: ::core::primitive::bool = $crate::r#macro::return_into_js_is_result::<$ty>();
		const VALUES: [&::core::primitive::str; 4] = if RESULT {
			["ret[2]", "ret[3]", "", ""]
		} else if DIRECT {
			["ret", "", "", ""]
		} else {
			["ret[0]", "ret[1]", "ret[2]", "ret[3]"]
		};

		$crate::r#macro::js_template!(
			$crate::r#macro::js_export_output_template::<$ty>(),
			slots = VALUES,
		)
	}};
}

/// Generates the complete JavaScript wrapper for one Rust export.
#[doc(hidden)]
#[macro_export]
macro_rules! js_export {
	($export:expr, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		$($crate::r#macro::validate_return_from_js::<$input>();)*
		const PARAMETERS: &::core::primitive::str =
			$crate::r#macro::js_export_parameters!($(($par, $input)),*);
		const ARGUMENTS: &::core::primitive::str =
			$crate::r#macro::js_export_arguments!($(($par, $input)),*);

		$crate::r#macro::const_concat!(
			"(",
			PARAMETERS,
			") => {\n    instance.exports['",
			$export,
			"'](",
			ARGUMENTS,
			")\n}"
		)
	}};
	($export:expr, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		$($crate::r#macro::validate_return_from_js::<$input>();)*
		$crate::r#macro::validate_return_into_js::<$output>();
		const PARAMETERS: &::core::primitive::str =
			$crate::r#macro::js_export_parameters!($(($par, $input)),*);
		const ARGUMENTS: &::core::primitive::str =
			$crate::r#macro::js_export_arguments!($(($par, $input)),*);
		const OUTPUT: &::core::primitive::str =
			$crate::r#macro::js_export_output_expression!($output);
		const THROW: &::core::primitive::str =
			if $crate::r#macro::return_into_js_is_result::<$output>() {
				"    if (ret[1] !== 0) throw ret[0]\n"
			} else {
				""
			};

		$crate::r#macro::const_concat!(
			"(",
			PARAMETERS,
			") => {\n    const ret = instance.exports['",
			$export,
			"'](",
			ARGUMENTS,
			")\n",
			THROW,
			"    return ",
			OUTPUT,
			"\n}"
		)
	}};
}
