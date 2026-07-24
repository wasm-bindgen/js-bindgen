use js_sys::js_sys;

#[js_sys]
fn i32_identity(value: i32) -> i32 {
	value
}
