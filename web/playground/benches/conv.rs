#![feature(test)]
extern crate test;

use test::Bencher;
use js_sys::js_sys;

js_bindgen::embed_js!(module = "conv", name = "bench", "(value) => value");

#[bench]
fn bench_conv_u128(b: &mut Bencher) {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "bench")]
		fn conv_u128(value: u128) -> u128;
	}

	b.iter(|| {
		assert_eq!(conv_u128(4242), 4242);
	});
}
