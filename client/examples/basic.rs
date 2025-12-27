use web_sys::console;
use web_sys::js_sys::JsValue;

#[unsafe(no_mangle)]
extern "C" fn foo() {
	console::log2(&JsValue::UNDEFINED, &JsValue::UNDEFINED);
}
