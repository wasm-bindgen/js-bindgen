use alloc::string::String;

use super::namespace::CompileOptions;
use crate::{Array, ArrayBuffer, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/exports_static)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImportExportKind {
	Function,
	Table,
	Memory,
	Global,
	Tag,
}

impl ImportExportKind {
	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"function" => Some(Self::Function),
			"table" => Some(Self::Table),
			"memory" => Some(Self::Memory),
			"global" => Some(Self::Global),
			"tag" => Some(Self::Tag),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/exports_static)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ModuleExportDescriptor;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/imports_static)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ModuleImportDescriptor;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/exports_static)
	#[must_use]
	#[js_sys(getter)]
	pub fn name(self: &ModuleExportDescriptor) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/imports_static)
	#[must_use]
	#[js_sys(getter)]
	pub fn module(self: &ModuleImportDescriptor) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/imports_static)
	#[must_use]
	#[js_sys(getter)]
	pub fn name(self: &ModuleImportDescriptor) -> JsString;
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Module;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/Module)
	#[js_sys(constructor)]
	pub fn new(bytes: &JsValue) -> Result<Module, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/Module)
	#[js_sys(constructor)]
	pub fn new_with_options(bytes: &JsValue, options: &CompileOptions) -> Result<Module, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/customSections_static)
	#[js_sys(static_of = Module, js_name = "customSections")]
	pub fn custom_sections(
		module: &Module,
		section_name: &str,
	) -> Result<Array<ArrayBuffer>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/exports_static)
	#[js_sys(static_of = Module)]
	pub fn exports(module: &Module) -> Result<Array<ModuleExportDescriptor>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/imports_static)
	#[js_sys(static_of = Module)]
	pub fn imports(module: &Module) -> Result<Array<ModuleImportDescriptor>, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "kind")]
	fn export_kind_raw(self: &ModuleExportDescriptor) -> JsString;

	#[js_sys(getter = "kind")]
	fn import_kind_raw(self: &ModuleImportDescriptor) -> JsString;
}

impl ModuleExportDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/exports_static)
	#[must_use]
	pub fn kind(&self) -> Option<ImportExportKind> {
		ImportExportKind::from_js_string(&self.export_kind_raw())
	}
}

impl ModuleImportDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Module/imports_static)
	#[must_use]
	pub fn kind(&self) -> Option<ImportExportKind> {
		ImportExportKind::from_js_string(&self.import_kind_raw())
	}
}
