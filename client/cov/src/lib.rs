#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use js_sys::{JsArray, JsString, js_sys};

#[js_sys]
extern "js-sys" {
	#[js_sys(js_import)]
	fn pid() -> u32;
	#[js_sys(js_import)]
	fn tmpdir() -> JsString;
	#[js_sys(js_import)]
	fn llvm_profile_file() -> JsString;
	#[js_sys(js_import)]
	fn next_profraw() -> JsArray<u8>;
	#[js_sys(js_import)]
	fn record_profraw(path: &JsString, buf: &JsArray<u8>);
}

#[unsafe(no_mangle)]
extern "C" fn set_profraw() {
	if !minicov::coverage_enabled() {
		return;
	}

	merge_profraw();

	let mut cov = Vec::new();
	// SAFETY: this function is not thread-safe, but our whole test runner is
	// running single-threaded.
	unsafe {
		minicov::capture_coverage(&mut cov).unwrap();
	}

	let env = llvm_profile_file().to_string();
	let path = coverage_path(
		(!env.is_empty()).then_some(env).as_deref(),
		pid(),
		&tmpdir().to_string(),
		minicov::module_signature(),
	);

	record_profraw(
		&JsString::from(path.as_str()),
		&JsArray::<u8>::from(cov.as_slice()),
	);
}

fn merge_profraw() {
	let profraw = next_profraw();

	if profraw.length() == 0 {
		return;
	}

	let mut data = vec![0; profraw.length() as usize];

	if profraw.to_slice(&mut data).is_ok() {
		// SAFETY: this function is not thread-safe, but our whole test runner is
		// running single-threaded.
		unsafe {
			let _ = minicov::merge_coverage(&data);
		}
	}
}

fn coverage_path(env: Option<&str>, pid: u32, tmpdir: &str, module_signature: u64) -> String {
	let env = env.unwrap_or("default_%m_%p.profraw");

	let mut path = String::new();
	let mut chars = env.chars().enumerate().peekable();

	while let Some((index, char)) = chars.next() {
		if char != '%' {
			path.push(char);
			continue;
		}

		if chars.next_if(|(_, c)| *c == 'p').is_some() {
			path.push_str(&pid.to_string());
		} else if chars.next_if(|(_, c)| *c == 'h').is_some() {
			path.push_str("jbgcov");
		} else if chars.next_if(|(_, c)| *c == 't').is_some() {
			path.push_str(tmpdir);
		} else {
			let mut last_index = index;

			loop {
				if let Some((index, _)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
					last_index = index;
				} else if chars.next_if(|(_, c)| *c == 'm').is_some() {
					path.push_str(&module_signature.to_string());
					path.push_str("_0");
					break;
				} else {
					path.push_str(&env[index..=last_index]);
					break;
				}
			}
		}
	}

	path
}

#[macro_export]
macro_rules! ensure_linked {
	() => {};
}
