use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use js_sys::js_sys;

js_bindgen::embed_js!(module = "conv", name = "bench", "(value) => value");
js_bindgen::embed_js!(
	module = "conv",
	name = "bench_str",
	"(value) => !!value"
);

fn bench_conv_u128(c: &mut Criterion) {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "bench")]
		fn conv_u128(value: u128) -> u128;
		#[js_sys(js_embed = "bench_str")]
		fn conv_str(value: &str) -> bool;
	}

	const SMALL: u128 = 4242;
	const WIDE: u128 = u128::MAX;

	let mut group = c.benchmark_group("conv_u128");
	group.bench_function("small", |b| {
		b.iter(|| black_box(conv_u128(black_box(SMALL))))
	});
	group.bench_function("wide", |b| b.iter(|| black_box(conv_u128(black_box(WIDE)))));
	group.finish();

	c.bench_function("conv_str", |b| {
		b.iter(|| black_box(conv_str(black_box("hello world"))))
	});
}

criterion_group!(benches, bench_conv_u128);
criterion_main!(benches);
