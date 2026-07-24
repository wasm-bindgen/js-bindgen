/// Generates the complete JavaScript shim for one import.
#[doc(hidden)]
#[macro_export]
macro_rules! js_import {
	(
		direct_open = $direct_open:expr,
		direct_call = $direct_call:expr,
		indirect_call = $indirect_call:expr,
		inputs = [$(($par:literal, $input:ty)),* $(,)?],
	) => {{
		const WRAPPED: ::core::primitive::bool =
			$crate::r#macro::js_needs_shim!(($($input),*));
		const OPEN: &::core::primitive::str = if WRAPPED {
			$crate::r#macro::js_function!("(", ") => {\n", $(($par, $input)),*)
		} else {
			$direct_open
		};
		const BODY: &::core::primitive::str = if WRAPPED {
			$crate::r#macro::const_concat!($indirect_call, "\n}")
		} else {
			$direct_call
		};

		$crate::r#macro::const_concat!(
			OPEN,
			$($crate::r#macro::js_parameter!($par, $input),)*
			BODY
		)
	}};
	(
		direct_open = $direct_open:expr,
		direct_call = $direct_call:expr,
		indirect_call = $indirect_call:expr,
		inputs = [$(($par:literal, $input:ty)),* $(,)?],
		output = $output:ty,
	) => {{
		const WRAPPED: ::core::primitive::bool =
			$crate::r#macro::js_needs_shim!(($($input),*), $output);
		const OPEN: &::core::primitive::str = if WRAPPED {
			$crate::r#macro::js_indirect_function!(
				"(",
				") => {\n",
				($output),
				$(($par, $input)),*
			)
		} else {
			$direct_open
		};

		$crate::r#macro::const_concat!(
			OPEN,
			$($crate::r#macro::js_parameter!($par, $input),)*
			$crate::r#macro::js_output!(
				WRAPPED,
				"    return ",
				$direct_call,
				$indirect_call,
				$output,
			)
		)
	}};
}

// Shim selection.

#[doc(hidden)]
#[macro_export]
macro_rules! js_needs_shim {
	(($($input:ty),*) $(, $output:ty)? $(,)?) => {{
		'outer: {
			$(
				$crate::r#macro::validate_into_js::<$input>();

				if ::core::option::Option::is_some(
					&<$input as $crate::hazard::IntoJS>::JS_CONV,
				) {
					break 'outer true;
				}
			)*

			$(
				$crate::r#macro::validate_return_from_js::<$output>();

				if $crate::r#macro::catches_result_in_js::<$output>()
					|| ::core::option::Option::is_some(
						&<$output as $crate::hazard::ReturnFromJS>::JS_CONV.conversion(),
					)
				{
					break 'outer true;
				}
			)?

			false
		}
	}};
}

// Input rendering.

