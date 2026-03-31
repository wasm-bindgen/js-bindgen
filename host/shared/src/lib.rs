#[cfg(feature = "memmap")]
mod memmap;
#[cfg(feature = "web-driver")]
mod web_driver;

#[cfg(feature = "memmap")]
pub use crate::memmap::{ReadFile, mtime};
#[cfg(feature = "web-driver")]
pub use crate::web_driver::{AtomicFlag, WebDriver, WebDriverKind, WebDriverLocation};
