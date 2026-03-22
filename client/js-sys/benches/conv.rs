use js_bindgen_test::{Criterion, bench};
use js_sys::js_sys;

js_bindgen::embed_js!(module = "conv", name = "bench", "(value) => value");

#[bench]
fn bench_conv_u128(c: &mut Criterion) {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "bench")]
		fn conv_u128(value: u128) -> u128;
	}

	c.bench_function("bench_conv_u128", |b| {
		b.iter(|| {
			assert_eq!(conv_u128(4242), 4242);
		})
	});
}
