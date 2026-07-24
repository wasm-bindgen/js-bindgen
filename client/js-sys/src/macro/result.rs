use crate::hazard::ReturnFromJS;

#[cfg(not(target_feature = "exception-handling"))]
const DIRECT_CATCH: &str = concat!(
	"\n    } catch ($error) {",
	"\n        const $index = this.#instance.exports['js_sys.exception.store']()",
	"\n        this.#jsEmbed.js_sys['externref.table'].set($index, $error)",
	"\n        return false",
	"\n    }",
	"\n}",
);
#[cfg(not(target_feature = "exception-handling"))]
const INDIRECT_CATCH: &str = concat!(
	"\n    } catch ($error) {",
	"\n        const $index = this.#instance.exports['js_sys.exception.store']()",
	"\n        this.#jsEmbed.js_sys['externref.table'].set($index, $error)",
	"\n    }",
	"\n}",
);

#[cfg(target_feature = "exception-handling")]
const WAT_TAG_IMPORT: &str = "(import \"js_sys\" \"exception.tag\" (tag $js_sys.exception.tag \
                              (@sym (name \"js_sys.exception.tag\")) (param externref)))";
#[cfg(target_feature = "exception-handling")]
const WAT_INSERT_IMPORT: &str = concat!(
	"(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) ",
	"(param externref) (result i32)))",
);
#[cfg(target_feature = "exception-handling")]
const WAT_STORE_IMPORT: &str = concat!(
	"(import \"env\" \"js_sys.exception.store\" (func $js_sys.exception.store (@sym) ",
	"(param i32)))",
);
#[cfg(target_feature = "exception-handling")]
const WAT_CATCH: &str = concat!(
	"\n      return",
	"\n    )",
	"\n    unreachable",
	"\n  )",
	"\n  call $js_sys.externref.insert (@reloc)",
	"\n  call $js_sys.exception.store (@reloc)",
);

#[must_use]
pub const fn catches_result_in_js<T: ReturnFromJS>() -> bool {
	#[cfg(target_feature = "exception-handling")]
	{
		false
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		crate::r#macro::return_from_js_is_result::<T>()
	}
}

#[must_use]
pub const fn js_result_try<T: ReturnFromJS>() -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		""
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		if crate::r#macro::return_from_js_is_result::<T>() {
			"    try {\n"
		} else {
			""
		}
	}
}

#[must_use]
pub const fn js_result_catch<T: ReturnFromJS>(direct: bool) -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		let _ = direct;
		""
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		if !crate::r#macro::return_from_js_is_result::<T>() {
			""
		} else if direct {
			DIRECT_CATCH
		} else {
			INDIRECT_CATCH
		}
	}
}

#[must_use]
pub const fn wat_result_imports<T: ReturnFromJS>() -> [&'static str; 3] {
	#[cfg(target_feature = "exception-handling")]
	{
		if crate::r#macro::return_from_js_is_result::<T>() {
			[WAT_TAG_IMPORT, WAT_INSERT_IMPORT, WAT_STORE_IMPORT]
		} else {
			[""; 3]
		}
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		[""; 3]
	}
}

#[must_use]
pub const fn wat_result_try<T: ReturnFromJS>() -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		if crate::r#macro::return_from_js_is_result::<T>() {
			"\n  (block $js_sys.exception.catch (result externref)\n    (try_table (catch \
			 $js_sys.exception.tag $js_sys.exception.catch) (@reloc)"
		} else {
			""
		}
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		""
	}
}

#[must_use]
pub const fn wat_result_catch<T: ReturnFromJS>() -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		if crate::r#macro::return_from_js_is_result::<T>() {
			WAT_CATCH
		} else {
			""
		}
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		""
	}
}

#[must_use]
pub const fn wat_result_default<T: ReturnFromJS>() -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		if !crate::r#macro::return_from_js_is_result::<T>()
			|| !crate::r#macro::return_from_js_is_direct::<T>()
		{
			return "";
		}

		match crate::r#macro::wat_direct::<T>().as_bytes() {
			b"i32" => "\n  i32.const 0",
			b"i64" => "\n  i64.const 0",
			b"f32" => "\n  f32.const 0",
			b"f64" => "\n  f64.const 0",
			_ => panic!("unsupported direct return type"),
		}
	}

	#[cfg(not(target_feature = "exception-handling"))]
	{
		""
	}
}
