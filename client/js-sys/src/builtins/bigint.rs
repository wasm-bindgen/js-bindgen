#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_name = "BigInt")]
	pub type JsBigInt;
}