#[doc(hidden)]
#[macro_export]
macro_rules! js_input_parameters {
	($par:literal, $ty:ty $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] =
			$crate::r#macro::into_js_wat_slots::<$ty>();

		$crate::r#macro::const_concat_if!(
			true => [$par, "_0"],
			!SLOTS[1].abi.is_empty() => [", ", $par, "_1"],
			!SLOTS[2].abi.is_empty() => [", ", $par, "_2"],
			!SLOTS[3].abi.is_empty() => [", ", $par, "_3"],
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_function {
	($pre:literal, $post:literal $(,)?) => {
		$crate::r#macro::const_concat!($pre, $post)
	};
	($pre:literal, $post:literal, ($par:literal, $ty:ty) $(, ($rest_par:literal, $rest_ty:ty))* $(,)?) => {
		$crate::r#macro::const_concat!(
			$pre,
			$crate::r#macro::js_input_parameters!($par, $ty),
			$(", ", $crate::r#macro::js_input_parameters!($rest_par, $rest_ty),)*
			$post
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_indirect_function {
	($pre:literal, $post:literal, (), $(($par:literal, $ty:ty)),* $(,)?) => {
		$crate::r#macro::js_function!($pre, $post, $(($par, $ty)),*)
	};
	($pre:literal, $post:literal, ($output:ty), $(($par:literal, $ty:ty)),* $(,)?) => {{
		const PARAMETERS: &::core::primitive::str =
			$crate::r#macro::js_function!("", "", $(($par, $ty)),*);
		const INDIRECT: ::core::primitive::bool =
			!$crate::r#macro::return_from_js_is_direct::<$output>();
		const RETURN: &::core::primitive::str = if INDIRECT { "$retptr" } else { "" };
		const SEPARATOR: &::core::primitive::str =
			if INDIRECT && !PARAMETERS.is_empty() { ", " } else { "" };

		$crate::r#macro::const_concat!($pre, RETURN, SEPARATOR, PARAMETERS, $post)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_parameter {
	($par:literal, $ty:ty $(,)?) => {{
		const HAS_CONV: ::core::primitive::bool =
			::core::option::Option::is_some(&<$ty as $crate::hazard::IntoJS>::JS_CONV);
		const TEMPLATE: &::core::primitive::str = $crate::r#macro::js_input_template::<$ty>();
		const SLOTS: [&::core::primitive::str; 4] = [
			$crate::r#macro::const_concat!($par, "_0"),
			$crate::r#macro::const_concat!($par, "_1"),
			$crate::r#macro::const_concat!($par, "_2"),
			$crate::r#macro::const_concat!($par, "_3"),
		];
		const CONV: &::core::primitive::str = $crate::r#macro::js_template!(
			TEMPLATE,
			slots = SLOTS,
		);

		$crate::r#macro::const_concat_if!(
			HAS_CONV => ["    ", $par, "_0 = ", CONV, "\n"],
		)
	}};
}

// Return rendering.

#[doc(hidden)]
#[macro_export]
macro_rules! js_output {
	($wrapped:expr, $start:literal, $direct_call:literal, $indirect_call:literal, $output:ty $(,)?) => {{
		const OUTPUT_WRAPPED: ::core::primitive::bool = $wrapped;
		const DIRECT_RETURN: ::core::primitive::bool =
			$crate::r#macro::return_from_js_is_direct::<$output>();
		const CATCH_RESULT: ::core::primitive::bool =
			$crate::r#macro::catches_result_in_js::<$output>();
		const CALL: &::core::primitive::str = if OUTPUT_WRAPPED {
			$indirect_call
		} else {
			$direct_call
		};
		const TEMPLATES: [&::core::primitive::str; 4] =
			$crate::r#macro::js_output_templates::<$output>();
		const TEMPLATE_VALUE: &::core::primitive::str = if DIRECT_RETURN { CALL } else { "$ret" };
		const SLOTS: [&::core::primitive::str; 4] = [
			$crate::r#macro::js_template!(TEMPLATES[0], value = TEMPLATE_VALUE),
			$crate::r#macro::js_template!(TEMPLATES[1], value = TEMPLATE_VALUE),
			$crate::r#macro::js_template!(TEMPLATES[2], value = TEMPLATE_VALUE),
			$crate::r#macro::js_template!(TEMPLATES[3], value = TEMPLATE_VALUE),
		];
		const SRET: &::core::primitive::str = $crate::r#macro::js_output_sret::<$output>();
		const INDENT: &::core::primitive::str = if CATCH_RESULT { "        " } else { "    " };
		const VALUE_START: &::core::primitive::str = if DIRECT_RETURN {
			if CATCH_RESULT {
				"        return "
			} else if OUTPUT_WRAPPED {
				$start
			} else {
				""
			}
		} else {
			$crate::r#macro::const_concat!(INDENT, "const $ret = ")
		};
		const OUTPUT_VALUE: &::core::primitive::str = if DIRECT_RETURN { SLOTS[0] } else { CALL };
		const SRET_CALL: &::core::primitive::str = $crate::r#macro::const_concat_if!(
			!DIRECT_RETURN => ["\n", INDENT, SRET, "(", SLOTS[0]],
			!DIRECT_RETURN && !SLOTS[1].is_empty() => [", ", SLOTS[1]],
			!DIRECT_RETURN && !SLOTS[2].is_empty() => [", ", SLOTS[2]],
			!DIRECT_RETURN && !SLOTS[3].is_empty() => [", ", SLOTS[3]],
			!DIRECT_RETURN => [", $retptr)"],
		);
		const TRY: &::core::primitive::str = $crate::r#macro::js_result_try::<$output>();
		const END: &::core::primitive::str = if CATCH_RESULT {
			$crate::r#macro::js_result_catch::<$output>(DIRECT_RETURN)
		} else if OUTPUT_WRAPPED {
			"\n}"
		} else {
			""
		};

		$crate::r#macro::const_concat!(TRY, VALUE_START, OUTPUT_VALUE, SRET_CALL, END)
	}};
}
