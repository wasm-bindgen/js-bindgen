use js_bindgen_test::test;
use js_sys::{JsValue, Promise};

#[test]
async fn promise() {
	let value = Promise::resolve(&JsValue::NULL).await.unwrap();

	assert_eq!(value, JsValue::NULL);
}

#[test]
#[should_panic(expected = "async panic")]
async fn should_panic() {
	Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
	panic!("async panic");
}

mod first {
	use js_bindgen_test::test;
	use js_sys::{JsValue, Promise};

	#[test]
	async fn same_name() {
		Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
	}
}

mod second {
	use js_bindgen_test::test;
	use js_sys::{JsValue, Promise};

	#[test]
	async fn same_name() {
		Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
	}
}
