use core::cell::Cell;
use std::rc::Rc;

use js_bindgen_test::test;
use js_sys::{JsString, JsValue, Promise, future_to_promise, spawn_local};

#[test]
async fn promise_to_future() {
	let value = Promise::resolve(&JsValue::NULL).await.unwrap();

	assert_eq!(value, JsValue::NULL);
	assert_eq!(
		Promise::reject(&JsValue::UNDEFINED).await.unwrap_err(),
		JsValue::UNDEFINED
	);
}

#[test]
async fn rust_future_to_promise() {
	let resolved = JsValue::from(JsString::from("resolved"));
	let value = resolved.clone();
	assert_eq!(
		future_to_promise(async move { Ok(value) }).await.unwrap(),
		resolved
	);

	let rejected = JsValue::from(JsString::from("rejected"));
	let error = rejected.clone();
	assert_eq!(
		future_to_promise::<JsValue>(async move { Err(error) })
			.await
			.unwrap_err(),
		rejected
	);
}

#[test]
async fn spawn_local_is_deferred() {
	let completed = Rc::new(Cell::new(false));
	let task_completed = Rc::clone(&completed);
	spawn_local(async move {
		task_completed.set(true);
	});

	assert!(!completed.get());
	Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
	assert!(completed.get());
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
		// Regression test: generated async-test exports include their module path.
		Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
	}
}

mod second {
	use js_bindgen_test::test;
	use js_sys::{JsValue, Promise};

	#[test]
	async fn same_name() {
		// Keep this name equal to `first::same_name` to exercise macro hygiene.
		Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
	}
}
