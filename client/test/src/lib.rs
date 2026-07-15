#[cfg(js_bindgen_cov)]
js_bindgen_cov::ensure_linked!();

#[cfg(all(target_family = "wasm", any(target_os = "none", target_os = "unknown")))]
mod unknown;

#[cfg(not(all(target_family = "wasm", any(target_os = "none", target_os = "unknown"))))]
pub use test;

#[cfg(all(target_family = "wasm", any(target_os = "none", target_os = "unknown")))]
pub use self::unknown::*;
