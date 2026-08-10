use crate::JsValue;
use crate::builtins::Array;
use crate::hazard::{IntoJS, IntoJsConv, JsCast};
use crate::util::{ExternSlice, JS_PTR_LEN_ARGS, PtrConst, PtrLength};

macro_rules! primitive_slices {
	($(
		$(#[$attribute:meta])*
		$ty:ty => {
			constructor: $constructor:literal,
			view: $view:literal,
			embed: $embed:literal,
			decode: $decode:ident,
			from_slice: $from_slice:ident,
		}
	),+ $(,)?) => {
		#[crate::js_sys(js_sys = crate)]
		extern "js-sys" {
			// SAFETY: The pointer and length must describe a valid `JsValue` slice.
			#[js_sys(js_embed = "array.js_value.decode")]
			#[expect(
				clippy::allow_attributes,
				reason = "the macro emits an unsafe ABI call"
			)]
			#[allow(
				clippy::undocumented_unsafe_blocks,
				reason = "the safety requirement is documented on this declaration"
			)]
			unsafe fn array_js_value_decode(
				array: PtrConst<JsValue>,
				len: PtrLength<JsValue>,
			) -> Array<JsValue>;

			$(
				$(#[$attribute])*
				// SAFETY: The pointer and length must describe a valid slice of the
				// declared element type.
				#[js_sys(js_embed = $embed)]
				#[expect(
					clippy::allow_attributes,
					reason = "the macro emits an unsafe ABI call"
				)]
				#[allow(
					clippy::undocumented_unsafe_blocks,
					reason = "the safety requirement is documented on this declaration"
				)]
				unsafe fn $decode(array: PtrConst<$ty>, len: PtrLength<$ty>) -> Array<$ty>;
			)+
		}

		$(
			$(#[$attribute])*
			pub(in crate::interop) fn $from_slice(value: &[$ty]) -> Array<$ty> {
				// SAFETY: The pointer and length describe `value`.
				unsafe { $decode(PtrConst::new(value), PtrLength::new(value)) }
			}

			// SAFETY: The two slots describe a borrowed primitive slice, which
			// JavaScript copies into an independent typed array before the import.
			$(#[$attribute])*
			#[expect(
				clippy::undocumented_unsafe_blocks,
				reason = "the safety requirement is documented in the macro definition"
			)]
			unsafe impl IntoJS for &[$ty] {
				const JS_CONV: Option<IntoJsConv> = Some(
					IntoJsConv::new(crate::const_concat!(
						"new ",
						$constructor,
						"(this.#jsEmbed.js_sys['view.get",
						$view,
						"'](",
						JS_PTR_LEN_ARGS,
						"))"
					))
					.with_embed(("js_sys", concat!("view.get", $view))),
				);

				type Abi = ExternSlice<$ty>;

				fn into_abi(self) -> Self::Abi {
					ExternSlice::new(self)
				}
			}

			$(#[$attribute])*
			js_bindgen::embed_js!(
				module = "js_sys",
				name = $embed,
				required_embeds = [("js_sys", concat!("view.get", $view))],
				"(ptr, len) => Array.from(",
				"    this.#jsEmbed.js_sys['view.get{view}'](ptr, len),",
				")",
				view = interpolate $view,
			);
		)+
	};
}

