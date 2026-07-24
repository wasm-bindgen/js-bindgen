use crate::hazard::{
	EmptySlot, FromJS, FromJsConv, IntoJS, IntoJsConv, OptionIntoJS, ReturnAbi, ReturnMode, Slot,
	WasmAbi,
};
use crate::r#macro::const_concat;

macro_rules! slot {
	($wat:literal, $($ty:ty),+ $(,)?) => {$(
		// SAFETY: The declared WAT type describes this primitive `ABI` slot.
		unsafe impl Slot for $ty {
			const WAT_TYPE: &'static str = $wat;
		}

		// SAFETY: Primitive scalar values are returned directly.
		unsafe impl ReturnAbi for $ty {
			const MODE: ReturnMode = ReturnMode::Direct;
		}
	)+};
}

macro_rules! from_js {
	($($ty:ty),+ $(,)?) => {$(
		// SAFETY: The JavaScript adapter produces this primitive's native `ABI`
		// slot, which is returned unchanged.
		unsafe impl FromJS for $ty {
			type Abi = Self;

			fn from_abi(raw: Self::Abi) -> Self {
				raw
			}
		}
	)*};
}

macro_rules! identity {
	($($ty:ty),+ $(,)?) => {$(
		// SAFETY: This primitive is already represented by its native `ABI` slot.
		unsafe impl IntoJS for $ty {
			type Abi = Self;

			fn into_abi(self) -> Self::Abi {
				self
			}
		}

		from_js!($ty);
	)*};
}

macro_rules! sentinel_option {
	(
		carrier: $carrier:ty,
		sentinel: $sentinel:expr,
		js_sentinel: $js_sentinel:literal,
		types: [$([$($ty:ident),+ $(,)?] => {
			to_js: $to_js:literal,
			from_js: $from_js:literal,
		}),+ $(,)?],
	) => {$($(
		// SAFETY: The sentinel lies outside the value range of this type.
		unsafe impl OptionIntoJS for $ty {
			const OPTION_JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new(const_concat!(
				"$slot1 === ",
				$js_sentinel,
				" ? undefined : ",
				$to_js
			)));

			type OptionAbi = $carrier;

			fn option_into_abi(value: Option<Self>) -> Self::OptionAbi {
				value.map_or($sentinel, |value| {
					sentinel_option!(@into_abi value, $ty, $carrier)
				})
			}
		}

		// SAFETY: The sentinel is decoded before the carrier is converted back.
		unsafe impl FromJS for Option<$ty> {
			const JS_CONV: Option<FromJsConv> = Some(FromJsConv::slot1(const_concat!(
				"((value) => value == null ? ",
				$js_sentinel,
				" : ",
				$from_js,
				")($value)"
			)));

			type Abi = $carrier;

			#[expect(
				clippy::allow_attributes,
				reason = "the generic expansion covers both signed and unsigned carriers"
			)]
			#[allow(
				clippy::cast_possible_truncation,
				clippy::cast_sign_loss,
				reason = "JavaScript normalizes the carrier to this type's value range"
			)]
			fn from_abi(raw: Self::Abi) -> Self {
				if raw == $sentinel {
					None
				} else {
					Some(sentinel_option!(@from_abi raw, $ty))
				}
			}
		}
	)+)+};
	(@into_abi $value:ident, bool, $carrier:ty) => {
		<$carrier>::from($value)
	};
	(@into_abi $value:ident, usize, $carrier:ty) => {
		{
			#[expect(
				clippy::cast_precision_loss,
				reason = "wasm32 usize values are exactly representable by f64"
			)]
			let carrier = $value as $carrier;
			carrier
		}
	};
	(@into_abi $value:ident, isize, $carrier:ty) => {
		{
			#[expect(
				clippy::cast_precision_loss,
				reason = "wasm32 isize values are exactly representable by f64"
			)]
			let carrier = $value as $carrier;
			carrier
		}
	};
	(@into_abi $value:ident, $ty:ident, $carrier:ty) => {
		$value as $carrier
	};
	(@from_abi $raw:ident, bool) => {
		$raw != 0
	};
	(@from_abi $raw:ident, $ty:ident) => {
		$raw as $ty
	};
}

