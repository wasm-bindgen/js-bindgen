/// Generates the complete WAT shim for one JavaScript import.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_import {
	(
		module = $crate_name:expr,
		import = $import_name:expr,
		shim = $foreign_name:expr,
		inputs = [$(($par:literal, $input:ty)),* $(,)?],
		$(output = $output:ty,)?
	) => {{
		const INPUT_TYPES: &::core::primitive::str =
			$crate::r#macro::wat_import_input_types!($($input),*);
		const INPUT_PARAM: &::core::primitive::str = $crate::r#macro::const_concat_if!(
			!INPUT_TYPES.is_empty() => [" (param ", INPUT_TYPES, ")"],
		);

		$crate::r#macro::const_concat!(
			"(import \"",
			$crate_name,
			"\" \"",
			$import_name,
			"\" (func $",
			$crate_name,
			".import.",
			$import_name,
			" (@sym (name \"",
			$crate_name,
			".import.",
			$import_name,
			"\"))",
			$($crate::r#macro::wat_output_import_param!($output),)?
			INPUT_PARAM,
			$($crate::r#macro::wat_import_result!($output),)?
			"))",
			$crate::r#macro::wat_imports!(($($input),*) $(, $output)?),
			"\n(func $",
			$foreign_name,
			" (@sym)",
			$($crate::r#macro::wat_output_param!($output),)?
			$($crate::r#macro::wat_input_params!($par, $input),)*
			$($crate::r#macro::wat_output_result!($output),)?
			$($crate::r#macro::wat_result_try!($output),)?
			$($crate::r#macro::wat_output_get!($output),)?
			$("\n", $crate::r#macro::wat_input_gets!($par, $input),)*
			"\n  call $",
			$crate_name,
			".import.",
			$import_name,
			" (@reloc)",
			$($crate::r#macro::wat_output!($output),)?
			$($crate::r#macro::wat_result_catch!($output),)?
			$($crate::r#macro::wat_result_default!($output),)?
			"\n)"
		)
	}};
}

// Imported function signature and conversion dependencies.

#[doc(hidden)]
#[macro_export]
macro_rules! wat_import_input_types {
	() => {
		""
	};
	($first:ty $(, $rest:ty)* $(,)?) => {
		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_input_import_types!($first),
			$(" ", $crate::r#macro::wat_input_import_types!($rest),)*
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_imports {
	(($($input:ty),*) $(, $output:ty)? $(,)?) => {
		$crate::r#macro::wat_import_list!(
			$($crate::r#macro::into_js_wat_slots::<$input>()[0].import,)*
			$($crate::r#macro::into_js_wat_slots::<$input>()[1].import,)*
			$($crate::r#macro::into_js_wat_slots::<$input>()[2].import,)*
			$($crate::r#macro::into_js_wat_slots::<$input>()[3].import,)*
			$($crate::r#macro::wat_output_import::<$output>(),)?
			$($crate::r#macro::wat_result_imports::<$output>()[0],)?
			$($crate::r#macro::wat_result_imports::<$output>()[1],)?
			$($crate::r#macro::wat_result_imports::<$output>()[2],)?
		)
	};
}

// Return shim.

#[doc(hidden)]
#[macro_export]
macro_rules! wat_output {
	($ty:ty) => {
		if !$crate::r#macro::return_from_js_is_direct::<$ty>() {
			""
		} else if !$crate::r#macro::wat_output_conv::<$ty>().is_empty() {
			const CONV: &::core::primitive::str = $crate::r#macro::wat_output_conv::<$ty>();

			$crate::r#macro::const_concat!("\n  ", CONV)
		} else {
			""
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_output_import_param {
	($ty:ty) => {
		if $crate::r#macro::return_from_js_is_direct::<$ty>() {
			""
		} else {
			$crate::r#macro::const_concat!(
				" (param $retptr ",
				$crate::r#macro::wat_indirect_import_type::<$ty>(),
				")"
			)
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_output_param {
	($ty:ty) => {
		if $crate::r#macro::return_from_js_is_direct::<$ty>() {
			""
		} else {
			$crate::r#macro::const_concat!(
				" (param $retptr ",
				$crate::r#macro::wat_indirect_type::<$ty>(),
				")"
			)
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_import_result {
	($ty:ty) => {
		if $crate::r#macro::return_from_js_is_direct::<$ty>() {
			$crate::r#macro::const_concat!(
				" (result ",
				$crate::r#macro::wat_output_import_type::<$ty>(),
				")"
			)
		} else {
			""
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_output_result {
	($ty:ty) => {
		if $crate::r#macro::return_from_js_is_direct::<$ty>() {
			$crate::r#macro::const_concat!(" (result ", $crate::r#macro::wat_direct::<$ty>(), ")")
		} else {
			""
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_output_get {
	($ty:ty) => {{
		if $crate::r#macro::return_from_js_is_direct::<$ty>() {
			""
		} else {
			const CONV: &::core::primitive::str = $crate::r#macro::wat_indirect_conv::<$ty>();

			$crate::r#macro::const_concat!(
				"\n  local.get $retptr",
				$crate::r#macro::wat_conv_prefix(CONV),
				CONV
			)
		}
	}};
}

// Input shim.

#[doc(hidden)]
#[macro_export]
macro_rules! wat_input_import_types {
	($ty:ty $(,)?) => {
		$crate::r#macro::wat_slot_types!($crate::r#macro::into_js_wat_slots::<$ty>(), boundary,)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_input_params {
	($par:literal, $ty:ty $(,)?) => {
		$crate::r#macro::wat_slot_params!($par, $crate::r#macro::into_js_wat_slots::<$ty>(), abi,)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_input_gets {
	($par:literal, $ty:ty $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] =
			$crate::r#macro::into_js_wat_slots::<$ty>();

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => ["", "  local.get $", $par, "_0", $crate::r#macro::wat_conv_prefix(SLOTS[0].conv), SLOTS[0].conv],
			!SLOTS[1].abi.is_empty() => ["\n", "  local.get $", $par, "_1", $crate::r#macro::wat_conv_prefix(SLOTS[1].conv), SLOTS[1].conv],
			!SLOTS[2].abi.is_empty() => ["\n", "  local.get $", $par, "_2", $crate::r#macro::wat_conv_prefix(SLOTS[2].conv), SLOTS[2].conv],
			!SLOTS[3].abi.is_empty() => ["\n", "  local.get $", $par, "_3", $crate::r#macro::wat_conv_prefix(SLOTS[3].conv), SLOTS[3].conv],
		)
	}};
}
