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
			$crate::r#macro::wat_input!(import types; $($input),*);
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
			$($crate::r#macro::wat_import_output!(import_param, $output),)?
			INPUT_PARAM,
			$($crate::r#macro::wat_import_output!(import_result, $output),)?
			"))",
			$crate::r#macro::wat_imports!(
				slots = [
					$($crate::r#macro::into_js_wat_slots::<$input>(),)*
				],
				extras = [
					$($crate::r#macro::wat_output_import::<$output>(),)?
					$($crate::r#macro::wat_result_imports::<$output>()[0],)?
					$($crate::r#macro::wat_result_imports::<$output>()[1],)?
					$($crate::r#macro::wat_result_imports::<$output>()[2],)?
				],
			),
			"\n(func $",
			$foreign_name,
			" (@sym)",
			$($crate::r#macro::wat_import_output!(shim_param, $output),)?
			$($crate::r#macro::wat_input!(import params; $par, $input),)*
			$($crate::r#macro::wat_import_output!(shim_result, $output),)?
			$($crate::r#macro::wat_result_try::<$output>(),)?
			$($crate::r#macro::wat_import_output!(shim_retptr, $output),)?
			$(
				"\n",
				$crate::r#macro::wat_input!(import gets; $par, $input),
			)*
			"\n  call $",
			$crate_name,
			".import.",
			$import_name,
			" (@reloc)",
			$($crate::r#macro::wat_import_output!(shim_convert, $output),)?
			$($crate::r#macro::wat_result_catch::<$output>(),)?
			$($crate::r#macro::wat_result_default::<$output>(),)?
			"\n)"
		)
	}};
}

/// Renders the direct or indirect output fragments of an import shim.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_import_output {
	(import_param, $ty:ty $(,)?) => {
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
	(import_result, $ty:ty $(,)?) => {
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
	(shim_param, $ty:ty $(,)?) => {
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
	(shim_result, $ty:ty $(,)?) => {
		if $crate::r#macro::return_from_js_is_direct::<$ty>() {
			$crate::r#macro::const_concat!(" (result ", $crate::r#macro::wat_direct::<$ty>(), ")")
		} else {
			""
		}
	};
	(shim_retptr, $ty:ty $(,)?) => {{
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
	(shim_convert, $ty:ty $(,)?) => {
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
