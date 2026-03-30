mod instant;
mod system_time;

pub use instant::Instant;
use js_sys::{js_bindgen, js_sys};
pub use system_time::SystemTime;
pub const UNIX_EPOCH: SystemTime = SystemTime::UNIX_EPOCH;

#[js_sys]
extern "js-sys" {
	pub type Performance;

	#[js_sys(js_embed = "performance")]
	pub fn performance() -> Performance;

	#[js_sys(js_name = "now")]
	pub fn now(self: &Performance) -> f64;

	#[cfg(target_feature = "atomics")]
	#[js_sys(property, js_name = "timeOrigin")]
	pub fn time_origin(self: &Performance) -> f64;
}

#[js_sys(namespace = "Date")]
extern "js-sys" {
	#[js_sys(js_name = "now")]
	pub fn date_now() -> f64;
}

js_bindgen::embed_js!(
	module = "js_bindgen_test",
	name = "performance",
	"() => {{
        return globalThis.performance
    }}"
);