primitive_slices! {
	i8 => {
		constructor: "Int8Array",
		view: "Int8",
		embed: "array.i8.decode",
		decode: array_i8_decode,
		from_slice: array_from_i8_slice,
	},
	u8 => {
		constructor: "Uint8Array",
		view: "Uint8",
		embed: "array.u8.decode",
		decode: array_u8_decode,
		from_slice: array_from_u8_slice,
	},
	i16 => {
		constructor: "Int16Array",
		view: "Int16",
		embed: "array.i16.decode",
		decode: array_i16_decode,
		from_slice: array_from_i16_slice,
	},
	u16 => {
		constructor: "Uint16Array",
		view: "Uint16",
		embed: "array.u16.decode",
		decode: array_u16_decode,
		from_slice: array_from_u16_slice,
	},
	i32 => {
		constructor: "Int32Array",
		view: "Int32",
		embed: "array.i32.decode",
		decode: array_i32_decode,
		from_slice: array_from_i32_slice,
	},
	u32 => {
		constructor: "Uint32Array",
		view: "Uint32",
		embed: "array.u32.decode",
		decode: array_u32_decode,
		from_slice: array_from_u32_slice,
	},
	i64 => {
		constructor: "BigInt64Array",
		view: "BigInt64",
		embed: "array.i64.decode",
		decode: array_i64_decode,
		from_slice: array_from_i64_slice,
	},
	u64 => {
		constructor: "BigUint64Array",
		view: "BigUint64",
		embed: "array.u64.decode",
		decode: array_u64_decode,
		from_slice: array_from_u64_slice,
	},
	f32 => {
		constructor: "Float32Array",
		view: "Float32",
		embed: "array.f32.decode",
		decode: array_f32_decode,
		from_slice: array_from_f32_slice,
	},
	f64 => {
		constructor: "Float64Array",
		view: "Float64",
		embed: "array.f64.decode",
		decode: array_f64_decode,
		from_slice: array_from_f64_slice,
	},
	#[cfg(target_arch = "wasm32")]
	isize => {
		constructor: "Int32Array",
		view: "Int32",
		embed: "array.isize.decode",
		decode: array_isize_decode,
		from_slice: array_from_isize_slice,
	},
	#[cfg(target_arch = "wasm64")]
	isize => {
		constructor: "BigInt64Array",
		view: "BigInt64",
		embed: "array.isize.decode",
		decode: array_isize_decode,
		from_slice: array_from_isize_slice,
	},
	#[cfg(target_arch = "wasm32")]
	usize => {
		constructor: "Uint32Array",
		view: "Uint32",
		embed: "array.usize.decode",
		decode: array_usize_decode,
		from_slice: array_from_usize_slice,
	},
	#[cfg(target_arch = "wasm64")]
	usize => {
		constructor: "BigUint64Array",
		view: "BigUint64",
		embed: "array.usize.decode",
		decode: array_usize_decode,
		from_slice: array_from_usize_slice,
	},
}

pub(in crate::interop) fn array_from_js_value_slice<T: JsCast>(value: &[T]) -> Array<T> {
	let slice = JsValue::from_slice(value);
	// SAFETY: Parameters are correct.
	let result = unsafe { array_js_value_decode(PtrConst::new(slice), PtrLength::new(slice)) };

	Array::unchecked_from(result.into())
}

// SAFETY: The array delegates to the slice implementation with the same
// element representation.
unsafe impl<'a, T, const N: usize> IntoJS for &'a [T; N]
where
	&'a [T]: IntoJS,
{
	const JS_CONV: Option<IntoJsConv> = <&[T] as IntoJS>::JS_CONV;

	type Abi = <&'a [T] as IntoJS>::Abi;

	fn into_abi(self) -> Self::Abi {
		self.as_slice().into_abi()
	}
}

// SAFETY: The two slots point to borrowed `JsValue` table indices, which the
// JavaScript decoder resolves before the import is called.
unsafe impl<T: JsCast> IntoJS for &[T] {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new(crate::const_concat!(
			"this.#jsEmbed.js_sys['array.js_value.decode'](",
			JS_PTR_LEN_ARGS,
			")"
		))
		.with_embed(("js_sys", "array.js_value.decode")),
	);

	type Abi = ExternSlice<JsValue>;

	fn into_abi(self) -> Self::Abi {
		ExternSlice::new(JsValue::from_slice(self))
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.js_value.decode",
	required_embeds = [("js_sys", "externref.table"), ("js_sys", "view.getUint32")],
	"(ptr, len) => {{",
	"    const array = new Array(len)",
	"    const table = this.#jsEmbed.js_sys['externref.table']",
	"    const refIndices = this.#jsEmbed.js_sys['view.getUint32'](ptr, len)",
	"    for (let arrayIndex = 0; arrayIndex < len; arrayIndex++) {{",
	"        array[arrayIndex] = table.get(refIndices[arrayIndex])",
	"    }}",
	"    return array",
	"}}",
);
