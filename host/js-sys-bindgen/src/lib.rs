mod closure;
mod export;
mod function;
mod hygiene;
mod r#macro;
#[cfg(test)]
mod tests;
mod r#type;
#[cfg(feature = "web-idl")]
mod web_idl;

pub use syn;

pub use crate::closure::closure;
pub use crate::r#macro::r#macro;
#[cfg(feature = "web-idl")]
pub use crate::web_idl::web_idl;
