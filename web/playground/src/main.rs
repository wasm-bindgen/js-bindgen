use std::time::Instant;

use js_sys::hazard::JsCast;
use js_sys::{Function, JsString, JsValue, Promise, closure};

fn main() {
	let ins = Instant::now();

	let value = JsString::from("hahaha").into();
	let executor = closure!(dyn FnMut(Function, Function), move |resolve, _reject| {
		resolve
			.call(&JsValue::UNDEFINED, core::slice::from_ref(&value))
			.unwrap();
	});
	let f1 = async {
		let ret = Promise::new(&executor).await.unwrap();
		String::from(&JsString::unchecked_from(ret))
	};
	let f2 = async move { 1 };

	let f1 = js_sys::block_on(f1);
	let f2 = js_sys::block_on(f2);

	println!("future1: {f1}, future2: {f2:?}, cost: {:?}", ins.elapsed());
}

#[cfg(test)]
mod tests {
	use js_sys::{JsValue, Promise, block_on};

	#[test]
	#[should_panic]
	fn test1() {
		panic!()
	}

	#[test]
	#[ignore = "test2"]
	fn test2() {
		panic!()
	}

	#[test]
	fn test3() {}

	#[test]
	fn jspi_block_on() {
		let value = String::from("resolved");
		let output = block_on(async {
			Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
			value.as_str()
		});

		assert_eq!(output, "resolved");
	}

	#[test]
	#[should_panic(expected = "JSPI panic")]
	fn jspi_should_panic() {
		block_on(async {
			Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
			panic!("JSPI panic");
		});
	}
}
