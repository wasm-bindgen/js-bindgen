use js_bindgen_test::{Criterion, bench};
use js_sys::js_sys;

js_bindgen::embed_js!(module = "interop", name = "bench", "(value) => value");

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "bench")]
	fn interop(value: u128) -> u128;
}

#[bench]
fn bench_interop_u128(c: &mut Criterion) {
	c.bench_function("bench_interop_u128", |b| {
		b.iter(|| {
			assert_eq!(interop(4242), 4242);
		})
	});
}
