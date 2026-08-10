mod address;
mod error;
mod exception;
mod global;
mod instance;
mod jspi;
mod memory;
mod module;
mod namespace;
mod table;

#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod WebAssembly {
	pub use super::address::AddressType;
	pub use super::error::{CompileError, LinkError, RuntimeError, SuspendError};
	pub use super::exception::{Exception, ExceptionOptions, Tag, TagDescriptor, TagType, js_tag};
	pub use super::global::{Global, GlobalDescriptor, ValueType};
	pub use super::instance::Instance;
	pub use super::jspi::{Suspending, promising};
	pub use super::memory::{Memory, MemoryDescriptor};
	pub use super::module::{
		ImportExportKind, Module, ModuleExportDescriptor, ModuleImportDescriptor,
	};
	pub use super::namespace::{
		CompileBuiltin, CompileOptions, InstantiatedSource, compile, compile_streaming,
		compile_streaming_with_options, compile_with_options, instantiate_bytes,
		instantiate_bytes_with_imports, instantiate_bytes_with_imports_and_options,
		instantiate_module, instantiate_module_with_imports, instantiate_streaming,
		instantiate_streaming_with_imports, instantiate_streaming_with_imports_and_options,
		validate, validate_with_options,
	};
	pub use super::table::{Table, TableDescriptor, TableElement};
}
