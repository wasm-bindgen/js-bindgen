use super::Iterable;
use crate::js_sys;
use crate::util::{PtrConst, PtrLength};

#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod Math {
	use super::*;

	#[js_sys(js_sys = crate, namespace = "Math")]
	extern "js-sys" {
		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/E)
		#[must_use]
		#[js_sys(getter = "E")]
		pub fn e() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/LN10)
		#[must_use]
		#[js_sys(getter = "LN10")]
		pub fn ln_10() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/LN2)
		#[must_use]
		#[js_sys(getter = "LN2")]
		pub fn ln_2() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/LOG10E)
		#[must_use]
		#[js_sys(getter = "LOG10E")]
		pub fn log_10_e() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/LOG2E)
		#[must_use]
		#[js_sys(getter = "LOG2E")]
		pub fn log_2_e() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/PI)
		#[must_use]
		#[js_sys(getter = "PI")]
		pub fn pi() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/SQRT1_2)
		#[must_use]
		#[js_sys(getter = "SQRT1_2")]
		pub fn sqrt_1_2() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/SQRT2)
		#[must_use]
		#[js_sys(getter = "SQRT2")]
		pub fn sqrt_2() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/abs)
		#[must_use]
		pub fn abs(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acos)
		#[must_use]
		pub fn acos(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acosh)
		#[must_use]
		pub fn acosh(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asin)
		#[must_use]
		pub fn asin(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asinh)
		#[must_use]
		pub fn asinh(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atan)
		#[must_use]
		pub fn atan(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atan2)
		#[must_use]
		pub fn atan2(y: f64, x: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atanh)
		#[must_use]
		pub fn atanh(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cbrt)
		#[must_use]
		pub fn cbrt(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/ceil)
		#[must_use]
		pub fn ceil(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/clz32)
		#[must_use]
		pub fn clz32(value: u32) -> u32;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cos)
		#[must_use]
		pub fn cos(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cosh)
		#[must_use]
		pub fn cosh(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/exp)
		#[must_use]
		pub fn exp(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/expm1)
		#[must_use]
		pub fn expm1(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/f16round)
		#[must_use]
		pub fn f16round(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/floor)
		#[must_use]
		pub fn floor(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/fround)
		#[must_use]
		pub fn fround(value: f64) -> f32;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/hypot)
		#[must_use]
		pub fn hypot(x: f64, y: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/imul)
		#[must_use]
		pub fn imul(left: i32, right: i32) -> i32;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log)
		#[must_use]
		pub fn log(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log1p)
		#[must_use]
		pub fn log1p(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log10)
		#[must_use]
		pub fn log10(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log2)
		#[must_use]
		pub fn log2(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/max)
		#[must_use]
		pub fn max(x: f64, y: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/min)
		#[must_use]
		pub fn min(x: f64, y: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/pow)
		#[must_use]
		pub fn pow(base: f64, exponent: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/random)
		#[must_use]
		pub fn random() -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/round)
		#[must_use]
		pub fn round(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sign)
		#[must_use]
		pub fn sign(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sin)
		#[must_use]
		pub fn sin(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sinh)
		#[must_use]
		pub fn sinh(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sqrt)
		#[must_use]
		pub fn sqrt(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sumPrecise)
		#[js_sys(js_name = "sumPrecise")]
		pub fn sum_precise<I: Iterable>(
			#[js_sys(type = &crate::JsValue)] numbers: &I,
		) -> Result<f64, crate::JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/tan)
		#[must_use]
		pub fn tan(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/tanh)
		#[must_use]
		pub fn tanh(value: f64) -> f64;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/trunc)
		#[must_use]
		pub fn trunc(value: f64) -> f64;
	}

	#[js_sys(js_sys = crate)]
	extern "js-sys" {
		#[js_sys(js_embed = "math.hypot")]
		unsafe fn hypot_many_raw(values: PtrConst<f64>, len: PtrLength<f64>) -> f64;

		#[js_sys(js_embed = "math.max")]
		unsafe fn max_many_raw(values: PtrConst<f64>, len: PtrLength<f64>) -> f64;

		#[js_sys(js_embed = "math.min")]
		unsafe fn min_many_raw(values: PtrConst<f64>, len: PtrLength<f64>) -> f64;
	}

	js_bindgen::embed_js!(
		module = "js_sys",
		name = "math.hypot",
		required_embeds = [("js_sys", "view.getFloat64")],
		"(ptr, len) => Math.hypot(...this.#jsEmbed.js_sys['view.getFloat64'](ptr, len))",
	);

	js_bindgen::embed_js!(
		module = "js_sys",
		name = "math.max",
		required_embeds = [("js_sys", "view.getFloat64")],
		"(ptr, len) => Math.max(...this.#jsEmbed.js_sys['view.getFloat64'](ptr, len))",
	);

	js_bindgen::embed_js!(
		module = "js_sys",
		name = "math.min",
		required_embeds = [("js_sys", "view.getFloat64")],
		"(ptr, len) => Math.min(...this.#jsEmbed.js_sys['view.getFloat64'](ptr, len))",
	);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/hypot)
	#[must_use]
	pub fn hypot_many(values: &[f64]) -> f64 {
		// SAFETY: The pointer and length describe `values` for the duration of the
		// call.
		unsafe { hypot_many_raw(PtrConst::new(values), PtrLength::new(values)) }
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/max)
	#[must_use]
	pub fn max_many(values: &[f64]) -> f64 {
		// SAFETY: The pointer and length describe `values` for the duration of the
		// call.
		unsafe { max_many_raw(PtrConst::new(values), PtrLength::new(values)) }
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/min)
	#[must_use]
	pub fn min_many(values: &[f64]) -> f64 {
		// SAFETY: The pointer and length describe `values` for the duration of the
		// call.
		unsafe { min_many_raw(PtrConst::new(values), PtrLength::new(values)) }
	}
}