macro_rules! indirect_option {
	($($ty:ty => {
		decode: $decode:literal,
		encode: $encode:literal,
		slots: [$($slot:literal),+ $(,)?],
	}),+ $(,)?) => {$(
		// SAFETY: The optional value is represented by a presence tag followed by
		// its payload slots and is returned through a hidden pointer.
		unsafe impl ReturnAbi for Option<$ty> {
			const MODE: ReturnMode = ReturnMode::Indirect;
		}

		// SAFETY: The decoder combines the presence tag and payload slots into one
		// optional JavaScript value.
		unsafe impl OptionIntoJS for $ty {
			const OPTION_JS_CONV: Option<IntoJsConv> = Some(
				IntoJsConv::new(indirect_option!(@decode $decode, [$($slot),+]))
					.with_embed(("js_sys", $decode)),
			);

			type OptionAbi = Option<Self>;

			fn option_into_abi(value: Option<Self>) -> Self::OptionAbi {
				value
			}
		}

		// SAFETY: The encoder writes a JavaScript value as a presence tag and the
		// payload slots expected by `Option<$ty>`.
		unsafe impl FromJS for Option<$ty> {
			const JS_CONV: Option<FromJsConv> = Some(
				indirect_option!(@output [$($slot),+])
					.sret(const_concat!("this.#jsEmbed.js_sys['", $encode, "']"))
					.with_embed(("js_sys", $encode)),
			);

			type Abi = Self;

			fn from_abi(raw: Self::Abi) -> Self {
				raw
			}
		}
	)+};
	(@decode $decode:literal, [$slot1:literal, $slot2:literal]) => {
		const_concat!("this.#jsEmbed.js_sys['", $decode, "']($slot1, $slot2)")
	};
	(@decode $decode:literal, [$slot1:literal, $slot2:literal, $slot3:literal]) => {
		const_concat!("this.#jsEmbed.js_sys['", $decode, "']($slot1, $slot2, $slot3)")
	};
	(@output [$slot1:literal, $slot2:literal]) => {
		FromJsConv::slot1($slot1).slot2($slot2)
	};
	(@output [$slot1:literal, $slot2:literal, $slot3:literal]) => {
		FromJsConv::slot1($slot1).slot2($slot2).slot3($slot3)
	};
}

slot!("i32", bool, u8, u16, u32, i8, i16, i32);
slot!("i64", u64, i64);
slot!("f32", f32);
slot!("f64", f64);
#[cfg(target_arch = "wasm32")]
slot!("i32", isize, usize);
#[cfg(target_arch = "wasm64")]
slot!("i64", isize, usize);

identity!(u8, u16, i8, i16, i32, i64, isize, f32, f64);
from_js!(bool, u32, u64, usize);

