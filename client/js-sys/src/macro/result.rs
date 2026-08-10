#[cfg(target_feature = "exception-handling")]
use crate::externref::{
	WAT_INDEX_LOCAL, WAT_INSERT_CONV, WAT_NEXT_IMPORT, WAT_TABLE_IMPORT, WAT_VALUE_LOCAL,
};
use crate::hazard::ReturnFromJS;

#[cfg(not(target_feature = "exception-handling"))]
const DIRECT_CATCH: &str = "
    } catch ($error) {
        const $index = this.#instance.exports['js_sys.exception.store']()
        this.#jsEmbed.js_sys['externref.table'].set($index, $error)
        return false
    }
}";
#[cfg(not(target_feature = "exception-handling"))]
const INDIRECT_CATCH: &str = "
    } catch ($error) {
        const $index = this.#instance.exports['js_sys.exception.store']()
        this.#jsEmbed.js_sys['externref.table'].set($index, $error)
    }
}";

#[cfg(target_feature = "exception-handling")]
const WAT_TAG_IMPORT: &str = "(import \"js_sys\" \"exception.tag\" (tag $js_sys.exception.tag \
                              (@sym (name \"js_sys.exception.tag\")) (param externref)))";
#[cfg(target_feature = "exception-handling")]
const WAT_STORE_IMPORT: &str =
	"(import \"env\" \"js_sys.exception.store\" (func $js_sys.exception.store (@sym) (param i32)))";
#[cfg(target_feature = "exception-handling")]
const WAT_IMPORTS: &str = crate::const_concat!(
	WAT_TAG_IMPORT,
	"\n",
	WAT_TABLE_IMPORT,
	"\n",
	WAT_NEXT_IMPORT,
	"\n",
	WAT_STORE_IMPORT,
);
#[cfg(target_feature = "exception-handling")]
const WAT_LOCALS: &str = crate::const_concat!(WAT_VALUE_LOCAL, "\n", WAT_INDEX_LOCAL);
#[cfg(target_feature = "exception-handling")]
const WAT_CATCH: &str = crate::const_concat!(
	"\n      return",
	"\n    )",
	"\n    unreachable",
	"\n  )",
	"\n  ",
	WAT_INSERT_CONV,
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
pub const fn wat_result_imports<T: ReturnFromJS>() -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		if crate::r#macro::return_from_js_is_result::<T>() {
			WAT_IMPORTS
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
pub const fn wat_result_locals<T: ReturnFromJS>() -> &'static str {
	#[cfg(target_feature = "exception-handling")]
	{
		if crate::r#macro::return_from_js_is_result::<T>() {
			WAT_LOCALS
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
