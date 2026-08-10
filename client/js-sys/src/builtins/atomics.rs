use crate::{
	BigInt64Array, BigUint64Array, Int8Array, Int16Array, Int32Array, JsString, JsValue, Object,
	Uint8Array, Uint16Array, Uint32Array, js_sys,
};

#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod Atomics {
	use super::*;

	#[js_sys(js_sys = crate)]
	extern "js-sys" {
		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
		#[js_sys(extends = Object)]
		#[derive(Clone, Debug, PartialEq)]
		pub type WaitAsyncResult;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
		#[must_use]
		#[js_sys(getter = "async")]
		pub fn async_(self: &WaitAsyncResult) -> bool;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
		#[must_use]
		#[js_sys(getter)]
		pub fn value(self: &WaitAsyncResult) -> JsValue;
	}

	mod sealed {
		pub trait Sealed {}
	}

	mod raw {
		use super::*;

		macro_rules! operations {
			(
				$value:ty,
				add = $add:ident,
				and = $and:ident,
				compare_exchange = $compare_exchange:ident,
				exchange = $exchange:ident,
				load = $load:ident,
				or = $or:ident,
				store = $store:ident,
				sub = $sub:ident,
				xor = $xor:ident,
			) => {
				#[js_sys(js_sys = crate, namespace = "Atomics")]
				extern "js-sys" {
					#[js_sys(js_name = "add")]
					pub(super) fn $add(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "and")]
					pub(super) fn $and(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "compareExchange")]
					pub(super) fn $compare_exchange(
						array: &JsValue,
						index: f64,
						expected: $value,
						replacement: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "exchange")]
					pub(super) fn $exchange(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "load")]
					pub(super) fn $load(array: &JsValue, index: f64) -> Result<$value, JsValue>;

					#[js_sys(js_name = "or")]
					pub(super) fn $or(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "store")]
					pub(super) fn $store(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "sub")]
					pub(super) fn $sub(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;

					#[js_sys(js_name = "xor")]
					pub(super) fn $xor(
						array: &JsValue,
						index: f64,
						value: $value,
					) -> Result<$value, JsValue>;
				}
			};
		}

		operations!(
			f64,
			add = add_number,
			and = and_number,
			compare_exchange = compare_exchange_number,
			exchange = exchange_number,
			load = load_number,
			or = or_number,
			store = store_number,
			sub = sub_number,
			xor = xor_number,
		);
		operations!(
			i64,
			add = add_i64,
			and = and_i64,
			compare_exchange = compare_exchange_i64,
			exchange = exchange_i64,
			load = load_i64,
			or = or_i64,
			store = store_i64,
			sub = sub_i64,
			xor = xor_i64,
		);
		operations!(
			u64,
			add = add_u64,
			and = and_u64,
			compare_exchange = compare_exchange_u64,
			exchange = exchange_u64,
			load = load_u64,
			or = or_u64,
			store = store_u64,
			sub = sub_u64,
			xor = xor_u64,
		);

		#[js_sys(js_sys = crate, namespace = "Atomics")]
		extern "js-sys" {
			#[js_sys(js_name = "isLockFree")]
			pub(super) fn is_lock_free(size: f64) -> bool;

			pub(super) fn notify(array: &Int32Array, index: f64) -> Result<f64, JsValue>;

			#[js_sys(js_name = "notify")]
			pub(super) fn notify_with_count(
				array: &Int32Array,
				index: f64,
				count: f64,
			) -> Result<f64, JsValue>;

			#[js_sys(js_name = "notify")]
			pub(super) fn notify_bigint(array: &BigInt64Array, index: f64) -> Result<f64, JsValue>;

			#[js_sys(js_name = "notify")]
			pub(super) fn notify_bigint_with_count(
				array: &BigInt64Array,
				index: f64,
				count: f64,
			) -> Result<f64, JsValue>;

			pub(super) fn pause() -> Result<(), JsValue>;

			#[js_sys(js_name = "pause")]
			pub(super) fn pause_with_hint(duration_hint: f64) -> Result<(), JsValue>;

			pub(super) fn wait(
				array: &Int32Array,
				index: f64,
				value: i32,
			) -> Result<JsString, JsValue>;

			#[js_sys(js_name = "wait")]
			pub(super) fn wait_with_timeout(
				array: &Int32Array,
				index: f64,
				value: i32,
				timeout: f64,
			) -> Result<JsString, JsValue>;

			#[js_sys(js_name = "wait")]
			pub(super) fn wait_bigint(
				array: &BigInt64Array,
				index: f64,
				value: i64,
			) -> Result<JsString, JsValue>;

			#[js_sys(js_name = "wait")]
			pub(super) fn wait_bigint_with_timeout(
				array: &BigInt64Array,
				index: f64,
				value: i64,
				timeout: f64,
			) -> Result<JsString, JsValue>;

			#[js_sys(js_name = "waitAsync")]
			pub(super) fn wait_async(
				array: &Int32Array,
				index: f64,
				value: i32,
			) -> Result<WaitAsyncResult, JsValue>;

			#[js_sys(js_name = "waitAsync")]
			pub(super) fn wait_async_with_timeout(
				array: &Int32Array,
				index: f64,
				value: i32,
				timeout: f64,
			) -> Result<WaitAsyncResult, JsValue>;

			#[js_sys(js_name = "waitAsync")]
			pub(super) fn wait_async_bigint(
				array: &BigInt64Array,
				index: f64,
				value: i64,
			) -> Result<WaitAsyncResult, JsValue>;

			#[js_sys(js_name = "waitAsync")]
			pub(super) fn wait_async_bigint_with_timeout(
				array: &BigInt64Array,
				index: f64,
				value: i64,
				timeout: f64,
			) -> Result<WaitAsyncResult, JsValue>;
		}
	}

	#[doc(hidden)]
	pub trait AtomicInteger: sealed::Sealed + AsRef<JsValue> {
		type Value: Copy;

		fn atomic_add(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
		fn atomic_and(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
		fn atomic_compare_exchange(
			&self,
			index: f64,
			expected: Self::Value,
			replacement: Self::Value,
		) -> Result<Self::Value, JsValue>;
		fn atomic_exchange(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
		fn atomic_load(&self, index: f64) -> Result<Self::Value, JsValue>;
		fn atomic_or(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
		fn atomic_store(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
		fn atomic_sub(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
		fn atomic_xor(&self, index: f64, value: Self::Value) -> Result<Self::Value, JsValue>;
	}

	macro_rules! impl_number {
		($array:ty, $value:ty, $($lint:path),+ $(,)?) => {
			impl sealed::Sealed for $array {}

			#[expect(
				$($lint),+,
				reason = "JavaScript returns a value represented by the typed array element"
			)]
			impl AtomicInteger for $array {
				type Value = $value;

				fn atomic_add(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::add_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}

				fn atomic_and(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::and_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}

				fn atomic_compare_exchange(
					&self,
					index: f64,
					expected: Self::Value,
					replacement: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::compare_exchange_number(
						self.as_ref(),
						index,
						f64::from(expected),
						f64::from(replacement),
					)
					.map(|value| value as $value)
				}

				fn atomic_exchange(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::exchange_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}

				fn atomic_load(&self, index: f64) -> Result<Self::Value, JsValue> {
					raw::load_number(self.as_ref(), index).map(|value| value as $value)
				}

				fn atomic_or(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::or_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}

				fn atomic_store(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::store_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}

				fn atomic_sub(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::sub_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}

				fn atomic_xor(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::xor_number(self.as_ref(), index, f64::from(value))
						.map(|value| value as $value)
				}
			}
		};
	}

	macro_rules! impl_bigint {
		(
			$array:ty,
			$value:ty,
			add = $add:ident,
			and = $and:ident,
			compare_exchange = $compare_exchange:ident,
			exchange = $exchange:ident,
			load = $load:ident,
			or = $or:ident,
			store = $store:ident,
			sub = $sub:ident,
			xor = $xor:ident,
		) => {
			impl sealed::Sealed for $array {}

			impl AtomicInteger for $array {
				type Value = $value;

				fn atomic_add(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$add(self.as_ref(), index, value)
				}

				fn atomic_and(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$and(self.as_ref(), index, value)
				}

				fn atomic_compare_exchange(
					&self,
					index: f64,
					expected: Self::Value,
					replacement: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$compare_exchange(self.as_ref(), index, expected, replacement)
				}

				fn atomic_exchange(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$exchange(self.as_ref(), index, value)
				}

				fn atomic_load(&self, index: f64) -> Result<Self::Value, JsValue> {
					raw::$load(self.as_ref(), index)
				}

				fn atomic_or(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$or(self.as_ref(), index, value)
				}

				fn atomic_store(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$store(self.as_ref(), index, value)
				}

				fn atomic_sub(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$sub(self.as_ref(), index, value)
				}

				fn atomic_xor(
					&self,
					index: f64,
					value: Self::Value,
				) -> Result<Self::Value, JsValue> {
					raw::$xor(self.as_ref(), index, value)
				}
			}
		};
	}

	impl_number!(Int8Array, i8, clippy::cast_possible_truncation);
	impl_number!(
		Uint8Array,
		u8,
		clippy::cast_possible_truncation,
		clippy::cast_sign_loss,
	);
	impl_number!(Int16Array, i16, clippy::cast_possible_truncation);
	impl_number!(
		Uint16Array,
		u16,
		clippy::cast_possible_truncation,
		clippy::cast_sign_loss,
	);
	impl_number!(Int32Array, i32, clippy::cast_possible_truncation);
	impl_number!(
		Uint32Array,
		u32,
		clippy::cast_possible_truncation,
		clippy::cast_sign_loss,
	);
	impl_bigint!(
		BigInt64Array,
		i64,
		add = add_i64,
		and = and_i64,
		compare_exchange = compare_exchange_i64,
		exchange = exchange_i64,
		load = load_i64,
		or = or_i64,
		store = store_i64,
		sub = sub_i64,
		xor = xor_i64,
	);
	impl_bigint!(
		BigUint64Array,
		u64,
		add = add_u64,
		and = and_u64,
		compare_exchange = compare_exchange_u64,
		exchange = exchange_u64,
		load = load_u64,
		or = or_u64,
		store = store_u64,
		sub = sub_u64,
		xor = xor_u64,
	);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/add)
	pub fn add<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_add(index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/and)
	pub fn and<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_and(index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/compareExchange)
	pub fn compare_exchange<A: AtomicInteger>(
		array: &A,
		index: f64,
		expected: A::Value,
		replacement: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_compare_exchange(index, expected, replacement)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/exchange)
	pub fn exchange<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_exchange(index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/isLockFree)
	#[must_use]
	pub fn is_lock_free(size: f64) -> bool {
		raw::is_lock_free(size)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/load)
	pub fn load<A: AtomicInteger>(array: &A, index: f64) -> Result<A::Value, JsValue> {
		array.atomic_load(index)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/notify)
	pub fn notify(array: &Int32Array, index: f64) -> Result<f64, JsValue> {
		raw::notify(array, index)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/notify)
	pub fn notify_with_count(array: &Int32Array, index: f64, count: f64) -> Result<f64, JsValue> {
		raw::notify_with_count(array, index, count)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/notify)
	pub fn notify_bigint(array: &BigInt64Array, index: f64) -> Result<f64, JsValue> {
		raw::notify_bigint(array, index)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/notify)
	pub fn notify_bigint_with_count(
		array: &BigInt64Array,
		index: f64,
		count: f64,
	) -> Result<f64, JsValue> {
		raw::notify_bigint_with_count(array, index, count)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/or)
	pub fn or<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_or(index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/pause)
	pub fn pause() -> Result<(), JsValue> {
		raw::pause()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/pause)
	pub fn pause_with_hint(duration_hint: f64) -> Result<(), JsValue> {
		raw::pause_with_hint(duration_hint)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/store)
	pub fn store<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_store(index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait)
	pub fn wait(array: &Int32Array, index: f64, value: i32) -> Result<JsString, JsValue> {
		raw::wait(array, index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait)
	pub fn wait_with_timeout(
		array: &Int32Array,
		index: f64,
		value: i32,
		timeout: f64,
	) -> Result<JsString, JsValue> {
		raw::wait_with_timeout(array, index, value, timeout)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait)
	pub fn wait_bigint(array: &BigInt64Array, index: f64, value: i64) -> Result<JsString, JsValue> {
		raw::wait_bigint(array, index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait)
	pub fn wait_bigint_with_timeout(
		array: &BigInt64Array,
		index: f64,
		value: i64,
		timeout: f64,
	) -> Result<JsString, JsValue> {
		raw::wait_bigint_with_timeout(array, index, value, timeout)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
	pub fn wait_async(
		array: &Int32Array,
		index: f64,
		value: i32,
	) -> Result<WaitAsyncResult, JsValue> {
		raw::wait_async(array, index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
	pub fn wait_async_with_timeout(
		array: &Int32Array,
		index: f64,
		value: i32,
		timeout: f64,
	) -> Result<WaitAsyncResult, JsValue> {
		raw::wait_async_with_timeout(array, index, value, timeout)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
	pub fn wait_async_bigint(
		array: &BigInt64Array,
		index: f64,
		value: i64,
	) -> Result<WaitAsyncResult, JsValue> {
		raw::wait_async_bigint(array, index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync)
	pub fn wait_async_bigint_with_timeout(
		array: &BigInt64Array,
		index: f64,
		value: i64,
		timeout: f64,
	) -> Result<WaitAsyncResult, JsValue> {
		raw::wait_async_bigint_with_timeout(array, index, value, timeout)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/sub)
	pub fn sub<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_sub(index, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/xor)
	pub fn xor<A: AtomicInteger>(
		array: &A,
		index: f64,
		value: A::Value,
	) -> Result<A::Value, JsValue> {
		array.atomic_xor(index, value)
	}
}