// SAFETY: The JavaScript conversion normalizes the `i32` Wasm slot to a
// `boolean`.
unsafe impl IntoJS for bool {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("!!$slot1"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: The JavaScript conversion reinterprets the `i32` Wasm slot as an
// unsigned 32-bit number.
unsafe impl IntoJS for u32 {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("$slot1 >>> 0"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: The JavaScript conversion normalizes the `i64` Wasm slot to an
// unsigned 64-bit `BigInt`.
unsafe impl IntoJS for u64 {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("BigInt.asUintN(64, $slot1)"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: On `wasm32`, `usize` uses an `i32` slot that JavaScript normalizes to
// an unsigned 32-bit number.
#[cfg(target_arch = "wasm32")]
unsafe impl IntoJS for usize {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("$slot1 >>> 0"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: On `wasm64`, `usize` uses an `i64` slot that JavaScript normalizes to
// an unsigned 64-bit `BigInt`.
#[cfg(target_arch = "wasm64")]
unsafe impl IntoJS for usize {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("BigInt.asUintN(64, $slot1)"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: `u128` is represented by its low and high 64-bit halves.
unsafe impl WasmAbi for u128 {
	type Slot1 = u64;
	type Slot2 = u64;
	type Slot3 = EmptySlot;
	type Slot4 = EmptySlot;

	#[expect(
		clippy::cast_possible_truncation,
		reason = "each cast extracts one 64-bit slot"
	)]
	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(
			self as u64,
			(self >> 64) as u64,
			EmptySlot::new(),
			EmptySlot::new(),
		)
	}

	fn join(slot1: Self::Slot1, slot2: Self::Slot2, _: Self::Slot3, _: Self::Slot4) -> Self {
		(Self::from(slot2) << 64) | Self::from(slot1)
	}
}

// SAFETY: `WasmRet<u128>` is returned through a hidden pointer.
unsafe impl ReturnAbi for u128 {
	const MODE: ReturnMode = ReturnMode::Indirect;
}

// SAFETY: The JavaScript decoder combines the low and high 64-bit slots into
// one unsigned `BigInt`.
unsafe impl IntoJS for u128 {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new("this.#jsEmbed.js_sys['numeric.u128.decode']($slot1, $slot2)")
			.with_embed(("js_sys", "numeric.u128.decode")),
	);

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: The JavaScript encoder splits an unsigned `BigInt` into the low and
// high 64-bit slots expected by `u128`.
unsafe impl FromJS for u128 {
	const JS_CONV: Option<FromJsConv> = Some(
		FromJsConv::slot1("$value")
			.slot2("$value >> 64n")
			.sret("this.#jsEmbed.js_sys['numeric.128.encode']")
			.with_embed(("js_sys", "numeric.128.encode")),
	);

	type Abi = Self;

	fn from_abi(raw: Self::Abi) -> Self {
		raw
	}
}

// SAFETY: `i128` is represented by its low unsigned and high signed 64-bit
// halves.
unsafe impl WasmAbi for i128 {
	type Slot1 = u64;
	type Slot2 = i64;
	type Slot3 = EmptySlot;
	type Slot4 = EmptySlot;

	#[expect(
		clippy::cast_possible_truncation,
		clippy::cast_sign_loss,
		reason = "each cast preserves the corresponding 64-bit bit pattern"
	)]
	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(
			self as u64,
			(self >> 64) as i64,
			EmptySlot::new(),
			EmptySlot::new(),
		)
	}

	fn join(slot1: Self::Slot1, slot2: Self::Slot2, _: Self::Slot3, _: Self::Slot4) -> Self {
		(Self::from(slot2) << 64) | Self::from(slot1)
	}
}

// SAFETY: `WasmRet<i128>` is returned through a hidden pointer.
unsafe impl ReturnAbi for i128 {
	const MODE: ReturnMode = ReturnMode::Indirect;
}

// SAFETY: The JavaScript decoder combines the low unsigned and high signed
// 64-bit slots into one signed `BigInt`.
unsafe impl IntoJS for i128 {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new("this.#jsEmbed.js_sys['numeric.i128.decode']($slot1, $slot2)")
			.with_embed(("js_sys", "numeric.i128.decode")),
	);

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: The JavaScript encoder splits a signed `BigInt` into the low
// unsigned and high signed 64-bit slots expected by `i128`.
unsafe impl FromJS for i128 {
	const JS_CONV: Option<FromJsConv> = Some(
		FromJsConv::slot1("$value")
			.slot2("$value >> 64n")
			.sret("this.#jsEmbed.js_sys['numeric.128.encode']")
			.with_embed(("js_sys", "numeric.128.encode")),
	);

	type Abi = Self;

	fn from_abi(raw: Self::Abi) -> Self {
		raw
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "numeric.u128.decode",
	"(lo, hi) => {{",
	"	return hi === 0n",
	"		? BigInt.asUintN(64, lo)",
	"		: BigInt.asUintN(64, lo) | (BigInt.asUintN(64, hi) << 64n)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "numeric.i128.decode",
	"(lo, hi) => {{",
	"	return BigInt.asUintN(64, lo) | (hi << 64n)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "numeric.128.encode",
	"(() => {{",
	"	const memory = this.#memory",
	"	let buffer = memory.buffer",
	"	let view = new DataView(buffer)",
	"	return (lo, hi, out) => {{",
	"		if (out + 16 > buffer.byteLength) {{",
	"			buffer = memory.buffer",
	"			view = new DataView(buffer)",
	"		}}",
	"		view.setBigInt64(out, lo, true)",
	"		view.setBigInt64(out + 8, hi, true)",
	"	}}",
	"}})()",
);

// Outside the value range of every type encoded by the `i32` sentinel scheme.
const I32_OPTION_SENTINEL: i32 = 0x00ff_ffff;
// `Number.MAX_SAFE_INTEGER` cannot collide with a `wasm32` `usize`, `i32`,
// `u32`, or widened `f32` value.
const F64_OPTION_SENTINEL: f64 = 9_007_199_254_740_991.0;

sentinel_option! {
	carrier: i32,
	sentinel: I32_OPTION_SENTINEL,
	js_sentinel: "0x00ff_ffff",
	types: [
		[i8, u8, i16, u16] => {
			to_js: "$slot1",
			from_js: "value",
		},
		[bool] => {
			to_js: "$slot1 !== 0",
			from_js: "value ? 1 : 0",
		},
	],
}

sentinel_option! {
	carrier: f64,
	sentinel: F64_OPTION_SENTINEL,
	js_sentinel: "Number.MAX_SAFE_INTEGER",
	types: [
		[i32] => {
			to_js: "$slot1",
			from_js: "value >> 0",
		},
		[u32] => {
			to_js: "$slot1",
			from_js: "value >>> 0",
		},
		[f32] => {
			to_js: "$slot1",
			from_js: "Math.fround(value)",
		},
	],
}

#[cfg(target_arch = "wasm32")]
sentinel_option! {
	carrier: f64,
	sentinel: F64_OPTION_SENTINEL,
	js_sentinel: "Number.MAX_SAFE_INTEGER",
	types: [
		[isize] => {
			to_js: "$slot1",
			from_js: "value >> 0",
		},
		[usize] => {
			to_js: "$slot1",
			from_js: "value >>> 0",
		},
	],
}

indirect_option! {
	f64 => {
		decode: "optional.f64.decode",
		encode: "optional.f64.encode",
		slots: ["$value == null ? 0 : 1", "$value == null ? 0 : $value"],
	},
	i64 => {
		decode: "optional.i64.decode",
		encode: "optional.i64.encode",
		slots: ["$value == null ? 0 : 1", "$value == null ? 0n : $value"],
	},
	u64 => {
		decode: "optional.u64.decode",
		encode: "optional.u64.encode",
		slots: ["$value == null ? 0 : 1", "$value == null ? 0n : $value"],
	},
}

#[cfg(target_arch = "wasm64")]
indirect_option! {
	isize => {
		decode: "optional.i64.decode",
		encode: "optional.i64.encode",
		slots: ["$value == null ? 0 : 1", "$value == null ? 0n : $value"],
	},
	usize => {
		decode: "optional.u64.decode",
		encode: "optional.u64.encode",
		slots: ["$value == null ? 0 : 1", "$value == null ? 0n : $value"],
	},
}

indirect_option! {
	u128 => {
		decode: "optional.u128.decode",
		encode: "optional.128.encode",
		slots: [
			"$value == null ? 0 : 1",
			"$value == null ? 0n : $value",
			"$value == null ? 0n : $value >> 64n",
		],
	},
	i128 => {
		decode: "optional.i128.decode",
		encode: "optional.128.encode",
		slots: [
			"$value == null ? 0 : 1",
			"$value == null ? 0n : $value",
			"$value == null ? 0n : $value >> 64n",
		],
	},
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.f64.decode",
	"(isSome, value) => {{",
	"	if (isSome === 0) return undefined",
	"	return value",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.f64.encode",
	"(() => {{",
	"	const memory = this.#memory",
	"	let buffer = memory.buffer",
	"	let view = new DataView(buffer)",
	"	return (isSome, value, out) => {{",
	"		if (out + 16 > buffer.byteLength) {{",
	"			buffer = memory.buffer",
	"			view = new DataView(buffer)",
	"		}}",
	"		view.setUint32(out, isSome, true)",
	"		view.setFloat64(out + 8, value, true)",
	"	}}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.i64.decode",
	"(isSome, value) => {{",
	"	if (isSome === 0) return undefined",
	"	return value",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.i64.encode",
	"(() => {{",
	"	const memory = this.#memory",
	"	let buffer = memory.buffer",
	"	let view = new DataView(buffer)",
	"	return (isSome, value, out) => {{",
	"		if (out + 16 > buffer.byteLength) {{",
	"			buffer = memory.buffer",
	"			view = new DataView(buffer)",
	"		}}",
	"		view.setUint32(out, isSome, true)",
	"		view.setBigInt64(out + 8, value, true)",
	"	}}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.u64.decode",
	"(isSome, value) => {{",
	"	if (isSome === 0) return undefined",
	"	return BigInt.asUintN(64, value)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.u64.encode",
	"(() => {{",
	"	const memory = this.#memory",
	"	let buffer = memory.buffer",
	"	let view = new DataView(buffer)",
	"	return (isSome, value, out) => {{",
	"		if (out + 16 > buffer.byteLength) {{",
	"			buffer = memory.buffer",
	"			view = new DataView(buffer)",
	"		}}",
	"		view.setUint32(out, isSome, true)",
	"		view.setBigUint64(out + 8, value, true)",
	"	}}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.u128.decode",
	"(isSome, lo, hi) => {{",
	"	if (isSome === 0) return undefined",
	"	return hi === 0n",
	"		? BigInt.asUintN(64, lo)",
	"		: BigInt.asUintN(64, lo) | (BigInt.asUintN(64, hi) << 64n)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.i128.decode",
	"(isSome, lo, hi) => {{",
	"	if (isSome === 0) return undefined",
	"	return BigInt.asUintN(64, lo) | (hi << 64n)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "optional.128.encode",
	"(() => {{",
	"	const memory = this.#memory",
	"	let buffer = memory.buffer",
	"	let view = new DataView(buffer)",
	"	return (isSome, lo, hi, out) => {{",
	"		if (out + 24 > buffer.byteLength) {{",
	"			buffer = memory.buffer",
	"			view = new DataView(buffer)",
	"		}}",
	"		view.setUint32(out, isSome, true)",
	"		view.setBigInt64(out + 8, lo, true)",
	"		view.setBigInt64(out + 16, hi, true)",
	"	}}",
	"}})()",
);
