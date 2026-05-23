#[cfg(feature = "memmap")]
mod memmap;
#[cfg(feature = "web-driver")]
mod web_driver;

pub const IS_TEST_SECTION: &str = "js_bindgen.is_test";
pub const IS_COMPAT_SECTION: &str = "js_bindgen.is_compat";

#[cfg(feature = "memmap")]
pub use crate::memmap::{ReadFile, mtime};
#[cfg(feature = "web-driver")]
pub use crate::web_driver::{AtomicFlag, WebDriver, WebDriverKind, WebDriverLocation};

#[cfg(not(any(feature = "memmap", feature = "web-driver")))]
compile_error!("pick at least one crate feature");
