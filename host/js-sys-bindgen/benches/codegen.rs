use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use js_sys_bindgen::r#macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::File;

fn bench_macro_basic_function(c: &mut Criterion) {
	let attr = TokenStream::new();
	let input = quote! {
		extern "js-sys" {
			pub fn log(data: &JsValue);
		}
	};

	c.bench_function("macro_basic_function", |b| {
		b.iter(|| {
			let foreign_mod = syn::parse2(input.clone()).unwrap();
			black_box(r#macro(attr.clone(), foreign_mod, None).unwrap());
		});
	});
}

fn bench_macro_two_parameters(c: &mut Criterion) {
	let attr = TokenStream::new();
	let input = quote! {
		extern "js-sys" {
			pub fn log(data1: &JsValue, data2: &JsValue);
		}
	};

	c.bench_function("macro_two_parameters", |b| {
		b.iter(|| {
			let foreign_mod = syn::parse2(input.clone()).unwrap();
			black_box(r#macro(attr.clone(), foreign_mod, None).unwrap());
		});
	});
}

fn bench_macro_with_return(c: &mut Criterion) {
	let attr = TokenStream::new();
	let input = quote! {
		extern "js-sys" {
			pub fn is_nan() -> JsValue;
		}
	};

	c.bench_function("macro_with_return", |b| {
		b.iter(|| {
			let foreign_mod = syn::parse2(input.clone()).unwrap();
			black_box(r#macro(attr.clone(), foreign_mod, None).unwrap());
		});
	});
}

fn bench_macro_with_namespace(c: &mut Criterion) {
	let attr = quote! { namespace = "console" };
	let input = quote! {
		extern "js-sys" {
			pub fn log(data: &JsValue);
		}
	};

	c.bench_function("macro_with_namespace", |b| {
		b.iter(|| {
			let foreign_mod = syn::parse2(input.clone()).unwrap();
			black_box(r#macro(attr.clone(), foreign_mod, None).unwrap());
		});
	});
}

fn bench_macro_multiple_functions(c: &mut Criterion) {
	let attr = TokenStream::new();
	let input = quote! {
		extern "js-sys" {
			pub fn alert(message: &JsValue);
			pub fn log(data: &JsValue);
			pub fn warn(data: &JsValue);
			pub fn error(data: &JsValue);
			pub fn is_nan() -> JsValue;
		}
	};

	c.bench_function("macro_multiple_functions", |b| {
		b.iter(|| {
			let foreign_mod = syn::parse2(input.clone()).unwrap();
			black_box(r#macro(attr.clone(), foreign_mod, None).unwrap());
		});
	});
}

fn bench_macro_type_definition(c: &mut Criterion) {
	let attr = TokenStream::new();
	let input = quote! {
		extern "js-sys" {
			pub type JsTest;
		}
	};

	c.bench_function("macro_type_definition", |b| {
		b.iter(|| {
			let foreign_mod = syn::parse2(input.clone()).unwrap();
			black_box(r#macro(attr.clone(), foreign_mod, None).unwrap());
		});
	});
}

fn bench_macro_output_unparse(c: &mut Criterion) {
	let attr = TokenStream::new();
	let input = quote! {
		extern "js-sys" {
			pub fn alert(message: &JsValue);
			pub fn log(data: &JsValue);
			pub fn warn(data: &JsValue);
			pub fn error(data: &JsValue);
			pub fn is_nan() -> JsValue;
		}
	};

	let foreign_mod = syn::parse2(input).unwrap();
	let output: TokenStream = r#macro(attr, foreign_mod, None).unwrap();
	let file: File = syn::parse2(output).unwrap();

	c.bench_function("macro_output_unparse", |b| {
		b.iter(|| {
			black_box(prettyplease::unparse(&file));
		});
	});
}

criterion_group!(
	benches,
	bench_macro_basic_function,
	bench_macro_two_parameters,
	bench_macro_with_return,
	bench_macro_with_namespace,
	bench_macro_multiple_functions,
	bench_macro_type_definition,
	bench_macro_output_unparse,
);
criterion_main!(benches);
