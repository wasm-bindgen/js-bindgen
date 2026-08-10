mod function;
mod hygiene;
mod r#macro;
#[cfg(test)]
mod tests;
mod r#type;
#[cfg(feature = "web-idl")]
mod web_idl;

pub use proc_macro2;
pub use quote;
pub use syn;

pub use crate::function::{Function, FunctionJsOutput};
pub use crate::hygiene::{Hygiene, ImportManager};
pub use crate::r#macro::r#macro;
pub use crate::r#type::Type;
#[cfg(feature = "web-idl")]
pub use crate::web_idl::web_idl;
