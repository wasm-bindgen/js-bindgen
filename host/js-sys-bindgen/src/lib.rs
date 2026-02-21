#[cfg(feature = "file")]
mod file;
mod function;
mod hygiene;
#[cfg(feature = "macro")]
mod r#macro;
#[cfg(test)]
mod tests;
mod r#type;
#[cfg(feature = "web-idl")]
mod web_idl;

pub use {proc_macro2, quote, syn};

#[cfg(feature = "file")]
pub use crate::file::file;
pub use crate::function::{Function, FunctionJsOutput};
pub use crate::hygiene::{Hygiene, ImportManager};
#[cfg(feature = "macro")]
pub use crate::r#macro::r#macro;
pub use crate::r#type::Type;
#[cfg(feature = "web-idl")]
pub use crate::web_idl::web_idl;
