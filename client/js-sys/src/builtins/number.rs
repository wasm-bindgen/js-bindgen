#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_name = "Number")]
	pub type JsNumber<T = f64>;
}
