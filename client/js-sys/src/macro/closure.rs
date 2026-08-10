// WAT shims for Rust closures called from JavaScript.

#[doc(hidden)]
#[macro_export]
macro_rules! wat_closure_table_import {
	() => {{
		const POINTER: &::core::primitive::str = $crate::r#macro::wat_pointer_type();

		$crate::r#macro::const_concat!(
			"\n(import \"env\" \"__indirect_function_table\" ",
			"(table $js_sys.closure.table ",
			"(@sym (name \"__indirect_function_table\")) ",
			POINTER,
			" 0 funcref))"
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_closure_call {
	($call_shim:ty $(,)?) => {{
		const POINTER: &::core::primitive::str = $crate::r#macro::wat_pointer_type();
		type StoredCallShim = $call_shim;
		const OFFSET_VALUE: ::core::primitive::usize =
			$crate::ClosureHeader::call_shim_offset::<StoredCallShim>();
		const OFFSET: &::core::primitive::str = $crate::r#macro::const_integer_str!(OFFSET_VALUE);

		$crate::r#macro::const_concat!(
			"  local.get $js_sys.closure.data\n  ",
			POINTER,
			".load offset=",
			OFFSET,
			"\n  call_indirect $js_sys.closure.table (type $js_sys.closure.call) (@reloc)"
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_closure_direct {
	($export:expr, $call_shim:ty, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		const POINTER: &::core::primitive::str = $crate::r#macro::wat_pointer_type();
		const DATA: $crate::r#macro::WatSlot =
			$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>()[0];

		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_imports!(
				slots = [
					$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>(),
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
				],
				extras = [],
			),
			$crate::r#macro::wat_closure_table_import!(),
			"\n(type $js_sys.closure.call (func (param ",
			POINTER,
			")",
			$($crate::r#macro::wat_input!(export raw_param; $input),)*
			"))\n",
			"(func $export (@sym (name \"",
			$export,
			"\")) (param $data ",
			DATA.boundary,
			")",
			$($crate::r#macro::wat_input!(export params; $par, $input),)*
			" (local $js_sys.closure.data ",
			POINTER,
			")",
			$crate::r#macro::wat_locals!(
				slots = [
					$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>(),
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
				],
				extras = [],
			),
			"\n  local.get $data",
			$crate::r#macro::wat_conv_prefix(DATA.conv),
			DATA.conv,
			"\n  local.set $js_sys.closure.data\n",
			"  local.get $js_sys.closure.data\n",
			$($crate::r#macro::wat_input!(export gets; $par, $input),)*
			$crate::r#macro::wat_closure_call!($call_shim),
			"\n)"
		)
	}};
	($export:expr, $call_shim:ty, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const POINTER: &::core::primitive::str = $crate::r#macro::wat_pointer_type();
		const DATA: $crate::r#macro::WatSlot =
			$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>()[0];
		const OUTPUT: $crate::r#macro::WatSlot =
			$crate::r#macro::return_into_js_wat_slots::<$output>()[0];

		$crate::r#macro::const_concat!(
			$crate::r#macro::wat_imports!(
				slots = [
					$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>(),
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			$crate::r#macro::wat_closure_table_import!(),
			"\n(type $js_sys.closure.call (func (param ",
			POINTER,
			")",
			$($crate::r#macro::wat_input!(export raw_param; $input),)*
			" (result ",
			OUTPUT.abi,
			")))\n",
			"(func $export (@sym (name \"",
			$export,
			"\")) (param $data ",
			DATA.boundary,
			")",
			$($crate::r#macro::wat_input!(export params; $par, $input),)*
			" (result ",
			OUTPUT.boundary,
			") (local $js_sys.closure.data ",
			POINTER,
			")",
			$crate::r#macro::wat_locals!(
				slots = [
					$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>(),
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			"\n  local.get $data",
			$crate::r#macro::wat_conv_prefix(DATA.conv),
			DATA.conv,
			"\n  local.set $js_sys.closure.data\n",
			"  local.get $js_sys.closure.data\n",
			$($crate::r#macro::wat_input!(export gets; $par, $input),)*
			$crate::r#macro::wat_closure_call!($call_shim),
			$crate::r#macro::wat_conv_prefix(OUTPUT.conv),
			OUTPUT.conv,
			"\n)"
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_closure_indirect {
	($export:expr, $call_shim:ty, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		const POINTER: &::core::primitive::str = $crate::r#macro::wat_pointer_type();
		const DATA: $crate::r#macro::WatSlot =
			$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>()[0];
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
					$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>(),
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			$crate::r#macro::wat_closure_table_import!(),
			"\n(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut ",
			POINTER,
			")))\n",
			"(type $js_sys.closure.call (func (param ",
			POINTER,
			") (param ",
			POINTER,
			")",
			$($crate::r#macro::wat_input!(export raw_param; $input),)*
			"))\n",
			"(func $export (@sym (name \"",
			$export,
			"\")) (param $data ",
			DATA.boundary,
			")",
			$($crate::r#macro::wat_input!(export params; $par, $input),)*
			" (result ",
			RESULT_TYPES,
			")\n  (local $retptr ",
			POINTER,
			")\n  (local $js_sys.closure.data ",
			POINTER,
			")",
			$crate::r#macro::wat_locals!(
				slots = [
					$crate::r#macro::from_js_wat_slots::<::core::primitive::usize>(),
					$($crate::r#macro::from_js_wat_slots::<$input>(),)*
					$crate::r#macro::return_into_js_wat_slots::<$output>(),
				],
				extras = [],
			),
			"\n  local.get $data",
			$crate::r#macro::wat_conv_prefix(DATA.conv),
			DATA.conv,
			"\n  local.set $js_sys.closure.data\n",
			"  global.get $__stack_pointer\n  ",
			POINTER,
			".const ",
			SIZE,
			"\n  ",
			POINTER,
			".sub\n  local.tee $retptr\n  global.set $__stack_pointer\n",
			"  local.get $retptr\n",
			"  local.get $js_sys.closure.data\n",
			$($crate::r#macro::wat_input!(export gets; $par, $input),)*
			$crate::r#macro::wat_closure_call!($call_shim),
			"\n",
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

/// Generates a closure dispatcher that calls the raw shim stored in its
/// allocation.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_closure {
	($export:expr, $call_shim:ty, ($(($par:literal, $input:ty)),*) $(,)?) => {{
		$crate::r#macro::validate_from_js::<::core::primitive::usize>();
		$($crate::r#macro::validate_from_js::<$input>();)*

		$crate::r#macro::wat_closure_direct!(
			$export,
			$call_shim,
			($(($par, $input)),*),
		)
	}};
	($export:expr, $call_shim:ty, ($(($par:literal, $input:ty)),*), $output:ty $(,)?) => {{
		$crate::r#macro::validate_from_js::<::core::primitive::usize>();
		$($crate::r#macro::validate_from_js::<$input>();)*
		$crate::r#macro::validate_return_into_js::<$output>();

		if $crate::r#macro::return_into_js_is_direct::<$output>() {
			$crate::r#macro::wat_closure_direct!(
				$export,
				$call_shim,
				($(($par, $input)),*),
				$output,
			)
		} else {
			$crate::r#macro::wat_closure_indirect!(
				$export,
				$call_shim,
				($(($par, $input)),*),
				$output,
			)
		}
	}};
}
