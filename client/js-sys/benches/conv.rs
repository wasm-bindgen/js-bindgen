use js_bindgen_test::{Criterion, criterion_group, criterion_main};
use js_sys::js_sys;

js_bindgen::embed_js!(module = "conv", name = "bench", "(value) => value");

fn bench_conv_u128(c: &mut Criterion) {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "bench")]
		fn conv_u128(value: u128) -> u128;
	}

	c.bench_function("bench_conv_u128", |b| {
		b.iter(|| {
			assert_eq!(conv_u128(4242), 4242);
		});
	});
}

fn bench_conv_i128(c: &mut Criterion) {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "bench")]
		fn conv_i128(value: i128) -> i128;
	}

	c.bench_function("bench_conv_i128", |b| {
		b.iter(|| {
			assert_eq!(conv_i128(4242), 4242);
		});
	});
}

criterion_group!(benches, bench_conv_u128, bench_conv_i128);
criterion_main!(benches);
